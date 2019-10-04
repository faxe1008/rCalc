pub mod lexer {
    use regex::Regex;

    #[derive(PartialEq, Debug, Copy, Clone)]
    pub enum TokenType {
        Plus,
        Minus,
        Multiply,
        Divide,
        Power,
        Number,
        OpeningParenthesis,
        ClosingParaenthesis,
    }
    #[derive(PartialEq, Debug, Copy, Clone)]
    pub enum Associativity {
        Left,
        Right,
    }
    #[derive(Debug, PartialEq, Copy, Clone)]
    pub struct Token {
        r#type: self::TokenType,
        value: f64,
        precedence: u8,
        associativity: self::Associativity,
    }
    #[derive(PartialEq)]
    enum ParserState {
        NONE,
        OPERATOR,
        PARENTHESIS,
        Number,
    }

    pub fn tokenize(content: &str) -> Result<Vec<Token>, &'static str> {
        let mut v: Vec<Token> = Vec::new();
        let mut state = ParserState::NONE;
        let mut buffer = String::new();
        for c in content.chars() {
            if c.is_alphabetic() {
                let error = format!("Invalid character: {}", c);
                return Err(Box::leak(error.into_boxed_str()));
            } else if state == ParserState::Number {
                if c.is_numeric() || c == '.' {
                    buffer.push(c);
                } else if c == '^' || c == '*' || c == '/' || c == '-' || c == '+' {
                    v.push(Token::new(&buffer)?);
                    buffer = String::new();
                    buffer.push(c);
                    state = ParserState::OPERATOR;
                } else if c == '(' || c == ')' {
                    v.push(Token::new(&buffer)?);
                    buffer = String::new();
                    buffer.push(c);
                    state = ParserState::PARENTHESIS;
                }
            } else if state == ParserState::OPERATOR {
                if c == '-' || c.is_numeric() {
                    v.push(Token::new(&buffer)?);
                    buffer = String::new();
                    buffer.push(c);
                    state = ParserState::Number;
                } else if c == '(' || c == ')' {
                    v.push(Token::new(&buffer)?);
                    buffer = String::new();
                    buffer.push(c);
                    state = ParserState::PARENTHESIS;
                }
            } else if state == ParserState::PARENTHESIS {
                if c == '-' || c.is_numeric() {
                    v.push(Token::new(&buffer)?);
                    buffer = String::new();
                    buffer.push(c);
                    state = ParserState::Number;
                } else if c == '^' || c == '*' || c == '/' || c == '-' || c == '+' {
                    v.push(Token::new(&buffer)?);
                    buffer = String::new();
                    buffer.push(c);
                    state = ParserState::OPERATOR;
                } else if c == '(' || c == ')' {
                    v.push(Token::new(&buffer)?);
                    buffer = String::new();
                    buffer.push(c);
                    state = ParserState::PARENTHESIS;
                }
            } else if state == ParserState::NONE {
                if c.is_numeric() || c == '-' {
                    buffer.push(c);
                    state = ParserState::Number;
                } else if c == '(' {
                    buffer.push(c);
                    state = ParserState::PARENTHESIS;
                }
            }
        }
        if !buffer.is_empty() && (state == ParserState::Number || state == ParserState::PARENTHESIS)
        {
            v.push(Token::new(&buffer)?);
        } else {
            return Err("Invalid end of expression");
        }

        Ok(v)
    }

    impl Token {
        pub fn is_operator(&self) -> bool {
            match self.kind() {
                TokenType::Divide
                | TokenType::Multiply
                | TokenType::Plus
                | TokenType::Minus
                | TokenType::Power => true,
                _ => false,
            }
        }

        pub fn new(content: &str) -> Result<Token, &'static str> {
            if content == "+" {
                Ok(Token {
                    r#type: TokenType::Plus,
                    value: 0f64,
                    precedence: 2,
                    associativity: Associativity::Left,
                })
            } else if content == "-" {
                Ok(Token {
                    r#type: TokenType::Minus,
                    value: 0f64,
                    precedence: 2,
                    associativity: Associativity::Left,
                })
            } else if content == "*" {
                Ok(Token {
                    r#type: TokenType::Multiply,
                    value: 0f64,
                    precedence: 3,
                    associativity: Associativity::Left,
                })
            } else if content == "/" {
                Ok(Token {
                    r#type: TokenType::Divide,
                    value: 0f64,
                    precedence: 3,
                    associativity: Associativity::Left,
                })
            } else if content == "^" {
                Ok(Token {
                    r#type: TokenType::Power,
                    value: 0f64,
                    precedence: 4,
                    associativity: Associativity::Right,
                })
            } else if Regex::new(r"^-?\d+\.?\d*$").unwrap().is_match(content) {
                Ok(Token {
                    r#type: TokenType::Number,
                    value: content.parse().expect("number required"),
                    precedence: 0,
                    associativity: Associativity::Right,
                })
            } else if content == "(" {
                Ok(Token {
                    r#type: TokenType::OpeningParenthesis,
                    value: 0f64,
                    precedence: 0,
                    associativity: Associativity::Right,
                })
            } else if content == ")" {
                Ok(Token {
                    r#type: TokenType::ClosingParaenthesis,
                    value: 0f64,
                    precedence: 0,
                    associativity: Associativity::Right,
                })
            } else {
                let error = format!("Error creating new token with content: {}", content);
                Err(Box::leak(error.into_boxed_str()))
            }
        }

        pub fn value(&self) -> f64 {
            self.value
        }
        pub fn kind(&self) -> TokenType {
            self.r#type
        }
        pub fn precedence(&self) -> u8 {
            self.precedence
        }
        pub fn associativity(&self) -> Associativity {
            self.associativity
        }
    }
}

pub mod calculator {
    use crate::parser::lexer::*;

    pub fn evaluate(expression: &str) -> Result<f64, &'static str> {
        calculate(&shunting_yard(&tokenize(expression)?)?)
    }

    fn calculate(tokens: &[Token]) -> Result<f64, &'static str> {
        let mut processing_numbers: Vec<f64> = Vec::new();
        for t in tokens {
            if t.kind() == TokenType::Number {
                processing_numbers.push(t.value());
            }
            if t.is_operator() {
                if let (Some(r_op),Some(l_op)) = (processing_numbers.pop(), processing_numbers.pop()) {
                        let result: f64 = match t.kind() {
                            TokenType::Divide => l_op / r_op,
                            TokenType::Multiply => l_op * r_op,
                            TokenType::Minus => l_op - r_op,
                            TokenType::Plus => l_op + r_op,
                            TokenType::Power => l_op.powf(r_op),
                            _ => return Err("Can not have number on operation stack"),
                        };
                        processing_numbers.push(result);
                }
            }
        }
    

    if processing_numbers.len() == 1usize {
        Ok(processing_numbers[0])
    } else {
        Err("Error parsing expresion")
    }
}

    fn shunting_yard(tokens: &[Token]) -> Result<Vec<Token>, &'static str> {
        let mut reverse_notation: Vec<Token> = Vec::new();
        let mut stack: Vec<Token> = Vec::new();

        for t in tokens.iter() {
            if t.kind() == TokenType::Number {
                reverse_notation.push(*t);
            } else if t.kind() == TokenType::OpeningParenthesis {
                stack.push(*t);
            } else if t.kind() == TokenType::ClosingParaenthesis {
                while !stack.is_empty()
                    && stack.last().unwrap().kind() != TokenType::OpeningParenthesis
                {
                    reverse_notation.push(stack.pop().unwrap());
                }
                if !stack.is_empty()
                    && stack.last().unwrap().kind() == TokenType::OpeningParenthesis
                {
                    stack.pop();
                } else {
                    return Err("Expected opening bracket");
                }
            } else if t.is_operator() {
                while !stack.is_empty()
                    && (stack.last().unwrap().precedence() > t.precedence()
                        || (stack.last().unwrap().precedence() == t.precedence()
                            && stack.last().unwrap().associativity() == Associativity::Left))
                {
                    reverse_notation.push(stack.pop().unwrap());
                }
                stack.push(*t);
            }
        }
        while !stack.is_empty() {
            reverse_notation.push(stack.pop().unwrap());
        }

        Ok(reverse_notation)
    }
}

#[cfg(test)]
mod lexer_tests {
    use crate::parser::lexer::*;

    #[test]
    fn valid_plus() {
        let plus: String = String::from("+");
        let token = Token::new(&plus).unwrap();

        assert_eq!(token.associativity(), Associativity::Left);
        assert_eq!(token.precedence(), 2);
        assert_eq!(token.kind(), TokenType::Plus);
    }

    #[test]
    fn valid_minus() {
        let minus: String = String::from("-");
        let token = Token::new(&minus).unwrap();

        assert_eq!(token.associativity(), Associativity::Left);
        assert_eq!(token.precedence(), 2);
        assert_eq!(token.kind(), TokenType::Minus);
    }

    #[test]
    fn valid_multiply() {
        let mult: String = String::from("*");
        let token = Token::new(&mult).unwrap();

        assert_eq!(token.associativity(), Associativity::Left);
        assert_eq!(token.precedence(), 3);
        assert_eq!(token.kind(), TokenType::Multiply);
    }
    #[test]
    fn valid_divide() {
        let div: String = String::from("/");
        let token = Token::new(&div).unwrap();

        assert_eq!(token.associativity(), Associativity::Left);
        assert_eq!(token.precedence(), 3);
        assert_eq!(token.kind(), TokenType::Divide);
    }

    #[test]
    fn valid_power() {
        let pow: String = String::from("^");
        let token = Token::new(&pow).unwrap();

        assert_eq!(token.associativity(), Associativity::Right);
        assert_eq!(token.precedence(), 4);
        assert_eq!(token.kind(), TokenType::Power);
    }

    #[test]
    fn valid_int() {
        let c: String = String::from("324");
        let token = Token::new(&c).unwrap();

        assert_eq!(token.kind(), TokenType::Number);
        assert_eq!(token.value(), 324f64);
    }

    #[test]
    fn valid_float_short() {
        let c: String = String::from("324.");
        let token = Token::new(&c).unwrap();

        assert_eq!(token.kind(), TokenType::Number);
        assert_eq!(token.value(), 324f64);
    }

    #[test]
    fn valid_float() {
        let c: String = String::from("324.34532342");
        let token = Token::new(&c).unwrap();

        assert_eq!(token.kind(), TokenType::Number);
        assert_eq!(token.value(), 324.34532342f64);
    }

    #[test]
    fn negative_number() {
        let c: String = String::from("-324.34532342");
        let token = Token::new(&c).unwrap();

        assert_eq!(token.kind(), TokenType::Number);
        assert_eq!(token.value(), -324.34532342f64);
    }

    #[test]
    fn invalid() {
        let c: String = String::from("sfnwo");
        let token = Token::new(&c);
        assert!(token.is_err());
    }

    #[test]
    fn single_num() {
        let v = tokenize("123").unwrap();
        assert_eq!(v[0].value(), 123f64);
        assert_eq!(v[0].kind(), TokenType::Number);
    }

    #[test]
    fn negative_num() {
        let v = tokenize("-123").unwrap();
        assert_eq!(v[0].value(), -123f64);
        assert_eq!(v[0].kind(), TokenType::Number);
    }

    #[test]
    fn single_short_float() {
        let v = tokenize("123.").unwrap();
        assert_eq!(v[0].value(), 123f64);
        assert_eq!(v[0].kind(), TokenType::Number);
    }

    #[test]
    fn single_float() {
        let v = tokenize("123.34").unwrap();
        assert_eq!(v[0].value(), 123.34f64);
        assert_eq!(v[0].kind(), TokenType::Number);
    }
    #[test]
    fn operator_plus() {
        let v = tokenize("123+54").unwrap();
        assert_eq!(v[0].value(), 123f64);
        assert_eq!(v[0].kind(), TokenType::Number);

        assert_eq!(v[1].kind(), TokenType::Plus);

        assert_eq!(v[2].value(), 54f64);
        assert_eq!(v[2].kind(), TokenType::Number);
    }

    #[test]
    fn operator_minus() {
        let v = tokenize("123-54").unwrap();
        assert_eq!(v[0].value(), 123f64);
        assert_eq!(v[0].kind(), TokenType::Number);

        assert_eq!(v[1].kind(), TokenType::Minus);

        assert_eq!(v[2].value(), 54f64);
        assert_eq!(v[2].kind(), TokenType::Number);
    }

    #[test]
    fn operator_mult() {
        let v = tokenize("123*54").unwrap();
        assert_eq!(v[0].value(), 123f64);
        assert_eq!(v[0].kind(), TokenType::Number);

        assert_eq!(v[1].kind(), TokenType::Multiply);

        assert_eq!(v[2].value(), 54f64);
        assert_eq!(v[2].kind(), TokenType::Number);
    }

    #[test]
    fn operator_div() {
        let v = tokenize("123/54").unwrap();
        assert_eq!(v[0].value(), 123f64);
        assert_eq!(v[0].kind(), TokenType::Number);

        assert_eq!(v[1].kind(), TokenType::Divide);

        assert_eq!(v[2].value(), 54f64);
        assert_eq!(v[2].kind(), TokenType::Number);
    }

    #[test]
    fn operator_minus_num() {
        let v = tokenize("123*-54").unwrap();
        assert_eq!(v[0].value(), 123f64);
        assert_eq!(v[0].kind(), TokenType::Number);

        assert_eq!(v[1].kind(), TokenType::Multiply);

        assert_eq!(v[2].value(), -54f64);
        assert_eq!(v[2].kind(), TokenType::Number);
    }

    #[test]
    fn operator_chain() {
        let v = tokenize("123+54-2*3/6^4").unwrap();
        assert_eq!(v[0].value(), 123f64);
        assert_eq!(v[0].kind(), TokenType::Number);

        assert_eq!(v[1].kind(), TokenType::Plus);

        assert_eq!(v[2].value(), 54f64);
        assert_eq!(v[2].kind(), TokenType::Number);

        assert_eq!(v[3].kind(), TokenType::Minus);

        assert_eq!(v[4].value(), 2f64);
        assert_eq!(v[4].kind(), TokenType::Number);

        assert_eq!(v[5].kind(), TokenType::Multiply);

        assert_eq!(v[6].value(), 3f64);
        assert_eq!(v[6].kind(), TokenType::Number);

        assert_eq!(v[7].kind(), TokenType::Divide);

        assert_eq!(v[8].value(), 6f64);
        assert_eq!(v[8].kind(), TokenType::Number);

        assert_eq!(v[9].kind(), TokenType::Power);

        assert_eq!(v[10].value(), 4f64);
        assert_eq!(v[10].kind(), TokenType::Number);
    }

    #[test]
    fn simple_brackets() {
        let v = tokenize("(12+6)").unwrap();
        assert_eq!(v[0].kind(), TokenType::OpeningParenthesis);

        assert_eq!(v[1].kind(), TokenType::Number);
        assert_eq!(v[1].value(), 12.0);

        assert_eq!(v[2].kind(), TokenType::Plus);

        assert_eq!(v[3].kind(), TokenType::Number);
        assert_eq!(v[3].value(), 6.0);

        assert_eq!(v[4].kind(), TokenType::ClosingParaenthesis);
    }

    #[test]
    fn multiply_bracket() {
        let v = tokenize("2*(12+6)").unwrap();
        assert_eq!(v[0].kind(), TokenType::Number);
        assert_eq!(v[0].value(), 2.0);
        assert_eq!(v[1].kind(), TokenType::Multiply);

        assert_eq!(v[2].kind(), TokenType::OpeningParenthesis);

        assert_eq!(v[3].kind(), TokenType::Number);
        assert_eq!(v[3].value(), 12.0);

        assert_eq!(v[4].kind(), TokenType::Plus);

        assert_eq!(v[5].kind(), TokenType::Number);
        assert_eq!(v[5].value(), 6.0);

        assert_eq!(v[6].kind(), TokenType::ClosingParaenthesis);
    }

    #[test]
    fn cascading_bracket() {
        let v = tokenize("(12+(3-(2*2)))").unwrap();

        assert_eq!(v[0].kind(), TokenType::OpeningParenthesis);
        assert_eq!(v[1].kind(), TokenType::Number);
        assert_eq!(v[2].kind(), TokenType::Plus);
        assert_eq!(v[3].kind(), TokenType::OpeningParenthesis);
        assert_eq!(v[4].kind(), TokenType::Number);
        assert_eq!(v[5].kind(), TokenType::Minus);
        assert_eq!(v[6].kind(), TokenType::OpeningParenthesis);
        assert_eq!(v[7].kind(), TokenType::Number);
        assert_eq!(v[8].kind(), TokenType::Multiply);
        assert_eq!(v[9].kind(), TokenType::Number);
        assert_eq!(v[10].kind(), TokenType::ClosingParaenthesis);
        assert_eq!(v[11].kind(), TokenType::ClosingParaenthesis);
        assert_eq!(v[12].kind(), TokenType::ClosingParaenthesis);
    }
}

#[cfg(test)]
mod calculator_tests {
    use crate::parser::calculator::*;

    #[test]
    fn addition() {
        assert_eq!(evaluate(&"2+5".to_string()).unwrap(), 7.0);
    }

    #[test]
    fn subtraction() {
        assert_eq!(evaluate(&"2-5".to_string()).unwrap(), -3.0);
    }

    #[test]
    fn multiply() {
        assert_eq!(evaluate(&"2*5".to_string()).unwrap(), 10.0);
    }

    #[test]
    fn division() {
        assert_eq!(evaluate(&"6/2".to_string()).unwrap(), 3.0);
    }

    #[test]
    fn power() {
        assert_eq!(evaluate(&"6^2".to_string()).unwrap(), 36.0);
    }

    #[test]
    fn negative_operand() {
        assert_eq!(evaluate(&"2^-2".to_string()).unwrap(), 0.25);
        assert_eq!(evaluate(&"3+-6".to_string()).unwrap(), -3.0);
        assert_eq!(evaluate(&"6--2".to_string()).unwrap(), 8.0);
        assert_eq!(evaluate(&"4*-2".to_string()).unwrap(), -8.0);
        assert_eq!(evaluate(&"6/-2".to_string()).unwrap(), -3.0);
    }

    #[test]
    fn chained_expression() {
        assert_eq!(evaluate(&"2+4-2*2/2^4".to_string()).unwrap(), 5.75);
    }

    #[test]
    fn associativity() {
        assert_eq!(evaluate(&"2*3*4".to_string()).unwrap(), 24.0);
        assert_eq!(evaluate(&"4-7-9".to_string()).unwrap(), -12.0);
        assert_eq!(evaluate(&"18/3/2".to_string()).unwrap(), 3.0);
        assert_eq!(evaluate(&"2^2^3".to_string()).unwrap(), 256.0);
    }
}
