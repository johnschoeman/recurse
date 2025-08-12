use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, digit1, multispace0},
    combinator::{map, map_res},
    multi::many0,
    sequence::preceded,
};
use std::error::Error;
use std::fmt;

// Lexer

#[derive(Debug, PartialEq)]
enum Token {
    Integer(i64),
    Symbol(String),
    LParen,
    RParen,
}

fn parse_l_paren(input: &str) -> IResult<&str, Token> {
    map(tag("("), |_| Token::LParen).parse(input)
}

fn parse_r_paren(input: &str) -> IResult<&str, Token> {
    map(tag(")"), |_| Token::RParen).parse(input)
}

fn parse_integer(input: &str) -> IResult<&str, Token> {
    map(map_res(digit1, |s: &str| s.parse::<i64>()), |d| {
        Token::Integer(d)
    })
    .parse(input)
}

fn parse_symbol_alpha(input: &str) -> IResult<&str, Token> {
    map(alpha1, |s: &str| Token::Symbol(s.to_string())).parse(input)
}

fn parse_symbol_plus(input: &str) -> IResult<&str, Token> {
    map(char('+'), |s| Token::Symbol(s.to_string())).parse(input)
}

fn parse_token(input: &str) -> IResult<&str, Token> {
    alt((
        parse_l_paren,
        parse_r_paren,
        parse_integer,
        parse_symbol_alpha,
        parse_symbol_plus,
    ))
    .parse(input)
}

fn tokenize(input: &str) -> IResult<&str, Vec<Token>> {
    many0(preceded(multispace0, parse_token)).parse(input)
}

// Parser

#[derive(Debug)]
pub struct ParseError {
    err: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse error: {}", self.err)
    }
}

impl Error for ParseError {}

#[derive(Debug, PartialEq)]
pub enum AST {
    Void,
    Integer(i64),
    Bool(bool),
    Symbol(String),
    Lambda(Vec<String>, Vec<AST>),
    List(Vec<AST>),
}

fn parse_lisp(input: &str) -> Result<AST, ParseError> {
    let Ok((_, token_result)) = tokenize(input) else {
        todo!()
    };

    let mut tokens = token_result.into_iter().rev().collect::<Vec<Token>>();
    let parsed = parse_tokens(&mut tokens)?;
    Ok(parsed)
}

fn parse_tokens(tokens: &mut Vec<Token>) -> Result<AST, ParseError> {
    let token = tokens.pop();

    if token != Some(Token::LParen) {
        return Err(ParseError {
            err: format!("expected Token::LParen, but found {:?}", token),
        });
    }

    let mut objects: Vec<AST> = vec![];

    if tokens.last() == Some(&Token::RParen) {
        return Ok(AST::List(vec![AST::Void]));
    }

    while !tokens.is_empty() {
        let option_token = tokens.pop();
        if option_token == None {
            return Err(ParseError {
                err: format!("Not enough tokens"),
            });
        }

        let token = option_token.unwrap();
        match token {
            Token::Symbol(s) => objects.push(AST::Symbol(s)),
            Token::Integer(i) => objects.push(AST::Integer(i)),
            Token::LParen => {
                tokens.push(Token::LParen);
                let next = parse_tokens(tokens)?;
                objects.push(next);
            }
            Token::RParen => {
                return Ok(AST::List(objects));
            }
        }
    }

    Ok(AST::List(objects))
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_nested() -> Result<(), Box<dyn std::error::Error>> {
        let expected: AST = AST::List(vec![
            AST::Symbol("first".to_string()),
            AST::List(vec![
                AST::Symbol("list".to_string()),
                AST::Integer(1),
                AST::List(vec![
                    AST::Symbol("+".to_string()),
                    AST::Integer(2),
                    AST::Integer(3),
                ]),
                AST::Integer(9),
            ]),
        ]);

        let input = "(first (list 1 (+ 2 3) 9))";

        let result = parse_lisp(input).unwrap();

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn parse_simple_add() -> Result<(), Box<dyn std::error::Error>> {
        let expected: AST = AST::List(vec![
            AST::Symbol("+".to_string()),
            AST::Integer(1),
            AST::Integer(2),
        ]);

        let input = "(+ 1 2)";

        let result = parse_lisp(input).unwrap();

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn parse_simple() -> Result<(), Box<dyn std::error::Error>> {
        let expected: AST = AST::List(vec![AST::Void]);

        let input = "()";

        let result = parse_lisp(input).unwrap();

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn tokenize_complex() -> Result<(), Box<dyn std::error::Error>> {
        let expected: Vec<Token> = vec![
            Token::LParen,
            Token::Symbol("first".to_string()),
            Token::LParen,
            Token::Symbol("list".to_string()),
            Token::Integer(1),
            Token::LParen,
            Token::Symbol("+".to_string()),
            Token::Integer(2),
            Token::Integer(3),
            Token::RParen,
            Token::Integer(9),
            Token::RParen,
            Token::RParen,
        ];

        let (_, result) = tokenize("(first (list 1 (+ 2 3) 9))")?;

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn tokenize_simple() -> Result<(), Box<dyn std::error::Error>> {
        let expected: Vec<Token> = vec![Token::LParen, Token::Integer(1), Token::RParen];

        let (_, result) = tokenize("(1)")?;

        assert_eq!(result, expected);

        Ok(())
    }
}
