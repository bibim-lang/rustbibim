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
            [0; @:2/7 = 112873] ~# 함수 @:1/3을 호출하기 위해 출력할 정수 대입 #~
            [1; @:3/7 = 2] ~# 함수 @:1/3을 호출 후 이동할 위치 2 대입 #~
            [2; @:1/3 = @:0 + 1] ~# 함수 @:1/3을 호출 #~
            [@:2; @:1 = @:6/7] ~# 함수 @:1/3의 결과로 얻은 변환된 정수 문자열 출력 #~
        
            ~# 함수: @:1/2 = 정수의 길이를 반환 #~
            ~# 입력: @:2/5 = 확인할 정수, @:3/5 = 함수 종료 후 이동할 위치 #~
            ~# 출력: @:4/5 = 확인한 정수의 길이 #~
            [@:1/2; @:1/5 = @:0 + 3]
            [@:1/5 - 2; @:4/5 = 0] ~# 출력 결과에 0을 대입 #~
            [@:1/5 - 1; @:(0 - 1)/5 = @:2/5] ~# @:(0 - 1)/5 = 확인할 변수를 담은 임시 변수 #~
            [@:1/5 + 0; @:4/5 = @:4/5 + 1] ~# 출력 결과를 1 증가 #~
            [@:1/5 + 2; @:(0 - 1)/5 = (@:(0 - 1)/5) / 10] ~# @:(0 - 1)/5 를 10으로 나누어 다시 저장 #~
            [@:1/5 + 3; { ~# @:(0 - 1)/5 가 1보다 크거나 같으면 @:1/5로 되돌아감 #~
                [1; @:1/5 = @:0 + 1]
            }:!(@:(0 - 1)/5 < 1)]
            [@:1/5 + 4; @:(@:3/5) = @:0 + 1] ~# 함수 탈출 #~
        
            ~# 함수: @:1/3 = 정수를 자릿수별로 나눠서 ascii로 변환한 Bowl으로 반환 #~
            ~# 입력: @:2/7 = 변환할 정수, @:3/7 = 함수 종료 후 이동할 위치 #~
            ~# 출력: @:6/7 = 변환된 Bowl #~
            [@:1/3; @:1/7 = @:0 + 1]
            [@:1/7 + 0; @:6/7 = {}] ~# 변환 결과를 저장할 Bowl 초기화 #~
            [@:1/7 + 1; @:(0 - 1)/7 = @:2/7] ~# 변환할 정수 임시 변수 @:(0 - 1)/7 에 복사 #~
            [@:1/7 + 2; @:2/5 = @:2/7] ~# 함수 @:1/2를 호출하기 위해 확인할 정수 대입 #~
            [@:1/7 + 3; @:3/5 = 1/13] ~# 함수 @:1/2를 호출 후 이동할 위치 1/13 지정 #~
            [@:1/7 + 4; @:1/2 = @:0 + 1] ~# 함수 @:1/2를 호출 #~
            [@:1/13 + 0; @:(0 - 2)/7 = @:4/5] ~# 함수 @:1/2 호출 결과(정수의 길이)를 임시 변수 @:(0 - 2)/7 에 저장 #~
            [@:1/13 + 1; @:(0 - 3)/7 = 0] ~# 자리수 확인용 변수 @:(0 - 3)/7 초기화 #~
            [@:1/13 + 2; { ~# 정수의 길이가 0이면 함수 종료 #~
                [0; @:1/17 = @:0 + 1] ~# 정수의 길이가 0이 아니므로 계속 실행 #~
                [1; @:(@:3/7) = @:0 + 1] ~# 정수의 길이가 0이므로 함수 종료 #~
            }:(@:(0 - 2)/7 ?= 0)]
            [@:1/17 + 0; @:(0 - 4)/7 = (@:(0 - 1)/7 - @:(0 - 3)/7)/10] ~# @:(0 - 4)/7 = (임시변수 1 - 임시변수 3) / 10 #~
            [@:1/17 + 1; { ~# @:(0 - 4)/7 의 분모가 1인 경우 @:(0 - 3)/7이 현재 마지막 자리의 자릿값임#~
                [0; @:1/19 = @:0 + 1] ~# @:(0 - 3)/7 를 1 증가시켜서 다시 확인하는 코드로 이동 #~
                [1; @:1/23 = @:0 + 1] ~# @:(0 - 3)/7 를 반환할 Bowl에 추가하는 코드로 이동 #~
            }:(^(@:(0 - 4)/7) ?= 1)]
            [@:1/19 + 0; @:(0 - 3)/7 = @:(0 - 3)/7 + 1] ~# @:(0 - 3)/7 를 1 증가시킴 #~
            [@:1/19 + 1; @:1/13 = @:0 - 1] ~# @:1/13 + 2 로 이동 #~
            [@:1/23 + 0; @:(0 - 2)/7 = @:(0 - 2)/7 - 1] ~# @:(0 - 2)/7 (정수의 길이)를 1 감소시킴 #~
            [@:1/23 + 1; (@:6/7):(@:(0 - 2)/7) = @:(0 - 3)/7 + 48] ~# 반환할 Bowl의 (정수의 길이)번째에 현재 자리값을 대입 #~
            [@:1/23 + 2; @:(0 - 1)/7 = @:(0 - 4)/7] ~# @:(0 - 1)/7 의 마지막 자리수를 제거하여 대입 #~
            [@:1/23 + 3; @:1/13 = @:0 - 0] ~# @:1/13 + 1 로 이동 (자리값 초기화 수행하는 자리로) #~
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
            [1/2; @:2/11 = @:1] ~# STDIN으로 수 입력받음 #~
            [2/3; @:3/11 = 2] ~# 함수 @:1/3을 호출 후 이동할 위치 2 대입 #~
            [3/4; @:1/11 = @:0 + 1] ~# 함수 @:1/11 호출 #~
            [@:2 + 0; @:2/7 = @:10/11 * @:10/11] ~# 함수 @:1/11 호출 결과를 제곱하여 @:2/7에 대입 #~
            [@:2 + 1; @:3/7 = 3] ~# 함수 @:1/3을 호출 후 이동할 위치 3 대입 #~
            [@:2 + 2; @:1/3 = @:0 + 1] ~# 함수 @:1/3을 호출 #~
            [@:3 + 0; @:1 = @:6/7] ~# 함수 @:1/3의 결과로 얻은 변환된 정수 문자열 출력 #~
            [@:3 + 1; @:1 = {
                [0; 10]
            }]
        
            ~# 함수: @:1/2 = 정수의 길이를 반환 #~
            ~# 입력: @:2/5 = 확인할 정수, @:3/5 = 함수 종료 후 이동할 위치 #~
            ~# 출력: @:4/5 = 확인한 정수의 길이 #~
            [@:1/2; @:1/5 = @:0 + 3]
            [@:1/5 - 2; @:4/5 = 0] ~# 출력 결과에 0을 대입 #~
            [@:1/5 - 1; @:(0 - 1)/5 = @:2/5] ~# @:(0 - 1)/5 = 확인할 변수를 담은 임시 변수 #~
            [@:1/5 + 0; @:4/5 = @:4/5 + 1] ~# 출력 결과를 1 증가 #~
            [@:1/5 + 2; @:(0 - 1)/5 = (@:(0 - 1)/5) / 10] ~# @:(0 - 1)/5 를 10으로 나누어 다시 저장 #~
            [@:1/5 + 3; { ~# @:(0 - 1)/5 가 1보다 크거나 같으면 @:1/5로 되돌아감 #~
                [1; @:1/5 = @:0 + 1]
            }:!(@:(0 - 1)/5 < 1)]
            [@:1/5 + 4; @:(@:3/5) = @:0 + 1] ~# 함수 탈출 #~
        
            ~# 함수: @:1/3 = 정수를 자릿수별로 나눠서 ascii로 변환한 Bowl으로 반환 #~
            ~# 입력: @:2/7 = 변환할 정수, @:3/7 = 함수 종료 후 이동할 위치 #~
            ~# 출력: @:6/7 = 변환된 Bowl #~
            [@:1/3; @:1/7 = @:0 + 1]
            [@:1/7 + 0; @:6/7 = {}] ~# 변환 결과를 저장할 Bowl 초기화 #~
            [@:1/7 + 1; @:(0 - 1)/7 = @:2/7] ~# 변환할 정수 임시 변수 @:(0 - 1)/7 에 복사 #~
            [@:1/7 + 2; @:2/5 = @:2/7] ~# 함수 @:1/2를 호출하기 위해 확인할 정수 대입 #~
            [@:1/7 + 3; @:3/5 = 1/13] ~# 함수 @:1/2를 호출 후 이동할 위치 1/13 지정 #~
            [@:1/7 + 4; @:1/2 = @:0 + 1] ~# 함수 @:1/2를 호출 #~
            [@:1/13 + 0; @:(0 - 2)/7 = @:4/5] ~# 함수 @:1/2 호출 결과(정수의 길이)를 임시 변수 @:(0 - 2)/7 에 저장 #~
            [@:1/13 + 1; @:(0 - 3)/7 = 0] ~# 자리수 확인용 변수 @:(0 - 3)/7 초기화 #~
            [@:1/13 + 2; { ~# 정수의 길이가 0이면 함수 종료 #~
                [0; @:1/17 = @:0 + 1] ~# 정수의 길이가 0이 아니므로 계속 실행 #~
                [1; @:(@:3/7) = @:0 + 1] ~# 정수의 길이가 0이므로 함수 종료 #~
            }:(@:(0 - 2)/7 ?= 0)]
            [@:1/17 + 0; @:(0 - 4)/7 = (@:(0 - 1)/7 - @:(0 - 3)/7)/10] ~# @:(0 - 4)/7 = (임시변수 1 - 임시변수 3) / 10 #~
            [@:1/17 + 1; { ~# @:(0 - 4)/7 의 분모가 1인 경우 @:(0 - 3)/7이 현재 마지막 자리의 자릿값임#~
                [0; @:1/19 = @:0 + 1] ~# @:(0 - 3)/7 를 1 증가시켜서 다시 확인하는 코드로 이동 #~
                [1; @:1/23 = @:0 + 1] ~# @:(0 - 3)/7 를 반환할 Bowl에 추가하는 코드로 이동 #~
            }:(^(@:(0 - 4)/7) ?= 1)]
            [@:1/19 + 0; @:(0 - 3)/7 = @:(0 - 3)/7 + 1] ~# @:(0 - 3)/7 를 1 증가시킴 #~
            [@:1/19 + 1; @:1/13 = @:0 - 1] ~# @:1/13 + 2 로 이동 #~
            [@:1/23 + 0; @:(0 - 2)/7 = @:(0 - 2)/7 - 1] ~# @:(0 - 2)/7 (정수의 길이)를 1 감소시킴 #~
            [@:1/23 + 1; (@:6/7):(@:(0 - 2)/7) = @:(0 - 3)/7 + 48] ~# 반환할 Bowl의 (정수의 길이)번째에 현재 자리값을 대입 #~
            [@:1/23 + 2; @:(0 - 1)/7 = @:(0 - 4)/7] ~# @:(0 - 1)/7 의 마지막 자리수를 제거하여 대입 #~
            [@:1/23 + 3; @:1/13 = @:0 - 0] ~# @:1/13 + 1 로 이동 (자리값 초기화 수행하는 자리로) #~
        
            ~# 함수: @:1/11 = 문자열을 정수로 변환 #~
            ~# 입력: @:2/11 = 변환할 문자열, @:3/11 = 함수 종료 후 이동할 위치 #~
            ~# 출력: @:10/11 = 반환할 정수 #~
            [@:1/11; @:1/29 = @:0 + 1]
            [@:1/29 + 0; @:10/11 = 0] ~# 반환할 값에 0 대입 #~
            [@:1/29 + 1; @:(0 - 1)/11 = 0] ~# @:(0 - 1)/11 = 문자열 인덱스를 담는 임시 변수 #~
            [@:1/29 + 2; @:(0 - 2)/11 = @:2/11] ~# @:(0 - 2)/11 = 복사한 문자열을 담은 임시 변수 #~
            [@:1/29 + 3; @:(0 - 3)/11 = (@:(0 - 2)/11):(@:(0 - 1)/11)] ~# @:(0 - 3)/11 = 현재 인덱스 위치의 값 #~
            [@:1/29 + 4; @:(0 - 4)/11 = @:0 + 3] ~# @:(0 - 3)/11가 NULL 일 경우 CATCH할 위치 #~
            [@:1/29 + 5; {
                [0; @:(@:1/37) = @:0 + 1] ~# 숫자가 아니므로 함수 종료 #~
                [1; @:1/31 = @:0 + 1] ~# 정수 처리 코드로 이동 #~
            }:(((@:(0 - 3)/11) > 47) & ((@:(0 - 3)/11) < 58))] ~# 복사한 문자열 인덱스 확인 #~
            [@:(0 - 4)/11; @:(@:3/11) = @:0 + 1] ~# @:(0 - 3)/11가 NULL 이므로 함수 종료 #~
            [@:1/31 + 0; @:(0 - 4)/11 = 0] ~# @:(0 - 3)/11 NULL CATCH 위치 등록 취소 #~
            [@:1/31 + 1; @:10/11 = @:10/11 * 10 + (@:(0 - 3)/11 - 48)] ~# 반환할 값 10배 + 현재 인덱스 값 대입 #~
            [@:1/31 + 2; @:(0 - 1)/11 = @:(0 - 1)/11 + 1] ~# 문자열 인덱스 1 증가 #~
            [@:1/31 + 3; @:1/29 = @:0 - 2] ~# @:1/29 + 3 로 이동 #~
            [@:1/37 + 0; @:(0 - 4)/11 = 0] ~# @:(0 - 3)/11 NULL CATCH 위치 등록 취소 #~
            [@:1/37 + 1; @:(@:3/11) = @:0 + 1] ~# 함수 종료 #~
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
        10보다 작은 자연수 중에서 3 또는 5의 배수는 3, 5, 6, 9 이고, 이것을 모두 더하면 23입니다.
        1000보다 작은 자연수 중에서 3 또는 5의 배수를 모두 더하면 얼마일까요?
        #~
        
        {
            [1/2; @:2 = 1] ~# 로직 시작지점 설정 #~
            [2/3; @:3/11 = 1] ~# 1부터 1000까지 증가할 변수 #~
            [3/4; @:4/11 = 0] ~# 3과 5의 배수를 저장할 변수 #~
            [@:2 + 0; { ~# @:3/11이 3 또는 5의 배수일 경우 @:4/11에 더한다 #~
                [1; @:4/11 = @:4/11 + @:3/11]
            }:((^((@:3/11)/3) ?= 1) | (^((@:3/11)/5) ?= 1))]
            [@:2 + 1; @:3/11 = @:3/11 + 1]  ~# @:3/11을 1 증가 #~
            [@:2 + 2; {
                [0; @:3 = @:0 + 1] ~# @:3/11이 1000보다 작지 않으면 @:3를 다음에 시작 #~
                [1; @:2 = @:0 + 1] ~# @:3/11이 1000보다 작으면 @:2를 다음에 시작 #~
            }:(@:3/11 < 1000)]
            [@:3 + 0; @:2/7 = @:4/11] ~# 함수 @:1/3을 호출하기 위해 출력할 정수 대입 #~
            [@:3 + 1; @:3/7 = 4] ~# 함수 @:1/3을 호출 후 이동할 위치 4 대입 #~
            [@:3 + 2; @:1/3 = @:0 + 1] ~# 함수 @:1/3을 호출 #~
            [@:4 + 0; @:1 = @:6/7] ~# 함수 @:1/3의 결과로 얻은 변환된 정수 문자열 출력 #~
            [@:4 + 1; @:1 = {
                [0; 10]
            }]
        
            ~# 함수: @:1/2 = 정수의 길이를 반환 #~
            ~# 입력: @:2/5 = 확인할 정수, @:3/5 = 함수 종료 후 이동할 위치 #~
            ~# 출력: @:4/5 = 확인한 정수의 길이 #~
            [@:1/2; @:1/5 = @:0 + 3]
            [@:1/5 - 2; @:4/5 = 0] ~# 출력 결과에 0을 대입 #~
            [@:1/5 - 1; @:(0 - 1)/5 = @:2/5] ~# @:(0 - 1)/5 = 확인할 변수를 담은 임시 변수 #~
            [@:1/5 + 0; @:4/5 = @:4/5 + 1] ~# 출력 결과를 1 증가 #~
            [@:1/5 + 2; @:(0 - 1)/5 = (@:(0 - 1)/5) / 10] ~# @:(0 - 1)/5 를 10으로 나누어 다시 저장 #~
            [@:1/5 + 3; { ~# @:(0 - 1)/5 가 1보다 크거나 같으면 @:1/5로 되돌아감 #~
                [1; @:1/5 = @:0 + 1]
            }:!(@:(0 - 1)/5 < 1)]
            [@:1/5 + 4; @:(@:3/5) = @:0 + 1] ~# 함수 탈출 #~
        
            ~# 함수: @:1/3 = 정수를 자릿수별로 나눠서 ascii로 변환한 Bowl으로 반환 #~
            ~# 입력: @:2/7 = 변환할 정수, @:3/7 = 함수 종료 후 이동할 위치 #~
            ~# 출력: @:6/7 = 변환된 Bowl #~
            [@:1/3; @:1/7 = @:0 + 1]
            [@:1/7 + 0; @:6/7 = {}] ~# 변환 결과를 저장할 Bowl 초기화 #~
            [@:1/7 + 1; @:(0 - 1)/7 = @:2/7] ~# 변환할 정수 임시 변수 @:(0 - 1)/7 에 복사 #~
            [@:1/7 + 2; @:2/5 = @:2/7] ~# 함수 @:1/2를 호출하기 위해 확인할 정수 대입 #~
            [@:1/7 + 3; @:3/5 = 1/13] ~# 함수 @:1/2를 호출 후 이동할 위치 1/13 지정 #~
            [@:1/7 + 4; @:1/2 = @:0 + 1] ~# 함수 @:1/2를 호출 #~
            [@:1/13 + 0; @:(0 - 2)/7 = @:4/5] ~# 함수 @:1/2 호출 결과(정수의 길이)를 임시 변수 @:(0 - 2)/7 에 저장 #~
            [@:1/13 + 1; @:(0 - 3)/7 = 0] ~# 자리수 확인용 변수 @:(0 - 3)/7 초기화 #~
            [@:1/13 + 2; { ~# 정수의 길이가 0이면 함수 종료 #~
                [0; @:1/17 = @:0 + 1] ~# 정수의 길이가 0이 아니므로 계속 실행 #~
                [1; @:(@:3/7) = @:0 + 1] ~# 정수의 길이가 0이므로 함수 종료 #~
            }:(@:(0 - 2)/7 ?= 0)]
            [@:1/17 + 0; @:(0 - 4)/7 = (@:(0 - 1)/7 - @:(0 - 3)/7)/10] ~# @:(0 - 4)/7 = (임시변수 1 - 임시변수 3) / 10 #~
            [@:1/17 + 1; { ~# @:(0 - 4)/7 의 분모가 1인 경우 @:(0 - 3)/7이 현재 마지막 자리의 자릿값임#~
                [0; @:1/19 = @:0 + 1] ~# @:(0 - 3)/7 를 1 증가시켜서 다시 확인하는 코드로 이동 #~
                [1; @:1/23 = @:0 + 1] ~# @:(0 - 3)/7 를 반환할 Bowl에 추가하는 코드로 이동 #~
            }:(^(@:(0 - 4)/7) ?= 1)]
            [@:1/19 + 0; @:(0 - 3)/7 = @:(0 - 3)/7 + 1] ~# @:(0 - 3)/7 를 1 증가시킴 #~
            [@:1/19 + 1; @:1/13 = @:0 - 1] ~# @:1/13 + 2 로 이동 #~
            [@:1/23 + 0; @:(0 - 2)/7 = @:(0 - 2)/7 - 1] ~# @:(0 - 2)/7 (정수의 길이)를 1 감소시킴 #~
            [@:1/23 + 1; (@:6/7):(@:(0 - 2)/7) = @:(0 - 3)/7 + 48] ~# 반환할 Bowl의 (정수의 길이)번째에 현재 자리값을 대입 #~
            [@:1/23 + 2; @:(0 - 1)/7 = @:(0 - 4)/7] ~# @:(0 - 1)/7 의 마지막 자리수를 제거하여 대입 #~
            [@:1/23 + 3; @:1/13 = @:0 - 0] ~# @:1/13 + 1 로 이동 (자리값 초기화 수행하는 자리로) #~
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
        피보나치 수열의 각 항은 바로 앞의 항 두 개를 더한 것이 됩니다. 1과 2로 시작하는 경우 이 수열은 아래와 같습니다.
            1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
        짝수이면서 4백만 이하인 모든 항을 더하면 얼마가 됩니까?
        #~
        
        {
            [1/2; @:2 = 1] ~# 로직 시작지점 설정 #~
            [2/3; @:2/11 = 1] ~# 피보나치 변수 1#~
            [3/4; @:3/11 = 2] ~# 피보나치 변수 2#~
            [4/5; @:4/11 = 0] ~# 합을 저장할 변수 #~
            [@:2 + 0; { ~# 피보나치 변수 2가 짝수일 경우 @:4/11에 더함 #~
                [1; @:4/11 = @:4/11 + @:3/11]
            }:(^((@:3/11)/2) ?= 1)]
            [@:2 + 1; @:5/11 = @:2/11] ~# 임시변수(@:5/11)에 피보나치 변수 1 대입 #~
            [@:2 + 2; @:2/11 = @:3/11] ~# 피보나치 변수 1에 피보나치 변수 2 대입 #~
            [@:2 + 3; @:3/11 = @:3/11 + @:5/11] ~# 피보나치 변수 2에 피보나치 변수 1 + 임시변수(@:5/11) 대입 #~
            [@:2 + 4; {
                [0; @:3 = @:0 + 1] ~# @:3/11이 4000001보다 작지 않으면 @:3를 다음에 시작 #~
                [1; @:2 = @:0 + 1] ~# @:3/11이 4000001보다 작으면 @:2를 다음에 시작 #~
            }:(@:3/11 < 4000001)]
            [@:3 + 0; @:2/7 = @:4/11] ~# 함수 @:1/3을 호출하기 위해 출력할 정수 대입 #~
            [@:3 + 1; @:3/7 = 4] ~# 함수 @:1/3을 호출 후 이동할 위치 4 대입 #~
            [@:3 + 2; @:1/3 = @:0 + 1] ~# 함수 @:1/3을 호출 #~
            [@:4 + 0; @:1 = @:6/7] ~# 함수 @:1/3의 결과로 얻은 변환된 정수 문자열 출력 #~
            [@:4 + 1; @:1 = {
                [0; 10]
            }]
        
            ~# 함수: @:1/2 = 정수의 길이를 반환 #~
            ~# 입력: @:2/5 = 확인할 정수, @:3/5 = 함수 종료 후 이동할 위치 #~
            ~# 출력: @:4/5 = 확인한 정수의 길이 #~
            [@:1/2; @:1/5 = @:0 + 3]
            [@:1/5 - 2; @:4/5 = 0] ~# 출력 결과에 0을 대입 #~
            [@:1/5 - 1; @:(0 - 1)/5 = @:2/5] ~# @:(0 - 1)/5 = 확인할 변수를 담은 임시 변수 #~
            [@:1/5 + 0; @:4/5 = @:4/5 + 1] ~# 출력 결과를 1 증가 #~
            [@:1/5 + 2; @:(0 - 1)/5 = (@:(0 - 1)/5) / 10] ~# @:(0 - 1)/5 를 10으로 나누어 다시 저장 #~
            [@:1/5 + 3; { ~# @:(0 - 1)/5 가 1보다 크거나 같으면 @:1/5로 되돌아감 #~
                [1; @:1/5 = @:0 + 1]
            }:!(@:(0 - 1)/5 < 1)]
            [@:1/5 + 4; @:(@:3/5) = @:0 + 1] ~# 함수 탈출 #~
        
            ~# 함수: @:1/3 = 정수를 자릿수별로 나눠서 ascii로 변환한 Bowl으로 반환 #~
            ~# 입력: @:2/7 = 변환할 정수, @:3/7 = 함수 종료 후 이동할 위치 #~
            ~# 출력: @:6/7 = 변환된 Bowl #~
            [@:1/3; @:1/7 = @:0 + 1]
            [@:1/7 + 0; @:6/7 = {}] ~# 변환 결과를 저장할 Bowl 초기화 #~
            [@:1/7 + 1; @:(0 - 1)/7 = @:2/7] ~# 변환할 정수 임시 변수 @:(0 - 1)/7 에 복사 #~
            [@:1/7 + 2; @:2/5 = @:2/7] ~# 함수 @:1/2를 호출하기 위해 확인할 정수 대입 #~
            [@:1/7 + 3; @:3/5 = 1/13] ~# 함수 @:1/2를 호출 후 이동할 위치 1/13 지정 #~
            [@:1/7 + 4; @:1/2 = @:0 + 1] ~# 함수 @:1/2를 호출 #~
            [@:1/13 + 0; @:(0 - 2)/7 = @:4/5] ~# 함수 @:1/2 호출 결과(정수의 길이)를 임시 변수 @:(0 - 2)/7 에 저장 #~
            [@:1/13 + 1; @:(0 - 3)/7 = 0] ~# 자리수 확인용 변수 @:(0 - 3)/7 초기화 #~
            [@:1/13 + 2; { ~# 정수의 길이가 0이면 함수 종료 #~
                [0; @:1/17 = @:0 + 1] ~# 정수의 길이가 0이 아니므로 계속 실행 #~
                [1; @:(@:3/7) = @:0 + 1] ~# 정수의 길이가 0이므로 함수 종료 #~
            }:(@:(0 - 2)/7 ?= 0)]
            [@:1/17 + 0; @:(0 - 4)/7 = (@:(0 - 1)/7 - @:(0 - 3)/7)/10] ~# @:(0 - 4)/7 = (임시변수 1 - 임시변수 3) / 10 #~
            [@:1/17 + 1; { ~# @:(0 - 4)/7 의 분모가 1인 경우 @:(0 - 3)/7이 현재 마지막 자리의 자릿값임#~
                [0; @:1/19 = @:0 + 1] ~# @:(0 - 3)/7 를 1 증가시켜서 다시 확인하는 코드로 이동 #~
                [1; @:1/23 = @:0 + 1] ~# @:(0 - 3)/7 를 반환할 Bowl에 추가하는 코드로 이동 #~
            }:(^(@:(0 - 4)/7) ?= 1)]
            [@:1/19 + 0; @:(0 - 3)/7 = @:(0 - 3)/7 + 1] ~# @:(0 - 3)/7 를 1 증가시킴 #~
            [@:1/19 + 1; @:1/13 = @:0 - 1] ~# @:1/13 + 2 로 이동 #~
            [@:1/23 + 0; @:(0 - 2)/7 = @:(0 - 2)/7 - 1] ~# @:(0 - 2)/7 (정수의 길이)를 1 감소시킴 #~
            [@:1/23 + 1; (@:6/7):(@:(0 - 2)/7) = @:(0 - 3)/7 + 48] ~# 반환할 Bowl의 (정수의 길이)번째에 현재 자리값을 대입 #~
            [@:1/23 + 2; @:(0 - 1)/7 = @:(0 - 4)/7] ~# @:(0 - 1)/7 의 마지막 자리수를 제거하여 대입 #~
            [@:1/23 + 3; @:1/13 = @:0 - 0] ~# @:1/13 + 1 로 이동 (자리값 초기화 수행하는 자리로) #~
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
        어떤 수를 소수의 곱으로만 나타내는 것을 소인수분해라 하고, 이 소수들을 그 수의 소인수라고 합니다.
        예를 들면 13195의 소인수는 5, 7, 13, 29 입니다.
        600851475143의 소인수 중에서 가장 큰 수를 구하세요.
        #~
        
        {
            [1/2; @:2 = 1] ~# 로직 시작지점 설정 #~
            [2/3; @:2/11 = 600851475143] ~# 소인수분해할 수 #~
            [3/4; @:3/11 = 2] ~# 나눌 소인수 #~
            [@:2 + 0; { ~# 피보나치 변수 2가 짝수일 경우 @:4/11에 더함 #~
                [0; { ~# 현재 나눌 소인수로 나누어 떨어지지 않는 경우 #~
                    [0; @:3/11 = @:3/11 + 2] ~# 현재 나눌 소인수가 2가 아니면 2를 더함 #~
                    [1; @:3/11 = @:3/11 + 1] ~# 현재 나눌 소인수가 2이면 1을 더함 #~
                }:(@:3/11 ?= 2)]
                [1; @:2/11 = (@:2/11)/(@:3/11)] ~# 현재 나눌 소인수로 나누어 떨어지는 경우 소인수분해할 수를 나눌 소인수로 나눈 몫을 소인수분해할 수에 대입 #~
            }:(^((@:2/11)/(@:3/11)) ?= 1)]
            [@:2 + 1; {
                [0; @:3 = @:0 + 1] ~# @:2/11이 @:3/11보다 크지 않으면 @:3를 다음에 시작 #~
                [1; @:2 = @:0 + 1] ~# @:3/11이 @:3/11보다 크면 @:2를 다음에 시작 #~
            }:(@:2/11 > @:3/11)]
            [@:3 + 0; @:2/7 = @:3/11] ~# 함수 @:1/3을 호출하기 위해 출력할 정수 대입 #~
            [@:3 + 1; @:3/7 = 4] ~# 함수 @:1/3을 호출 후 이동할 위치 4 대입 #~
            [@:3 + 2; @:1/3 = @:0 + 1] ~# 함수 @:1/3을 호출 #~
            [@:4 + 0; @:1 = @:6/7] ~# 함수 @:1/3의 결과로 얻은 변환된 정수 문자열 출력 #~
            [@:4 + 1; @:1 = {
                [0; 10]
            }]
        
            ~# 함수: @:1/2 = 정수의 길이를 반환 #~
            ~# 입력: @:2/5 = 확인할 정수, @:3/5 = 함수 종료 후 이동할 위치 #~
            ~# 출력: @:4/5 = 확인한 정수의 길이 #~
            [@:1/2; @:1/5 = @:0 + 3]
            [@:1/5 - 2; @:4/5 = 0] ~# 출력 결과에 0을 대입 #~
            [@:1/5 - 1; @:(0 - 1)/5 = @:2/5] ~# @:(0 - 1)/5 = 확인할 변수를 담은 임시 변수 #~
            [@:1/5 + 0; @:4/5 = @:4/5 + 1] ~# 출력 결과를 1 증가 #~
            [@:1/5 + 2; @:(0 - 1)/5 = (@:(0 - 1)/5) / 10] ~# @:(0 - 1)/5 를 10으로 나누어 다시 저장 #~
            [@:1/5 + 3; { ~# @:(0 - 1)/5 가 1보다 크거나 같으면 @:1/5로 되돌아감 #~
                [1; @:1/5 = @:0 + 1]
            }:!(@:(0 - 1)/5 < 1)]
            [@:1/5 + 4; @:(@:3/5) = @:0 + 1] ~# 함수 탈출 #~
        
            ~# 함수: @:1/3 = 정수를 자릿수별로 나눠서 ascii로 변환한 Bowl으로 반환 #~
            ~# 입력: @:2/7 = 변환할 정수, @:3/7 = 함수 종료 후 이동할 위치 #~
            ~# 출력: @:6/7 = 변환된 Bowl #~
            [@:1/3; @:1/7 = @:0 + 1]
            [@:1/7 + 0; @:6/7 = {}] ~# 변환 결과를 저장할 Bowl 초기화 #~
            [@:1/7 + 1; @:(0 - 1)/7 = @:2/7] ~# 변환할 정수 임시 변수 @:(0 - 1)/7 에 복사 #~
            [@:1/7 + 2; @:2/5 = @:2/7] ~# 함수 @:1/2를 호출하기 위해 확인할 정수 대입 #~
            [@:1/7 + 3; @:3/5 = 1/13] ~# 함수 @:1/2를 호출 후 이동할 위치 1/13 지정 #~
            [@:1/7 + 4; @:1/2 = @:0 + 1] ~# 함수 @:1/2를 호출 #~
            [@:1/13 + 0; @:(0 - 2)/7 = @:4/5] ~# 함수 @:1/2 호출 결과(정수의 길이)를 임시 변수 @:(0 - 2)/7 에 저장 #~
            [@:1/13 + 1; @:(0 - 3)/7 = 0] ~# 자리수 확인용 변수 @:(0 - 3)/7 초기화 #~
            [@:1/13 + 2; { ~# 정수의 길이가 0이면 함수 종료 #~
                [0; @:1/17 = @:0 + 1] ~# 정수의 길이가 0이 아니므로 계속 실행 #~
                [1; @:(@:3/7) = @:0 + 1] ~# 정수의 길이가 0이므로 함수 종료 #~
            }:(@:(0 - 2)/7 ?= 0)]
            [@:1/17 + 0; @:(0 - 4)/7 = (@:(0 - 1)/7 - @:(0 - 3)/7)/10] ~# @:(0 - 4)/7 = (임시변수 1 - 임시변수 3) / 10 #~
            [@:1/17 + 1; { ~# @:(0 - 4)/7 의 분모가 1인 경우 @:(0 - 3)/7이 현재 마지막 자리의 자릿값임#~
                [0; @:1/19 = @:0 + 1] ~# @:(0 - 3)/7 를 1 증가시켜서 다시 확인하는 코드로 이동 #~
                [1; @:1/23 = @:0 + 1] ~# @:(0 - 3)/7 를 반환할 Bowl에 추가하는 코드로 이동 #~
            }:(^(@:(0 - 4)/7) ?= 1)]
            [@:1/19 + 0; @:(0 - 3)/7 = @:(0 - 3)/7 + 1] ~# @:(0 - 3)/7 를 1 증가시킴 #~
            [@:1/19 + 1; @:1/13 = @:0 - 1] ~# @:1/13 + 2 로 이동 #~
            [@:1/23 + 0; @:(0 - 2)/7 = @:(0 - 2)/7 - 1] ~# @:(0 - 2)/7 (정수의 길이)를 1 감소시킴 #~
            [@:1/23 + 1; (@:6/7):(@:(0 - 2)/7) = @:(0 - 3)/7 + 48] ~# 반환할 Bowl의 (정수의 길이)번째에 현재 자리값을 대입 #~
            [@:1/23 + 2; @:(0 - 1)/7 = @:(0 - 4)/7] ~# @:(0 - 1)/7 의 마지막 자리수를 제거하여 대입 #~
            [@:1/23 + 3; @:1/13 = @:0 - 0] ~# @:1/13 + 1 로 이동 (자리값 초기화 수행하는 자리로) #~
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
