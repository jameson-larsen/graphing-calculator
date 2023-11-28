/*
GRAMMAR RECOGNIZED BY PARSER:
<Expr> ::= <Expr> + <Term> | <Expr> - <Term> | <Term>

<Term> ::= <Term> * <Fact> | <Term> / <Fact> | <Fact>

<Unary> ::= - <Unary> | <Pow>

<Pow> ::= <Func> ^ <Pow> | <Func>

<Func> ::= func_name ( <Expr> ) | <Atom>

<Atom> ::= variable | constant | num | ( <Expr> )
*/

use crate::scanner::TokenType;

#[derive(Debug, PartialEq)]
pub enum ASTNodeType {
    BinaryExpression(TokenType, Box<ASTNodeType>, Box<ASTNodeType>),
    UnaryExpression(TokenType, Box<ASTNodeType>),
    FunctionExpression(TokenType, Box<ASTNodeType>),
    AtomicExpression(TokenType)
}

macro_rules! expect_more_tokens {
    ($tokens:ident,$start:expr) => {
        if $start >= $tokens.len() { return Err(String::from("Error: unexpected end of input while parsing expression")); }
    }
}

macro_rules! return_if_error_or_unwrap {
    ($item:ident, $idx:ident) => {
        if let Err(e) = $item { return Err(e); }
        let ( $item, $idx ) = $item.unwrap();
    }
}

macro_rules! expect_token {
    ($tokens:ident, $idx:expr, $token_type:expr) => {
        expect_more_tokens!($tokens, $idx);
        if $tokens[$idx] != $token_type { return Err(format!("Error: expected {}, found {}", $token_type, $tokens[$idx])); }
    }
}

pub fn parse(tokens: &Vec<TokenType>) -> Result<ASTNodeType, String> {
    let expr = parse_expression(tokens, 0);
    return_if_error_or_unwrap!(expr, stop);
    if stop != tokens.len() { return Err(String::from("Error: failed to parse expression")); }
    Ok(expr)
}

fn parse_expression(tokens: &Vec<TokenType>, start: usize) -> Result<(ASTNodeType, usize), String> {
    expect_more_tokens!(tokens, start);
    let left = parse_term(tokens, start);
    if let Err(e) = left { return Err(e); }
    let ( mut left, mut next ) = left.unwrap();

    while next < tokens.len() && (tokens[next] == TokenType::Add || tokens[next] == TokenType::Sub) {
        let operator = tokens[next].clone();
        let right = parse_term(tokens, next + 1);
        if let Err(e) = right { return Err(e); }
        let right_unwrapped = right.unwrap();
        let right = right_unwrapped.0;
        next = right_unwrapped.1;
        left = ASTNodeType::BinaryExpression(operator, Box::new(left), Box::new(right));
    }

    Ok((left, next))
}

fn parse_term(tokens: &Vec<TokenType>, start: usize) -> Result<(ASTNodeType, usize), String> {
    expect_more_tokens!(tokens, start);
    let left = parse_unary(tokens, start);
    if let Err(e) = left { return Err(e); }
    let ( mut left, mut next ) = left.unwrap();

    while next < tokens.len() && (tokens[next] == TokenType::Mul || tokens[next] == TokenType::Div) {
        let operator = tokens[next].clone();
        let right = parse_unary(tokens, next + 1);
        if let Err(e) = right { return Err(e); }
        let right_unwrapped = right.unwrap();
        let right = right_unwrapped.0;
        next = right_unwrapped.1;
        left = ASTNodeType::BinaryExpression(operator, Box::new(left), Box::new(right));
    }

    Ok((left, next))
}

fn parse_unary(tokens: &Vec<TokenType>, start: usize) -> Result<(ASTNodeType, usize), String> {
    expect_more_tokens!(tokens, start);
    if tokens[start] == TokenType::Sub {
        let operator = tokens[start].clone();
        let expr = parse_unary(tokens, start + 1);
        return_if_error_or_unwrap!(expr, next);
        let unary = ASTNodeType::UnaryExpression(operator, Box::new(expr));
        return Ok((unary, next));
    }
    let power = parse_power(tokens, start);
    return_if_error_or_unwrap!(power, next);
    Ok((power, next))
}

fn parse_power(tokens: &Vec<TokenType>, start: usize) -> Result<(ASTNodeType, usize), String> {
    expect_more_tokens!(tokens, start);
    let base = parse_function(tokens, start);
    return_if_error_or_unwrap!(base, next);
    if next < tokens.len() && tokens[next] == TokenType::Exp {
        let operator = tokens[next].clone();
        let power = parse_power(tokens, next + 1);
        return_if_error_or_unwrap!(power, next);
        let power = ASTNodeType::BinaryExpression(operator, Box::new(base), Box::new(power));
        return Ok((power, next));
    }
    Ok((base, next))
}

fn parse_function(tokens: &Vec<TokenType>, start: usize) -> Result<(ASTNodeType, usize), String> {
    expect_more_tokens!(tokens, start);
    if let TokenType::FunctionName(_) = &tokens[start] {
        let name = tokens[start].clone();
        expect_token!(tokens, start + 1, TokenType::LeftParen);
        let expression = parse_expression(tokens, start + 2);
        return_if_error_or_unwrap!(expression, next);
        expect_token!(tokens, next, TokenType::RightParen);
        let function = ASTNodeType::FunctionExpression(name, Box::new(expression));
        return Ok((function, next + 1));
    }
    let atom = parse_atom(tokens, start);
    return_if_error_or_unwrap!(atom, next);
    Ok((atom, next))
}

fn parse_atom(tokens: &Vec<TokenType>, start: usize) -> Result<(ASTNodeType, usize), String> {
    expect_more_tokens!(tokens, start);
    match &tokens[start] {
        TokenType::NumLiteral(_) | TokenType::Constant(_) | TokenType::Variable(_) => {
            Ok((ASTNodeType::AtomicExpression(tokens[start].clone()), start + 1))
        }
        TokenType::LeftParen => {
            let expression = parse_expression(tokens, start + 1);
            return_if_error_or_unwrap!(expression, next);
            expect_token!(tokens, next, TokenType::RightParen);
            Ok((expression, next+1))
        }
        other => Err(format!("Error: expected expression, found {}", other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::scan;

    #[test]
    fn test_parse() {
        let expr = String::from("2 * sin(x)");
        let tokens = scan(&expr).unwrap();
        let result = parse(&tokens);
        println!("{:?}", result);
    }
}