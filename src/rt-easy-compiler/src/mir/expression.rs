use super::*;

#[derive(Debug)]
pub enum Expression<'s> {
    Atom(Atom<'s>),
    BinaryTerm(Box<BinaryTerm<'s>>),
    UnaryTerm(Box<UnaryTerm<'s>>),
}

#[derive(Debug)]
pub enum Atom<'s> {
    Concat(ConcatExpr<'s>),
    Register(Register<'s>),
    Bus(Bus<'s>),
    RegisterArray(RegisterArray<'s>),
    Number(Number),
}

#[derive(Debug)]
pub struct BinaryTerm<'s> {
    pub lhs: Expression<'s>,
    pub rhs: Expression<'s>,
    pub operator: BinaryOperator,
    pub ctx_size: CtxSize,
}

#[derive(Debug)]
pub struct UnaryTerm<'s> {
    pub expression: Expression<'s>,
    pub operator: UnaryOperator,
    pub ctx_size: CtxSize,
}

#[derive(Debug, Clone)]
pub struct Register<'s> {
    pub ident: Ident<'s>,
    pub range: Option<BitRange>,
}

#[derive(Debug)]
pub struct Bus<'s> {
    pub ident: Ident<'s>,
    pub range: Option<BitRange>,
}

#[derive(Debug)]
pub struct RegisterArray<'s> {
    pub ident: Ident<'s>,
    pub index: Box<Expression<'s>>,
    pub index_ctx_size: usize,
}
