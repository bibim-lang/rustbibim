use std::error;

use num_bigint::BigUint;

use crate::datatype::{Bowl, Expr, Noodle, Number, Value};
use crate::error::RootMemBowlError;

pub struct Env<'a> {
    pub mem: Bowl,
    pub cursor: Option<Number>,
    pub is_debug: bool,
    pub on_read: Box<dyn Fn() -> Vec<u8> + 'a>,
    pub on_write: Box<dyn Fn(Vec<u8>) -> () + 'a>,
}

impl Env<'_> {
    pub fn read(&self) -> Bowl {
        let data = (self.on_read)();
        let mut index = Number::zero();
        let mut noodles = vec![];
        for byte in data {
            noodles.push(Noodle {
                nn_expr: Expr::ValueExpr(Value::Number(index.clone())),
                expr: Expr::ValueExpr(Value::Number(Number::new(
                    BigUint::from(byte),
                    BigUint::from(1u32),
                ))),
            });
            index = index.add(&Number::one());
        }
        Bowl {
            is_mem: false,
            noodles,
        }
    }

    pub fn write(&mut self, bowl: &Bowl) {
        let mut data = vec![];
        for noodle in bowl.noodles.iter() {
            let value = self.eval_expr(&noodle.expr);
            if let Value::Number(number) = value {
                if number.denominator != BigUint::from(1u32) {
                    panic!("Cannot write non-integer value");
                }
                let num_vec = number.numerator.to_bytes_be();
                if num_vec.len() > 1 {
                    panic!("Cannot write too large value");
                }
                data.push(num_vec[0]);
            } else {
                panic!("Cannot write non-number value");
            }
        }
        (self.on_write)(data);
    }

    pub fn eval(&mut self, bowl: Bowl) -> Result<bool, Box<dyn error::Error>> {
        if bowl.is_mem {
            return Err(Box::new(RootMemBowlError));
        }
        while let Some(noodle) = self.get_next_noodle(&bowl) {
            if self.is_debug {
                println!("[=] noodle: {}", noodle);
            }
            let new_cursor = self.eval_expr(&noodle.nn_expr);
            if self.is_debug {
                println!("[.] noodle number value: {}", new_cursor);
            }
            if let Value::Number(number) = new_cursor {
                self.cursor = Some(number.clone());
            } else {
                panic!("Cannot set cursor to non-number value");
            }
            let result = self.eval_expr(&noodle.expr);
            if self.is_debug {
                println!("[.] noodle eval value: {}", result);
                println!("[.] mem state: {}", self.mem);
            }
        }
        Ok(true)
    }

    pub fn eval_expr(&mut self, expr: &Expr) -> Value {
        match expr {
            Expr::ValueExpr(value) => self.eval_value(value),
            Expr::BowlAceessFuncExpr(bowl_access) => Value::BowlAccess(bowl_access.clone()),
            Expr::AssignFuncExpr(bowl_access, expr) => {
                let bowl_access_value = self.eval_expr(bowl_access);
                let expr = self.eval_expr(expr);
                if let Value::BowlAccess(bowl_access) = bowl_access_value {
                    let bowl_value = self.eval_expr(&bowl_access.bowl_expr);
                    let bowl_value = self.eval_value(&bowl_value);
                    let access_value = self.eval_expr(&bowl_access.access_expr);
                    if self.is_debug {
                        println!("[AssignFuncExpr] bowl_value: {}", bowl_value);
                        println!("[AssignFuncExpr] access_value: {}", access_value);
                        // panic!("debug");
                    }
                    // todo: should handle recursive bowl access in value
                    if let Value::Bowl(mut bowl) = bowl_value {
                        if bowl.is_mem {
                            let is_write_assign =
                                if let Value::Number(number) = self.eval_value(&access_value) {
                                    number.eq(&Number::one())
                                } else {
                                    false
                                };
                            if is_write_assign {
                                let value = self.eval_value(&expr);
                                if let Value::Bowl(bowl) = value {
                                    if bowl.is_mem {
                                        panic!("can not write Mem(@) bowl data");
                                    }
                                    self.write(&bowl);
                                    return Value::Null;
                                }
                                panic!("Cannot write non-bowl data");
                            } else {
                                let mut mem_bowl = self.mem.clone();
                                let new_noodles =
                                    self.updated_noodles(&mem_bowl, access_value, expr);
                                {
                                    if self.is_debug {
                                        println!("[-] old mem bowl: {}", mem_bowl);
                                    }
                                    self.set_noodles(&mut mem_bowl, new_noodles);
                                    if self.is_debug {
                                        println!("[-] new mem bowl: {}", mem_bowl);
                                    }
                                    self.mem = mem_bowl;
                                    Value::Null
                                }
                            }
                        } else {
                            let new_noodles = self.updated_noodles(&bowl, access_value, expr);
                            if self.is_debug {
                                println!("[-] old bowl: {}", bowl);
                            }
                            self.set_noodles(&mut bowl, new_noodles);
                            if self.is_debug {
                                println!("[+] new bowl: {}", bowl);
                            }
                            Value::Null
                        }
                    } else {
                        Value::Null
                    }
                } else {
                    Value::Null
                }
            }
            Expr::DenoFuncExpr(expr) => {
                let expr = self.eval_expr(expr);
                if let Value::Number(number) = expr {
                    Value::Number(Number::new(number.denominator, BigUint::from(1u32)))
                } else {
                    Value::Null
                }
            }
            Expr::PlusFuncExpr(expr1, expr2) => {
                let expr1 = self.eval_expr(expr1);
                let expr2 = self.eval_expr(expr2);
                if let (Value::Number(number1), Value::Number(number2)) = (expr1, expr2) {
                    Value::Number(number1.add(&number2))
                } else {
                    Value::Null
                }
            }
            Expr::MinusFuncExpr(expr1, expr2) => {
                let expr1 = self.eval_expr(expr1);
                let expr2 = self.eval_expr(expr2);
                if let (Value::Number(number1), Value::Number(number2)) = (expr1, expr2) {
                    Value::Number(number1.sub(&number2))
                } else {
                    Value::Null
                }
            }
            Expr::MulFuncExpr(expr1, expr2) => {
                let expr1 = self.eval_expr(expr1);
                let expr2 = self.eval_expr(expr2);
                if let (Value::Number(number1), Value::Number(number2)) = (expr1, expr2) {
                    Value::Number(number1.mul(&number2))
                } else {
                    Value::Null
                }
            }
            Expr::NumberSepFuncExpr(expr1, expr2) => {
                let expr1 = self.eval_expr(expr1);
                let expr2 = self.eval_expr(expr2);
                if let (Value::Number(number1), Value::Number(number2)) = (expr1, expr2) {
                    Value::Number(number1.div(&number2))
                } else {
                    Value::Null
                }
            }
            Expr::AndFuncExpr(expr1, expr2) => {
                let expr1 = self.eval_expr(expr1);
                let expr2 = self.eval_expr(expr2);
                if let (Value::Number(number1), Value::Number(number2)) = (expr1, expr2) {
                    if number1.and(&number2) {
                        Value::Number(Number::one())
                    } else {
                        Value::Number(Number::zero())
                    }
                } else {
                    Value::Number(Number::zero())
                }
            }
            Expr::OrFuncExpr(expr1, expr2) => {
                let expr1 = self.eval_expr(expr1);
                let expr2 = self.eval_expr(expr2);
                if let (Value::Number(number1), Value::Number(number2)) = (expr1, expr2) {
                    if number1.or(&number2) {
                        Value::Number(Number::one())
                    } else {
                        Value::Number(Number::zero())
                    }
                } else {
                    Value::Number(Number::zero())
                }
            }
            Expr::NotFuncExpr(expr) => {
                let expr = self.eval_expr(expr);
                if let Value::Number(number) = expr {
                    if number.eq(&Number::one()) {
                        Value::Number(Number::zero())
                    } else {
                        Value::Number(Number::one())
                    }
                } else {
                    Value::Number(Number::one())
                }
            }
            Expr::EqFuncExpr(expr1, expr2) => {
                let expr1 = self.eval_expr(expr1);
                let expr2 = self.eval_expr(expr2);
                if let (Value::Number(number1), Value::Number(number2)) = (expr1, expr2) {
                    if number1.eq(&number2) {
                        Value::Number(Number::one())
                    } else {
                        Value::Number(Number::zero())
                    }
                } else {
                    Value::Number(Number::zero())
                }
            }
            Expr::GtFuncExpr(expr1, expr2) => {
                let expr1 = self.eval_expr(expr1);
                let expr2 = self.eval_expr(expr2);
                if let (Value::Number(number1), Value::Number(number2)) = (expr1, expr2) {
                    if number1.gt(&number2) {
                        Value::Number(Number::one())
                    } else {
                        Value::Number(Number::zero())
                    }
                } else {
                    Value::Number(Number::zero())
                }
            }
            Expr::LtFuncExpr(expr1, expr2) => {
                let expr1 = self.eval_expr(expr1);
                let expr2 = self.eval_expr(expr2);
                if let (Value::Number(number1), Value::Number(number2)) = (expr1, expr2) {
                    if number1.lt(&number2) {
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

    pub fn eval_value(&mut self, value: &Value) -> Value {
        match value {
            Value::Number(n) => Value::Number(n.clone()),
            Value::Bowl(b) => Value::Bowl(b.clone()),
            Value::Null => Value::Null,
            Value::BowlAccess(bowl_access) => {
                let bowl_value = self.eval_expr(&bowl_access.bowl_expr);
                let bowl_value = self.eval_value(&bowl_value);
                if let Value::Bowl(bowl) = bowl_value {
                    let noodle_number = self.eval_expr(&bowl_access.access_expr);
                    let target_bowl = if bowl.is_mem {
                        self.mem.clone()
                    } else {
                        bowl.clone()
                    };
                    let result = self.find_noodle(&target_bowl, &noodle_number);
                    if bowl.is_mem {
                        self.mem = target_bowl;
                    }
                    if let Some(noodle) = result {
                        return self.eval_expr(&noodle.expr);
                    }
                }
                Value::Null
            }
        }
    }

    pub fn find_noodle(&mut self, bowl: &Bowl, noodle_number: &Value) -> Option<Noodle> {
        if bowl.is_mem {
            if let Value::Number(number) = self.eval_value(noodle_number) {
                if number.eq(&Number::zero()) {
                    return Some(Noodle {
                        nn_expr: Expr::ValueExpr(Value::Number(number.clone())),
                        expr: Expr::ValueExpr(match &self.cursor {
                            Some(n) => Value::Number(n.clone()),
                            None => Value::Null
                        }),
                    });
                } else if number.eq(&Number::one()) {
                    let data = self.read();
                    return Some(Noodle {
                        nn_expr: Expr::ValueExpr(Value::Number(number.clone())),
                        expr: Expr::ValueExpr(Value::Bowl(data)),
                    });
                }
            }
        }
        for noodle in &bowl.noodles {
            let nn = self.eval_expr(&noodle.nn_expr);
            match (nn, noodle_number) {
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

    pub fn updated_noodles(
        &mut self,
        bowl: &Bowl,
        noodle_number: Value,
        value: Value,
    ) -> Vec<Noodle> {
        let noodle_number = self.eval_value(&noodle_number);
        let value = self.eval_value(&value);
        let mut new_noodles = bowl.noodles.clone();
        let updated = match new_noodles.iter_mut().find(|noodle| {
            match (noodle_number.clone(), self.eval_expr(&noodle.nn_expr)) {
                (Value::Number(a), Value::Number(b)) => a.eq(&b),
                _ => false,
            }
        }) {
            Some(noodle) => {
                noodle.expr = Expr::ValueExpr(value.clone());
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

    pub fn set_noodles(&self, bowl: &mut Bowl, noodles: Vec<Noodle>) {
        bowl.noodles = noodles;
    }

    pub fn is_nextable(&self, noodle_number: &Value) -> bool {
        match noodle_number {
            Value::Number(noodle_number_number) => match &self.cursor {
                Some(current_cursor_number) => {
                    noodle_number_number.gt(current_cursor_number)
                }
                _ => true,
            },
            _ => false,
        }
    }

    pub fn get_next_noodle(&mut self, bowl: &Bowl) -> Option<Noodle> {
        let mut min_nextable_noodle_number = Value::Null;
        let mut min_nextable_noodle = None;
        for noodle in &bowl.noodles {
            let noodle_number = self.eval_expr(&noodle.nn_expr);
            if self.is_nextable(&noodle_number) {
                match (min_nextable_noodle_number.clone(), noodle_number.clone()) {
                    (Value::Null, _) => {
                        min_nextable_noodle_number = noodle_number;
                        min_nextable_noodle = Some(noodle);
                    }
                    (
                        Value::Number(min_nextable_noodle_number_number),
                        Value::Number(noodle_number_number),
                    ) => {
                        if noodle_number_number.lt(&min_nextable_noodle_number_number) {
                            min_nextable_noodle_number = noodle_number;
                            min_nextable_noodle = Some(noodle);
                        }
                    }
                    _ => {}
                }
            }
        }
        min_nextable_noodle.cloned()
    }
}
