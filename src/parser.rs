// Here is where the various combinators are imported. You can find all the combinators here:
// https://docs.rs/nom/7.1.3/nom/
// If you want to use it in your parser, you need to import it here. I've already imported a couple.

use nom::{
    IResult,
    branch::alt,
    combinator::opt,
    multi::{many1, many0},
    bytes::complete::{tag},
    character::complete::{alpha1, alphanumeric1, digit1, digit0, alphanumeric0, space0, space1},
    bytes::complete::{take_while, take_while1},
    sequence::pair,
  };
  
  // Here are the different node types. You will use these to make your parser and your grammar.
  // You may add other nodes as you see fit, but these are expected by the runtime.
  
  #[derive(Debug, Clone)]
  pub enum Node {
    Program { children: Vec<Node> },
    Statement { children: Vec<Node> },
    FunctionReturn { children: Vec<Node> },
    FunctionDefine { children: Vec<Node> },
    FunctionArguments { children: Vec<Node> },
    FunctionStatements { children: Vec<Node> },
    Expression { children: Vec<Node> },
    MathExpression {name: String, children: Vec<Node> },
    FunctionCall { name: String, children: Vec<Node> },
    VariableDefine { children: Vec<Node> },
    Number { value: i32 },
    Bool { value: bool },
    Identifier { value: String },
    String { value: String },
    Null,
  }
  
  // Here is the grammar, for your reference:
  
  // identifier = alnum , {alnum} ;
  // Passed test
  pub fn identifier(input: &str) -> IResult<&str, Node> {
    // first index in string has to be an alpha
    let (input, alphas) = alphanumeric1(input)?;
  
    // rest of string can be alnumeric
    let (input, alpha_nums) = alphanumeric0(input)?;
    let identifier = format!("{}{}", alphas, alpha_nums);
    Ok((input, Node::Identifier{ value: identifier }))
  }
  
  // number = digit+ ;
  // Passed test
  pub fn number(input: &str) -> IResult<&str, Node> {
    let (input, digits) = many1(digit1)(input)?;
  
    // loop through digits, and convert from str to i32
    let mut value = 0;
    for digit in digits {
      // to account for hundredth and ten places
      value *= 10;
      // convert to i32
      value += digit.parse::<i32>().unwrap();
    }
    // return number node with parse value
    Ok((input, Node::Number{ value }))
  }
  
  // boolean = "true" | "false" ;
  // Passed test
  pub fn boolean(input: &str) -> IResult<&str, Node> {
    // alt to match to true or false
    let (input, value) = alt((tag("true"), tag("false")))(input)?;
    let value = value.parse::<bool>().unwrap(); // convert to bool
    // return bool node
    Ok((input, Node::Bool { value }))
  }
  
  // string = "\"" , {alnum | " "} , "\"" ;
  // Passed test
  pub fn string(input: &str) -> IResult<&str, Node> {
    let (input, first_quote) = tag("\"")(input)?;
    let (input, string_content) = take_while(|c| c != '\"')(input)?;
    let (input, second_quote) = tag("\"")(input)?;
    let string = format!("{}{}{}", first_quote, string_content, second_quote);
    Ok((input, Node::String{ value: string }))
  }
  
  // function_call = identifier , "(" , [arguments] , ")" ;
  pub fn function_call(input: &str) -> IResult<&str, Node> {
    let (input, variable) = identifier(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, opt_arg) = opt(arguments)(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, Node::FunctionCall{ children: vec![variable, opt_arg]}))
  }
  
  // value = number | identifier | boolean ;
  // Compiled with no error
  pub fn value(input: &str) -> IResult<&str, Node> {
    let (input, value) = alt((number, identifier, boolean))(input)?;
    match value {
      Node::Number{value} => Ok((input, Node::Number{value})),
      Node::Bool{value} => Ok((input, Node::Bool{value})),
      Node::Identifier{value} => Ok((input, Node::Identifier{value})),
      _ => unreachable!(),
    }
  }
  
  // math_expression = value , { ("+" | "-") , value } ;  
  pub fn math_expression(input: &str) -> IResult<&str, Node> {
    let (input, first_value) = value(input)?;
    let (input, expression) = many0(pair(alt((tag("+"), tag("-"))), value))(input)?;
  
    // create a vector 
    let mut children = vec![first_value];
    let mut name = "".to_string();
  
    // loop over operator-value pairs and add to children vector
    for (operator, value) in expression {
        name.push_str(operator);
        children.push(value);
    }
  
    Ok((input, Node::MathExpression{ name, children }))
  }
  
  // expression = boolean | math_expression | function_call | number | string | identifier ;
  pub fn expression(input: &str) -> IResult<&str, Node> {
    let (input, matched_node) = alt((boolean, math_expression, function_call, number, string, identifier))(input)?;
    Ok((input, Node::Expression{ children: vec![matched_node] }))
  }
  
  // statement = variable_define , ";" | function_return , ";" ;
  pub fn statement(input: &str) -> IResult<&str, Node> {
    let (input, var_def) = variable_define(input)?;
    let (input, _) = alt(tag(";"), function_return)(input)?;
    let (input, _) = tag(";")(input)?;
    Ok((input, Node::Statement {children: vec![var_def]}))
  }
  
  // function_return = "return" , (function_call | expression | identifier) ;
  pub fn function_return(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("return")(input)?;
    let (input, value) = alt((function_call, expression, identifier))(input)?;
    Ok((input, Node::FunctionReturn { children: vec![value] }))
  }
  
  // variable_define = "let" , identifier , "=" , expression ;
  pub fn variable_define(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("let ")(input)?;
    let (input, variable) = identifier(input)?;
    let (input, _) = many0(tag(" "))(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, _) = many0(tag(" "))(input)?;
    let (input, expression) = expression(input)?;
    Ok((input, Node::VariableDefine{ children: vec![variable, expression]}))
  }
  
  // arguments = expression , { "," , expression } ;
  pub fn arguments(input: &str) -> IResult<&str, Node> {
    let (input, _) = expression(input)?;
    let (input, _) = many1(tag(","), expression)(input)?;
    Ok((input, Node::argument))
  }
  
  // Like the first argument but with a comma in front
  pub fn other_arg(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag(",")(input)?;
    let (input, expression) = expression(input)?;
    Ok((input, expression))
  }
  
  // function_definition = "fn" , identifier , "(" , [arguments] , ")" , "{" , [statement+] , "}" ;
  pub fn function_definition(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("fn")(input)?;
    let (input, identifier) = identifier(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, arguments) = opt(arguments)(input)?;
    let (input, _) = tag(")")(input)?;
    let (input, _) = tag("{")(input)?;
    let (input, statements) = many0(statement)(input)?;
    let (input, _) = tag("}")(input)?;
  
  
    // create vector to store all values above
    let mut children = vec![identifier];
      if let Some(args) = arguments {
          children.push(args);
      }
      children.extend(statements);
  
  
    Ok((input, Node::FunctionDefine{ children }))
  }
  
  // comment = "//" , (?any-character? - newline);
  // compiled with no error
  pub fn comment(input: &str) -> IResult<&str, Node> {
    let (input, comment_symbol) = tag("//")(input)?;
    let (input, comment) = take_while1(|c| c != '\n')(input)?;
    Ok((input, Node::String{ value: format!("{}{}", comment_symbol, comment) }))
  }
  
  // program = function_definition+ ;
  pub fn program(input: &str) -> IResult<&str, Node> {
    let (input, functions) = many1(function_definition)(input)?;
    Ok((input, Node::Program{ children: functions }))
  }
  