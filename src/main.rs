extern crate nom;
extern crate asalang_parser;

use asalang_parser::{math_expression, Node};

fn main() -> Result<(), nom::Err<(&'static str, nom::error::ErrorKind)>> {
  let result = math_expression(r#"4+2"#);
  match result {
    Ok((unparsed,tree)) => {
      println!("Unparsed Text: {:?}", unparsed);
      println!("Parse Tree:\n {:?}", tree);
    }
    Err(error) => {
      println!("ERROR {:?}", error);
    }
  }
    
  Ok(())
}
