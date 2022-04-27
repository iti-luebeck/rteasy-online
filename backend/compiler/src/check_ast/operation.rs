use super::expression::CheckExpr;
use crate::{
    symbols::{Symbol, Symbols},
    util, CompilerError, CompilerErrorKind, InternalError, SymbolType,
};
use ast::*;

pub type Result = std::result::Result<Res, InternalError>;

#[derive(Debug, Default)]
pub struct Res {
    /// If the operation(s) contains a goto
    pub contains_goto: bool,
    /// If the operation(s) contains a mutating op
    pub contains_mutate: bool,
}

impl Res {
    fn merge(a: Self, b: Self) -> Self {
        Self {
            contains_goto: a.contains_goto || b.contains_goto,
            contains_mutate: a.contains_mutate || b.contains_mutate,
        }
    }
}

pub trait CheckOp<'s> {
    fn check_op(&self, symbols: &Symbols<'_>, error_sink: &mut impl FnMut(CompilerError))
        -> Result;
}

impl<'s> CheckOp<'s> for [Operation<'s>] {
    fn check_op(
        &self,
        symbols: &Symbols<'_>,
        error_sink: &mut impl FnMut(CompilerError),
    ) -> Result {
        let mut operations = self.iter();
        let mut res = operations
            .next()
            .expect("expected at least one operation")
            .check_op(symbols, error_sink)?;

        for operation in operations {
            res = Res::merge(res, operation.check_op(symbols, error_sink)?);
        }

        Ok(res)
    }
}

impl<'s> CheckOp<'s> for Operation<'s> {
    fn check_op(
        &self,
        symbols: &Symbols<'_>,
        error_sink: &mut impl FnMut(CompilerError),
    ) -> Result {
        match self {
            Operation::Nop(nop) => nop.check_op(symbols, error_sink),
            Operation::Goto(goto) => goto.check_op(symbols, error_sink),
            Operation::If(if_) => if_.check_op(symbols, error_sink),
            Operation::Switch(switch) => switch.check_op(symbols, error_sink),
            Operation::Write(write) => write.check_op(symbols, error_sink),
            Operation::Read(read) => read.check_op(symbols, error_sink),
            Operation::Assignment(assignment) => assignment.check_op(symbols, error_sink),
            Operation::Assert(assert) => assert.check_op(symbols, error_sink),
        }
    }
}

impl<'s> CheckOp<'s> for Nop {
    fn check_op(&self, _: &Symbols<'_>, _: &mut impl FnMut(CompilerError)) -> Result {
        Ok(Res::default())
    }
}

impl<'s> CheckOp<'s> for Goto<'s> {
    fn check_op(
        &self,
        symbols: &Symbols<'_>,
        error_sink: &mut impl FnMut(CompilerError),
    ) -> Result {
        if !symbols.contains_label(self.label.node) {
            if self.label.node.0 == "MOON" {
                return Err(InternalError(format!(
                    "`goto MOON`. The compiler panicked as expected. This is a feature."
                )));
            }

            error_sink(CompilerError::new(
                CompilerErrorKind::LabelNotFound(self.label.node.0.to_string()),
                self.label.span,
            ));
        }

        Ok(Res { contains_goto: true, contains_mutate: false })
    }
}

impl<'s> CheckOp<'s> for If<'s> {
    fn check_op(
        &self,
        symbols: &Symbols<'_>,
        error_sink: &mut impl FnMut(CompilerError),
    ) -> Result {
        if let Some(size) = self.condition.check_expr(symbols, error_sink).size {
            if size > 1 {
                error_sink(CompilerError::new(
                    CompilerErrorKind::ConditionTooWide(size),
                    self.condition.span(),
                ));
            }
        }

        let mut res = self.operations_if.check_op(symbols, error_sink)?;
        if let Some(operations_else) = &self.operations_else {
            res = Res::merge(res, operations_else.check_op(symbols, error_sink)?);
        }

        Ok(res)
    }
}

impl<'s> CheckOp<'s> for Switch<'s> {
    fn check_op(
        &self,
        symbols: &Symbols<'_>,
        error_sink: &mut impl FnMut(CompilerError),
    ) -> Result {
        let expr_res = self.expression.check_expr(symbols, error_sink);
        if !expr_res.fixed_size {
            error_sink(CompilerError::new(
                CompilerErrorKind::ExpectedFixedSize,
                self.expression.span(),
            ));
        }

        let mut res = Res { contains_goto: false, contains_mutate: false };
        let mut default_clauses_count = 0;

        for clause in &self.clauses {
            match &clause.clause {
                Either::Left(case) => {
                    let value_res = case.value.check_expr(symbols, error_sink);

                    if !value_res.constant {
                        error_sink(CompilerError::new(
                            CompilerErrorKind::ExpectedConstantExpression,
                            case.value.span(),
                        ));
                    }

                    match (expr_res.size, value_res.size) {
                        (Some(expr_size), Some(case_value_size)) if case_value_size > expr_size => {
                            error_sink(CompilerError::new(
                                CompilerErrorKind::CaseValueTooWide { expr_size, case_value_size },
                                case.value.span(),
                            ));
                        }
                        _ => (),
                    }

                    res = Res::merge(res, clause.operations.check_op(symbols, error_sink)?);
                }
                Either::Right(_default) => {
                    default_clauses_count += 1;
                    res = Res::merge(res, clause.operations.check_op(symbols, error_sink)?);
                }
            }
        }

        if default_clauses_count != 1 {
            error_sink(CompilerError::new(
                CompilerErrorKind::ExpectedExactlyOneDefaultClause,
                self.span,
            ));
        }

        Ok(res)
    }
}

impl<'s> CheckOp<'s> for Write<'s> {
    fn check_op(
        &self,
        symbols: &Symbols<'_>,
        error_sink: &mut impl FnMut(CompilerError),
    ) -> Result {
        match symbols.symbol(self.ident.node) {
            Some(Symbol::Memory(_)) => (),
            Some(symbol) => error_sink(CompilerError::new(
                CompilerErrorKind::WrongSymbolType {
                    expected: &[SymbolType::Memory],
                    found: symbol.type_(),
                },
                self.ident.span,
            )),
            _ => error_sink(CompilerError::new(
                CompilerErrorKind::SymbolNotFound(
                    &[SymbolType::Memory],
                    self.ident.node.0.to_string(),
                ),
                self.ident.span,
            )),
        }

        Ok(Res { contains_goto: false, contains_mutate: true })
    }
}

impl<'s> CheckOp<'s> for Read<'s> {
    fn check_op(
        &self,
        symbols: &Symbols<'_>,
        error_sink: &mut impl FnMut(CompilerError),
    ) -> Result {
        match symbols.symbol(self.ident.node) {
            Some(Symbol::Memory(_)) => (),
            Some(symbol) => error_sink(CompilerError::new(
                CompilerErrorKind::WrongSymbolType {
                    expected: &[SymbolType::Memory],
                    found: symbol.type_(),
                },
                self.ident.span,
            )),
            _ => error_sink(CompilerError::new(
                CompilerErrorKind::SymbolNotFound(
                    &[SymbolType::Memory],
                    self.ident.node.0.to_string(),
                ),
                self.ident.span,
            )),
        }

        Ok(Res { contains_goto: false, contains_mutate: true })
    }
}

impl<'s> CheckOp<'s> for Assignment<'s> {
    fn check_op(
        &self,
        symbols: &Symbols<'_>,
        error_sink: &mut impl FnMut(CompilerError),
    ) -> Result {
        // Check lhs/rhs as expr and size
        let lhs = match &self.lhs {
            Lvalue::RegBus(reg_bus) => reg_bus.check_expr(symbols, error_sink),
            Lvalue::RegisterArray(reg_array) => reg_array.check_expr(symbols, error_sink),
            Lvalue::Concat(concat) => concat.check_expr(symbols, error_sink),
        };
        let rhs = self.rhs.check_expr(symbols, error_sink);
        if let (Some(lhs), Some(rhs)) = (lhs.size, rhs.size) {
            if lhs < rhs {
                error_sink(CompilerError::new(
                    CompilerErrorKind::AssignmentDoesNotFit { lhs_size: lhs, rhs_size: rhs },
                    self.span,
                ))
            }
        }

        // Check concat
        if let Lvalue::Concat(concat) = &self.lhs {
            let info = util::concat_info(concat, symbols);
            if info.contains_clocked && info.contains_unclocked {
                error_sink(CompilerError::new(
                    CompilerErrorKind::AssignmentLhsContainsClockedAndUnclocked,
                    self.lhs.span(),
                ));
            }
            if info.contains_non_lvalue {
                error_sink(CompilerError::new(
                    CompilerErrorKind::AssignmentLhsContainsANonLvalue,
                    self.lhs.span(),
                ));
            }
        };

        // Check assign to input
        match &self.lhs {
            Lvalue::RegBus(reg_bus) => {
                if reg_bus_is_input(reg_bus, symbols) {
                    error_sink(CompilerError::new(
                        CompilerErrorKind::AssignmentLhsContainsInput,
                        self.lhs.span(),
                    ));
                }
            }
            Lvalue::RegisterArray(_reg_array) => (),
            Lvalue::Concat(concat) => {
                for part in &concat.parts {
                    match part {
                        ConcatPart::RegBus(reg_bus) => {
                            if reg_bus_is_input(reg_bus, symbols) {
                                error_sink(CompilerError::new(
                                    CompilerErrorKind::AssignmentLhsContainsInput,
                                    self.lhs.span(),
                                ));
                            }
                        }
                        ConcatPart::RegisterArray(_) => (),
                        ConcatPart::Number(_number) => (),
                    }
                }
            }
        }

        // Check assign to register array with bit range
        match &self.lhs {
            Lvalue::RegBus(_) => (),
            Lvalue::RegisterArray(reg_array) => {
                if reg_array.range.is_some() {
                    error_sink(CompilerError::new(
                        CompilerErrorKind::AssignmentLhsRegisterArrayWithBitRange,
                        reg_array.span,
                    ));
                }
            }
            Lvalue::Concat(concat) => {
                for part in &concat.parts {
                    match part {
                        ConcatPart::RegisterArray(reg_array) => {
                            if reg_array.range.is_some() {
                                error_sink(CompilerError::new(
                                    CompilerErrorKind::AssignmentLhsRegisterArrayWithBitRange,
                                    reg_array.span,
                                ));
                            }
                        }
                        ConcatPart::RegBus(_) | ConcatPart::Number(_) => (),
                    }
                }
            }
        }

        Ok(Res { contains_goto: false, contains_mutate: true })
    }
}

impl<'s> CheckOp<'s> for Assert<'s> {
    fn check_op(
        &self,
        symbols: &Symbols<'_>,
        error_sink: &mut impl FnMut(CompilerError),
    ) -> Result {
        if let Some(size) = self.condition.check_expr(symbols, error_sink).size {
            if size > 1 {
                error_sink(CompilerError::new(
                    CompilerErrorKind::ConditionTooWide(size),
                    self.condition.span(),
                ));
            }
        }

        Ok(Res { contains_goto: false, contains_mutate: false })
    }
}

impl<'s, L, R> CheckOp<'s> for Either<L, R>
where
    L: CheckOp<'s>,
    R: CheckOp<'s>,
{
    fn check_op(
        &self,
        symbols: &Symbols<'_>,
        error_sink: &mut impl FnMut(CompilerError),
    ) -> Result {
        match self {
            Self::Left(left) => left.check_op(symbols, error_sink),
            Self::Right(right) => right.check_op(symbols, error_sink),
        }
    }
}

fn reg_bus_is_input(reg_bus: &RegBus<'_>, symbols: &Symbols<'_>) -> bool {
    match symbols.symbol(reg_bus.ident.node) {
        Some(Symbol::Bus(_, BusKind::Input)) => true,
        _ => false,
    }
}
