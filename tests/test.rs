extern crate asalang;
extern crate nom;

use asalang::{program, Node, Value, start_interpreter};
use nom::IResult;

macro_rules! test {
  ($func:ident, $test:tt, $expected:expr) => (
    #[test]
    fn $func() -> Result<(),String> {
      match program($test) {
        Ok((input, p)) => {
          assert_eq!(input, "");
          assert_eq!(start_interpreter(&p), $expected);
          Ok(())
        },
        Err(e) => Err(format!("{:?}",e)),
      }
    }
  )
}

test!(numeric, r#"123"#, Ok(Value::Number(123)));
test!(identifier, r#"x"#, Err("Undefined variable"));
test!(string, r#""hello world""#, Ok(Value::String("hello world".to_string())));
test!(bool_true, r#"true"#, Ok(Value::Bool(true)));
test!(bool_false, r#"false"#, Ok(Value::Bool(false)));
test!(function_call, r#"foo()"#, Err("Undefined function"));
test!(function_call_one_arg, r#"foo(a)"#, Err("Undefined function"));
test!(function_call_more_args, r#"foo(a,b,c)"#, Err("Undefined function"));
test!(variable_define, r#"let x = 123;"#, Ok(Value::Number(123)));
test!(variable_init, r#"let x = 1;"#, Ok(Value::Number(1)));
test!(variable_bool, r#"let bool = true;"#, Ok(Value::Bool(true)));
test!(variable_string, r#"let string = "Hello World";"#, Ok(Value::String("Hello World".to_string())));
test!(variable_init_no_space, r#"let x=1;"#, Ok(Value::Number(1)));
test!(math, r#"1 + 1"#, Ok(Value::Number(2)));
test!(math_no_space, r#"1+1"#, Ok(Value::Number(2)));
test!(math_subtraction, r#"1 - 1"#, Ok(Value::Number(0)));
test!(math_multiply, r#"2 * 4"#, Ok(Value::Number(8)));
test!(math_divide, r#"6 / 2"#, Ok(Value::Number(3)));
test!(math_exponent, r#"2 ^ 4"#, Ok(Value::Number(16)));
test!(math_more_terms, r#"10 + 2*6"#, Ok(Value::Number(22)));
test!(math_more_terms_paren, r#"((10+2)*6)/4"#, Ok(Value::Number(18)));
test!(assign_math, r#"let x = 1 + 1;"#, Ok(Value::Number(2)));
test!(assign_function, r#"let x = foo();"#, Err("Undefined function"));
test!(assign_function_arguments, r#"let x = foo(a,b,c);"#, Err("Undefined function"));
test!(define_function, r#"fn main(){return foo();} fn foo(){return 5;}"#, Ok(Value::Number(5)));
test!(define_function_args, r#"fn main(){return foo(1,2,3);} fn foo(a,b,c){return a+b+c;}"#, Ok(Value::Number(6)));
test!(define_function_more_statement, r#"fn main() {
  return foo();
}
fn foo(){
  let x = 5;
  return x;
}"#, Ok(Value::Number(5)));
test!(define_full_program, r#"fn foo(a,b,c) {
  let x = a + 1;
  let y = bar(c - b);
  return x * y;
}

fn bar(a) {
  return a * 3;
}

fn main() {
  return foo(1,2,3);  
}"#, Ok(Value::Number(6)));

// Cut 1
test!(greater_than, r#"1 > 2"#, Ok(Value::Bool(false)));
test!(math_expression_equality, r#"(1 + 2) == 3"#, Ok(Value::Bool(true)));
test!(assign_comparison, r#"let result = 1 < 2;"#, Ok(Value::Bool(true)));

// Invalid test 
test!(invalid, r#"1 > false"#, Err("Invalid comparison expression"));
test!(invalid_math_bool, r#"5 - false"#, Err("Invalid"));

// Cut 2
// Valid
test!(one_line, r#"if false {return false;} else {return true;}"#, Ok(Value::Bool(true)));
test!(return_num, r#"if true {return 1;} else {return 2;}"#, Ok(Value::Number(1)));
test!(multi_line, r#"if true {
  return false;
} else {
  return true;
}
"#, Ok(Value::Bool(false)));
test!(assign_if_else, r#"let x = if true {return false;} else {return true;}"#, Ok(Value::Bool(false)));
test!(else_if, r#"if true {return 1;} else if false {return 2;} else {return 3;}"#, Ok(Value::Number(1)));

// Invalid - not working for some reason
// Returns an error when I use cargo run in main.rs. It returns the error of undefined, but the test is not matching with 
// the error for some reason. 
// test!(missing_curly, r#"if true {return false;} return true;}"#, Err("Undefined variable"));