use crate::datatype::{Bowl, DefaultBowl, MutMemBowl, Noodle, Value};
use std::{error, fmt, rc::Rc};

pub fn eval(bowl: Bowl, mem: MutMemBowl) -> Result<bool, Box<dyn error::Error>> {
    match bowl {
        Bowl::MemBowl => Err(Box::new(RootMemBowlError)),
        Bowl::DefaultBowl(default_bowl) => {
            while let Some(noodle) = get_next_noodle(default_bowl.clone(), Rc::clone(&mem)) {
                mem.borrow_mut().cursor = noodle.nn_expr.eval(Rc::clone(&mem));
                noodle.expr.eval(Rc::clone(&mem));
            }
            Ok(true)
        }
    }
}

fn is_nextable(noodle_number: Value, mem: MutMemBowl) -> bool {
    match noodle_number {
        Value::Number(noodle_number_number) => {
            let current_cursor = mem.borrow().cursor.clone();
            match current_cursor {
                Value::Number(current_cursor_number) => {
                    noodle_number_number.gt(current_cursor_number)
                }
                _ => true,
            }
        }
        _ => false,
    }
}

fn get_next_noodle(bowl: DefaultBowl, mem: MutMemBowl) -> Option<Noodle> {
    let mut min_nextable_noodle_number = Value::Null;
    let mut min_nextable_noodle = None;
    for noodle in bowl.noodles {
        let noodle_number = noodle.nn_expr.clone().eval(Rc::clone(&mem));
        if is_nextable(noodle_number.clone(), Rc::clone(&mem)) {
            match (min_nextable_noodle_number.clone(), noodle_number.clone()) {
                (Value::Null, _) => {
                    min_nextable_noodle_number = noodle_number;
                    min_nextable_noodle = Some(noodle);
                }
                (
                    Value::Number(min_nextable_noodle_number_number),
                    Value::Number(noodle_number_number),
                ) => {
                    if noodle_number_number.lt(min_nextable_noodle_number_number) {
                        min_nextable_noodle_number = noodle_number;
                        min_nextable_noodle = Some(noodle);
                    }
                }
                _ => {}
            }
        }
    }
    min_nextable_noodle
}

#[derive(Debug, Clone)]
struct RootMemBowlError;
impl error::Error for RootMemBowlError {}
impl fmt::Display for RootMemBowlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "root bowl should not be Mem(@) bowl")
    }
}
