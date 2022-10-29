use std::{
    io::{self, Read, Write},
    rc::Rc,
};

use num_bigint::BigUint;

use crate::datatype::{DefaultBowl, Expr, MutMemBowl, Noodle, Number, Value};

pub fn read_data() -> String {
    let mut stdin = io::stdin();
    let mut buffer = Vec::new();
    stdin.read_to_end(&mut buffer).unwrap();
    unsafe { String::from_utf8_unchecked(buffer) }
}

pub fn write_data(data: String) {
    let mut stdout = io::stdout();
    stdout.write(data.as_bytes()).unwrap();
    stdout.flush().ok();
}

pub fn read() -> DefaultBowl {
    let data = read_data();
    let mut index = Number::zero();
    let mut noodles = vec![];
    for c in data.chars() {
        let unicode = c as u32;
        noodles.push(Noodle {
            nn_expr: Expr::ValueExpr(Value::Number(index.clone())),
            expr: Expr::ValueExpr(Value::Number(Number::new(
                BigUint::from(unicode),
                BigUint::from(1u32),
            ))),
        });
        index = index.add(Number::one());
    }
    DefaultBowl { noodles }
}

pub fn write(bowl: DefaultBowl, mem: MutMemBowl) {
    let mut data = String::new();
    for noodle in bowl.noodles {
        let value = noodle.expr.eval(Rc::clone(&mem));
        if let Value::Number(number) = value {
            if number.denominator != BigUint::from(1u32) {
                panic!("Cannot write non-integer value");
            }
            let num_vec = number.numerator.to_u32_digits();
            if num_vec.len() > 1 {
                panic!("Cannot write too large value");
            }
            let unicode = num_vec[0];
            data.push(char::from_u32(unicode).unwrap());
        } else {
            panic!("Cannot write non-number value");
        }
    }
    write_data(data);
}
