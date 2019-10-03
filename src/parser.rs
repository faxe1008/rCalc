pub mod lexer {
    use regex::Regex;

    #[derive(PartialEq, Debug, Copy, Clone)]
    pub enum TokenType {
        PLUS,
        MINUS,
        MULTIPLY,
        DIVIDE,
        POWER,
        NUMBER,
    }
    #[derive(PartialEq, Debug, Copy, Clone)]
    pub enum Associativity {
        LEFT,
        RIGHT,
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
        NUMBER,
    }

    pub fn tokenize(content: &String) -> Result<Vec<Token>, &'static str> {
        let mut v: Vec<Token> = Vec::new();
        let mut state = ParserState::NONE;
        let mut buffer = String::new();
        for c in content.chars() {
            if c.is_alphabetic() {
                let error = format!("Invalid character: {}", c);
                return Err(Box::leak(error.into_boxed_str()));
            } else if state == ParserState::NUMBER {
                if c.is_numeric() || c == '.' {
                    buffer.push(c);
                } else if c == '^' || c == '*' || c == '/' || c == '-' || c == '+' {
                    v.push(Token::new(&buffer).unwrap());
                    buffer = String::new();
                    buffer.push(c);
                    state = ParserState::OPERATOR;
                }
            } else if state == ParserState::OPERATOR {
                if c == '-' || c.is_numeric() {
                    v.push(Token::new(&buffer).unwrap());
                    buffer = String::new();
                    buffer.push(c);
                    state = ParserState::NUMBER;
                }
            } else if state == ParserState::NONE {
                if c.is_numeric() || c == '-' {
                    buffer.push(c);
                    state = ParserState::NUMBER;
                }
            }
        }
        if !buffer.is_empty() && state == ParserState::NUMBER {
            let token = Token::new(&buffer);
            v.push(Token::new(&buffer).unwrap());
        } else {
            return Err("Invalid end of expression");
        }

        return Ok(v);
    }

    impl Token {
        pub fn is_operator(&self) -> bool {
            self.get_type() == TokenType::DIVIDE
                || self.get_type() == TokenType::MULTIPLY
                || self.get_type() == TokenType::PLUS
                || self.get_type() == TokenType::MINUS
                || self.get_type() == TokenType::POWER
        }

        pub fn new(content: &str) -> Result<Token, &'static str> {
            if Regex::new(r"^\+$").unwrap().is_match(content) {
                Ok(Token {
                    r#type: TokenType::PLUS,
                    value: 0f64,
                    precedence: 2,
                    associativity: Associativity::LEFT,
                })
            } else if Regex::new(r"^\-$").unwrap().is_match(content) {
                Ok(Token {
                    r#type: TokenType::MINUS,
                    value: 0f64,
                    precedence: 2,
                    associativity: Associativity::LEFT,
                })
            } else if Regex::new(r"^\*$").unwrap().is_match(content) {
                Ok(Token {
                    r#type: TokenType::MULTIPLY,
                    value: 0f64,
                    precedence: 3,
                    associativity: Associativity::LEFT,
                })
            } else if Regex::new(r"^/$").unwrap().is_match(content) {
                Ok(Token {
                    r#type: TokenType::DIVIDE,
                    value: 0f64,
                    precedence: 3,
                    associativity: Associativity::LEFT,
                })
            } else if Regex::new(r"^\^$").unwrap().is_match(content) {
                Ok(Token {
                    r#type: TokenType::POWER,
                    value: 0f64,
                    precedence: 4,
                    associativity: Associativity::RIGHT,
                })
            } else if Regex::new(r"^-?\d+\.?\d*$").unwrap().is_match(content) {
                Ok(Token {
                    r#type: TokenType::NUMBER,
                    value: content.parse().expect("number required"),
                    precedence: 0,
                    associativity: Associativity::RIGHT,
                })
            } else {
                let error = format!("Error creating new token with content: {}", content);
                return Err(Box::leak(error.into_boxed_str()));
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
            if t.get_type() == TokenType::NUMBER {
                processing_numbers.push(t.get_value());
            }
            if t.is_operator() {
                let r_operand = processing_numbers.pop();
                let l_operand = processing_numbers.pop();
                if r_operand.is_some() && l_operand.is_some() {
                    let result: f64 = match t.get_type() {
                        TokenType::DIVIDE => l_operand.unwrap() / r_operand.unwrap(),
                        TokenType::MULTIPLY => l_operand.unwrap() * r_operand.unwrap(),
                        TokenType::MINUS => l_operand.unwrap() - r_operand.unwrap(),
                        TokenType::PLUS => l_operand.unwrap() + r_operand.unwrap(),
                        TokenType::POWER => l_operand.unwrap().powf(r_operand.unwrap()),
                        TokenType::NUMBER => return Err("Can not have number on operation stack"),
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
            if t.get_type() == TokenType::NUMBER {
                reverse_notation.push(*t);
            } else if t.is_operator() {
                while !stack.is_empty()
                    && (stack.last().unwrap().get_precedence() > t.get_precedence()
                        || (stack.last().unwrap().get_precedence() == t.get_precedence()
                            && stack.last().unwrap().get_associativity() == Associativity::LEFT))
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
