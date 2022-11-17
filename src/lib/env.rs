use num_bigint::{BigInt, BigUint, Sign};

use crate::{
    datatype::{Bowl, Expr, Noodle, Number, Value},
    eval::eval_expr,
};

pub struct Env<'a> {
    pub mem: Vec<(Number, Value)>,
    pub cursor: Option<Number>,
    pub is_debug: bool,
    pub on_read_io: Box<dyn Fn() -> Vec<u8> + 'a>,
    pub on_write_io: Box<dyn Fn(Vec<u8>) -> () + 'a>,
}

impl Env<'_> {
    pub fn read_io(&self) -> Bowl {
        let data = (self.on_read_io)();
        let mut index = Number::zero();
        let mut noodles = vec![];
        for byte in data {
            noodles.push(Noodle {
                nn_expr: Expr::ValueExpr(Value::from_number(&index)),
                expr: Expr::ValueExpr(Value::from_big_int(
                    &BigInt::from(byte),
                    &BigUint::from(1u32),
                )),
            });
            index = index.add(&Number::one());
        }
        Bowl { noodles }
    }

    pub fn write_io(&mut self, bowl: &Bowl) {
        let mut data = vec![];
        let mut nn_index = Number::zero();

        loop {
            let value = bowl.read(self, &Value::from_number(&nn_index));
            if let Value::Number(number) = value {
                if number.denominator != BigUint::from(1u32) {
                    panic!("Cannot write non-integer value");
                }
                let num_vec = number.numerator.to_bytes_be();
                if num_vec.0 == Sign::Minus {
                    panic!("Cannot write negative value");
                }
                if num_vec.1.len() > 1 {
                    panic!("Cannot write too large value");
                }
                data.push(num_vec.1[0]);
                nn_index = nn_index.add(&Number::one());
            } else {
                break;
            }
        }
        (self.on_write_io)(data);
    }

    pub fn read_mem(&self, noodle_number: &Value) -> Value {
        if let Value::Number(nn_number) = noodle_number {
            if nn_number.eq(&Number::zero()) {
                return match self.cursor {
                    Some(ref cursor) => Value::from_number(&cursor),
                    None => Value::Null,
                };
            }
            if nn_number.eq(&Number::one()) {
                return Value::from_bowl(self.read_io());
            }
            for noodle_like in &self.mem {
                let inner_nn = &noodle_like.0;
                if inner_nn.eq(nn_number) {
                    return noodle_like.1.clone();
                }
            }
        }
        Value::Null
    }

    pub fn write_mem(&mut self, noodle_number: &Value, value: &Value) {
        if let Value::Number(nn_number) = noodle_number {
            if nn_number.eq(&Number::one()) {
                if let Value::Bowl(bowl) = value {
                    return self.write_io(&bowl.borrow());
                } else {
                    panic!("Cannot write non-bowl value");
                }
            }
            let mut is_written = false;
            for noodle_like in self.mem.iter_mut() {
                let inner_nn = &noodle_like.0;
                if inner_nn.eq(nn_number) {
                    noodle_like.1 = value.clone();
                    is_written = true;
                    break;
                }
            }
            if !is_written {
                self.mem.push((*nn_number.clone(), value.clone()));
            }
        }
    }

    pub fn is_nextable(&mut self, noodle_number: &Value) -> bool {
        match noodle_number {
            Value::Number(noodle_number_number) => match &self.cursor {
                Some(current_cursor_number) => noodle_number_number.gt(current_cursor_number),
                _ => true,
            },
            _ => false,
        }
    }

    pub fn get_next_noodle(&mut self, bowl: &Bowl) -> Option<Noodle> {
        let mut min_nextable_noodle_number = Value::Null;
        let mut min_nextable_noodle = None;
        for noodle in &bowl.noodles {
            let noodle_number = eval_expr(self, &noodle.nn_expr);
            if self.is_nextable(&noodle_number) {
                match (&min_nextable_noodle_number, &noodle_number) {
                    (Value::Null, _) => {
                        min_nextable_noodle_number = noodle_number.clone();
                        min_nextable_noodle = Some(noodle);
                    }
                    (
                        Value::Number(min_nextable_noodle_number_number),
                        Value::Number(noodle_number_number),
                    ) => {
                        if noodle_number_number.lt(&min_nextable_noodle_number_number) {
                            min_nextable_noodle_number = noodle_number.clone();
                            min_nextable_noodle = Some(noodle);
                        }
                    }
                    _ => {}
                }
            }
        }
        min_nextable_noodle.cloned()
    }

    pub fn mem_to_bowl(&self) -> Bowl {
        let mut noodles = vec![];
        for noodle_like in &self.mem {
            noodles.push(Noodle {
                nn_expr: Expr::ValueExpr(Value::from_number(&noodle_like.0)),
                expr: Expr::ValueExpr(noodle_like.1.clone()),
            });
        }
        Bowl { noodles }
    }
}
