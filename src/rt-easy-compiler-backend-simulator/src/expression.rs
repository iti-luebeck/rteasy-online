use crate::{gen_ident, Generate, Result};
use compiler::mir;
use rtprogram::*;

impl Generate<mir::Expression<'_>> for Expression {
    fn generate(expression: mir::Expression<'_>) -> Result<Self> {
        let span = expression.span();
        let kind = match expression {
            mir::Expression::Atom(atom) => ExpressionKind::Atom(Generate::generate(atom)?),
            mir::Expression::BinaryTerm(binary_term) => {
                ExpressionKind::BinaryTerm(Box::new(BinaryTerm {
                    lhs: Generate::generate(binary_term.lhs)?,
                    rhs: Generate::generate(binary_term.rhs)?,
                    operator: binary_term.operator.node,
                    ctx_size: binary_term.ctx_size,
                }))
            }
            mir::Expression::UnaryTerm(unary_term) => {
                ExpressionKind::UnaryTerm(Box::new(UnaryTerm {
                    expression: Generate::generate(unary_term.expression)?,
                    operator: unary_term.operator.node,
                    ctx_size: unary_term.ctx_size,
                }))
            }
        };

        Ok(Expression { kind, span })
    }
}

impl Generate<mir::Atom<'_>> for Atom {
    fn generate(atom: mir::Atom<'_>) -> Result<Self> {
        match atom {
            mir::Atom::Concat(concat) => Ok(Atom::Concat(Generate::generate(concat)?)),
            mir::Atom::Register(reg) => Ok(Atom::Register(Generate::generate(reg)?)),
            mir::Atom::Bus(bus) => Ok(Atom::Bus(Generate::generate(bus)?)),
            mir::Atom::RegisterArray(reg_array) => {
                Ok(Atom::RegisterArray(Generate::generate(reg_array)?))
            }
            mir::Atom::Number(number) => Ok(Atom::Number(number.node)),
        }
    }
}

impl Generate<mir::Register<'_>> for Register {
    fn generate(reg: mir::Register<'_>) -> Result<Self> {
        Ok(Register {
            ident: gen_ident(reg.ident.node),
            range: reg.range.map(|s| s.node),
            kind: reg.kind,
        })
    }
}

impl Generate<mir::Bus<'_>> for Bus {
    fn generate(bus: mir::Bus<'_>) -> Result<Self> {
        Ok(Bus {
            ident: gen_ident(bus.ident.node),
            range: bus.range.map(|s| s.node),
            kind: bus.kind,
        })
    }
}

impl Generate<mir::RegisterArray<'_>> for RegisterArray {
    fn generate(reg_array: mir::RegisterArray<'_>) -> Result<Self> {
        Ok(RegisterArray {
            ident: gen_ident(reg_array.ident.node),
            index: Box::new(Generate::generate(*reg_array.index)?),
            index_ctx_size: reg_array.index_ctx_size,
            range: reg_array.range.map(|s| s.node),
        })
    }
}
