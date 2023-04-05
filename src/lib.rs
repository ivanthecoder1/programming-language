#[derive(Debug, PartialEq, Clone)]
pub enum Token {
  Keyword(Vec<u8>), // false, true, fn, return, let
  Alpha(u8), // a-z, A-Z
  Digit(u8), // 0 - 9 
  LeftParen(u8), 
  RightParen(u8),
  LeftCurly(u8),
  RightCurly(u8),
  Equal(u8),
  Plus(u8),
  Dash(u8),
  Quote(u8),
  WhiteSpace(u8), //space, \t, \r, \n
  Semicolon(u8),
  Comma(u8),
  Other(),
  EOF,
}

extern crate nom;

mod parser;

pub use self::parser::{math_expression, Node};

// lex function should iterate over every byte of the input string, 
// and categorize each one according to the token descriptions above.
pub fn lex(input: &str) -> Vec<Token> {
  let bytes = input.as_bytes();
  let mut tokens = vec![];
  
  // check for keywords with looping
  let mut counter = 0;
  while counter < bytes.len(){
    if bytes[counter] == b'f' { // starts with a f: can be fn or false
      if counter + 1 < bytes.len() { // make sure we don't go out of bounds
        if bytes[counter + 1] == b'n' { // fn
          counter += 2;
          tokens.push(Token::Keyword(b"fn".to_vec()));
        } 
        else if counter + 4 < bytes.len() && &bytes[counter + 1..=counter + 4] == b"alse" { // false
          counter += 5;
          tokens.push(Token::Keyword(b"false".to_vec()));
        }
        else {
          tokens.push(Token::Alpha(bytes[counter]));
          counter += 1;
        }
      }
    }
    else if bytes[counter] == b't' { // start with t: can be true 
      if counter + 3 < bytes.len() && &bytes[counter..=counter + 3] == b"true" { // true
          counter += 4;
          tokens.push(Token::Keyword(b"true".to_vec()));
      } else {
          tokens.push(Token::Alpha(bytes[counter]));
          counter += 1;
      }
    }
    else if bytes[counter] == b'r' { // start with r: can be return
      if counter + 5 < bytes.len() && &bytes[counter..=counter + 5] == b"return" {
          counter += 6;
          tokens.push(Token::Keyword(b"return".to_vec()));
      } else {
          tokens.push(Token::Alpha(bytes[counter]));
          counter += 1;
      }
    }
    else if bytes[counter] == b'l' { // let
      if counter + 2 < bytes.len() && &bytes[counter..=counter + 2] == b"let" {
          counter += 3;
          tokens.push(Token::Keyword(b"let".to_vec()));
      }
      else {
        tokens.push(Token::Alpha(bytes[counter]));
        counter = counter + 1;
      }
    }
    else {
      // check for other tokens
      let token = match bytes[counter] {
        // token patterns
        0x41..=0x5A | 0x61..=0x7A => Token::Alpha(bytes[counter]),
        0x30..=0x39 => Token::Digit(bytes[counter]),
        0x28 => Token::LeftParen(bytes[counter]),
        0x29 => Token::RightParen(bytes[counter]),
        0x7B => Token::LeftCurly(bytes[counter]),
        0x7D => Token::RightCurly(bytes[counter]),
        0x3D => Token::Equal(bytes[counter]),
        0x2B => Token::Plus(bytes[counter]),
        0x2D => Token::Dash(bytes[counter]),
        0x22 => Token::Quote(bytes[counter]),
        0x20 | 0x0D | 0x09 | 0x0A => Token::WhiteSpace(bytes[counter]), // space, cr, tab, new line
        0x3B => Token::Semicolon(bytes[counter]),
        0x2C => Token::Comma(bytes[counter]),
        _ => Token::Other(),
      };
      tokens.push(token);
      counter = counter + 1;
    }
  }
  tokens.push(Token::EOF);
  println!("{:#?}", tokens);
  return tokens;
}

pub fn strip_whitespace(tokens: &Vec<Token>) -> Vec<Token> {
  let mut new_tokens = Vec::new(); // empty vector to store tokens w/o whitespace
  for token in tokens {
      match token { // only push non white space tokens
          Token::WhiteSpace(_) => continue,
          _ => new_tokens.push(token.clone()),
      }
  }
  return new_tokens
}

