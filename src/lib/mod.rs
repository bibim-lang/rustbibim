pub mod datatype;
pub mod eval;
pub mod io;

use std::{cell::RefCell, error, rc::Rc};

use datatype::{DefaultBowl, MemBowl, MutMemBowl, Value};
use lrlex::lrlex_mod;
use lrpar::lrpar_mod;

lrlex_mod!("bibim.l");
lrpar_mod!("bibim.y");

pub fn run(code: String) -> Result<(), Box<dyn error::Error>> {
    let lexerdef = bibim_l::lexerdef();
    let lexer = lexerdef.lexer(code.as_str());
    let (res, errs) = bibim_y::parse(&lexer);
    for e in errs {
        panic!("{}", e.to_string());
    }
    if let Some(Ok(r)) = res {
        let mem = MutMemBowl::new(RefCell::new(MemBowl {
            inner_bowl: DefaultBowl { noodles: vec![] },
            cursor: Value::Null,
        }));
        return match eval::eval(r, Rc::clone(&mem)) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        };
    }
    panic!("No result");
}

#[cfg(test)]
mod tests {
    use datatype::Number;

    use super::*;

    #[test]
    fn simple_code() {
        let code = r"{
            [1; !0]
        }";
        let lexerdef = bibim_l::lexerdef();
        let lexer = lexerdef.lexer(code);
        let (res, errs) = bibim_y::parse(&lexer);
        assert_eq!(errs.len(), 0);
        let bowl = res.unwrap().unwrap();
        let mem = MutMemBowl::new(RefCell::new(MemBowl {
            inner_bowl: DefaultBowl { noodles: vec![] },
            cursor: Value::Null,
        }));
        let result = eval::eval(bowl, Rc::clone(&mem));
        assert_eq!(result.unwrap(), true);
        if let Value::Number(n) = mem.borrow().cursor.clone() {
            assert!(n.eq(Number::one()))
        } else {
            assert!(false)
        };
    }
}
