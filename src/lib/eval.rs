use std::error;

use num_bigint::{BigUint, ToBigInt};

use crate::{
    datatype::{Bowl, Expr, Noodle, Number, Value},
    env::Env,
};

pub fn eval(env: &mut Env, bowl: Bowl) -> Result<bool, Box<dyn error::Error>> {
    while let Some(noodle) = env.get_next_noodle(&bowl) {
        if env.is_debug {
            println!("[=] noodle: {}", noodle);
        }
        let new_cursor = eval_expr(env, &noodle.nn_expr);
        if env.is_debug {
            println!("[.] noodle number: {}", new_cursor);
        }
        if let Value::Number(number) = new_cursor {
            env.cursor = Some(*number.clone());
        } else {
            panic!("Cannot set cursor to non-number value");
        }
        eval_expr(env, &noodle.expr);
        if env.is_debug {
            println!("[.] cursor: {}", env.cursor.as_ref().unwrap());
            println!("[.] mem state: {}", env.mem_to_bowl());
        }
    }
    Ok(true)
}

pub fn eval_expr(env: &mut Env, expr: &Expr) -> Value {
    match expr {
        Expr::ValueExpr(value) => value.clone(),
        Expr::BowlReadExpr(bowl_expr, nn_expr) => {
            let bowl = eval_expr(env, bowl_expr);
            let nn = eval_expr(env, nn_expr);
            if let (Value::Bowl(bowl), Value::Number(_)) = (bowl, &nn) {
                bowl.borrow().read(env, &nn)
            } else {
                Value::Null
            }
        }
        Expr::MemReadExpr(nn_expr) => {
            let nn = eval_expr(env, nn_expr);
            if let Value::Number(_) = nn {
                env.read_mem(&nn)
            } else {
                Value::Null
            }
        }
        Expr::BowlWriteExpr(bowl_expr, nn_expr, value_expr) => {
            let bowl = eval_expr(env, bowl_expr);
            let nn = eval_expr(env, nn_expr);
            let value = eval_expr(env, value_expr);
            // println!("BowlWriteExpr: {}, {}, {}", bowl, nn, value);
            if let (Value::Bowl(bowl), Value::Number(_)) = (bowl, &nn) {
                bowl.borrow_mut().write(env, &nn, &value);
            }
            Value::Null
        }
        Expr::MemWriteExpr(nn_expr, value_expr) => {
            let nn = eval_expr(env, nn_expr);
            let value = eval_expr(env, value_expr);
            // println!("MemWriteExpr: {}", value);
            if let Value::Number(_) = nn {
                env.write_mem(&nn, &value);
            }
            Value::Null
        }
        Expr::DenoFuncExpr(expr) => {
            let value = eval_expr(env, expr);
            if let Value::Number(number) = value {
                Value::from_big_int(
                    &number.denominator.to_bigint().unwrap(),
                    &BigUint::from(1u32),
                )
            } else {
                Value::Null
            }
        }
        Expr::PlusFuncExpr(expr1, expr2) => {
            let value1 = eval_expr(env, expr1);
            let value2 = eval_expr(env, expr2);
            if let (Value::Number(number1), Value::Number(number2)) = (value1, value2) {
                Value::from_number(&number1.add(&number2))
            } else {
                Value::Null
            }
        }
        Expr::MinusFuncExpr(expr1, expr2) => {
            let value1 = eval_expr(env, expr1);
            let value2 = eval_expr(env, expr2);
            if let (Value::Number(number1), Value::Number(number2)) = (value1, value2) {
                Value::from_number(&number1.sub(&number2))
            } else {
                Value::Null
            }
        }
        Expr::MulFuncExpr(expr1, expr2) => {
            let value1 = eval_expr(env, expr1);
            let value2 = eval_expr(env, expr2);
            if let (Value::Number(number1), Value::Number(number2)) = (value1, value2) {
                Value::from_number(&number1.mul(&number2))
            } else {
                Value::Null
            }
        }
        Expr::NumberSepFuncExpr(expr1, expr2) => {
            let value1 = eval_expr(env, expr1);
            let value2 = eval_expr(env, expr2);
            if let (Value::Number(number1), Value::Number(number2)) = (value1, value2) {
                Value::from_number(&number1.div(&number2))
            } else {
                Value::Null
            }
        }
        Expr::AndFuncExpr(expr1, expr2) => {
            let value1 = eval_expr(env, expr1);
            let value2 = eval_expr(env, expr2);
            if let (Value::Number(number1), Value::Number(number2)) = (value1, value2) {
                if number1.and(&number2) {
                    Value::new_one()
                } else {
                    Value::new_zero()
                }
            } else {
                Value::new_zero()
            }
        }
        Expr::OrFuncExpr(expr1, expr2) => {
            let value1 = eval_expr(env, expr1);
            let value2 = eval_expr(env, expr2);
            if let (Value::Number(number1), Value::Number(number2)) = (value1, value2) {
                if number1.or(&number2) {
                    Value::new_one()
                } else {
                    Value::new_zero()
                }
            } else {
                Value::new_zero()
            }
        }
        Expr::NotFuncExpr(expr) => {
            let value = eval_expr(env, expr);
            if let Value::Number(number) = value {
                if number.eq(&Number::one()) {
                    Value::new_zero()
                } else {
                    Value::new_one()
                }
            } else {
                Value::new_one()
            }
        }
        Expr::EqFuncExpr(expr1, expr2) => {
            let value1 = eval_expr(env, expr1);
            let value2 = eval_expr(env, expr2);
            if let (Value::Number(number1), Value::Number(number2)) = (value1, value2) {
                if number1.eq(&number2) {
                    Value::new_one()
                } else {
                    Value::new_zero()
                }
            } else {
                Value::new_zero()
            }
        }
        Expr::GtFuncExpr(expr1, expr2) => {
            let value1 = eval_expr(env, expr1);
            let value2 = eval_expr(env, expr2);
            if let (Value::Number(number1), Value::Number(number2)) = (value1, value2) {
                if number1.gt(&number2) {
                    Value::new_one()
                } else {
                    Value::new_zero()
                }
            } else {
                Value::new_zero()
            }
        }
        Expr::LtFuncExpr(expr1, expr2) => {
            let value1 = eval_expr(env, expr1);
            let value2 = eval_expr(env, expr2);
            if let (Value::Number(number1), Value::Number(number2)) = (value1, value2) {
                if number1.lt(&number2) {
                    Value::new_one()
                } else {
                    Value::new_zero()
                }
            } else {
                Value::new_zero()
            }
        }
    }
}

impl Bowl {
    pub fn read(&self, env: &mut Env, noodle_number: &Value) -> Value {
        for noodle in &self.noodles {
            let inner_nn = eval_expr(env, &noodle.nn_expr);
            match (inner_nn, noodle_number) {
                (Value::Number(a), Value::Number(b)) => {
                    if a.eq(b) {
                        return eval_expr(env, &noodle.expr);
                    }
                }
                _ => {}
            }
        }
        Value::Null
    }

    pub fn write(&mut self, env: &mut Env, noodle_number: &Value, value: &Value) -> Value {
        let mut is_written = false;
        for noodle in self.noodles.iter_mut() {
            let inner_nn = eval_expr(env, &noodle.nn_expr);
            match (inner_nn, noodle_number) {
                (Value::Number(a), Value::Number(b)) => {
                    if a.eq(b) {
                        noodle.expr = Expr::ValueExpr(value.clone());
                        is_written = true;
                        break;
                    }
                }
                _ => {}
            }
        }
        if !is_written {
            self.noodles.push(Noodle {
                nn_expr: Expr::ValueExpr(noodle_number.clone()),
                expr: Expr::ValueExpr(value.clone()),
            });
        }
        Value::Null
    }
}
