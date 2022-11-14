use std::{cell::RefCell, fmt, rc::Rc};

use num_bigint::BigInt;

#[derive(Debug, Clone)]
pub struct Bowl {
    pub noodles: Vec<Noodle>,
}

impl fmt::Display for Bowl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut noodles = String::new();
        for noodle in &self.noodles {
            noodles.push_str(&format!("{}", noodle));
        }
        write!(f, "{{{}}}", noodles)
    }
}

#[derive(Debug, Clone)]
pub struct Noodle {
    pub nn_expr: Expr,
    pub expr: Expr,
}

impl fmt::Display for Noodle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}; {}]", self.nn_expr, self.expr)
    }
}

#[derive(Debug, Clone)]
pub struct BowlAccess {
    pub bowl_expr: Expr,
    pub access_expr: Expr,
}

impl fmt::Display for BowlAccess {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}):({})", self.bowl_expr, self.access_expr)
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    ValueExpr(Value),
    BowlReadExpr(Box<Expr>, Box<Expr>),
    MemReadExpr(Box<Expr>),
    BowlWriteExpr(Box<Expr>, Box<Expr>, Box<Expr>),
    MemWriteExpr(Box<Expr>, Box<Expr>),
    DenoFuncExpr(Box<Expr>),
    PlusFuncExpr(Box<Expr>, Box<Expr>),
    MinusFuncExpr(Box<Expr>, Box<Expr>),
    MulFuncExpr(Box<Expr>, Box<Expr>),
    NumberSepFuncExpr(Box<Expr>, Box<Expr>),
    AndFuncExpr(Box<Expr>, Box<Expr>),
    OrFuncExpr(Box<Expr>, Box<Expr>),
    NotFuncExpr(Box<Expr>),
    EqFuncExpr(Box<Expr>, Box<Expr>),
    GtFuncExpr(Box<Expr>, Box<Expr>),
    LtFuncExpr(Box<Expr>, Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::ValueExpr(value) => write!(f, "({})", value),
            Expr::BowlReadExpr(bowl_expr, nn_expr) => write!(f, "(({}):({}))", bowl_expr, nn_expr),
            Expr::MemReadExpr(nn_expr) => write!(f, "(@:({}))", nn_expr),
            Expr::BowlWriteExpr(bowl_expr, nn_expr, value_expr) => {
                write!(f, "(({}):({}) = ({}))", bowl_expr, nn_expr, value_expr)
            }
            Expr::MemWriteExpr(nn_expr, value_expr) => {
                write!(f, "(@:({}) = ({}))", nn_expr, value_expr)
            }
            Expr::DenoFuncExpr(expr) => write!(f, "^({})", expr),
            Expr::PlusFuncExpr(expr1, expr2) => write!(f, "({})+({})", expr1, expr2),
            Expr::MinusFuncExpr(expr1, expr2) => write!(f, "({})-({})", expr1, expr2),
            Expr::MulFuncExpr(expr1, expr2) => write!(f, "({})*({})", expr1, expr2),
            Expr::NumberSepFuncExpr(expr1, expr2) => write!(f, "({})/({})", expr1, expr2),
            Expr::AndFuncExpr(expr1, expr2) => write!(f, "({})&({})", expr1, expr2),
            Expr::OrFuncExpr(expr1, expr2) => write!(f, "({})|({})", expr1, expr2),
            Expr::NotFuncExpr(expr) => write!(f, "!({})", expr),
            Expr::EqFuncExpr(expr1, expr2) => write!(f, "({})?=({})", expr1, expr2),
            Expr::GtFuncExpr(expr1, expr2) => write!(f, "({})>({})", expr1, expr2),
            Expr::LtFuncExpr(expr1, expr2) => write!(f, "({})<({})", expr1, expr2),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(Box<Number>),
    Bowl(Rc<RefCell<Bowl>>),
    Null,
}

impl Value {
    pub fn from_number(number: &Number) -> Value {
        Value::Number(Box::new(number.clone()))
    }

    pub fn from_big_uint(numerator: &BigInt, denominator: &BigInt) -> Value {
        Value::from_number(&Number::new(numerator.clone(), denominator.clone()))
    }

    pub fn from_bowl(bowl: Bowl) -> Value {
        Value::Bowl(Rc::new(RefCell::new(bowl)))
    }

    pub fn new_one() -> Value {
        Value::from_number(&Number::one())
    }

    pub fn new_zero() -> Value {
        Value::from_number(&Number::zero())
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(number) => write!(f, "{}", number),
            Value::Bowl(bowl) => write!(f, "{}", bowl.borrow()),
            Value::Null => write!(f, "NULL"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Number {
    pub numerator: BigInt,
    pub denominator: BigInt,
}

fn gcd(a: BigInt, b: BigInt) -> BigInt {
    let mut a = a;
    let mut b = b;
    while b != BigInt::from(0u32) {
        (a, b) = (b.clone(), a % b);
    }
    a
}

impl Number {
    pub fn new(numerator: BigInt, denominator: BigInt) -> Number {
        if denominator == BigInt::from(0u32) {
            panic!("denominator is zero");
        }
        if denominator == BigInt::from(1u32) {
            return Number {
                numerator,
                denominator,
            };
        }
        let g = gcd(numerator.clone(), denominator.clone());
        Number {
            numerator: numerator / g.clone(),
            denominator: denominator / g,
        }
    }

    pub fn one() -> Number {
        Number::new(BigInt::from(1u32), BigInt::from(1u32))
    }

    pub fn zero() -> Number {
        Number::new(BigInt::from(0u32), BigInt::from(1u32))
    }

    pub fn neg(&self) -> Number {
        Number::new(
            BigInt::from(0u32) - self.numerator.clone(),
            self.denominator.clone(),
        )
    }

    pub fn add(&self, other: &Number) -> Number {
        let numerator = self.numerator.clone() * other.denominator.clone()
            + other.numerator.clone() * self.denominator.clone();
        let denominator = self.denominator.clone() * other.denominator.clone();
        Number::new(numerator, denominator)
    }

    pub fn sub(&self, other: &Number) -> Number {
        self.add(&other.neg())
    }

    pub fn mul(&self, other: &Number) -> Number {
        Number::new(
            self.numerator.clone() * other.numerator.clone(),
            self.denominator.clone() * other.denominator.clone(),
        )
    }

    pub fn div(&self, other: &Number) -> Number {
        Number::new(
            self.numerator.clone() * other.denominator.clone(),
            self.denominator.clone() * other.numerator.clone(),
        )
    }

    pub fn bool(&self) -> bool {
        self.numerator != BigInt::from(0u32)
    }

    pub fn and(&self, other: &Number) -> bool {
        self.bool() && other.bool()
    }

    pub fn or(&self, other: &Number) -> bool {
        self.bool() || other.bool()
    }

    pub fn lt(&self, other: &Number) -> bool {
        self.numerator.clone() * other.denominator.clone()
            < other.numerator.clone() * self.denominator.clone()
    }

    pub fn gt(&self, other: &Number) -> bool {
        self.numerator.clone() * other.denominator.clone()
            > other.numerator.clone() * self.denominator.clone()
    }

    pub fn eq(&self, other: &Number) -> bool {
        self.numerator.clone() == other.numerator.clone()
            && self.denominator.clone() == other.denominator.clone()
    }

    pub fn ge(&self, other: &Number) -> bool {
        self.eq(other) || self.gt(other)
    }

    pub fn le(&self, other: &Number) -> bool {
        self.eq(other) || self.lt(other)
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}
