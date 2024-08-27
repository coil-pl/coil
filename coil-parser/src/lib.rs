use coil_lexer::Operator;

#[derive(Debug, Clone, Hash)]
pub enum ArgName {
    Unnamed(Box<str>), // _ inner: T
    Single(Box<str>), // argname: T
    Assigned { outer: Box<str>, inner: Box<str> }, // outer inner: T
}

impl PartialEq for ArgName {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Unnamed(_) => false,
            Self::Single(x) => match other {
                Self::Unnamed(_) => false,
                Self::Single(y) => x == y,
                Self::Assigned { outer: y, .. } => x == y,
            },
            Self::Assigned { outer: x, .. } => match other {
                Self::Unnamed(_) => false,
                Self::Single(y) => x == y,
                Self::Assigned { outer: y, .. } => x == y,
            },
        }
    }
}

#[derive(Debug, Clone, Hash)]
pub struct Signature {
    returns: Box<Expr>,
    named_args: Vec<(ArgName, Expr)>,
}

#[derive(Debug, Clone, Hash)]
pub enum Statement {
    Module {
        name: Box<Expr>,
    },
    Use {
        name: Box<Expr>,
    },
    Fn {
        name: Box<str>,
        signature: Signature,
    },
}

pub enum BinaryOperator {
    Dot,
    Comma,
    Range,
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Mul,
    MulAssign,
    Div,
    DivAssign,
    Mod,
    ModAssign,
    Eq,
    NotEq,
    Greater,
    GreaterEq,
    Lesser,
    LesserEq,
    And,
    AndAssign,
    Or,
    OrAssign,
    BitNot,
    BitAnd,
    BitAndAssign,
    BitOr,
    BitOrAssign,
    BitXor,
    BitXorAssign,
    BitShiftLeft,
    BitShiftLeftAssign,
    BitShiftRight,
    BitShiftRightAssign,
    Assign,
}

pub enum UnaryOperator {
    Try,
    Not,
    DoubleReference,
    Reference,
    Dereference,
    Positive,
    Negative,
}

#[derive(Debug, Clone, Hash)]
pub enum Expr {
    Statement(Statement),
    Binary {
        op: Operator,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Unary {
        op: Operator,
        expr: Box<Expr>,
    },
}

#[cfg(test)]
mod tests;
