%start BowlP
%avoid_insert "NUMBER"
%left ASSIGN
%left AND OR
%left NOT
%left EQ GT LT
%left PLUS MINUS
%left BOWL
%left MUL
%left DENO
%left NUMBER_SEP
%left NOODLE_SEP
%left BOWL_OPEN BOWL_CLOSE
%left NOODLE_OPEN NOODLE_CLOSE
%left EXPR_OPEN EXPR_CLOSE
%left MEM
%left NUMBER
%%

BowlP -> Result<Bowl, ()>:
    'MEM' { Ok(Bowl {
        is_mem: true,
        noodles: Vec::new(),
    }) }
    | 'BOWL_OPEN' 'BOWL_CLOSE' { Ok(Bowl {
        is_mem: false,
        noodles: Vec::new(),
    }) }
    | 'BOWL_OPEN' BowlContents 'BOWL_CLOSE' { Ok(Bowl {
        is_mem: false,
        noodles: $2?,
    }) }
    ;

Number -> Result<BigUint, ()>:
    'NUMBER' {
        let v = $1.map_err(|_| ())?;
        parse_bigint(remove_whitespace($lexer.span_str(v.span())).as_str())
    }
    ;

NoodleP -> Result<Noodle, ()>:
    'NOODLE_OPEN' ExprP 'NOODLE_SEP' ExprP 'NOODLE_CLOSE' { Ok(Noodle {
        nn_expr: $2?,
        expr: $4?,
    }) }
    ;

BowlContents -> Result<Vec<Noodle>, ()>:
    NoodleP { Ok(vec![$1?]) }
    | BowlContents NoodleP {
        let mut $1 = $1?;
        $1.push($2?);
        Ok($1)
    }
    ;

BowlAccesser -> Result<BowlAccess, ()>:
    ExprP 'BOWL' ExprP { Ok(BowlAccess {
        bowl_expr: $1?,
        access_expr: $3?,
    }) }
    ;

ExprP -> Result<Expr, ()>:
    Number { Ok(Expr::ValueExpr(Value::Number(Number::new($1?, BigUint::from(1u32))))) }
    | BowlP { Ok(Expr::ValueExpr(Value::Bowl($1?))) }
    | BowlAccesser { Ok(Expr::BowlAceessFuncExpr(Box::new($1?))) }
    | 'EXPR_OPEN' ExprP 'EXPR_CLOSE' { $2 }
    | ExprP 'ASSIGN' ExprP { Ok(Expr::AssignFuncExpr(Box::new($1?), Box::new($3?))) }
    | 'DENO' ExprP { Ok(Expr::DenoFuncExpr(Box::new($2?))) }
    | ExprP 'PLUS' ExprP { Ok(Expr::PlusFuncExpr(Box::new($1?), Box::new($3?))) }
    | ExprP 'MINUS' ExprP { Ok(Expr::MinusFuncExpr(Box::new($1?), Box::new($3?))) }
    | ExprP 'MUL' ExprP { Ok(Expr::MulFuncExpr(Box::new($1?), Box::new($3?))) }
    | ExprP 'NUMBER_SEP' ExprP { Ok(Expr::NumberSepFuncExpr(Box::new($1?), Box::new($3?))) }
    | ExprP 'AND' ExprP { Ok(Expr::AndFuncExpr(Box::new($1?), Box::new($3?))) }
    | ExprP 'OR' ExprP { Ok(Expr::OrFuncExpr(Box::new($1?), Box::new($3?))) }
    | 'NOT' ExprP { Ok(Expr::NotFuncExpr(Box::new($2?))) }
    | ExprP 'EQ' ExprP { Ok(Expr::EqFuncExpr(Box::new($1?), Box::new($3?))) }
    | ExprP 'GT' ExprP { Ok(Expr::GtFuncExpr(Box::new($1?), Box::new($3?))) }
    | ExprP 'LT' ExprP { Ok(Expr::LtFuncExpr(Box::new($1?), Box::new($3?))) }
    ;
%%

use num_bigint::BigUint;
use crate::datatype::{Bowl, BowlAccess, Expr, Noodle, Value, Number};

fn parse_bigint(s: &str) -> Result<BigUint, ()> {
    match BigUint::parse_bytes(s.as_bytes(), 10) {
        Some(val) => Ok(val),
        None => {
            eprintln!("{} cannot be represented as a number", s);
            Err(())
        }
    }
}

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}
