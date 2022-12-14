pub mod datatype;
pub mod env;
pub mod error;
pub mod eval;

use env::Env;
use lrlex::lrlex_mod;
use lrpar::lrpar_mod;
use std::error as std_error;

lrlex_mod!("bibim.l");
lrpar_mod!("bibim.y");

pub fn run(code: String, env: &mut Env) -> Result<(), Box<dyn std_error::Error>> {
    let lexerdef = bibim_l::lexerdef();
    let lexer = lexerdef.lexer(code.as_str());
    let (res, errs) = bibim_y::parse(&lexer);
    let mut is_failed = false;
    for e in errs {
        println!("{}", e.pp(&lexer, &bibim_y::token_epp));
        is_failed = true;
    }
    if is_failed {
        return Err(Box::new(error::ParseError));
    }
    match eval::eval(env, res.unwrap().unwrap()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;

    #[test]
    fn simple_code() {
        // code to test
        let code = r"{
            [1; !0]
        }";

        // setup env
        let input = "".as_bytes();
        let output = Arc::new(Mutex::new(Vec::new()));
        let mut env = Env {
            cursor: None,
            mem: vec![],
            is_debug: true,
            on_read_io: Box::new(|| input.to_vec()),
            on_write_io: Box::new(|data| output.lock().unwrap().extend(data)),
        };

        // run code
        run(code.to_string(), &mut env).unwrap();

        // test output
        assert_eq!(*output.lock().unwrap(), "".as_bytes());
    }

    #[test]
    fn hello_world_1() {
        // code to test
        let code = r"{
            [0; @:1 = {
                [0; 72]
                [1; 69]
                [2; 76]
                [3; 76]
                [4; 79]
                [5; 32]
                [6; 87]
                [7; 79]
                [8; 82]
                [9; 76]
                [10; 68]
                [11; 10]
            }]
        }";

        // setup env
        let input = "".as_bytes();
        let output = Arc::new(Mutex::new(Vec::new()));
        let mut env = Env {
            cursor: None,
            mem: vec![],
            is_debug: true,
            on_read_io: Box::new(|| input.to_vec()),
            on_write_io: Box::new(|data| output.lock().unwrap().extend(data)),
        };

        // run code
        run(code.to_string(), &mut env).unwrap();

        // test output
        assert_eq!(*output.lock().unwrap(), "HELLO WORLD\n".as_bytes());
    }

    #[test]
    fn hello_world_2() {
        // code to test
        let code = r"{
            [0; @:1 = {
                [0; 72]
                [1; 69]
                [2; 76]
                [3; 76]
                [4; 79]
            }]
            [1; @:1 = {
                [0; 32]
                [1; 87]
                [2; 79]
                [3; 82]
                [4; 76]
                [5; 68]
                [6; 10]
            }]
        }";

        // setup env
        let input = "".as_bytes();
        let output = Arc::new(Mutex::new(Vec::new()));
        let mut env = Env {
            cursor: None,
            mem: vec![],
            is_debug: true,
            on_read_io: Box::new(|| input.to_vec()),
            on_write_io: Box::new(|data| output.lock().unwrap().extend(data)),
        };

        // run code
        run(code.to_string(), &mut env).unwrap();

        // test output
        assert_eq!(*output.lock().unwrap(), "HELLO WORLD\n".as_bytes());
    }

    #[test]
    fn hello_world_3() {
        // code to test
        let code = r"{
            [0; @:2 = {
                [0; 69]
                [1; 69]
                [2; 76]
                [3; 76]
                [4; 79]
                [5; 32]
                [6; 87]
                [7; 79]
                [8; 82]
                [9; 76]
                [10; 68]
                [11; 10]
            }]
            [1; (@:2):0 = 72]
            [2; @:1 = @:2]
        }";

        // setup env
        let input = "".as_bytes();
        let output = Arc::new(Mutex::new(Vec::new()));
        let mut env = Env {
            cursor: None,
            mem: vec![],
            is_debug: true,
            on_read_io: Box::new(|| input.to_vec()),
            on_write_io: Box::new(|data| output.lock().unwrap().extend(data)),
        };

        // run code
        run(code.to_string(), &mut env).unwrap();

        // test output
        assert_eq!(*output.lock().unwrap(), "HELLO WORLD\n".as_bytes());
    }

    #[test]
    fn echo() {
        // code to test
        let code = r"{
            [0; @:1 = @:1]
        }";

        // setup env
        let input = "test\n".as_bytes();
        let output = Arc::new(Mutex::new(Vec::new()));
        let mut env = Env {
            cursor: None,
            mem: vec![],
            is_debug: true,
            on_read_io: Box::new(|| input.to_vec()),
            on_write_io: Box::new(|data| output.lock().unwrap().extend(data)),
        };

        // run code
        run(code.to_string(), &mut env).unwrap();

        // test output
        assert_eq!(*output.lock().unwrap(), "test\n".as_bytes());
    }

    #[test]
    fn print_int() {
        // code to test
        let code = r"{
            [0; @:2/7 = 112873] ~# ?????? @:1/3??? ???????????? ?????? ????????? ?????? ?????? #~
            [1; @:3/7 = 2] ~# ?????? @:1/3??? ?????? ??? ????????? ?????? 2 ?????? #~
            [2; @:1/3 = @:0 + 1] ~# ?????? @:1/3??? ?????? #~
            [@:2; @:1 = @:6/7] ~# ?????? @:1/3??? ????????? ?????? ????????? ?????? ????????? ?????? #~
        
            ~# ??????: @:1/2 = ????????? ????????? ?????? #~
            ~# ??????: @:2/5 = ????????? ??????, @:3/5 = ?????? ?????? ??? ????????? ?????? #~
            ~# ??????: @:4/5 = ????????? ????????? ?????? #~
            [@:1/2; @:1/5 = @:0 + 3]
            [@:1/5 - 2; @:4/5 = 0] ~# ?????? ????????? 0??? ?????? #~
            [@:1/5 - 1; @:(0 - 1)/5 = @:2/5] ~# @:(0 - 1)/5 = ????????? ????????? ?????? ?????? ?????? #~
            [@:1/5 + 0; @:4/5 = @:4/5 + 1] ~# ?????? ????????? 1 ?????? #~
            [@:1/5 + 2; @:(0 - 1)/5 = (@:(0 - 1)/5) / 10] ~# @:(0 - 1)/5 ??? 10?????? ????????? ?????? ?????? #~
            [@:1/5 + 3; { ~# @:(0 - 1)/5 ??? 1?????? ????????? ????????? @:1/5??? ???????????? #~
                [1; @:1/5 = @:0 + 1]
            }:!(@:(0 - 1)/5 < 1)]
            [@:1/5 + 4; @:(@:3/5) = @:0 + 1] ~# ?????? ?????? #~
        
            ~# ??????: @:1/3 = ????????? ??????????????? ????????? ascii??? ????????? Bowl?????? ?????? #~
            ~# ??????: @:2/7 = ????????? ??????, @:3/7 = ?????? ?????? ??? ????????? ?????? #~
            ~# ??????: @:6/7 = ????????? Bowl #~
            [@:1/3; @:1/7 = @:0 + 1]
            [@:1/7 + 0; @:6/7 = {}] ~# ?????? ????????? ????????? Bowl ????????? #~
            [@:1/7 + 1; @:(0 - 1)/7 = @:2/7] ~# ????????? ?????? ?????? ?????? @:(0 - 1)/7 ??? ?????? #~
            [@:1/7 + 2; @:2/5 = @:2/7] ~# ?????? @:1/2??? ???????????? ?????? ????????? ?????? ?????? #~
            [@:1/7 + 3; @:3/5 = 1/13] ~# ?????? @:1/2??? ?????? ??? ????????? ?????? 1/13 ?????? #~
            [@:1/7 + 4; @:1/2 = @:0 + 1] ~# ?????? @:1/2??? ?????? #~
            [@:1/13 + 0; @:(0 - 2)/7 = @:4/5] ~# ?????? @:1/2 ?????? ??????(????????? ??????)??? ?????? ?????? @:(0 - 2)/7 ??? ?????? #~
            [@:1/13 + 1; @:(0 - 3)/7 = 0] ~# ????????? ????????? ?????? @:(0 - 3)/7 ????????? #~
            [@:1/13 + 2; { ~# ????????? ????????? 0?????? ?????? ?????? #~
                [0; @:1/17 = @:0 + 1] ~# ????????? ????????? 0??? ???????????? ?????? ?????? #~
                [1; @:(@:3/7) = @:0 + 1] ~# ????????? ????????? 0????????? ?????? ?????? #~
            }:(@:(0 - 2)/7 ?= 0)]
            [@:1/17 + 0; @:(0 - 4)/7 = (@:(0 - 1)/7 - @:(0 - 3)/7)/10] ~# @:(0 - 4)/7 = (???????????? 1 - ???????????? 3) / 10 #~
            [@:1/17 + 1; { ~# @:(0 - 4)/7 ??? ????????? 1??? ?????? @:(0 - 3)/7??? ?????? ????????? ????????? ????????????#~
                [0; @:1/19 = @:0 + 1] ~# @:(0 - 3)/7 ??? 1 ??????????????? ?????? ???????????? ????????? ?????? #~
                [1; @:1/23 = @:0 + 1] ~# @:(0 - 3)/7 ??? ????????? Bowl??? ???????????? ????????? ?????? #~
            }:(^(@:(0 - 4)/7) ?= 1)]
            [@:1/19 + 0; @:(0 - 3)/7 = @:(0 - 3)/7 + 1] ~# @:(0 - 3)/7 ??? 1 ???????????? #~
            [@:1/19 + 1; @:1/13 = @:0 - 1] ~# @:1/13 + 2 ??? ?????? #~
            [@:1/23 + 0; @:(0 - 2)/7 = @:(0 - 2)/7 - 1] ~# @:(0 - 2)/7 (????????? ??????)??? 1 ???????????? #~
            [@:1/23 + 1; (@:6/7):(@:(0 - 2)/7) = @:(0 - 3)/7 + 48] ~# ????????? Bowl??? (????????? ??????)????????? ?????? ???????????? ?????? #~
            [@:1/23 + 2; @:(0 - 1)/7 = @:(0 - 4)/7] ~# @:(0 - 1)/7 ??? ????????? ???????????? ???????????? ?????? #~
            [@:1/23 + 3; @:1/13 = @:0 - 0] ~# @:1/13 + 1 ??? ?????? (????????? ????????? ???????????? ?????????) #~
        }";

        // setup env
        let input = "".as_bytes();
        let output = Arc::new(Mutex::new(Vec::new()));
        let mut env = Env {
            cursor: None,
            mem: vec![],
            is_debug: true,
            on_read_io: Box::new(|| input.to_vec()),
            on_write_io: Box::new(|data| output.lock().unwrap().extend(data)),
        };

        // run code
        run(code.to_string(), &mut env).unwrap();

        // test output
        assert_eq!(*output.lock().unwrap(), "112873".as_bytes());
    }

    #[test]
    fn print_square() {
        // code to test
        let code = r"{
            [1/2; @:2/11 = @:1] ~# STDIN?????? ??? ???????????? #~
            [2/3; @:3/11 = 2] ~# ?????? @:1/3??? ?????? ??? ????????? ?????? 2 ?????? #~
            [3/4; @:1/11 = @:0 + 1] ~# ?????? @:1/11 ?????? #~
            [@:2 + 0; @:2/7 = @:10/11 * @:10/11] ~# ?????? @:1/11 ?????? ????????? ???????????? @:2/7??? ?????? #~
            [@:2 + 1; @:3/7 = 3] ~# ?????? @:1/3??? ?????? ??? ????????? ?????? 3 ?????? #~
            [@:2 + 2; @:1/3 = @:0 + 1] ~# ?????? @:1/3??? ?????? #~
            [@:3 + 0; @:1 = @:6/7] ~# ?????? @:1/3??? ????????? ?????? ????????? ?????? ????????? ?????? #~
            [@:3 + 1; @:1 = {
                [0; 10]
            }]
        
            ~# ??????: @:1/2 = ????????? ????????? ?????? #~
            ~# ??????: @:2/5 = ????????? ??????, @:3/5 = ?????? ?????? ??? ????????? ?????? #~
            ~# ??????: @:4/5 = ????????? ????????? ?????? #~
            [@:1/2; @:1/5 = @:0 + 3]
            [@:1/5 - 2; @:4/5 = 0] ~# ?????? ????????? 0??? ?????? #~
            [@:1/5 - 1; @:(0 - 1)/5 = @:2/5] ~# @:(0 - 1)/5 = ????????? ????????? ?????? ?????? ?????? #~
            [@:1/5 + 0; @:4/5 = @:4/5 + 1] ~# ?????? ????????? 1 ?????? #~
            [@:1/5 + 2; @:(0 - 1)/5 = (@:(0 - 1)/5) / 10] ~# @:(0 - 1)/5 ??? 10?????? ????????? ?????? ?????? #~
            [@:1/5 + 3; { ~# @:(0 - 1)/5 ??? 1?????? ????????? ????????? @:1/5??? ???????????? #~
                [1; @:1/5 = @:0 + 1]
            }:!(@:(0 - 1)/5 < 1)]
            [@:1/5 + 4; @:(@:3/5) = @:0 + 1] ~# ?????? ?????? #~
        
            ~# ??????: @:1/3 = ????????? ??????????????? ????????? ascii??? ????????? Bowl?????? ?????? #~
            ~# ??????: @:2/7 = ????????? ??????, @:3/7 = ?????? ?????? ??? ????????? ?????? #~
            ~# ??????: @:6/7 = ????????? Bowl #~
            [@:1/3; @:1/7 = @:0 + 1]
            [@:1/7 + 0; @:6/7 = {}] ~# ?????? ????????? ????????? Bowl ????????? #~
            [@:1/7 + 1; @:(0 - 1)/7 = @:2/7] ~# ????????? ?????? ?????? ?????? @:(0 - 1)/7 ??? ?????? #~
            [@:1/7 + 2; @:2/5 = @:2/7] ~# ?????? @:1/2??? ???????????? ?????? ????????? ?????? ?????? #~
            [@:1/7 + 3; @:3/5 = 1/13] ~# ?????? @:1/2??? ?????? ??? ????????? ?????? 1/13 ?????? #~
            [@:1/7 + 4; @:1/2 = @:0 + 1] ~# ?????? @:1/2??? ?????? #~
            [@:1/13 + 0; @:(0 - 2)/7 = @:4/5] ~# ?????? @:1/2 ?????? ??????(????????? ??????)??? ?????? ?????? @:(0 - 2)/7 ??? ?????? #~
            [@:1/13 + 1; @:(0 - 3)/7 = 0] ~# ????????? ????????? ?????? @:(0 - 3)/7 ????????? #~
            [@:1/13 + 2; { ~# ????????? ????????? 0?????? ?????? ?????? #~
                [0; @:1/17 = @:0 + 1] ~# ????????? ????????? 0??? ???????????? ?????? ?????? #~
                [1; @:(@:3/7) = @:0 + 1] ~# ????????? ????????? 0????????? ?????? ?????? #~
            }:(@:(0 - 2)/7 ?= 0)]
            [@:1/17 + 0; @:(0 - 4)/7 = (@:(0 - 1)/7 - @:(0 - 3)/7)/10] ~# @:(0 - 4)/7 = (???????????? 1 - ???????????? 3) / 10 #~
            [@:1/17 + 1; { ~# @:(0 - 4)/7 ??? ????????? 1??? ?????? @:(0 - 3)/7??? ?????? ????????? ????????? ????????????#~
                [0; @:1/19 = @:0 + 1] ~# @:(0 - 3)/7 ??? 1 ??????????????? ?????? ???????????? ????????? ?????? #~
                [1; @:1/23 = @:0 + 1] ~# @:(0 - 3)/7 ??? ????????? Bowl??? ???????????? ????????? ?????? #~
            }:(^(@:(0 - 4)/7) ?= 1)]
            [@:1/19 + 0; @:(0 - 3)/7 = @:(0 - 3)/7 + 1] ~# @:(0 - 3)/7 ??? 1 ???????????? #~
            [@:1/19 + 1; @:1/13 = @:0 - 1] ~# @:1/13 + 2 ??? ?????? #~
            [@:1/23 + 0; @:(0 - 2)/7 = @:(0 - 2)/7 - 1] ~# @:(0 - 2)/7 (????????? ??????)??? 1 ???????????? #~
            [@:1/23 + 1; (@:6/7):(@:(0 - 2)/7) = @:(0 - 3)/7 + 48] ~# ????????? Bowl??? (????????? ??????)????????? ?????? ???????????? ?????? #~
            [@:1/23 + 2; @:(0 - 1)/7 = @:(0 - 4)/7] ~# @:(0 - 1)/7 ??? ????????? ???????????? ???????????? ?????? #~
            [@:1/23 + 3; @:1/13 = @:0 - 0] ~# @:1/13 + 1 ??? ?????? (????????? ????????? ???????????? ?????????) #~
        
            ~# ??????: @:1/11 = ???????????? ????????? ?????? #~
            ~# ??????: @:2/11 = ????????? ?????????, @:3/11 = ?????? ?????? ??? ????????? ?????? #~
            ~# ??????: @:10/11 = ????????? ?????? #~
            [@:1/11; @:1/29 = @:0 + 1]
            [@:1/29 + 0; @:10/11 = 0] ~# ????????? ?????? 0 ?????? #~
            [@:1/29 + 1; @:(0 - 1)/11 = 0] ~# @:(0 - 1)/11 = ????????? ???????????? ?????? ?????? ?????? #~
            [@:1/29 + 2; @:(0 - 2)/11 = @:2/11] ~# @:(0 - 2)/11 = ????????? ???????????? ?????? ?????? ?????? #~
            [@:1/29 + 3; @:(0 - 3)/11 = (@:(0 - 2)/11):(@:(0 - 1)/11)] ~# @:(0 - 3)/11 = ?????? ????????? ????????? ??? #~
            [@:1/29 + 4; @:(0 - 4)/11 = @:0 + 3] ~# @:(0 - 3)/11??? NULL ??? ?????? CATCH??? ?????? #~
            [@:1/29 + 5; {
                [0; @:(@:1/37) = @:0 + 1] ~# ????????? ???????????? ?????? ?????? #~
                [1; @:1/31 = @:0 + 1] ~# ?????? ?????? ????????? ?????? #~
            }:(((@:(0 - 3)/11) > 47) & ((@:(0 - 3)/11) < 58))] ~# ????????? ????????? ????????? ?????? #~
            [@:(0 - 4)/11; @:(@:3/11) = @:0 + 1] ~# @:(0 - 3)/11??? NULL ????????? ?????? ?????? #~
            [@:1/31 + 0; @:(0 - 4)/11 = 0] ~# @:(0 - 3)/11 NULL CATCH ?????? ?????? ?????? #~
            [@:1/31 + 1; @:10/11 = @:10/11 * 10 + (@:(0 - 3)/11 - 48)] ~# ????????? ??? 10??? + ?????? ????????? ??? ?????? #~
            [@:1/31 + 2; @:(0 - 1)/11 = @:(0 - 1)/11 + 1] ~# ????????? ????????? 1 ?????? #~
            [@:1/31 + 3; @:1/29 = @:0 - 2] ~# @:1/29 + 3 ??? ?????? #~
            [@:1/37 + 0; @:(0 - 4)/11 = 0] ~# @:(0 - 3)/11 NULL CATCH ?????? ?????? ?????? #~
            [@:1/37 + 1; @:(@:3/11) = @:0 + 1] ~# ?????? ?????? #~
        }";

        // setup env
        let input = "4\n".as_bytes();
        let output = Arc::new(Mutex::new(Vec::new()));
        let mut env = Env {
            cursor: None,
            mem: vec![],
            is_debug: true,
            on_read_io: Box::new(|| input.to_vec()),
            on_write_io: Box::new(|data| output.lock().unwrap().extend(data)),
        };

        // run code
        run(code.to_string(), &mut env).unwrap();

        // test output
        assert_eq!(*output.lock().unwrap(), "16\n".as_bytes());
    }

    #[test]
    fn fizzbuzz() {
        // code to test
        let code = r"{
            [1/2; @:1/3 = {
                [0; 102]
                [1; 105]
                [2; 122]
                [3; 122]
            }]
            [2/3; @:2/3 = {
                [0; 98]
                [1; 117]
                [2; 122]
                [3; 122]
            }]
            [3/4; @:2 = 1]
            [4/5; @:1/2 = 1]
            [@:2; {
                [1; @:1 = @:1/3]
            }:(^((@:1/2)/3) ?= 1)]
            [@:2 + 1; {
                [1; @:1 = @:2/3]
            }:(^((@:1/2)/5) ?= 1)]
            [@:2 + 2; @:1/2 = @:1/2 + 1]
            [@:2 + 3; @:1 = {
                [0; 10]
            }]
            [@:2 + 4; {
                [1; @:2 = @:0 + 1]
            }:(@:1/2 < 100 + 1)]
        }";

        // setup env
        let input = "".as_bytes();
        let output = Arc::new(Mutex::new(Vec::new()));
        let mut env = Env {
            cursor: None,
            mem: vec![],
            is_debug: true,
            on_read_io: Box::new(|| input.to_vec()),
            on_write_io: Box::new(|data| output.lock().unwrap().extend(data)),
        };

        // run code
        run(code.to_string(), &mut env).unwrap();

        // test output
        assert_eq!(*output.lock().unwrap(), "\n\nfizz\n\nbuzz\nfizz\n\n\nfizz\nbuzz\n\nfizz\n\n\nfizzbuzz\n\n\nfizz\n\nbuzz\nfizz\n\n\nfizz\nbuzz\n\nfizz\n\n\nfizzbuzz\n\n\nfizz\n\nbuzz\nfizz\n\n\nfizz\nbuzz\n\nfizz\n\n\nfizzbuzz\n\n\nfizz\n\nbuzz\nfizz\n\n\nfizz\nbuzz\n\nfizz\n\n\nfizzbuzz\n\n\nfizz\n\nbuzz\nfizz\n\n\nfizz\nbuzz\n\nfizz\n\n\nfizzbuzz\n\n\nfizz\n\nbuzz\nfizz\n\n\nfizz\nbuzz\n\nfizz\n\n\nfizzbuzz\n\n\nfizz\n\nbuzz\nfizz\n\n\nfizz\nbuzz\n".as_bytes());
    }

    #[test]
    fn euler_1() {
        // code to test
        let code = r"~# http://euler.synap.co.kr/prob_detail.php?id=1
        10?????? ?????? ????????? ????????? 3 ?????? 5??? ????????? 3, 5, 6, 9 ??????, ????????? ?????? ????????? 23?????????.
        1000?????? ?????? ????????? ????????? 3 ?????? 5??? ????????? ?????? ????????? ????????????????
        #~
        
        {
            [1/2; @:2 = 1] ~# ?????? ???????????? ?????? #~
            [2/3; @:3/11 = 1] ~# 1?????? 1000?????? ????????? ?????? #~
            [3/4; @:4/11 = 0] ~# 3??? 5??? ????????? ????????? ?????? #~
            [@:2 + 0; { ~# @:3/11??? 3 ?????? 5??? ????????? ?????? @:4/11??? ????????? #~
                [1; @:4/11 = @:4/11 + @:3/11]
            }:((^((@:3/11)/3) ?= 1) | (^((@:3/11)/5) ?= 1))]
            [@:2 + 1; @:3/11 = @:3/11 + 1]  ~# @:3/11??? 1 ?????? #~
            [@:2 + 2; {
                [0; @:3 = @:0 + 1] ~# @:3/11??? 1000?????? ?????? ????????? @:3??? ????????? ?????? #~
                [1; @:2 = @:0 + 1] ~# @:3/11??? 1000?????? ????????? @:2??? ????????? ?????? #~
            }:(@:3/11 < 1000)]
            [@:3 + 0; @:2/7 = @:4/11] ~# ?????? @:1/3??? ???????????? ?????? ????????? ?????? ?????? #~
            [@:3 + 1; @:3/7 = 4] ~# ?????? @:1/3??? ?????? ??? ????????? ?????? 4 ?????? #~
            [@:3 + 2; @:1/3 = @:0 + 1] ~# ?????? @:1/3??? ?????? #~
            [@:4 + 0; @:1 = @:6/7] ~# ?????? @:1/3??? ????????? ?????? ????????? ?????? ????????? ?????? #~
            [@:4 + 1; @:1 = {
                [0; 10]
            }]
        
            ~# ??????: @:1/2 = ????????? ????????? ?????? #~
            ~# ??????: @:2/5 = ????????? ??????, @:3/5 = ?????? ?????? ??? ????????? ?????? #~
            ~# ??????: @:4/5 = ????????? ????????? ?????? #~
            [@:1/2; @:1/5 = @:0 + 3]
            [@:1/5 - 2; @:4/5 = 0] ~# ?????? ????????? 0??? ?????? #~
            [@:1/5 - 1; @:(0 - 1)/5 = @:2/5] ~# @:(0 - 1)/5 = ????????? ????????? ?????? ?????? ?????? #~
            [@:1/5 + 0; @:4/5 = @:4/5 + 1] ~# ?????? ????????? 1 ?????? #~
            [@:1/5 + 2; @:(0 - 1)/5 = (@:(0 - 1)/5) / 10] ~# @:(0 - 1)/5 ??? 10?????? ????????? ?????? ?????? #~
            [@:1/5 + 3; { ~# @:(0 - 1)/5 ??? 1?????? ????????? ????????? @:1/5??? ???????????? #~
                [1; @:1/5 = @:0 + 1]
            }:!(@:(0 - 1)/5 < 1)]
            [@:1/5 + 4; @:(@:3/5) = @:0 + 1] ~# ?????? ?????? #~
        
            ~# ??????: @:1/3 = ????????? ??????????????? ????????? ascii??? ????????? Bowl?????? ?????? #~
            ~# ??????: @:2/7 = ????????? ??????, @:3/7 = ?????? ?????? ??? ????????? ?????? #~
            ~# ??????: @:6/7 = ????????? Bowl #~
            [@:1/3; @:1/7 = @:0 + 1]
            [@:1/7 + 0; @:6/7 = {}] ~# ?????? ????????? ????????? Bowl ????????? #~
            [@:1/7 + 1; @:(0 - 1)/7 = @:2/7] ~# ????????? ?????? ?????? ?????? @:(0 - 1)/7 ??? ?????? #~
            [@:1/7 + 2; @:2/5 = @:2/7] ~# ?????? @:1/2??? ???????????? ?????? ????????? ?????? ?????? #~
            [@:1/7 + 3; @:3/5 = 1/13] ~# ?????? @:1/2??? ?????? ??? ????????? ?????? 1/13 ?????? #~
            [@:1/7 + 4; @:1/2 = @:0 + 1] ~# ?????? @:1/2??? ?????? #~
            [@:1/13 + 0; @:(0 - 2)/7 = @:4/5] ~# ?????? @:1/2 ?????? ??????(????????? ??????)??? ?????? ?????? @:(0 - 2)/7 ??? ?????? #~
            [@:1/13 + 1; @:(0 - 3)/7 = 0] ~# ????????? ????????? ?????? @:(0 - 3)/7 ????????? #~
            [@:1/13 + 2; { ~# ????????? ????????? 0?????? ?????? ?????? #~
                [0; @:1/17 = @:0 + 1] ~# ????????? ????????? 0??? ???????????? ?????? ?????? #~
                [1; @:(@:3/7) = @:0 + 1] ~# ????????? ????????? 0????????? ?????? ?????? #~
            }:(@:(0 - 2)/7 ?= 0)]
            [@:1/17 + 0; @:(0 - 4)/7 = (@:(0 - 1)/7 - @:(0 - 3)/7)/10] ~# @:(0 - 4)/7 = (???????????? 1 - ???????????? 3) / 10 #~
            [@:1/17 + 1; { ~# @:(0 - 4)/7 ??? ????????? 1??? ?????? @:(0 - 3)/7??? ?????? ????????? ????????? ????????????#~
                [0; @:1/19 = @:0 + 1] ~# @:(0 - 3)/7 ??? 1 ??????????????? ?????? ???????????? ????????? ?????? #~
                [1; @:1/23 = @:0 + 1] ~# @:(0 - 3)/7 ??? ????????? Bowl??? ???????????? ????????? ?????? #~
            }:(^(@:(0 - 4)/7) ?= 1)]
            [@:1/19 + 0; @:(0 - 3)/7 = @:(0 - 3)/7 + 1] ~# @:(0 - 3)/7 ??? 1 ???????????? #~
            [@:1/19 + 1; @:1/13 = @:0 - 1] ~# @:1/13 + 2 ??? ?????? #~
            [@:1/23 + 0; @:(0 - 2)/7 = @:(0 - 2)/7 - 1] ~# @:(0 - 2)/7 (????????? ??????)??? 1 ???????????? #~
            [@:1/23 + 1; (@:6/7):(@:(0 - 2)/7) = @:(0 - 3)/7 + 48] ~# ????????? Bowl??? (????????? ??????)????????? ?????? ???????????? ?????? #~
            [@:1/23 + 2; @:(0 - 1)/7 = @:(0 - 4)/7] ~# @:(0 - 1)/7 ??? ????????? ???????????? ???????????? ?????? #~
            [@:1/23 + 3; @:1/13 = @:0 - 0] ~# @:1/13 + 1 ??? ?????? (????????? ????????? ???????????? ?????????) #~
        }";

        // setup env
        let input = "".as_bytes();
        let output = Arc::new(Mutex::new(Vec::new()));
        let mut env = Env {
            cursor: None,
            mem: vec![],
            is_debug: true,
            on_read_io: Box::new(|| input.to_vec()),
            on_write_io: Box::new(|data| output.lock().unwrap().extend(data)),
        };

        // run code
        run(code.to_string(), &mut env).unwrap();

        // test output
        assert_eq!(*output.lock().unwrap(), "233168\n".as_bytes());
    }

    #[test]
    fn euler_2() {
        // code to test
        let code = r"~# http://euler.synap.co.kr/prob_detail.php?id=2
        ???????????? ????????? ??? ?????? ?????? ?????? ??? ??? ?????? ?????? ?????? ?????????. 1??? 2??? ???????????? ?????? ??? ????????? ????????? ????????????.
            1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
        ??????????????? 4?????? ????????? ?????? ?????? ????????? ????????? ??????????
        #~
        
        {
            [1/2; @:2 = 1] ~# ?????? ???????????? ?????? #~
            [2/3; @:2/11 = 1] ~# ???????????? ?????? 1#~
            [3/4; @:3/11 = 2] ~# ???????????? ?????? 2#~
            [4/5; @:4/11 = 0] ~# ?????? ????????? ?????? #~
            [@:2 + 0; { ~# ???????????? ?????? 2??? ????????? ?????? @:4/11??? ?????? #~
                [1; @:4/11 = @:4/11 + @:3/11]
            }:(^((@:3/11)/2) ?= 1)]
            [@:2 + 1; @:5/11 = @:2/11] ~# ????????????(@:5/11)??? ???????????? ?????? 1 ?????? #~
            [@:2 + 2; @:2/11 = @:3/11] ~# ???????????? ?????? 1??? ???????????? ?????? 2 ?????? #~
            [@:2 + 3; @:3/11 = @:3/11 + @:5/11] ~# ???????????? ?????? 2??? ???????????? ?????? 1 + ????????????(@:5/11) ?????? #~
            [@:2 + 4; {
                [0; @:3 = @:0 + 1] ~# @:3/11??? 4000001?????? ?????? ????????? @:3??? ????????? ?????? #~
                [1; @:2 = @:0 + 1] ~# @:3/11??? 4000001?????? ????????? @:2??? ????????? ?????? #~
            }:(@:3/11 < 4000001)]
            [@:3 + 0; @:2/7 = @:4/11] ~# ?????? @:1/3??? ???????????? ?????? ????????? ?????? ?????? #~
            [@:3 + 1; @:3/7 = 4] ~# ?????? @:1/3??? ?????? ??? ????????? ?????? 4 ?????? #~
            [@:3 + 2; @:1/3 = @:0 + 1] ~# ?????? @:1/3??? ?????? #~
            [@:4 + 0; @:1 = @:6/7] ~# ?????? @:1/3??? ????????? ?????? ????????? ?????? ????????? ?????? #~
            [@:4 + 1; @:1 = {
                [0; 10]
            }]
        
            ~# ??????: @:1/2 = ????????? ????????? ?????? #~
            ~# ??????: @:2/5 = ????????? ??????, @:3/5 = ?????? ?????? ??? ????????? ?????? #~
            ~# ??????: @:4/5 = ????????? ????????? ?????? #~
            [@:1/2; @:1/5 = @:0 + 3]
            [@:1/5 - 2; @:4/5 = 0] ~# ?????? ????????? 0??? ?????? #~
            [@:1/5 - 1; @:(0 - 1)/5 = @:2/5] ~# @:(0 - 1)/5 = ????????? ????????? ?????? ?????? ?????? #~
            [@:1/5 + 0; @:4/5 = @:4/5 + 1] ~# ?????? ????????? 1 ?????? #~
            [@:1/5 + 2; @:(0 - 1)/5 = (@:(0 - 1)/5) / 10] ~# @:(0 - 1)/5 ??? 10?????? ????????? ?????? ?????? #~
            [@:1/5 + 3; { ~# @:(0 - 1)/5 ??? 1?????? ????????? ????????? @:1/5??? ???????????? #~
                [1; @:1/5 = @:0 + 1]
            }:!(@:(0 - 1)/5 < 1)]
            [@:1/5 + 4; @:(@:3/5) = @:0 + 1] ~# ?????? ?????? #~
        
            ~# ??????: @:1/3 = ????????? ??????????????? ????????? ascii??? ????????? Bowl?????? ?????? #~
            ~# ??????: @:2/7 = ????????? ??????, @:3/7 = ?????? ?????? ??? ????????? ?????? #~
            ~# ??????: @:6/7 = ????????? Bowl #~
            [@:1/3; @:1/7 = @:0 + 1]
            [@:1/7 + 0; @:6/7 = {}] ~# ?????? ????????? ????????? Bowl ????????? #~
            [@:1/7 + 1; @:(0 - 1)/7 = @:2/7] ~# ????????? ?????? ?????? ?????? @:(0 - 1)/7 ??? ?????? #~
            [@:1/7 + 2; @:2/5 = @:2/7] ~# ?????? @:1/2??? ???????????? ?????? ????????? ?????? ?????? #~
            [@:1/7 + 3; @:3/5 = 1/13] ~# ?????? @:1/2??? ?????? ??? ????????? ?????? 1/13 ?????? #~
            [@:1/7 + 4; @:1/2 = @:0 + 1] ~# ?????? @:1/2??? ?????? #~
            [@:1/13 + 0; @:(0 - 2)/7 = @:4/5] ~# ?????? @:1/2 ?????? ??????(????????? ??????)??? ?????? ?????? @:(0 - 2)/7 ??? ?????? #~
            [@:1/13 + 1; @:(0 - 3)/7 = 0] ~# ????????? ????????? ?????? @:(0 - 3)/7 ????????? #~
            [@:1/13 + 2; { ~# ????????? ????????? 0?????? ?????? ?????? #~
                [0; @:1/17 = @:0 + 1] ~# ????????? ????????? 0??? ???????????? ?????? ?????? #~
                [1; @:(@:3/7) = @:0 + 1] ~# ????????? ????????? 0????????? ?????? ?????? #~
            }:(@:(0 - 2)/7 ?= 0)]
            [@:1/17 + 0; @:(0 - 4)/7 = (@:(0 - 1)/7 - @:(0 - 3)/7)/10] ~# @:(0 - 4)/7 = (???????????? 1 - ???????????? 3) / 10 #~
            [@:1/17 + 1; { ~# @:(0 - 4)/7 ??? ????????? 1??? ?????? @:(0 - 3)/7??? ?????? ????????? ????????? ????????????#~
                [0; @:1/19 = @:0 + 1] ~# @:(0 - 3)/7 ??? 1 ??????????????? ?????? ???????????? ????????? ?????? #~
                [1; @:1/23 = @:0 + 1] ~# @:(0 - 3)/7 ??? ????????? Bowl??? ???????????? ????????? ?????? #~
            }:(^(@:(0 - 4)/7) ?= 1)]
            [@:1/19 + 0; @:(0 - 3)/7 = @:(0 - 3)/7 + 1] ~# @:(0 - 3)/7 ??? 1 ???????????? #~
            [@:1/19 + 1; @:1/13 = @:0 - 1] ~# @:1/13 + 2 ??? ?????? #~
            [@:1/23 + 0; @:(0 - 2)/7 = @:(0 - 2)/7 - 1] ~# @:(0 - 2)/7 (????????? ??????)??? 1 ???????????? #~
            [@:1/23 + 1; (@:6/7):(@:(0 - 2)/7) = @:(0 - 3)/7 + 48] ~# ????????? Bowl??? (????????? ??????)????????? ?????? ???????????? ?????? #~
            [@:1/23 + 2; @:(0 - 1)/7 = @:(0 - 4)/7] ~# @:(0 - 1)/7 ??? ????????? ???????????? ???????????? ?????? #~
            [@:1/23 + 3; @:1/13 = @:0 - 0] ~# @:1/13 + 1 ??? ?????? (????????? ????????? ???????????? ?????????) #~
        }";

        // setup env
        let input = "".as_bytes();
        let output = Arc::new(Mutex::new(Vec::new()));
        let mut env = Env {
            cursor: None,
            mem: vec![],
            is_debug: true,
            on_read_io: Box::new(|| input.to_vec()),
            on_write_io: Box::new(|data| output.lock().unwrap().extend(data)),
        };

        // run code
        run(code.to_string(), &mut env).unwrap();

        // test output
        assert_eq!(*output.lock().unwrap(), "4613732\n".as_bytes());
    }

    #[test]
    fn euler_3() {
        // code to test
        let code = r"~# http://euler.synap.co.kr/prob_detail.php?id=3
        ?????? ?????? ????????? ???????????? ???????????? ?????? ?????????????????? ??????, ??? ???????????? ??? ?????? ??????????????? ?????????.
        ?????? ?????? 13195??? ???????????? 5, 7, 13, 29 ?????????.
        600851475143??? ????????? ????????? ?????? ??? ?????? ????????????.
        #~
        
        {
            [1/2; @:2 = 1] ~# ?????? ???????????? ?????? #~
            [2/3; @:2/11 = 600851475143] ~# ?????????????????? ??? #~
            [3/4; @:3/11 = 2] ~# ?????? ????????? #~
            [@:2 + 0; { ~# ???????????? ?????? 2??? ????????? ?????? @:4/11??? ?????? #~
                [0; { ~# ?????? ?????? ???????????? ????????? ???????????? ?????? ?????? #~
                    [0; @:3/11 = @:3/11 + 2] ~# ?????? ?????? ???????????? 2??? ????????? 2??? ?????? #~
                    [1; @:3/11 = @:3/11 + 1] ~# ?????? ?????? ???????????? 2?????? 1??? ?????? #~
                }:(@:3/11 ?= 2)]
                [1; @:2/11 = (@:2/11)/(@:3/11)] ~# ?????? ?????? ???????????? ????????? ???????????? ?????? ?????????????????? ?????? ?????? ???????????? ?????? ?????? ?????????????????? ?????? ?????? #~
            }:(^((@:2/11)/(@:3/11)) ?= 1)]
            [@:2 + 1; {
                [0; @:3 = @:0 + 1] ~# @:2/11??? @:3/11?????? ?????? ????????? @:3??? ????????? ?????? #~
                [1; @:2 = @:0 + 1] ~# @:3/11??? @:3/11?????? ?????? @:2??? ????????? ?????? #~
            }:(@:2/11 > @:3/11)]
            [@:3 + 0; @:2/7 = @:3/11] ~# ?????? @:1/3??? ???????????? ?????? ????????? ?????? ?????? #~
            [@:3 + 1; @:3/7 = 4] ~# ?????? @:1/3??? ?????? ??? ????????? ?????? 4 ?????? #~
            [@:3 + 2; @:1/3 = @:0 + 1] ~# ?????? @:1/3??? ?????? #~
            [@:4 + 0; @:1 = @:6/7] ~# ?????? @:1/3??? ????????? ?????? ????????? ?????? ????????? ?????? #~
            [@:4 + 1; @:1 = {
                [0; 10]
            }]
        
            ~# ??????: @:1/2 = ????????? ????????? ?????? #~
            ~# ??????: @:2/5 = ????????? ??????, @:3/5 = ?????? ?????? ??? ????????? ?????? #~
            ~# ??????: @:4/5 = ????????? ????????? ?????? #~
            [@:1/2; @:1/5 = @:0 + 3]
            [@:1/5 - 2; @:4/5 = 0] ~# ?????? ????????? 0??? ?????? #~
            [@:1/5 - 1; @:(0 - 1)/5 = @:2/5] ~# @:(0 - 1)/5 = ????????? ????????? ?????? ?????? ?????? #~
            [@:1/5 + 0; @:4/5 = @:4/5 + 1] ~# ?????? ????????? 1 ?????? #~
            [@:1/5 + 2; @:(0 - 1)/5 = (@:(0 - 1)/5) / 10] ~# @:(0 - 1)/5 ??? 10?????? ????????? ?????? ?????? #~
            [@:1/5 + 3; { ~# @:(0 - 1)/5 ??? 1?????? ????????? ????????? @:1/5??? ???????????? #~
                [1; @:1/5 = @:0 + 1]
            }:!(@:(0 - 1)/5 < 1)]
            [@:1/5 + 4; @:(@:3/5) = @:0 + 1] ~# ?????? ?????? #~
        
            ~# ??????: @:1/3 = ????????? ??????????????? ????????? ascii??? ????????? Bowl?????? ?????? #~
            ~# ??????: @:2/7 = ????????? ??????, @:3/7 = ?????? ?????? ??? ????????? ?????? #~
            ~# ??????: @:6/7 = ????????? Bowl #~
            [@:1/3; @:1/7 = @:0 + 1]
            [@:1/7 + 0; @:6/7 = {}] ~# ?????? ????????? ????????? Bowl ????????? #~
            [@:1/7 + 1; @:(0 - 1)/7 = @:2/7] ~# ????????? ?????? ?????? ?????? @:(0 - 1)/7 ??? ?????? #~
            [@:1/7 + 2; @:2/5 = @:2/7] ~# ?????? @:1/2??? ???????????? ?????? ????????? ?????? ?????? #~
            [@:1/7 + 3; @:3/5 = 1/13] ~# ?????? @:1/2??? ?????? ??? ????????? ?????? 1/13 ?????? #~
            [@:1/7 + 4; @:1/2 = @:0 + 1] ~# ?????? @:1/2??? ?????? #~
            [@:1/13 + 0; @:(0 - 2)/7 = @:4/5] ~# ?????? @:1/2 ?????? ??????(????????? ??????)??? ?????? ?????? @:(0 - 2)/7 ??? ?????? #~
            [@:1/13 + 1; @:(0 - 3)/7 = 0] ~# ????????? ????????? ?????? @:(0 - 3)/7 ????????? #~
            [@:1/13 + 2; { ~# ????????? ????????? 0?????? ?????? ?????? #~
                [0; @:1/17 = @:0 + 1] ~# ????????? ????????? 0??? ???????????? ?????? ?????? #~
                [1; @:(@:3/7) = @:0 + 1] ~# ????????? ????????? 0????????? ?????? ?????? #~
            }:(@:(0 - 2)/7 ?= 0)]
            [@:1/17 + 0; @:(0 - 4)/7 = (@:(0 - 1)/7 - @:(0 - 3)/7)/10] ~# @:(0 - 4)/7 = (???????????? 1 - ???????????? 3) / 10 #~
            [@:1/17 + 1; { ~# @:(0 - 4)/7 ??? ????????? 1??? ?????? @:(0 - 3)/7??? ?????? ????????? ????????? ????????????#~
                [0; @:1/19 = @:0 + 1] ~# @:(0 - 3)/7 ??? 1 ??????????????? ?????? ???????????? ????????? ?????? #~
                [1; @:1/23 = @:0 + 1] ~# @:(0 - 3)/7 ??? ????????? Bowl??? ???????????? ????????? ?????? #~
            }:(^(@:(0 - 4)/7) ?= 1)]
            [@:1/19 + 0; @:(0 - 3)/7 = @:(0 - 3)/7 + 1] ~# @:(0 - 3)/7 ??? 1 ???????????? #~
            [@:1/19 + 1; @:1/13 = @:0 - 1] ~# @:1/13 + 2 ??? ?????? #~
            [@:1/23 + 0; @:(0 - 2)/7 = @:(0 - 2)/7 - 1] ~# @:(0 - 2)/7 (????????? ??????)??? 1 ???????????? #~
            [@:1/23 + 1; (@:6/7):(@:(0 - 2)/7) = @:(0 - 3)/7 + 48] ~# ????????? Bowl??? (????????? ??????)????????? ?????? ???????????? ?????? #~
            [@:1/23 + 2; @:(0 - 1)/7 = @:(0 - 4)/7] ~# @:(0 - 1)/7 ??? ????????? ???????????? ???????????? ?????? #~
            [@:1/23 + 3; @:1/13 = @:0 - 0] ~# @:1/13 + 1 ??? ?????? (????????? ????????? ???????????? ?????????) #~
        }";

        // setup env
        let input = "".as_bytes();
        let output = Arc::new(Mutex::new(Vec::new()));
        let mut env = Env {
            cursor: None,
            mem: vec![],
            is_debug: true,
            on_read_io: Box::new(|| input.to_vec()),
            on_write_io: Box::new(|data| output.lock().unwrap().extend(data)),
        };

        // run code
        run(code.to_string(), &mut env).unwrap();

        // test output
        assert_eq!(*output.lock().unwrap(), "6857\n".as_bytes());
    }
}
