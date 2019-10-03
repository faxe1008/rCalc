extern crate regex;
pub mod parser;

#[cfg(test)]
mod lexer_tests {
    use crate::parser::lexer::*;

    #[test]
    fn valid_plus() {
        let plus: String = String::from("+");
        let token = Token::new(&plus).unwrap();

        assert_eq!(token.get_associativity(), Associativity::LEFT);
        assert_eq!(token.get_precedence(), 2);
        assert_eq!(token.get_type(), TokenType::PLUS);
    }

    #[test]
    fn valid_minus() {
        let minus: String = String::from("-");
        let token = Token::new(&minus).unwrap();

        assert_eq!(token.get_associativity(), Associativity::LEFT);
        assert_eq!(token.get_precedence(), 2);
        assert_eq!(token.get_type(), TokenType::MINUS);
    }

    #[test]
    fn valid_multiply() {
        let mult: String = String::from("*");
        let token = Token::new(&mult).unwrap();

        assert_eq!(token.get_associativity(), Associativity::LEFT);
        assert_eq!(token.get_precedence(), 3);
        assert_eq!(token.get_type(), TokenType::MULTIPLY);
    }
    #[test]
    fn valid_divide() {
        let div: String = String::from("/");
        let token = Token::new(&div).unwrap();

        assert_eq!(token.get_associativity(), Associativity::LEFT);
        assert_eq!(token.get_precedence(), 3);
        assert_eq!(token.get_type(), TokenType::DIVIDE);
    }

    #[test]
    fn valid_power() {
        let pow: String = String::from("^");
        let token = Token::new(&pow).unwrap();

        assert_eq!(token.get_associativity(), Associativity::RIGHT);
        assert_eq!(token.get_precedence(), 4);
        assert_eq!(token.get_type(), TokenType::POWER);
    }

    #[test]
    fn valid_int() {
        let c: String = String::from("324");
        let token = Token::new(&c).unwrap();

        assert_eq!(token.get_type(), TokenType::NUMBER);
        assert_eq!(token.get_value(), 324f64);
    }

    #[test]
    fn valid_float_short() {
        let c: String = String::from("324.");
        let token = Token::new(&c).unwrap();

        assert_eq!(token.get_type(), TokenType::NUMBER);
        assert_eq!(token.get_value(), 324f64);
    }

    #[test]
    fn valid_float() {
        let c: String = String::from("324.34532342");
        let token = Token::new(&c).unwrap();

        assert_eq!(token.get_type(), TokenType::NUMBER);
        assert_eq!(token.get_value(), 324.34532342f64);
    }

    #[test]
    fn negative_number() {
        let c: String = String::from("-324.34532342");
        let token = Token::new(&c).unwrap();

        assert_eq!(token.get_type(), TokenType::NUMBER);
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
        let v = tokenize(String::from("123")).unwrap();
        assert_eq!(v[0].get_value(), 123f64);
        assert_eq!(v[0].get_type(), TokenType::NUMBER);
    }

    #[test]
    fn negative_num() {
        let v = tokenize(String::from("-123")).unwrap();
        assert_eq!(v[0].get_value(), -123f64);
        assert_eq!(v[0].get_type(), TokenType::NUMBER);
    }

    #[test]
    fn single_short_float() {
        let v = tokenize(String::from("123.")).unwrap();
        assert_eq!(v[0].get_value(), 123f64);
        assert_eq!(v[0].get_type(), TokenType::NUMBER);
    }

    #[test]
    fn single_float() {
        let v = tokenize(String::from("123.34")).unwrap();
        assert_eq!(v[0].get_value(), 123.34f64);
        assert_eq!(v[0].get_type(), TokenType::NUMBER);
    }
    #[test]
    fn operator_plus() {
        let v = tokenize(String::from("123+54")).unwrap();
        assert_eq!(v[0].get_value(), 123f64);
        assert_eq!(v[0].get_type(), TokenType::NUMBER);

        assert_eq!(v[1].get_type(), TokenType::PLUS);

        assert_eq!(v[2].get_value(), 54f64);
        assert_eq!(v[2].get_type(), TokenType::NUMBER);
    }

    #[test]
    fn operator_minus() {
        let v = tokenize(String::from("123-54")).unwrap();
        assert_eq!(v[0].get_value(), 123f64);
        assert_eq!(v[0].get_type(), TokenType::NUMBER);

        assert_eq!(v[1].get_type(), TokenType::MINUS);

        assert_eq!(v[2].get_value(), 54f64);
        assert_eq!(v[2].get_type(), TokenType::NUMBER);
    }

    #[test]
    fn operator_mult() {
        let v = tokenize(String::from("123*54")).unwrap();
        assert_eq!(v[0].get_value(), 123f64);
        assert_eq!(v[0].get_type(), TokenType::NUMBER);

        assert_eq!(v[1].get_type(), TokenType::MULTIPLY);

        assert_eq!(v[2].get_value(), 54f64);
        assert_eq!(v[2].get_type(), TokenType::NUMBER);
    }

    #[test]
    fn operator_div() {
        let v = tokenize(String::from("123/54")).unwrap();
        assert_eq!(v[0].get_value(), 123f64);
        assert_eq!(v[0].get_type(), TokenType::NUMBER);

        assert_eq!(v[1].get_type(), TokenType::DIVIDE);

        assert_eq!(v[2].get_value(), 54f64);
        assert_eq!(v[2].get_type(), TokenType::NUMBER);
    }

    #[test]
    fn operator_minus_num() {
        let v = tokenize(String::from("123*-54")).unwrap();
        assert_eq!(v[0].get_value(), 123f64);
        assert_eq!(v[0].get_type(), TokenType::NUMBER);

        assert_eq!(v[1].get_type(), TokenType::MULTIPLY);

        assert_eq!(v[2].get_value(), -54f64);
        assert_eq!(v[2].get_type(), TokenType::NUMBER);
    }

    #[test]
    fn operator_chain() {
        let v = tokenize(String::from("123+54-2*3/6^4")).unwrap();
        assert_eq!(v[0].get_value(), 123f64);
        assert_eq!(v[0].get_type(), TokenType::NUMBER);

        assert_eq!(v[1].get_type(), TokenType::PLUS);

        assert_eq!(v[2].get_value(), 54f64);
        assert_eq!(v[2].get_type(), TokenType::NUMBER);

        assert_eq!(v[3].get_type(), TokenType::MINUS);

        assert_eq!(v[4].get_value(), 2f64);
        assert_eq!(v[4].get_type(), TokenType::NUMBER);

        assert_eq!(v[5].get_type(), TokenType::MULTIPLY);

        assert_eq!(v[6].get_value(), 3f64);
        assert_eq!(v[6].get_type(), TokenType::NUMBER);

        assert_eq!(v[7].get_type(), TokenType::DIVIDE);

        assert_eq!(v[8].get_value(), 6f64);
        assert_eq!(v[8].get_type(), TokenType::NUMBER);

        assert_eq!(v[9].get_type(), TokenType::POWER);

        assert_eq!(v[10].get_value(), 4f64);
        assert_eq!(v[10].get_type(), TokenType::NUMBER);
    }
}

#[cfg(test)]
mod calculator_tests {
    use crate::parser::calculator::*;

    #[test]
    fn addition() {
        assert_eq!(evaluate("2+5".to_string()).unwrap(), 7.0);
    }

    #[test]
    fn subtraction() {
        assert_eq!(evaluate("2-5".to_string()).unwrap(), -3.0);
    }

    #[test]
    fn multiply() {
        assert_eq!(evaluate("2*5".to_string()).unwrap(), 10.0);
    }

    #[test]
    fn division() {
        assert_eq!(evaluate("6/2".to_string()).unwrap(), 3.0);
    }

    #[test]
    fn power() {
        assert_eq!(evaluate("6^2".to_string()).unwrap(), 36.0);
    }

    #[test]
    fn negative_operand() {
        assert_eq!(evaluate("2^-2".to_string()).unwrap(), 0.25);
        assert_eq!(evaluate("3+-6".to_string()).unwrap(), -3.0);
        assert_eq!(evaluate("6--2".to_string()).unwrap(), 8.0);
        assert_eq!(evaluate("4*-2".to_string()).unwrap(), -8.0);
        assert_eq!(evaluate("6/-2".to_string()).unwrap(), -3.0);
    }

    #[test]
    fn chained_expression() {
        assert_eq!(evaluate("2+4-2*2/2^4".to_string()).unwrap(), 5.75);
    }

    #[test]
    fn associativity() {
        assert_eq!(evaluate("2*3*4".to_string()).unwrap(), 24.0);
        assert_eq!(evaluate("4-7-9".to_string()).unwrap(), -12.0);
        assert_eq!(evaluate("18/3/2".to_string()).unwrap(), 3.0);
        assert_eq!(evaluate("2^2^3".to_string()).unwrap(), 256.0);
    }
}
