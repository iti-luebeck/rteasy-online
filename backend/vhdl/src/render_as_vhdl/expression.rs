use super::RenderAsVhdl;
use crate::*;
use std::fmt::{Display, Formatter, Result};

impl Display for RenderAsVhdl<&Expression> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.0.extend_to {
            Extend::Zero(size) => {
                write!(f, "zero_extend({}, {})", RenderAsVhdl(&self.0.kind), size)
            }
            Extend::Sign(size) => {
                write!(f, "sign_extend({}, {})", RenderAsVhdl(&self.0.kind), size)
            }
        }
    }
}

impl Display for RenderAsVhdl<&ExpressionKind> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.0 {
            ExpressionKind::Atom(atom) => write!(f, "{}", RenderAsVhdl(atom)),
            ExpressionKind::BinaryTerm(term) => write!(f, "{}", RenderAsVhdl(&**term)),
            ExpressionKind::UnaryTerm(term) => write!(f, "{}", RenderAsVhdl(&**term)),
        }
    }
}

impl Display for RenderAsVhdl<&Atom> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.0 {
            Atom::Concat(concat) => write!(f, "{}", RenderAsVhdl(concat)),
            Atom::Register(register) => write!(f, "{}", RenderAsVhdl(register)),
            Atom::Bus(bus) => write!(f, "{}", RenderAsVhdl(bus)),
            Atom::RegisterArray(reg_array) => write!(f, "{}", RenderAsVhdl(reg_array)),
            Atom::Number(number) => write!(f, "{}", RenderAsVhdl(number)),
        }
    }
}

impl Display for RenderAsVhdl<&BinaryTerm> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}({}, {})",
            binary_operator(self.0.operator),
            RenderAsVhdl(&self.0.lhs),
            RenderAsVhdl(&self.0.rhs),
        )?;

        Ok(())
    }
}

impl Display for RenderAsVhdl<&UnaryTerm> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}({})", unary_operator(self.0.operator), RenderAsVhdl(&self.0.expression))?;

        Ok(())
    }
}

impl Display for RenderAsVhdl<&Register> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "register_{}{}", self.0.ident, RenderAsVhdl(self.0.range))
    }
}

impl Display for RenderAsVhdl<&Bus> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let prefix = match self.0.kind {
            BusKind::Intern => "bus",
            BusKind::Input => "input",
        };

        write!(f, "{}_{}{}", prefix, self.0.ident, RenderAsVhdl(self.0.range))
    }
}

impl Display for RenderAsVhdl<&RegisterArray> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "register_array_{}(to_integer({})){}",
            self.0.ident,
            RenderAsVhdl(&*self.0.index),
            RenderAsVhdl(self.0.range),
        )
    }
}

impl Display for RenderAsVhdl<&Number> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "\"{}\"", self.0.value.as_bin(true))
    }
}

fn binary_operator(op: BinaryOperator) -> &'static str {
    match op {
        BinaryOperator::Eq => "f_eq",
        BinaryOperator::Ne => "f_ne",
        BinaryOperator::Le => "f_le",
        BinaryOperator::Lt => "f_lt",
        BinaryOperator::Ge => "f_ge",
        BinaryOperator::Gt => "f_gt",
        BinaryOperator::Add => "f_add",
        BinaryOperator::Sub => "f_sub",
        BinaryOperator::And => "f_and",
        BinaryOperator::Nand => "f_nand",
        BinaryOperator::Or => "f_or",
        BinaryOperator::Nor => "f_nor",
        BinaryOperator::Xor => "f_xor",
    }
}

fn unary_operator(op: UnaryOperator) -> &'static str {
    match op {
        UnaryOperator::Sign | UnaryOperator::Neg => "f_neg",
        UnaryOperator::Not => "f_not",
        UnaryOperator::Sxt => "f_sxt",
    }
}
