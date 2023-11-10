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
}


pub fn lex(input: String) -> Vec<Token> {
    println!("{input}");
    let mut vec: Vec<Token> = Vec::new();

    let chars = input.as_str().chars();

    for current_character in chars {
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
            _ => ()
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
        assert_eq!(result,expected)
    }
}

