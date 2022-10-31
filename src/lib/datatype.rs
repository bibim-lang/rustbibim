use std::fmt;

use num_bigint::BigUint;

#[derive(Debug, Clone)]
pub struct Bowl {
    pub is_mem: bool,
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
    BowlAceessFuncExpr(Box<BowlAccess>),
    AssignFuncExpr(Box<Expr>, Box<Expr>),
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
            Expr::BowlAceessFuncExpr(bowl_access) => write!(f, "({})", bowl_access),
            Expr::AssignFuncExpr(bowl_access, expr) => write!(f, "({})=({})", bowl_access, expr),
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
    Number(Number),
    Bowl(Bowl),
    BowlAccess(Box<BowlAccess>),
    Null,
}
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(number) => write!(f, "{}", number),
            Value::Bowl(bowl) => write!(f, "{}", bowl),
            Value::Null => write!(f, "NULL"),
            Value::BowlAccess(bowl_access) => write!(f, "{}", bowl_access),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Number {
    pub numerator: BigUint,
    pub denominator: BigUint,
}

fn gcd(a: BigUint, b: BigUint) -> BigUint {
    let mut a = a;
    let mut b = b;
    while b != BigUint::from(0u32) {
        (a, b) = (b.clone(), a % b);
    }
    a
}

impl Number {
    pub fn new(numerator: BigUint, denominator: BigUint) -> Number {
        if denominator == BigUint::from(0u32) {
            panic!("denominator is zero");
        }
        if denominator == BigUint::from(1u32) {
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
        Number::new(BigUint::from(1u32), BigUint::from(1u32))
    }

    pub fn zero() -> Number {
        Number::new(BigUint::from(0u32), BigUint::from(1u32))
    }

    pub fn neg(&self) -> Number {
        Number::new(
            BigUint::from(0u32) - self.numerator.clone(),
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
        self.numerator != BigUint::from(0u32)
    }

    pub fn and(&self, other: &Number) -> bool {
        self.bool() && other.bool()
    }

    pub fn or(&self, other: &Number) -> bool {
        self.bool() || other.bool()
    }

    pub fn lt(&self, other: &Number) -> bool {
        self.numerator.clone() * other.denominator.clone() < other.numerator.clone() * self.denominator.clone()
    }

    pub fn gt(&self, other: &Number) -> bool {
        self.numerator.clone() * other.denominator.clone() > other.numerator.clone() * self.denominator.clone()
    }

    pub fn eq(&self, other: &Number) -> bool {
        self.numerator.clone() == other.numerator.clone() && self.denominator.clone() == other.denominator.clone()
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
