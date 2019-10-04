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
        ClosingParaenthesis
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

    pub fn tokenize(content: &String) -> Result<Vec<Token>, &'static str> {
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
                } else if c == '^' || c == '*' || c == '/' || c == '-' || c == '+'{
                    v.push(Token::new(&buffer).unwrap());
                    buffer = String::new();
                    buffer.push(c);
                    state = ParserState::OPERATOR;
                } else if c == '(' || c == ')' {
                    v.push(Token::new(&buffer).unwrap());
                    buffer = String::new();
                    buffer.push(c);
                    state = ParserState::PARENTHESIS;
                }
            } else if state == ParserState::OPERATOR {
                if c == '-' || c.is_numeric() {
                    v.push(Token::new(&buffer).unwrap());
                    buffer = String::new();
                    buffer.push(c);
                    state = ParserState::Number;
                } else if c == '(' || c == ')' {
                    v.push(Token::new(&buffer).unwrap());
                    buffer = String::new();
                    buffer.push(c);
                    state = ParserState::PARENTHESIS;
                }
            } else if state == ParserState::PARENTHESIS{
                if c == '-' || c.is_numeric(){
                    v.push(Token::new(&buffer).unwrap());
                    buffer = String::new();
                    buffer.push(c);
                    state = ParserState::Number;
                } else if c == '^' || c == '*' || c == '/' || c == '-' || c == '+' {
                    v.push(Token::new(&buffer).unwrap());
                    buffer = String::new();
                    buffer.push(c);
                    state = ParserState::OPERATOR;
                } else if c == '(' || c == ')'{
                    v.push(Token::new(&buffer).unwrap());
                    buffer = String::new();
                    buffer.push(c);
                    state = ParserState::PARENTHESIS;
                }
            } else if state == ParserState::NONE {
                if c.is_numeric() || c == '-' {
                    buffer.push(c);
                    state = ParserState::Number;
                }
                else if c == '(' {
                    buffer.push(c);
                    state = ParserState::PARENTHESIS;
                }
            }
        }
        if !buffer.is_empty() && (state == ParserState::Number || state == ParserState::PARENTHESIS) {
            v.push(Token::new(&buffer).unwrap());
        } else {
            return Err("Invalid end of expression");
        }

        Ok(v)
    }

    impl Token {
        pub fn is_operator(&self) -> bool {
           match self.get_type() {
            TokenType::Divide | TokenType::Multiply | TokenType::Plus | TokenType::Minus | TokenType::Power => true,
            _ => false,
           }
        }

        pub fn new(content: &str) -> Result<Token, &'static str> {
            if Regex::new(r"^\+$").unwrap().is_match(content) {
                Ok(Token {
                    r#type: TokenType::Plus,
                    value: 0f64,
                    precedence: 2,
                    associativity: Associativity::Left,
                })
            } else if Regex::new(r"^\-$").unwrap().is_match(content) {
                Ok(Token {
                    r#type: TokenType::Minus,
                    value: 0f64,
                    precedence: 2,
                    associativity: Associativity::Left,
                })
            } else if Regex::new(r"^\*$").unwrap().is_match(content) {
                Ok(Token {
                    r#type: TokenType::Multiply,
                    value: 0f64,
                    precedence: 3,
                    associativity: Associativity::Left,
                })
            } else if Regex::new(r"^/$").unwrap().is_match(content) {
                Ok(Token {
                    r#type: TokenType::Divide,
                    value: 0f64,
                    precedence: 3,
                    associativity: Associativity::Left,
                })
            } else if Regex::new(r"^\^$").unwrap().is_match(content) {
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
            } else if Regex::new(r"^\($").unwrap().is_match(content){
                 Ok(Token {
                    r#type: TokenType::OpeningParenthesis,
                    value: 0f64,
                    precedence: 0,
                    associativity: Associativity::Right,
                })
             } else if Regex::new(r"^\)$").unwrap().is_match(content){
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

        pub fn get_value(&self) -> f64 {
            self.value
        }
        pub fn get_type(&self) -> TokenType {
            self.r#type
        }
        pub fn get_precedence(&self) -> u8 {
            self.precedence
        }
        pub fn get_associativity(&self) -> Associativity {
            self.associativity
        }
    }
}

pub mod calculator {
    use crate::parser::lexer::*;

    pub fn evaluate(expression: &String) -> Result<f64, &'static str> {
        match tokenize(expression) {
            Ok(v) => match shunting_yard(&v) {
                Ok(s) => calculate(&s),
                Err(x) => Err(x),
            },
            Err(e) => Err(e),
        }
    }

    fn calculate(tokens: &Vec<Token>) -> Result<f64, &'static str> {
        let mut processing_numbers: Vec<f64> = Vec::new();
        for t in tokens.iter() {
            if t.get_type() == TokenType::Number {
                processing_numbers.push(t.get_value());
            }
            if t.is_operator() {
                let r_operand = processing_numbers.pop();
                let l_operand = processing_numbers.pop();
                if r_operand.is_some() && l_operand.is_some() {
                    let result: f64 = match t.get_type() {
                        TokenType::Divide => l_operand.unwrap() / r_operand.unwrap(),
                        TokenType::Multiply => l_operand.unwrap() * r_operand.unwrap(),
                        TokenType::Minus => l_operand.unwrap() - r_operand.unwrap(),
                        TokenType::Plus => l_operand.unwrap() + r_operand.unwrap(),
                        TokenType::Power => l_operand.unwrap().powf(r_operand.unwrap()),
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

    fn shunting_yard(tokens: &Vec<Token>) -> Result<Vec<Token>, &'static str> {
        let mut reverse_notation: Vec<Token> = Vec::new();
        let mut stack: Vec<Token> = Vec::new();

        for t in tokens.iter() {
            if t.get_type() == TokenType::Number {
                reverse_notation.push(*t);
            }else if t.get_type() == TokenType::OpeningParenthesis{
                stack.push(*t);
            }else if t.get_type() == TokenType::ClosingParaenthesis{
                while !stack.is_empty() && stack.last().unwrap().get_type() != TokenType::OpeningParenthesis 
                {
                    reverse_notation.push(stack.pop().unwrap());
                }
                if !stack.is_empty() && stack.last().unwrap().get_type() == TokenType::OpeningParenthesis
                {
                    stack.pop();
                }else{
                    return Err("Expected opening bracket");
                }
            } else if t.is_operator() {
                while !stack.is_empty()
                    && (stack.last().unwrap().get_precedence() > t.get_precedence()
                        || (stack.last().unwrap().get_precedence() == t.get_precedence()
                            && stack.last().unwrap().get_associativity() == Associativity::Left))
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

        assert_eq!(token.get_associativity(), Associativity::Left);
        assert_eq!(token.get_precedence(), 2);
        assert_eq!(token.get_type(), TokenType::Plus);
    }

    #[test]
    fn valid_minus() {
        let minus: String = String::from("-");
        let token = Token::new(&minus).unwrap();

        assert_eq!(token.get_associativity(), Associativity::Left);
        assert_eq!(token.get_precedence(), 2);
        assert_eq!(token.get_type(), TokenType::Minus);
    }

    #[test]
    fn valid_multiply() {
        let mult: String = String::from("*");
        let token = Token::new(&mult).unwrap();

        assert_eq!(token.get_associativity(), Associativity::Left);
        assert_eq!(token.get_precedence(), 3);
        assert_eq!(token.get_type(), TokenType::Multiply);
    }
    #[test]
    fn valid_divide() {
        let div: String = String::from("/");
        let token = Token::new(&div).unwrap();

        assert_eq!(token.get_associativity(), Associativity::Left);
        assert_eq!(token.get_precedence(), 3);
        assert_eq!(token.get_type(), TokenType::Divide);
    }

    #[test]
    fn valid_power() {
        let pow: String = String::from("^");
        let token = Token::new(&pow).unwrap();

        assert_eq!(token.get_associativity(), Associativity::Right);
        assert_eq!(token.get_precedence(), 4);
        assert_eq!(token.get_type(), TokenType::Power);
    }

    #[test]
    fn valid_int() {
        let c: String = String::from("324");
        let token = Token::new(&c).unwrap();

        assert_eq!(token.get_type(), TokenType::Number);
        assert_eq!(token.get_value(), 324f64);
    }

    #[test]
    fn valid_float_short() {
        let c: String = String::from("324.");
        let token = Token::new(&c).unwrap();

        assert_eq!(token.get_type(), TokenType::Number);
        assert_eq!(token.get_value(), 324f64);
    }

    #[test]
    fn valid_float() {
        let c: String = String::from("324.34532342");
        let token = Token::new(&c).unwrap();

        assert_eq!(token.get_type(), TokenType::Number);
        assert_eq!(token.get_value(), 324.34532342f64);
    }

    #[test]
    fn negative_number() {
        let c: String = String::from("-324.34532342");
        let token = Token::new(&c).unwrap();

        assert_eq!(token.get_type(), TokenType::Number);
        assert_eq!(token.get_value(), -324.34532342f64);
    }

    #[test]
    fn invalid() {
        let c: String = String::from("sfnwo");
        let token = Token::new(&c);
        assert!(token.is_err());
    }

    #[test]
    fn single_num() {
        let v = tokenize(&String::from("123")).unwrap();
        assert_eq!(v[0].get_value(), 123f64);
        assert_eq!(v[0].get_type(), TokenType::Number);
    }

    #[test]
    fn negative_num() {
        let v = tokenize(&String::from("-123")).unwrap();
        assert_eq!(v[0].get_value(), -123f64);
        assert_eq!(v[0].get_type(), TokenType::Number);
    }

    #[test]
    fn single_short_float() {
        let v = tokenize(&String::from("123.")).unwrap();
        assert_eq!(v[0].get_value(), 123f64);
        assert_eq!(v[0].get_type(), TokenType::Number);
    }

    #[test]
    fn single_float() {
        let v = tokenize(&String::from("123.34")).unwrap();
        assert_eq!(v[0].get_value(), 123.34f64);
        assert_eq!(v[0].get_type(), TokenType::Number);
    }
    #[test]
    fn operator_plus() {
        let v = tokenize(&String::from("123+54")).unwrap();
        assert_eq!(v[0].get_value(), 123f64);
        assert_eq!(v[0].get_type(), TokenType::Number);

        assert_eq!(v[1].get_type(), TokenType::Plus);

        assert_eq!(v[2].get_value(), 54f64);
        assert_eq!(v[2].get_type(), TokenType::Number);
    }

    #[test]
    fn operator_minus() {
        let v = tokenize(&String::from("123-54")).unwrap();
        assert_eq!(v[0].get_value(), 123f64);
        assert_eq!(v[0].get_type(), TokenType::Number);

        assert_eq!(v[1].get_type(), TokenType::Minus);

        assert_eq!(v[2].get_value(), 54f64);
        assert_eq!(v[2].get_type(), TokenType::Number);
    }

    #[test]
    fn operator_mult() {
        let v = tokenize(&String::from("123*54")).unwrap();
        assert_eq!(v[0].get_value(), 123f64);
        assert_eq!(v[0].get_type(), TokenType::Number);

        assert_eq!(v[1].get_type(), TokenType::Multiply);

        assert_eq!(v[2].get_value(), 54f64);
        assert_eq!(v[2].get_type(), TokenType::Number);
    }

    #[test]
    fn operator_div() {
        let v = tokenize(&String::from("123/54")).unwrap();
        assert_eq!(v[0].get_value(), 123f64);
        assert_eq!(v[0].get_type(), TokenType::Number);

        assert_eq!(v[1].get_type(), TokenType::Divide);

        assert_eq!(v[2].get_value(), 54f64);
        assert_eq!(v[2].get_type(), TokenType::Number);
    }

    #[test]
    fn operator_minus_num() {
        let v = tokenize(&String::from("123*-54")).unwrap();
        assert_eq!(v[0].get_value(), 123f64);
        assert_eq!(v[0].get_type(), TokenType::Number);

        assert_eq!(v[1].get_type(), TokenType::Multiply);

        assert_eq!(v[2].get_value(), -54f64);
        assert_eq!(v[2].get_type(), TokenType::Number);
    }

    #[test]
    fn operator_chain() {
        let v = tokenize(&String::from("123+54-2*3/6^4")).unwrap();
        assert_eq!(v[0].get_value(), 123f64);
        assert_eq!(v[0].get_type(), TokenType::Number);

        assert_eq!(v[1].get_type(), TokenType::Plus);

        assert_eq!(v[2].get_value(), 54f64);
        assert_eq!(v[2].get_type(), TokenType::Number);

        assert_eq!(v[3].get_type(), TokenType::Minus);

        assert_eq!(v[4].get_value(), 2f64);
        assert_eq!(v[4].get_type(), TokenType::Number);

        assert_eq!(v[5].get_type(), TokenType::Multiply);

        assert_eq!(v[6].get_value(), 3f64);
        assert_eq!(v[6].get_type(), TokenType::Number);

        assert_eq!(v[7].get_type(), TokenType::Divide);

        assert_eq!(v[8].get_value(), 6f64);
        assert_eq!(v[8].get_type(), TokenType::Number);

        assert_eq!(v[9].get_type(), TokenType::Power);

        assert_eq!(v[10].get_value(), 4f64);
        assert_eq!(v[10].get_type(), TokenType::Number);
    }

    #[test]
    fn simple_brackets() {
        let v = tokenize(&String::from("(12+6)")).unwrap();
        assert_eq!(v[0].get_type(), TokenType::OpeningParenthesis);

        assert_eq!(v[1].get_type(), TokenType::Number);
        assert_eq!(v[1].get_value(), 12.0);

        assert_eq!(v[2].get_type(), TokenType::Plus);

        assert_eq!(v[3].get_type(), TokenType::Number);
        assert_eq!(v[3].get_value(), 6.0);

        assert_eq!(v[4].get_type(), TokenType::ClosingParaenthesis);
    }

    #[test]
    fn multiply_bracket() {
        let v = tokenize(&String::from("2*(12+6)")).unwrap();
        assert_eq!(v[0].get_type(), TokenType::Number);
        assert_eq!(v[0].get_value(), 2.0);
        
        assert_eq!(v[1].get_type(), TokenType::Multiply);

        assert_eq!(v[2].get_type(), TokenType::OpeningParenthesis);

        assert_eq!(v[3].get_type(), TokenType::Number);
        assert_eq!(v[3].get_value(), 12.0);

        assert_eq!(v[4].get_type(), TokenType::Plus);

        assert_eq!(v[5].get_type(), TokenType::Number);
        assert_eq!(v[5].get_value(), 6.0);

        assert_eq!(v[6].get_type(), TokenType::ClosingParaenthesis);
    }

    #[test]
    fn cascading_bracket() {
        let v = tokenize(&String::from("(12+(3-(2*2)))")).unwrap();

        assert_eq!(v[0].get_type(), TokenType::OpeningParenthesis);
        assert_eq!(v[1].get_type(), TokenType::Number);
        assert_eq!(v[2].get_type(), TokenType::Plus);
        assert_eq!(v[3].get_type(), TokenType::OpeningParenthesis);
        assert_eq!(v[4].get_type(), TokenType::Number);
        assert_eq!(v[5].get_type(), TokenType::Minus);
        assert_eq!(v[6].get_type(), TokenType::OpeningParenthesis);
        assert_eq!(v[7].get_type(), TokenType::Number);
        assert_eq!(v[8].get_type(), TokenType::Multiply);
        assert_eq!(v[9].get_type(), TokenType::Number);
        assert_eq!(v[10].get_type(), TokenType::ClosingParaenthesis);
        assert_eq!(v[11].get_type(), TokenType::ClosingParaenthesis);
        assert_eq!(v[12].get_type(), TokenType::ClosingParaenthesis);
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
