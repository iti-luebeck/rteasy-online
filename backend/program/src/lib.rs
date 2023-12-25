#![deny(rust_2018_idioms)]

mod concat;
mod declaration;
mod expression;
mod impl_display;
mod operation;

pub use self::{concat::*, declaration::*, expression::*, operation::*};
pub use rtcore::common::*;
pub use split_vec::SplitVec;

#[derive(Debug)]
pub struct Program {
    declarations: Vec<Declaration>,
    statements: Vec<Statement>,
}

impl Program {
    pub fn new_unchecked(declarations: Vec<Declaration>, statements: Vec<Statement>) -> Self {
        Self { declarations, statements }
    }

    pub fn declarations(&self) -> &[Declaration] {
        &self.declarations
    }

    pub fn statements(&self) -> &[Statement] {
        &self.statements
    }
}

#[derive(Debug)]
pub struct Statement {
    pub label: Option<Label>,
    pub steps: Spanned<SplitVec<Step>>,
    pub span: Span,
    pub span_semicolon: Span,
    pub span_pipe: Option<Span>,
}

#[derive(Debug)]
pub struct Step {
    pub criteria: Vec<Criterion>,
    pub operation: Operation,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CriterionId(pub usize);

#[derive(Debug, Clone, Copy)]
pub enum Criterion {
    True(CriterionId),
    False(CriterionId),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ident(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Label(pub String);
