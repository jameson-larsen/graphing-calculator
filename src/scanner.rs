use std::fmt;

const RECOGNIZED_FUNCTIONS: &[& str] = &["sin", "cos", "tan", "log", "ln"];
const RECOGNIZED_CONSTANTS: &[& str] = &["e", "pi"];
const RECOGNIZED_VARIABLES: &[& str] = &["x"];


#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Variable(String),
    FunctionName(String),
    NumLiteral(f64),
    Constant(String),
    LeftParen,
    RightParen,
    Mul,
    Div,
    Exp,
    Sub,
    Add
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            &TokenType::Variable(s) | &TokenType::FunctionName(s) | &TokenType::Constant(s) => write!(f, "{}", s),
            &TokenType::NumLiteral(num) => write!(f, "{}", num),
            &TokenType::LeftParen => write!(f, "("),
            &TokenType::RightParen => write!(f, ")"),
            &TokenType::Mul => write!(f, "*"),
            &TokenType::Div => write!(f, "/"),
            &TokenType::Exp => write!(f, "^"),
            &TokenType::Sub => write!(f, "-"),
            &TokenType::Add => write!(f, "+")
        }
    }
}


//main scanning function - takes string and produces vector of tokens
pub fn scan(input_string: &String) -> Result<Vec<TokenType>, String> {
    
    let mut tokens = Vec::new();
    let chars : Vec<char>  = input_string.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c  = chars[i];
        
        //skip whitespace
        if c.is_whitespace() { i = i + 1; continue; }

        if c == '(' { tokens.push(TokenType::LeftParen); }
        else if c == ')' { tokens.push(TokenType::RightParen); }
        else if c == '*' { tokens.push(TokenType::Mul); }
        else if c == '/' { tokens.push(TokenType::Div); }
        else if c == '^' { tokens.push(TokenType::Exp); }
        else if c == '-' { tokens.push(TokenType::Sub); }
        else if c == '+' { tokens.push(TokenType::Add); }
        else if c.is_numeric() {
            match scan_number(&chars, i) {
                Ok((token, resume_idx)) => {
                    tokens.push(token);
                    i = resume_idx;
                    continue;
                }
                Err(e) => { return Err(e); }
            }
        }
        else if c.is_alphabetic() {
            match scan_letters(&chars, i) {
                Ok((token, resume_idx)) => {
                    tokens.push(token);
                    i = resume_idx;
                    continue;
                }
                Err(e) => { return Err(e); }
            }
        }
        else {
            return Err(format!("Error: invalid character '{c}' found while scanning input"))
        }

        i = i + 1;
    }

    Ok(tokens)
}

//function to scan a number from a numeric string and produce a token containing the parsed floating point value
fn scan_number(chars: &Vec<char>, start_idx: usize) -> Result<(TokenType, usize), String> {
    let mut curr = String::new();
    let mut i = start_idx;
    let decimal_found = false;
    while i < chars.len() {
        let c = chars[i];
        if c.is_numeric() || c == '.' {
            //only allow number tokens to contain one decimal point
            if c == '.' && decimal_found {
                return Err(String::from("Error: invalid number literal. Numbers may only contain one decimal point"));
            }
            curr.push(c);
            i = i + 1;
        }
        else { break; }
    }
    Ok((TokenType::NumLiteral(curr.parse::<f64>().unwrap()), i))
}

//function to scan a word from a string and produce a token containing the appropriate constant, function name, or variable
fn scan_letters(chars: &Vec<char>, start_idx: usize) -> Result<(TokenType, usize), String> {
    let mut curr = String::new();
    let mut i = start_idx;

    //create vector of all possible words that the scanner can recognize
    let mut candidates = vec![RECOGNIZED_CONSTANTS.to_vec(), RECOGNIZED_FUNCTIONS.to_vec(), RECOGNIZED_VARIABLES.to_vec()];
    let mut candidates_left = true;

    //iterate until we've reached the end of the char array, no words match our current scanned word, or we encounter a non-letter character
    while i < chars.len() && candidates_left && chars[i].is_alphabetic() {
        curr.push(chars[i]);
        i = i + 1;
        candidates = candidates
                    .iter()
                    .map(|v| (*v).iter().map(|s| *s).filter(|el| el.starts_with(&curr)).collect()).collect();
        if match_found(&candidates, &curr) { break; }
        candidates_left = !candidates.iter().all(|el| el.is_empty());
    }

    let constant_candidates = &candidates[0];
    let function_candidates = &candidates[1];
    let variable_candidates = &candidates[2];
    
    if !constant_candidates.is_empty() { return Ok((TokenType::Constant(curr), i)); }
    else if !function_candidates.is_empty() { return Ok((TokenType::FunctionName(curr), i)); }
    else if !variable_candidates.is_empty() { return Ok((TokenType::Variable(curr), i)); }

    return Err(format!("Error: invalid word '{curr}' found"))
}


//function to check if we have found a match based on current token string
fn match_found<'a>(candidates: &'a Vec<Vec<&'a str>>, search_string: &'a String) -> bool {
    for c_vec in candidates.iter() {
        for s in c_vec.iter().map(|s| *s) {
            if s == search_string { return true; }
        }
    }
    return false;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_num() {
        let result = scan_number(&"123.45abc".chars().collect(), 0).unwrap().0;
        let expected = TokenType::NumLiteral(123.45);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_scan_letters() {
        let result = scan_letters(&"sin2x".chars().collect(), 0).unwrap().0;
        let expected = TokenType::FunctionName(String::from("sin"));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_scan() {
        let result = scan(&String::from("x * sin(x) / e")).unwrap();
        let x = TokenType::Variable(String::from("x"));
        let x2 = TokenType::Variable(String::from("x"));
        let mul = TokenType::Mul;
        let sin = TokenType::FunctionName(String::from("sin"));
        let lp = TokenType::LeftParen;
        let rp = TokenType::RightParen;
        let div = TokenType::Div;
        let e = TokenType::Constant(String::from("e"));
        let expected = vec![x, mul, sin, lp, x2, rp, div, e];
        assert_eq!(result, expected);
    }
}