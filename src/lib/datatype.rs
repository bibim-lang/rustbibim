use std::{cell::RefCell, rc::Rc};

use num_bigint::BigUint;

use crate::io::{read, write};

#[derive(Debug, Clone)]
pub enum Bowl {
    DefaultBowl(DefaultBowl),
    MemBowl,
}

#[derive(Debug, Clone)]
pub struct MemBowl {
    pub inner_bowl: DefaultBowl,
    pub cursor: Value,
}

pub type MutMemBowl = Rc<RefCell<MemBowl>>;

#[derive(Debug, Clone)]
pub struct DefaultBowl {
    pub noodles: Vec<Noodle>,
}

pub trait BowlTrait {
    fn find_noodle(&self, noodle_number: Value, mem: MutMemBowl) -> Option<Noodle>;
    fn updated_noodles(&self, noodle_number: Value, value: Value, mem: MutMemBowl) -> Vec<Noodle>;
    fn set_noodles(&mut self, noodles: Vec<Noodle>) -> Value;
}

impl BowlTrait for DefaultBowl {
    fn find_noodle(&self, noodle_number: Value, mem: MutMemBowl) -> Option<Noodle> {
        for noodle in &self.noodles {
            let nn = noodle.nn_expr.clone().eval(Rc::clone(&mem));
            match (nn, noodle_number.clone()) {
                (Value::Number(a), Value::Number(b)) => {
                    if a.eq(b) {
                        return Some(noodle.clone());
                    }
                }
                _ => {}
            }
        }
        None
    }

    fn updated_noodles(&self, noodle_number: Value, value: Value, mem: MutMemBowl) -> Vec<Noodle> {
        let mut new_noodles = self.noodles.clone();
        let updated = match new_noodles.iter_mut().find(|ref noodle| {
            match (
                noodle_number.clone(),
                noodle.nn_expr.clone().eval(Rc::clone(&mem)),
            ) {
                (Value::Number(a), Value::Number(b)) => a.eq(b),
                _ => false,
            }
        }) {
            Some(noodle) => {
                noodle.expr = Expr::ValueExpr(value.clone().eval(Rc::clone(&mem)));
                true
            }
            None => false,
        };
        if !updated {
            new_noodles.push(Noodle {
                nn_expr: Expr::ValueExpr(noodle_number),
                expr: Expr::ValueExpr(value),
            });
        }
        new_noodles
    }

    fn set_noodles(&mut self, noodles: Vec<Noodle>) -> Value {
        self.noodles = noodles;
        Value::Null
    }
}

impl BowlTrait for MemBowl {
    fn find_noodle(&self, noodle_number: Value, mem: MutMemBowl) -> Option<Noodle> {
        if let Value::Number(number) = noodle_number.clone().eval(Rc::clone(&mem)) {
            if number.clone().eq(Number::zero()) {
                return Some(Noodle {
                    nn_expr: Expr::ValueExpr(Value::Number(number)),
                    expr: Expr::ValueExpr(mem.borrow().cursor.clone()),
                });
            } else if number.clone().eq(Number::one()) {
                let data = read();
                return Some(Noodle {
                    nn_expr: Expr::ValueExpr(Value::Number(number)),
                    expr: Expr::ValueExpr(Value::Bowl(Bowl::DefaultBowl(data))),
                });
            }
        }
        self.inner_bowl.find_noodle(noodle_number, Rc::clone(&mem))
    }

    fn updated_noodles(&self, noodle_number: Value, value: Value, mem: MutMemBowl) -> Vec<Noodle> {
        self.inner_bowl.updated_noodles(noodle_number, value, mem)
    }

    fn set_noodles(&mut self, noodles: Vec<Noodle>) -> Value {
        self.inner_bowl.set_noodles(noodles)
    }
}

impl MemBowl {
    fn is_write_assign(&self, noodle_number: Value, mem: MutMemBowl) -> bool {
        if let Value::Number(number) = noodle_number.clone().eval(Rc::clone(&mem)) {
            if number.clone().eq(Number::one()) {
                return true;
            }
        }
        false
    }
}

#[derive(Debug, Clone)]
pub struct Noodle {
    pub nn_expr: Expr,
    pub expr: Expr,
}

#[derive(Debug, Clone)]
pub struct BowlAccess {
    pub bowl_expr: Expr,
    pub access_expr: Expr,
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

impl Expr {
    pub fn eval(self, mem: MutMemBowl) -> Value {
        match self {
            Expr::ValueExpr(value) => value.clone(),
            Expr::BowlAceessFuncExpr(bowl_access) => Value::BowlAccess(bowl_access.clone()),
            Expr::AssignFuncExpr(bowl_access, expr) => {
                let bowl_access = bowl_access.eval(Rc::clone(&mem));
                let expr = expr.eval(Rc::clone(&mem));
                if let Value::BowlAccess(bowl_access) = bowl_access {
                    let bowl_value = bowl_access.bowl_expr.eval(Rc::clone(&mem));
                    let access_value = bowl_access.access_expr.eval(Rc::clone(&mem));
                    if let Value::Bowl(bowl) = bowl_value {
                        match bowl {
                            Bowl::DefaultBowl(default_bowl) => {
                                let new_noodles = default_bowl.updated_noodles(
                                    access_value,
                                    expr,
                                    Rc::clone(&mem),
                                );
                                let mut new_bowl = default_bowl;
                                new_bowl.set_noodles(new_noodles)
                                // todo: it maybe not work with nested bowl
                            }
                            Bowl::MemBowl => {
                                if mem
                                    .borrow()
                                    .is_write_assign(access_value.clone(), Rc::clone(&mem))
                                {
                                    let value = expr.eval(Rc::clone(&mem));
                                    if let Value::Bowl(bowl) = value {
                                        match bowl {
                                            Bowl::DefaultBowl(data) => {
                                                write(data, Rc::clone(&mem));
                                            }
                                            Bowl::MemBowl => {
                                                panic!("can not write Mem(@) bowl data")
                                            }
                                        }
                                        return Value::Null;
                                    } else {
                                        println!("{:?}", value);
                                        panic!("Cannot write non-bowl data");
                                    }
                                }
                                let new_noodles = mem.borrow().updated_noodles(
                                    access_value,
                                    expr,
                                    Rc::clone(&mem),
                                );
                                mem.borrow_mut().set_noodles(new_noodles)
                            }
                        }
                    } else {
                        Value::Null
                    }
                } else {
                    Value::Null
                }
            }
            Expr::DenoFuncExpr(expr) => {
                let expr = expr.eval(Rc::clone(&mem));
                if let Value::Number(number) = expr {
                    Value::Number(Number::new(number.denominator, BigUint::from(1u32)))
                } else {
                    Value::Null
                }
            }
            Expr::PlusFuncExpr(expr1, expr2) => {
                let expr1 = expr1.eval(Rc::clone(&mem));
                let expr2 = expr2.eval(Rc::clone(&mem));
                if let (Value::Number(number1), Value::Number(number2)) = (expr1, expr2) {
                    Value::Number(number1.add(number2))
                } else {
                    Value::Null
                }
            }
            Expr::MinusFuncExpr(expr1, expr2) => {
                let expr1 = expr1.eval(Rc::clone(&mem));
                let expr2 = expr2.eval(Rc::clone(&mem));
                if let (Value::Number(number1), Value::Number(number2)) = (expr1, expr2) {
                    Value::Number(number1.sub(number2))
                } else {
                    Value::Null
                }
            }
            Expr::MulFuncExpr(expr1, expr2) => {
                let expr1 = expr1.eval(Rc::clone(&mem));
                let expr2 = expr2.eval(Rc::clone(&mem));
                if let (Value::Number(number1), Value::Number(number2)) = (expr1, expr2) {
                    Value::Number(number1.mul(number2))
                } else {
                    Value::Null
                }
            }
            Expr::NumberSepFuncExpr(expr1, expr2) => {
                let expr1 = expr1.eval(Rc::clone(&mem));
                let expr2 = expr2.eval(Rc::clone(&mem));
                if let (Value::Number(number1), Value::Number(number2)) = (expr1, expr2) {
                    Value::Number(number1.div(number2))
                } else {
                    Value::Null
                }
            }
            Expr::AndFuncExpr(expr1, expr2) => {
                let expr1 = expr1.eval(Rc::clone(&mem));
                let expr2 = expr2.eval(Rc::clone(&mem));
                if let (Value::Number(number1), Value::Number(number2)) = (expr1, expr2) {
                    if number1.and(number2) {
                        Value::Number(Number::one())
                    } else {
                        Value::Number(Number::zero())
                    }
                } else {
                    Value::Number(Number::zero())
                }
            }
            Expr::OrFuncExpr(expr1, expr2) => {
                let expr1 = expr1.eval(Rc::clone(&mem));
                let expr2 = expr2.eval(Rc::clone(&mem));
                if let (Value::Number(number1), Value::Number(number2)) = (expr1, expr2) {
                    if number1.or(number2) {
                        Value::Number(Number::one())
                    } else {
                        Value::Number(Number::zero())
                    }
                } else {
                    Value::Number(Number::zero())
                }
            }
            Expr::NotFuncExpr(expr) => {
                let expr = expr.eval(Rc::clone(&mem));
                if let Value::Number(number) = expr {
                    if number.eq(Number::one()) {
                        Value::Number(Number::zero())
                    } else {
                        Value::Number(Number::one())
                    }
                } else {
                    Value::Number(Number::one())
                }
            }
            Expr::EqFuncExpr(expr1, expr2) => {
                let expr1 = expr1.eval(Rc::clone(&mem));
                let expr2 = expr2.eval(Rc::clone(&mem));
                if let (Value::Number(number1), Value::Number(number2)) = (expr1, expr2) {
                    if number1.eq(number2) {
                        Value::Number(Number::one())
                    } else {
                        Value::Number(Number::zero())
                    }
                } else {
                    Value::Number(Number::zero())
                }
            }
            Expr::GtFuncExpr(expr1, expr2) => {
                let expr1 = expr1.eval(Rc::clone(&mem));
                let expr2 = expr2.eval(Rc::clone(&mem));
                if let (Value::Number(number1), Value::Number(number2)) = (expr1, expr2) {
                    if number1.gt(number2) {
                        Value::Number(Number::one())
                    } else {
                        Value::Number(Number::zero())
                    }
                } else {
                    Value::Number(Number::zero())
                }
            }
            Expr::LtFuncExpr(expr1, expr2) => {
                let expr1 = expr1.eval(Rc::clone(&mem));
                let expr2 = expr2.eval(Rc::clone(&mem));
                if let (Value::Number(number1), Value::Number(number2)) = (expr1, expr2) {
                    if number1.lt(number2) {
                        Value::Number(Number::one())
                    } else {
                        Value::Number(Number::zero())
                    }
                } else {
                    Value::Number(Number::zero())
                }
            }
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

impl Value {
    fn eval(self, mem: MutMemBowl) -> Value {
        match self {
            Value::Number(_) => self,
            Value::Bowl(_) => self,
            Value::Null => self,
            Value::BowlAccess(bowl_access) => {
                let bowl_value = bowl_access.bowl_expr.eval(Rc::clone(&mem));
                if let Value::Bowl(bowl) = bowl_value {
                    if let Some(noodle) = match bowl {
                        Bowl::DefaultBowl(default_bowl) => default_bowl.find_noodle(
                            bowl_access.access_expr.eval(Rc::clone(&mem)),
                            Rc::clone(&mem),
                        ),
                        Bowl::MemBowl => mem.borrow().find_noodle(
                            bowl_access.access_expr.eval(Rc::clone(&mem)),
                            Rc::clone(&mem),
                        ),
                    } {
                        return noodle.expr.eval(Rc::clone(&mem));
                    }
                }
                Value::Null
            }
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

    pub fn neg(self) -> Number {
        Number::new(BigUint::from(0u32) - self.numerator, self.denominator)
    }

    pub fn add(self, other: Number) -> Number {
        let numerator =
            self.numerator * other.denominator.clone() + other.numerator * self.denominator.clone();
        let denominator = self.denominator * other.denominator;
        Number::new(numerator, denominator)
    }

    pub fn sub(self, other: Number) -> Number {
        self.add(other.neg())
    }

    pub fn mul(self, other: Number) -> Number {
        Number::new(
            self.numerator * other.numerator,
            self.denominator * other.denominator,
        )
    }

    pub fn div(self, other: Number) -> Number {
        Number::new(
            self.numerator * other.denominator,
            self.denominator * other.numerator,
        )
    }

    pub fn bool(self) -> bool {
        self.numerator != BigUint::from(0u32)
    }

    pub fn and(self, other: Number) -> bool {
        self.bool() && other.bool()
    }

    pub fn or(self, other: Number) -> bool {
        self.bool() || other.bool()
    }

    pub fn lt(self, other: Number) -> bool {
        self.numerator * other.denominator < other.numerator * self.denominator
    }

    pub fn gt(self, other: Number) -> bool {
        self.numerator * other.denominator > other.numerator * self.denominator
    }

    pub fn eq(self, other: Number) -> bool {
        self.numerator == other.numerator && self.denominator == other.denominator
    }

    pub fn ge(self, other: Number) -> bool {
        self.clone().eq(other.clone()) || self.gt(other)
    }

    pub fn le(self, other: Number) -> bool {
        self.clone().eq(other.clone()) || self.lt(other)
    }
}
