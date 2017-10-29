use linked_hash_map::LinkedHashMap;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum UnaryOp {
    Neg,
    Not,
    BitNot,
    PreIncr,
    PostIncr,
    PreDecr,
    PostDecr,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum PathOp {
    Slash,
    Dot,
    Colon,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum BinaryOp {
    Pow,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Less,
    Greater,
    LessEq,
    GreaterEq,
    LShift,
    RShift,
    Eq,
    NotEq,
    BitAnd,
    BitXor,
    BitOr,
    And,
    Or,
}

impl BinaryOp {
    pub fn assignop(self) -> Option<AssignOp> {
        None  // TODO
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum AssignOp {
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    BitAndAssign,
    BitOrAssign,
    BitXorAssign,
    LShiftAssign,
    RShiftAssign,
}

impl AssignOp {
    pub fn binop(self) -> Option<BinaryOp> {
        None  // TODO
    }
}

pub type TypePath = Vec<(PathOp, String)>;

#[derive(Clone, PartialEq, Debug)]
pub struct Prefab<E=Expression> {
    pub path: TypePath,
    pub vars: LinkedHashMap<String, E>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum NewType<E=Expression> {
    Implicit,
    Ident(String),
    Prefab(Prefab<E>),
}

#[derive(Clone, PartialEq, Debug)]
pub enum Expression {
    /// An expression containing a term directly. The term is evaluated first,
    /// then its follows, then its unary operators in reverse order.
    Base {
        /// The unary operations applied to this value, in reverse order.
        unary: Vec<UnaryOp>,
        /// The term of the expression.
        term: Term,
        /// The follow operations applied to this value.
        follow: Vec<Follow>,
    },
    /// A binary operation.
    BinaryOp {
        /// The binary operation.
        op: BinaryOp,
        /// The left-hand side of the operation.
        lhs: Box<Expression>,
        /// The right-hand side of the operation.
        rhs: Box<Expression>,
    },
    /// An assignment operation.
    AssignOp {
        /// The assignment operation.
        op: AssignOp,
        /// The left-hand side of the assignment.
        lhs: Box<Expression>,
        /// The right-hand side of the assignment.
        rhs: Box<Expression>,
    },
}

impl From<Term> for Expression {
    fn from(term: Term) -> Expression {
        Expression::Base {
            unary: vec![],
            follow: vec![],
            term,
        }
    }
}

/// The structure of a term, the basic building block of the AST.
#[derive(Clone, PartialEq, Debug)]
pub enum Term {
    /// The literal `null`.
    Null,
    /// A `new` call.
    New {
        type_: NewType,
        args: Vec<Expression>,
    },
    /// A `list` call.
    List(Vec<(Expression, Option<Expression>)>),
    /// An unscoped function call.
    Call(String, Vec<Expression>),
    /// A prefab literal (path + vars).
    Prefab(Prefab),
    /// An identifier.
    Ident(String),
    /// A string literal.
    String(String),
    /// A resource literal.
    Resource(String),
    /// An integer literal.
    Int(i32),
    /// A floating-point literal.
    Float(f32),
    /// An expression contained in a term.
    Expr(Box<Expression>),
}

impl From<Expression> for Term {
    fn from(expr: Expression) -> Term {
        match expr {
            Expression::Base { term, unary, follow } => if unary.is_empty() && follow.is_empty() {
                match term {
                    Term::Expr(expr) => Term::from(*expr),
                    other => other,
                }
            } else {
                Term::Expr(Box::new(Expression::Base { term, unary, follow }))
            },
            other => Term::Expr(Box::new(other))
        }
    }
}

/// A "follow", an expression part which is applied to a term or another follow.
#[derive(Debug, Clone, PartialEq)]
pub enum Follow {
    /// Access a field of the value.
    Field(String),
    /// Index the value by an expression.
    Index(Box<Expression>),
    /// Call a method of the value.
    Call(String, Vec<Expression>),
}