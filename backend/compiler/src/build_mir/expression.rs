use super::Result;
use crate::{mir::*, util};
use crate::{
    symbols::{Symbol, Symbols},
    InternalError,
};
use ast::Either;

pub trait BuildExpr<'s, I>: Sized {
    fn build(item: I, symbols: &Symbols<'s>) -> Result<Expr<Self>>;
}

#[derive(Debug)]
pub struct Expr<T> {
    pub inner: T,
    pub size: usize,
}

impl<'s> BuildExpr<'s, ast::Expression<'s>> for Expression<'s> {
    fn build(item: ast::Expression<'s>, symbols: &Symbols<'s>) -> Result<Expr<Self>> {
        Ok(match item {
            ast::Expression::Atom(atom) => {
                let atom = Atom::build(atom, symbols)?;
                Expr { inner: Expression::Atom(atom.inner), size: atom.size }
            }
            ast::Expression::BinaryTerm(binary_term) => {
                let binary_term = BinaryTerm::build(*binary_term, symbols)?;
                Expr {
                    inner: Expression::BinaryTerm(Box::new(binary_term.inner)),
                    size: binary_term.size,
                }
            }
            ast::Expression::UnaryTerm(unary_term) => {
                let unary_term = UnaryTerm::build(*unary_term, symbols)?;
                Expr {
                    inner: Expression::UnaryTerm(Box::new(unary_term.inner)),
                    size: unary_term.size,
                }
            }
        })
    }
}

impl<'s> BuildExpr<'s, ast::Atom<'s>> for Atom<'s> {
    fn build(item: ast::Atom<'s>, symbols: &Symbols<'s>) -> Result<Expr<Self>> {
        Ok(match item {
            ast::Atom::Concat(concat) => {
                let concat = ConcatExpr::build(concat, symbols)?;
                Expr { inner: Atom::Concat(concat.inner), size: concat.size }
            }
            ast::Atom::RegBusAlias(reg_bus) => {
                let reg_bus = <Either<_, _>>::build(reg_bus, symbols)?;
                match reg_bus.inner {
                    Either::Left(reg) => Expr { inner: Atom::Register(reg), size: reg_bus.size },
                    Either::Right(bus) => Expr { inner: Atom::Bus(bus), size: reg_bus.size },
                }
            }
            ast::Atom::RegisterArray(reg_array) => {
                let reg_array = RegisterArray::build(reg_array, symbols)?;
                Expr { inner: Atom::RegisterArray(reg_array.inner), size: reg_array.size }
            }
            ast::Atom::Number(number) => {
                Expr { size: number.node.value.size(), inner: Atom::Number(number.node) }
            }
        })
    }
}

impl<'s> BuildExpr<'s, ast::BinaryTerm<'s>> for BinaryTerm<'s> {
    fn build(item: ast::BinaryTerm<'s>, symbols: &Symbols<'s>) -> Result<Expr<Self>> {
        let lhs = Expression::build(item.lhs, symbols)?;
        let rhs = Expression::build(item.rhs, symbols)?;

        Ok(Expr {
            inner: BinaryTerm {
                lhs: lhs.inner,
                rhs: rhs.inner,
                operator: item.operator.node,
                ctx_size: util::ctx_size_binary_op(lhs.size, rhs.size, item.operator.node),
                span: item.span,
            },
            size: util::size_binary_op(lhs.size, rhs.size, item.operator.node),
        })
    }
}

impl<'s> BuildExpr<'s, ast::UnaryTerm<'s>> for UnaryTerm<'s> {
    fn build(item: ast::UnaryTerm<'s>, symbols: &Symbols<'s>) -> Result<Expr<Self>> {
        let expr = Expression::build(item.expression, symbols)?;

        Ok(Expr {
            inner: UnaryTerm {
                expression: expr.inner,
                operator: item.operator.node,
                ctx_size: util::ctx_size_unary_op(expr.size, item.operator.node),
                span: item.span,
            },
            size: util::size_unary_op(expr.size, item.operator.node),
        })
    }
}

impl<'s> BuildExpr<'s, ast::RegBusAlias<'s>> for Either<Register<'s>, Bus<'s>> {
    fn build(item: ast::RegBusAlias<'s>, symbols: &Symbols<'s>) -> Result<Expr<Self>> {
        match symbols.symbol(item.ident.node) {
            Some(Symbol::Register(range, kind)) => {
                let size = util::range_into(range, item.range)?
                    .ok_or_else(|| InternalError("expected size".to_string()))?;
                Ok(Expr {
                    inner: Either::Left(Register {
                        ident: item.ident.node,
                        range: item.range.map(|s| s.node),
                        kind,
                    }),
                    size,
                })
            }
            Some(Symbol::Bus(range, kind)) => {
                let size = util::range_into(range, item.range)?
                    .ok_or_else(|| InternalError("expected size".to_string()))?;
                Ok(Expr {
                    inner: Either::Right(Bus {
                        ident: item.ident.node,
                        range: item.range.map(|s| s.node),
                        kind,
                    }),
                    size,
                })
            }
            Some(Symbol::Alias(ref_ident, def_range)) => {
                let used_range = match item.range {
                    Some(range) => def_range.slice_relative(range.node).ok_or_else(|| {
                        InternalError(
                            "this range should always be valid since it was checked earlier"
                                .to_string(),
                        )
                    })?,
                    None => def_range,
                };
                let ref_symbol = symbols.symbol(ref_ident);

                let size =
                    used_range.size().ok_or_else(|| InternalError("expected size".to_string()))?;
                let ref_ident = Ident(ref_ident.0);

                match ref_symbol {
                    Some(Symbol::Bus(_, kind)) => Ok(Expr {
                        inner: Either::Right(Bus {
                            ident: ref_ident,
                            range: Some(used_range),
                            kind,
                        }),
                        size,
                    }),
                    Some(Symbol::Register(_, kind)) => Ok(Expr {
                        inner: Either::Left(Register {
                            ident: ref_ident,
                            range: Some(used_range),
                            kind,
                        }),
                        size,
                    }),
                    _ => Err(InternalError(format!(
                        "alias {} is referencing a non-register/bus symbol {}",
                        item.ident.node.0, ref_ident.0
                    ))),
                }
            }
            _ => Err(InternalError("missing register, bus or alias".to_string())),
        }
    }
}

impl<'s> BuildExpr<'s, ast::RegBusAlias<'s>> for Register<'s> {
    fn build(item: ast::RegBusAlias<'s>, symbols: &Symbols<'s>) -> Result<Expr<Self>> {
        match symbols.symbol(item.ident.node) {
            Some(Symbol::Register(range, kind)) => {
                let size = util::range_into(range, item.range)?
                    .ok_or_else(|| InternalError("expected size".to_string()))?;
                Ok(Expr {
                    inner: Register {
                        ident: item.ident.node,
                        range: item.range.map(|s| s.node),
                        kind,
                    },
                    size,
                })
            }
            _ => Err(InternalError("missing register".to_string())),
        }
    }
}

impl<'s> BuildExpr<'s, ast::RegBusAlias<'s>> for Bus<'s> {
    fn build(item: ast::RegBusAlias<'s>, symbols: &Symbols<'s>) -> Result<Expr<Self>> {
        match symbols.symbol(item.ident.node) {
            Some(Symbol::Bus(range, kind)) => {
                let size = util::range_into(range, item.range)?
                    .ok_or_else(|| InternalError("expected size".to_string()))?;
                Ok(Expr {
                    inner: Bus { ident: item.ident.node, range: item.range.map(|s| s.node), kind },
                    size,
                })
            }
            _ => Err(InternalError("missing bus".to_string())),
        }
    }
}

impl<'s> BuildExpr<'s, ast::RegisterArray<'s>> for RegisterArray<'s> {
    fn build(item: ast::RegisterArray<'s>, symbols: &Symbols<'s>) -> Result<Expr<Self>> {
        match symbols.symbol(item.ident.node) {
            Some(Symbol::RegisterArray { range, len }) => {
                let index = Expression::build(*item.index, symbols)?;
                let size = util::range_into(range, item.range)?
                    .ok_or_else(|| InternalError("expected size".to_string()))?;

                Ok(Expr {
                    inner: RegisterArray {
                        ident: item.ident.node,
                        index: Box::new(index.inner),
                        index_ctx_size: util::log_2(len),
                        range: item.range.map(|s| s.node),
                    },
                    size,
                })
            }
            _ => Err(InternalError("missing register array".to_string())),
        }
    }
}

// -------------------------------- Concat --------------------------------

impl<'s, P> BuildExpr<'s, ast::Concat<'s>> for Concat<P>
where
    P: BuildExpr<'s, ast::ConcatPart<'s>>,
{
    fn build(item: ast::Concat<'s>, symbols: &Symbols<'s>) -> Result<Expr<Self>> {
        let mut parts = Vec::new();
        let mut size = 0;

        for part in item.parts {
            let part = P::build(part, symbols)?;
            size += part.size;
            parts.push(part.inner);
        }

        Ok(Expr { inner: Concat { parts }, size })
    }
}

impl<'s> BuildExpr<'s, ast::ConcatPart<'s>> for ConcatPartLvalueClocked<'s> {
    fn build(item: ast::ConcatPart<'s>, symbols: &Symbols<'s>) -> Result<Expr<Self>> {
        Ok(match item {
            ast::ConcatPart::RegBusAlias(reg_bus) => {
                let register = Register::build(reg_bus, symbols)?;
                Expr {
                    inner: ConcatPartLvalueClocked::Register(register.inner, register.size),
                    size: register.size,
                }
            }
            ast::ConcatPart::RegisterArray(reg_array) => {
                let reg_array = RegisterArray::build(reg_array, symbols)?;
                Expr {
                    inner: ConcatPartLvalueClocked::RegisterArray(reg_array.inner, reg_array.size),
                    size: reg_array.size,
                }
            }
            ast::ConcatPart::Number(_) => {
                return Err(InternalError("unexpected number in lvalue".to_string()))
            }
        })
    }
}

impl<'s> BuildExpr<'s, ast::ConcatPart<'s>> for ConcatPartLvalueUnclocked<'s> {
    fn build(item: ast::ConcatPart<'s>, symbols: &Symbols<'s>) -> Result<Expr<Self>> {
        Ok(match item {
            ast::ConcatPart::RegBusAlias(reg_bus) => {
                let bus = Bus::build(reg_bus, symbols)?;
                Expr { inner: ConcatPartLvalueUnclocked::Bus(bus.inner, bus.size), size: bus.size }
            }
            ast::ConcatPart::RegisterArray(_) => {
                return Err(InternalError(
                    "unexpected register array in unclocked lvalue".to_string(),
                ))
            }
            ast::ConcatPart::Number(_) => {
                return Err(InternalError("unexpected number in lvalue".to_string()))
            }
        })
    }
}

impl<'s> BuildExpr<'s, ast::ConcatPart<'s>> for ConcatPartExpr<'s> {
    fn build(item: ast::ConcatPart<'s>, symbols: &Symbols<'s>) -> Result<Expr<Self>> {
        Ok(match item {
            ast::ConcatPart::RegBusAlias(reg_bus) => {
                let reg_bus = <Either<_, _>>::build(reg_bus, symbols)?;
                match reg_bus.inner {
                    Either::Left(reg) => {
                        Expr { inner: ConcatPartExpr::Register(reg), size: reg_bus.size }
                    }
                    Either::Right(bus) => {
                        Expr { inner: ConcatPartExpr::Bus(bus), size: reg_bus.size }
                    }
                }
            }
            ast::ConcatPart::RegisterArray(reg_array) => {
                let reg_array = RegisterArray::build(reg_array, symbols)?;
                Expr { inner: ConcatPartExpr::RegisterArray(reg_array.inner), size: reg_array.size }
            }
            ast::ConcatPart::Number(number) => {
                Expr { size: number.node.value.size(), inner: ConcatPartExpr::Number(number.node) }
            }
        })
    }
}
