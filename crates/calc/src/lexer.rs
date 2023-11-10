use std::str::{Chars, FromStr};

use crate::token::Operator::*;
use crate::token::Token;

pub fn is_an_allowed_char(character: char) -> bool {
    character.is_alphanumeric()
        || character == '+'
        || character == '-'
        || character == '*'
        || character == '/'
        || character == '('
        || character == ')'
        || character == '"'
        || character == '.'
}


fn lex_int(mut current_char: char, chars: &mut Vec<char>, mut current_pos: usize, len: usize) -> (i64, usize) {
    let mut str: String = String::new();
    while current_pos < len && current_char.is_numeric() {
        str += &*current_char.to_string();

        current_pos += 1;
        let a = chars.get(current_pos);
        match a {
            Some(t) => current_char = *t,
            None => break
        }
    }
    (i64::from_str(&str).unwrap(), current_pos)
}

pub fn lex(input: String) -> Vec<Token> {
    let mut vec: Vec<Token> = Vec::new();

    let mut current_pos = 0;

    let mut chars = input.as_str().chars().collect::<Vec<char>>();

    let length = input.len();

    while current_pos < input.len() {
        let current_character: char = chars.get(current_pos).unwrap().to_ascii_lowercase();
        if !is_an_allowed_char(current_character) {
            continue;
        };

        match current_character {
            '+' => vec.push(Token::OPE(PLUS)),
            '-' => vec.push(Token::OPE(MINUS)),
            '*' => vec.push(Token::OPE(MULTIPLICATION)),
            '/' => vec.push(Token::OPE(DIVIDE)),
            ')' => vec.push(Token::RPAR),
            '(' => vec.push(Token::LPAR),
            '"' => vec.push(Token::QUOTE),
            ch => {
                if ch.is_numeric() {
                    let (a,b) = lex_int(current_character, &mut chars, current_pos, length);
                    current_pos = b;
                    vec.push(Token::INT(a))
                }
            }
        }
    }

    vec
}

#[cfg(test)]
mod tests {
    use crate::lexer::lex;
    use crate::token::Operator::*;
    use crate::token::Token::*;

    #[test]
    fn lex_plus() {
        let mut expected = Vec::new();
        expected.push(OPE(PLUS));
        let result = lex("+".to_string());
        assert_eq!(result, expected)
    }

    #[test]
    fn lex_minus() {
        let mut expected = Vec::new();
        expected.push(OPE(MINUS));
        let result = lex("-".to_string());
        assert_eq!(result, expected)
    }

    #[test]
    fn lex_mult() {
        let mut expected = Vec::new();
        expected.push(OPE(MULTIPLICATION));
        let result = lex("*".to_string());
        assert_eq!(result, expected)
    }

    #[test]
    fn lex_divide() {
        let mut expected = Vec::new();
        expected.push(OPE(DIVIDE));
        let result = lex("/".to_string());
        assert_eq!(result, expected)
    }


    #[test]
    fn lex_operators() {
        let mut expected = Vec::new();
        expected.push(OPE(PLUS));
        expected.push(OPE(MULTIPLICATION));
        expected.push(OPE(MINUS));
        expected.push(OPE(DIVIDE));
        let result = lex("+*-/".to_string());
        assert_eq!(result, expected)
    }

    #[test]
    fn lex_lpar() {
        let mut expected = Vec::new();
        expected.push(LPAR);
        let result = lex("(".to_string());
        assert_eq!(result, expected)
    }

    #[test]
    fn lex_rpar() {
        let mut expected = Vec::new();
        expected.push(RPAR);
        let result = lex(")".to_string());
        assert_eq!(result, expected)
    }

    #[test]
    fn lex_quote() {
        let mut expected = Vec::new();
        expected.push(QUOTE);
        let result = lex("\"".to_string());
        assert_eq!(result, expected)
    }


    #[test]
    fn lex_tokens() {
        let mut expected = Vec::new();
        expected.push(LPAR);
        expected.push(RPAR);
        expected.push(QUOTE);
        let result = lex("()\"".to_string());
        assert_eq!(result, expected)
    }

    #[test]
    fn lex_simple_int() {
        let mut expected = Vec::new();
        expected.push(INT(1));
        let result = lex("1".to_string());
        assert_eq!(result,expected);
    }

    #[test]
    fn lex_complex_int() {
        let mut expected = Vec::new();
        expected.push(INT(100));
        let result = lex("100".to_string());
        assert_eq!(result,expected);
    }
}

