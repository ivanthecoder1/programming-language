// Here is where the various combinators are imported. You can find all the combinators here:
// https://docs.rs/nom/5.0.1/nom/
// If you want to use it in your parser, you need to import it here. I've already imported a couple.
use nom::{
  IResult,
  branch::alt,
  combinator::opt, 
  multi::{many1, many0},
  bytes::complete::{tag},
  character::complete::{alphanumeric1, digit1},
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
  IfExpression { children: Vec<Node> },
  IfBranch { children: Vec<Node> },
  ElifBranch { children: Vec<Node> },
  ElseBranch { children: Vec<Node> },
  ComparisonExpression { name: String, children: Vec<Node> },
  MathExpression {name: String, children: Vec<Node> },
  FunctionCall { name: String, children: Vec<Node> },
  VariableDefine { children: Vec<Node> },
  Number { value: i32 },
  Bool { value: bool },
  Identifier { value: String },
  String { value: String },
}
// Define production rules for an identifier
pub fn identifier(input: &str) -> IResult<&str, Node> {
  let (input, result) = alphanumeric1(input)?;              // Consume at least 1 alphanumeric character. The ? automatically unwraps the result if it's okay and bails if it is an error.
  Ok((input, Node::Identifier{ value: result.to_string()})) // Return the now partially consumed input, as well as a node with the string on it.
}
// Define an integer number
pub fn number(input: &str) -> IResult<&str, Node> {
  let (input, result) = digit1(input)?;                     // Consume at least 1 digit 0-9
  let number = result.parse::<i32>().unwrap();              // Parse the string result into a usize
  Ok((input, Node::Number{ value: number}))                 // Return the now partially consumed input with a number as well
}
pub fn boolean(input: &str) -> IResult<&str, Node> {
  let (input, result) = alt((tag("true"),tag("false")))(input)?;
  let bool_value = if result == "true" {true} else {false};
  Ok((input, Node::Bool{ value: bool_value}))
}
pub fn string(input: &str) -> IResult<&str, Node> {
  let (input, _) = tag("\"")(input)?;
  let (input, string) = many1(alt((alphanumeric1,tag(" "))))(input)?;
  let (input, _) = tag("\"")(input)?;
  Ok((input, Node::String{ value: string.join("")}))
}
pub fn function_call(input: &str) -> IResult<&str, Node> {
  let (input, name) = alphanumeric1(input)?;
  let (input, _) = tag("(")(input)?;
  let (input, mut args) = many0(arguments)(input)?;
  let (input, _) = tag(")")(input)?;
  Ok((input, Node::FunctionCall{name: name.to_string(), children: args}))   
}
pub fn parenthetical_expression(input: &str) -> IResult<&str, Node> {
  let (input, _) = many0(tag(" "))(input)?;
  let (input, _) = tag("(")(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, args) = l1(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, _) = tag(")")(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  Ok((input, args))
}
pub fn l4(input: &str) -> IResult<&str, Node> {
  alt((function_call, number, identifier, parenthetical_expression))(input)
}
pub fn l3_infix(input: &str) -> IResult<&str, Node> {
  let (input, _) = many0(tag(" "))(input)?;
  let (input, op) = tag("^")(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, args) = l4(input)?;
  Ok((input, Node::MathExpression{name: op.to_string(), children: vec![args]}))
}
pub fn l3(input: &str) -> IResult<&str, Node> {
  let (input, mut head) = l4(input)?;
  let (input, tail) = many0(l3_infix)(input)?;
  for n in tail {
    match n {
      Node::MathExpression{name, mut children} => {
        let mut new_children = vec![head.clone()];
        new_children.append(&mut children);
        head = Node::MathExpression{name, children: new_children};
      }
      _ => () 
    };
  }
  Ok((input, head))
}
pub fn l2_infix(input: &str) -> IResult<&str, Node> {
  let (input, _) = many0(tag(" "))(input)?;
  let (input, op) = alt((tag("*"),tag("/")))(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, args) = l2(input)?;
  Ok((input, Node::MathExpression{name: op.to_string(), children: vec![args]}))
}
pub fn l2(input: &str) -> IResult<&str, Node> {
  let (input, mut head) = l3(input)?;
  let (input, tail) = many0(l2_infix)(input)?;
  for n in tail {
    match n {
      Node::MathExpression{name, mut children} => {
        let mut new_children = vec![head.clone()];
        new_children.append(&mut children);
        head = Node::MathExpression{name, children: new_children};
      }
      _ => () 
    };
  }
  Ok((input, head))
}
pub fn l1_infix(input: &str) -> IResult<&str, Node> {
  let (input, _) = many0(tag(" "))(input)?;
  let (input, op) = alt((tag("+"),tag("-")))(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, args) = l2(input)?;
  Ok((input, Node::MathExpression{name: op.to_string(), children: vec![args]}))
}
pub fn l1(input: &str) -> IResult<&str, Node> {
  let (input, mut head) = l2(input)?;
  let (input, tail) = many0(l1_infix)(input)?;
  for n in tail {
    match n {
      Node::MathExpression{name, mut children} => {
        let mut new_children = vec![head.clone()];
        new_children.append(&mut children);
        head = Node::MathExpression{name, children: new_children};
      }
      _ => () 
    };
  }
  Ok((input, head))
}

// l0 - to account for comparison operators 

pub fn math_expression(input: &str) -> IResult<&str, Node> {
  l1(input)
}

// Cut 1
// ebnf: comparison_expression = number | boolean | identifier | expression, comparison_operators, number | boolean | identifier | expression;
pub fn comparison_expression(input: &str) -> IResult<&str, Node> {
  let (input, left) = alt((math_expression, function_call, number, boolean, identifier))(input)?; // match left side
  let (input, _) = many0(tag(" "))(input)?;
  let (input, operator) = alt((tag(">"), tag("<"), tag(">="), tag("<="), tag("=="), tag("!=")))(input)?; // choose a comparison operator
  let (input, _) = many0(tag(" "))(input)?;
  let (input, right) = alt((math_expression, function_call, number, boolean, identifier))(input)?; // match right side
  Ok((input, Node::ComparisonExpression{ name: operator.to_string(), children: vec![left, right]}))   
}

// Cut 2
// Rules
// Evaluation order: Condition is evaluated first, followed by the true/false branch.
// Type consistency: Ensure both expressions have compatible types.
// Short-circuit evaluation: If condition is true, false branch is not evaluated.
// Return value: The if-expression should return a single value that can be assigned to a variable or used in an expression.

// Ebnf: if_branch = "if", boolean, "{", function_return , ";", "}";
pub fn if_branch(input: &str) -> IResult<&str, Node> {
  let (input, _) = tag("if")(input)?; // look for keyword if
  let (input, _) = many0(alt((tag("\n"),tag(" "),tag("\n"))))(input)?;
  let (input, cond_1) = boolean(input)?; // expression must evaluate be a true
  let (input, _) = many0(alt((tag("\n"),tag(" "),tag("\n"))))(input)?; 
  let (input, _) = tag("{")(input)?;
  let (input, _) = many0(alt((tag("\n"),tag(" "),tag("\n"))))(input)?; 
  let (input, result1) = function_return(input)?; // return result inside
  let (input, _) = tag(";")(input)?;
  let (input, _) = many0(alt((tag("\n"),tag(" "),tag("\n"))))(input)?; 
  let (input, _) = tag("}")(input)?;
  let (input, _) = many0(alt((tag("\n"),tag(" "),tag("\n"))))(input)?;
  Ok((input, Node::IfBranch{ children: vec![cond_1, result1]}))   
}

// Ebnf: else_if_branch = "else if", boolean, "{", function_return , ";", "}";
pub fn elif_branch(input: &str) -> IResult<&str, Node> {
  let (input, _) = tag("else if")(input)?; // look for keyword else if
  let (input, _) = many0(alt((tag("\n"),tag(" "),tag("\n"))))(input)?;
  let (input, cond_elif) = boolean(input)?; // expression must evaluate be a true
  let (input, _) = many0(alt((tag("\n"),tag(" "),tag("\n"))))(input)?; 
  let (input, _) = tag("{")(input)?;
  let (input, _) = many0(alt((tag("\n"),tag(" "),tag("\n"))))(input)?; 
  let (input, result_elif) = function_return(input)?;
  let (input, _) = tag(";")(input)?;
  let (input, _) = many0(alt((tag("\n"),tag(" "),tag("\n"))))(input)?; 
  let (input, _) = tag("}")(input)?;
  let (input, _) = many0(alt((tag("\n"),tag(" "),tag("\n"))))(input)?;
  Ok((input, Node::ElifBranch{ children: vec![cond_elif, result_elif]}))   
}

// Ebnf: else_branch = "else", "{", function_return , ";", "}";
pub fn else_branch(input: &str) -> IResult<&str, Node> {
  let (input, _) = tag("else")(input)?; // look for keyword if
  let (input, _) = many0(alt((tag("\n"),tag(" "),tag("\n"))))(input)?;
  let (input, _) = tag("{")(input)?;
  let (input, _) = many0(alt((tag("\n"),tag(" "),tag("\n"))))(input)?; 
  let (input, result2) = function_return(input)?;
  let (input, _) = tag(";")(input)?;
  let (input, _) = many0(alt((tag("\n"),tag(" "),tag("\n"))))(input)?; 
  let (input, _) = tag("}")(input)?;
  let (input, _) = many0(alt((tag("\n"),tag(" "),tag("\n"))))(input)?;
  Ok((input, Node::ElseBranch{ children: vec![result2]}))   
}


// Ebnf: if_expressions = if_branch, [else_if_branch], else_branch;
pub fn if_expression(input: &str) -> IResult<&str, Node> {
  let (input, _) = many0(alt((tag("\n"), tag(" "), tag("\n"))))(input)?;
  let (input, if_condition) = if_branch(input)?;
  let (input, _) = many0(alt((tag("\n"), tag(" "), tag("\n"))))(input)?;
  let (input, elif_condition) = opt(elif_branch)(input)?; // Optional else if branch
  let (input, _) = many0(alt((tag("\n"), tag(" "), tag("\n"))))(input)?;
  let (input, else_condition) = else_branch(input)?;
  
  // since elif branch is optional, check if it exists, then we push it to children
  let children = if let Some(elif_node) = elif_condition {
    vec![if_condition, elif_node, else_condition]
  } else { // if there is no elif branch, we only push if and else brnach
    vec![if_condition, else_condition]
  };
  
  Ok((input, Node::IfExpression { children }))
}

pub fn expression(input: &str) -> IResult<&str, Node> {
  let (input, result) = alt((if_expression, boolean, comparison_expression, math_expression, function_call, number, string, identifier))(input)?;
  Ok((input, Node::Expression{ children: vec![result]}))   
}

pub fn statement(input: &str) -> IResult<&str, Node> {
  let (input, _) = many0(alt((tag(" "),tag("\t"))))(input)?;
  let (input, result) = alt((variable_define, function_return))(input)?;
  let (input, _) = many0(tag(";"))(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, _) = many0(tag("\n"))(input)?;
  Ok((input, Node::Statement{ children: vec![result]}))   
}
pub fn function_return(input: &str) -> IResult<&str, Node> {
  let (input, _) = tag("return ")(input)?;
  let (input, return_value) = alt((function_call, expression, identifier))(input)?;
  Ok((input, Node::FunctionReturn{ children: vec![return_value]}))
}
pub fn variable_define(input: &str) -> IResult<&str, Node> {
  let (input, _) = tag("let ")(input)?;
  let (input, variable) = identifier(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, _) = tag("=")(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, expression) = expression(input)?;
  Ok((input, Node::VariableDefine{ children: vec![variable, expression]}))   
}
pub fn arguments(input: &str) -> IResult<&str, Node> {
  let (input, arg) = expression(input)?;
  let (input, mut others) = many0(other_arg)(input)?;
  let mut args = vec![arg];
  args.append(&mut others);
  Ok((input, Node::FunctionArguments{children: args}))
}
pub fn other_arg(input: &str) -> IResult<&str, Node> {
  let (input, _) = tag(",")(input)?;
  expression(input)
}
pub fn function_definition(input: &str) -> IResult<&str, Node> {
  let (input, _) = tag("fn ")(input)?;
  let (input, function_name) = identifier(input)?;
  let (input, _) = tag("(")(input)?;
  let (input, mut args) = many0(arguments)(input)?;
  let (input, _) = tag(")")(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, _) = tag("{")(input)?;
  let (input, _) = many0(tag("\n"))(input)?;
  let (input, mut statements) = many1(statement)(input)?;
  let (input, _) = tag("}")(input)?;
  let (input, _) = many0(alt((tag("\n"),tag(" "))))(input)?;
  let mut children = vec![function_name];
  println!("args, {:?}", args);
  children.append(&mut args);
  children.append(&mut statements);
  Ok((input, Node::FunctionDefine{ children: children }))   
}

pub fn program(input: &str) -> IResult<&str, Node> {
  let (input, result) = many1(alt((function_definition, statement, expression)))(input)?;  // Now that we've defined a number and an identifier, we can compose them using more combinators. Here we use the "alt" combinator to propose a choice.
  Ok((input, Node::Program{ children: result}))       // Whether the result is an identifier or a number, we attach that to the program
}
