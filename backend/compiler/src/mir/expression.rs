use super::*;

#[derive(Debug, Clone)]
pub enum Expression<'s> {
    Atom(Atom<'s>),
    BinaryTerm(Box<BinaryTerm<'s>>),
    UnaryTerm(Box<UnaryTerm<'s>>),
}

impl Expression<'_> {
    pub fn precedence(&self) -> u32 {
        match self {
            Self::Atom(_) => u32::MAX,
            Self::BinaryTerm(binary) => binary.operator.precedence(),
            Self::UnaryTerm(unary) => unary.operator.precedence(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Atom<'s> {
    Concat(ConcatExpr<'s>),
    Register(Register<'s>),
    Bus(Bus<'s>),
    RegisterArray(RegisterArray<'s>),
    Number(Number),
}

#[derive(Debug, Clone)]
pub struct BinaryTerm<'s> {
    pub lhs: Expression<'s>,
    pub rhs: Expression<'s>,
    pub operator: BinaryOperator,
    pub ctx_size: CtxSize,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct UnaryTerm<'s> {
    pub expression: Expression<'s>,
    pub operator: UnaryOperator,
    pub ctx_size: CtxSize,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Register<'s> {
    pub ident: Ident<'s>,
    pub range: Option<BitRange>,
    pub kind: RegisterKind,
}

#[derive(Debug, Clone)]
pub struct Bus<'s> {
    pub ident: Ident<'s>,
    pub range: Option<BitRange>,
    pub kind: BusKind,
}

#[derive(Debug, Clone)]
pub struct RegisterArray<'s> {
    pub ident: Ident<'s>,
    pub index: Box<Expression<'s>>,
    pub index_ctx_size: usize,
    pub range: Option<BitRange>,
}
