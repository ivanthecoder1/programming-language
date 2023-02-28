use lexer::*;

#[test]
fn test_01() {
  assert_eq!(lex("123"), vec![Token::Digit(b'1'), Token::Digit(b'2'), Token::Digit(b'3'), Token::EOF]);
}

#[test]
fn test_02() {
  assert_eq!(lex("abc"),vec![Token::Alpha(b'a'), Token::Alpha(b'b'), Token::Alpha(b'c'), Token::EOF]);
}

#[test]
fn test_03() {
  assert_eq!(lex("hello world"),vec![Token::Alpha(b'h'), Token::Alpha(b'e'), Token::Alpha(b'l'), Token::Alpha(b'l'), Token::Alpha(b'o'), 
  Token::WhiteSpace(b' '), Token::Alpha(b'w'), Token::Alpha(b'o'), Token::Alpha(b'r'), Token::Alpha(b'l'), Token::Alpha(b'd'), Token::EOF]);
}

#[test]
fn test_04() {
  assert_eq!(lex("true"),vec![Token::Keyword(b"true".to_vec()), Token::EOF]);
}

#[test]
fn test_05() {
  assert_eq!(lex("false"),vec![Token::Keyword(b"false".to_vec()), Token::EOF]);
}

#[test]
fn test_06() {
  assert_eq!(lex("let x = 123;"),vec![
    Token::Keyword(b"let".to_vec()), 
    Token::WhiteSpace(b' '), 
    Token::Alpha(b'x'), 
    Token::WhiteSpace(b' '),
    Token::Equal(b'='),
    Token::WhiteSpace(b' '),
    Token::Digit(b'1'),
    Token::Digit(b'2'),
    Token::Digit(b'3'),
    Token::Semicolon(b';'),
    Token::EOF,
  ]);
}

#[test]
fn test_07() {
  assert_eq!(lex(r#"let x = 123;let y="abc";"#),vec![
    Token::Keyword(b"let".to_vec()), 
    Token::WhiteSpace(b' '), 
    Token::Alpha(b'x'), 
    Token::WhiteSpace(b' '),
    Token::Equal(b'='),
    Token::WhiteSpace(b' '),
    Token::Digit(b'1'),
    Token::Digit(b'2'),
    Token::Digit(b'3'),
    Token::Semicolon(b';'),
    Token::Keyword(b"let".to_vec()), 
    Token::WhiteSpace(b' '), 
    Token::Alpha(b'y'), 
    Token::Equal(b'='),
    Token::Quote(b'"'),
    Token::Alpha(b'a'), 
    Token::Alpha(b'b'), 
    Token::Alpha(b'c'), 
    Token::Quote(b'"'),
    Token::Semicolon(b';'),
    Token::EOF,
  ]);
}

#[test]
fn test_08() {
  assert_eq!(lex(r#"fn main() {}"#),vec![
    Token::Keyword(b"fn".to_vec()),  
    Token::WhiteSpace(b' '), 
    Token::Alpha(b'm'), 
    Token::Alpha(b'a'),
    Token::Alpha(b'i'),
    Token::Alpha(b'n'),
    Token::LeftParen(b'('),
    Token::RightParen(b')'),
    Token::WhiteSpace(b' '),
    Token::LeftCurly(b'{'),
    Token::RightCurly(b'}'),
    Token::EOF,
  ]);
}


#[test]
fn test_09() {
  assert_eq!(lex(r#"fn foo(a,b,c) {
  let x=a+1;
	let y=bar(c-b);
  return x*y;
}"#),vec![
    Token::Keyword(b"fn".to_vec()), 
    Token::WhiteSpace(b' '), 
    Token::Alpha(b'f'), 
    Token::Alpha(b'o'),
    Token::Alpha(b'o'),
    Token::LeftParen(b'('),
    Token::Alpha(b'a'),
    Token::Comma(b','),
    Token::Alpha(b'b'),
    Token::Comma(b','),
    Token::Alpha(b'c'),
    Token::RightParen(b')'),
    Token::WhiteSpace(b' '),
    Token::LeftCurly(b'{'),
    Token::WhiteSpace(b'\n'),
    Token::WhiteSpace(b' '),
    Token::WhiteSpace(b' '),
    Token::Keyword(b"let".to_vec()), 
    Token::WhiteSpace(b' '), 
    Token::Alpha(b'x'),
    Token::Equal(b'='),
    Token::Alpha(b'a'),
    Token::Plus(b'+'),
    Token::Digit(b'1'),
    Token::Semicolon(b';'),
    Token::WhiteSpace(b'\n'), 
    Token::WhiteSpace(b'\t'), 
    Token::Keyword(b"let".to_vec()), 
    Token::WhiteSpace(b' '),
    Token::Alpha(b'y'),
    Token::Equal(b'='),
    Token::Alpha(b'b'),
    Token::Alpha(b'a'),
    Token::Alpha(b'r'),
    Token::LeftParen(b'('),
    Token::Alpha(b'c'),
    Token::Dash(b'-'),
    Token::Alpha(b'b'),
    Token::RightParen(b')'),
    Token::Semicolon(b';'),
    Token::WhiteSpace(b'\n'),
    Token::WhiteSpace(b' '),
    Token::WhiteSpace(b' '),
    Token::Keyword(b"return".to_vec()), 
    Token::WhiteSpace(b' '),
    Token::Alpha(b'x'),
    Token::Other(),
    Token::Alpha(b'y'),
    Token::Semicolon(b';'),
    Token::WhiteSpace(b'\n'),
    Token::RightCurly(b'}'),
    Token::EOF,
  ]);
}

#[test]
fn test_10() {
  assert_eq!(strip_whitespace(&lex(r#"fn foo(a,b,c) {
  let x=a+1;
	let y=bar(c-b);
  return x+y;
}"#)),vec![
    Token::Keyword(b"fn".to_vec()), 
    Token::Alpha(b'f'), 
    Token::Alpha(b'o'),
    Token::Alpha(b'o'),
    Token::LeftParen(b'('),
    Token::Alpha(b'a'),
    Token::Comma(b','),
    Token::Alpha(b'b'),
    Token::Comma(b','),
    Token::Alpha(b'c'),
    Token::RightParen(b')'),
    Token::LeftCurly(b'{'),
    Token::Keyword(b"let".to_vec()), 
    Token::Alpha(b'x'),
    Token::Equal(b'='),
    Token::Alpha(b'a'),
    Token::Plus(b'+'),
    Token::Digit(b'1'),
    Token::Semicolon(b';'),
    Token::Keyword(b"let".to_vec()), 
    Token::Alpha(b'y'),
    Token::Equal(b'='),
    Token::Alpha(b'b'),
    Token::Alpha(b'a'),
    Token::Alpha(b'r'),
    Token::LeftParen(b'('),
    Token::Alpha(b'c'),
    Token::Dash(b'-'),
    Token::Alpha(b'b'),
    Token::RightParen(b')'),
    Token::Semicolon(b';'),
    Token::Keyword(b"return".to_vec()), 
    Token::Alpha(b'x'),
    Token::Plus(b'+'),
    Token::Alpha(b'y'),
    Token::Semicolon(b';'),
    Token::RightCurly(b'}'),
    Token::EOF,
  ]);
}
