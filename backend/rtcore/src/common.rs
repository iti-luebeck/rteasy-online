use std::cmp::Ordering;
use std::fmt;
use std::ops::Range;
use value::Value;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum OperatorAssociativity {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BinaryOperator {
    Eq,
    Ne,
    Le,
    Lt,
    Ge,
    Gt,
    Add,
    Sub,
    And,
    Nand,
    Or,
    Nor,
    Xor,
}

impl BinaryOperator {
    pub fn precedence(&self) -> u32 {
        use BinaryOperator::*;
        match self {
            Add | Sub => 8,
            Le | Lt | Ge | Gt => 7,
            Eq | Ne => 6,
            Nand => 4,
            And => 3,
            Nor => 2,
            Or => 1,
            Xor => 0,
        }
    }

    pub fn associativity(&self) -> OperatorAssociativity {
        use BinaryOperator::*;
        match self {
            Eq | Ne | Le | Lt | Ge | Gt | Add | Sub | And | Nand | Or | Nor | Xor => {
                OperatorAssociativity::Left
            }
        }
    }
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use BinaryOperator::*;
        match self {
            Eq => write!(f, "="),
            Ne => write!(f, "<>"),
            Le => write!(f, "<="),
            Lt => write!(f, "<"),
            Ge => write!(f, ">="),
            Gt => write!(f, ">"),
            Add => write!(f, "+"),
            Sub => write!(f, "-"),
            And => write!(f, "and"),
            Nand => write!(f, "nand"),
            Or => write!(f, "or"),
            Nor => write!(f, "nor"),
            Xor => write!(f, "xor"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum UnaryOperator {
    Sign,
    Neg,
    Not,
    Sxt,
}

impl UnaryOperator {
    pub fn precedence(&self) -> u32 {
        use UnaryOperator::*;
        match self {
            Sign | Neg => 10,
            Sxt => 9,
            Not => 5,
        }
    }

    pub fn associativity(&self) -> OperatorAssociativity {
        use UnaryOperator::*;
        match self {
            Sign | Neg | Not | Sxt => OperatorAssociativity::Right,
        }
    }
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use UnaryOperator::*;
        match self {
            Sign => write!(f, "-"),
            Neg => write!(f, "neg"),
            Not => write!(f, "not"),
            Sxt => write!(f, "sxt"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Number {
    pub value: Value,
    pub kind: NumberKind,
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            NumberKind::BitString => write!(f, "\"{}\"", self.value.as_bin(true)),
            NumberKind::Binary => write!(f, "0b{}", self.value.as_bin(false)),
            NumberKind::Decimal => write!(f, "{}", self.value.as_dec()),
            NumberKind::Hexadecimal => write!(f, "0x{}", self.value.as_hex()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NumberKind {
    BitString,
    Binary,
    Decimal,
    Hexadecimal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RegisterKind {
    Intern,
    Output,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BusKind {
    Intern,
    Input,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct BitRange(pub usize, pub usize);

impl BitRange {
    pub fn is_downto(&self) -> bool {
        self.0 >= self.1
    }

    pub fn normalize(&self) -> Self {
        if self.is_downto() {
            BitRange(self.0 - self.1, 0)
        } else {
            BitRange(0, self.1 - self.0)
        }
    }

    pub fn size(&self) -> Option<usize> {
        // This overflows if either msb or lsb is usize::MAX and the other is 0. In this case size
        // can not be represented in a usize.
        if self.is_downto() {
            usize::checked_add(self.0 - self.1, 1)
        } else {
            usize::checked_add(self.1 - self.0, 1)
        }
    }

    pub fn contains(&self, pos: usize) -> bool {
        if self.is_downto() {
            pos >= self.1 && pos <= self.0
        } else {
            pos >= self.0 && pos <= self.1
        }
    }

    pub fn contains_range(&self, idx: Self) -> bool {
        let contains_0 = self.contains(idx.0);
        let contains_1 = self.contains(idx.1);
        let is_in_order = (idx.1 == idx.0) || (self.is_downto() == idx.is_downto());
        contains_0 && contains_1 && is_in_order
    }

    pub fn slice_relative(&self, range: Self) -> Option<Self> {
        if self.is_downto() != range.is_downto() {
            return None;
        } else if self.normalize().contains_range(range) {
            // !todo: check if this is correct
            if self.is_downto() {
                Some(Self(self.1 + range.0, self.1 + range.1))
            } else {
                Some(Self(self.0 + range.0, self.0 + range.1))
            }
        } else {
            None
        }
    }

    pub fn bits(&self) -> impl Iterator<Item = usize> {
        let start = self.1;
        let size = self.size().unwrap();
        let is_downto = self.is_downto();

        (0..size).into_iter().map(move |idx| if is_downto { start + idx } else { start - idx })
    }

    pub fn intersect(a: Option<Self>, b: Option<Self>) -> bool {
        match (a, b) {
            (None, _) | (_, None) => true,
            (Some(a), Some(b)) => {
                a.contains(b.0) || a.contains(b.1) || b.contains(a.0) || b.contains(a.1)
            }
        }
    }
}

impl fmt::Display for BitRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 == self.1 {
            write!(f, "({})", self.0)
        } else {
            write!(f, "({}:{})", self.0, self.1)
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum CtxSize {
    Size(usize),
    Inherit,
}

impl CtxSize {
    pub fn calc(self, parent: usize) -> usize {
        match self {
            Self::Size(size) => size,
            Self::Inherit => parent,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn dummy() -> Self {
        Self { start: 0, end: 0 }
    }

    pub fn range(self) -> Range<usize> {
        self.start..self.end
    }
}

impl PartialOrd for Span {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Span {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.start.cmp(&other.start) {
            Ordering::Equal => self.end.cmp(&other.end),
            ordering => ordering,
        }
    }
}

impl From<Range<usize>> for Span {
    fn from(range: Range<usize>) -> Self {
        Self { start: range.start, end: range.end }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn new(node: T, span: Span) -> Spanned<T> {
        Self { node, span }
    }

    pub fn map<F, U>(self, f: F) -> Spanned<U>
    where
        F: FnOnce(T) -> U,
    {
        Spanned { node: f(self.node), span: self.span }
    }
}
