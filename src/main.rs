#[derive(Debug, Clone)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl PartialEq for Operator {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Operator::Add, Operator::Add)
                | (Operator::Subtract, Operator::Subtract)
                | (Operator::Multiply, Operator::Multiply)
                | (Operator::Divide, Operator::Divide)
        )
    }
}

impl Operator {
    fn precedence(&self) -> u8 {
        match self {
            Operator::Add | Operator::Subtract => 1,
            Operator::Multiply | Operator::Divide => 2,
        }
    }
    
    fn apply(&self, left: f64, right: f64) -> Result<f64, Error> {
        match self {
            Operator::Add => Ok(left + right),
            Operator::Subtract => Ok(left - right),
            Operator::Multiply => Ok(left * right),
            Operator::Divide => {
                if right == 0.0 {
                    Err(Error::DivisionByZero)
                } else {
                    Ok(left / right)
                }
            }
        }
    }
}

impl PartialOrd for Operator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.precedence().partial_cmp(&other.precedence())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
     Op(Operator),
     Bracket(char),
}

pub struct Calculator {}

#[derive(Debug)]
pub enum Error {
    BadToken(char), 
    MismatchedParens,
    DivisionByZero,
    InvalidExpression,
}

impl Calculator {
    pub fn parse<T: AsRef<str>>(expr: T) -> Result<Vec<Token>, Error> {
        let expr = expr.as_ref(); 
        let chars: Vec<char> = expr.chars().collect();
        let mut tokens: Vec<Token> = Vec::new(); 
        let mut parens = Vec::new(); 
        let mut i = 0;

        while i < chars.len() {
            let c = chars[i];
            match c {
                '0'..='9' => {
                    let mut num_str = String::new();
                    let mut j = i;
                    
                    while j < chars.len() && (chars[j].is_ascii_digit() || chars[j] == '.') {
                        num_str.push(chars[j]);
                        j += 1;
                    }
                    
                    if let Ok(num) = num_str.parse::<f64>() {
                        tokens.push(Token::Number(num));
                    } else {
                        return Err(Error::BadToken(c));
                    }
                    i = j - 1; 
                }
                '(' => {
                    tokens.push(Token::Bracket('('));
                    parens.push(c);
                }
                ')' => {
                    tokens.push(Token::Bracket(')'));
                    if let Some(p) = parens.pop() {
                        if p != '(' {
                            return Err(Error::MismatchedParens);
                        }
                    } else {
                        return Err(Error::MismatchedParens);
                    }
                }
                '+' => tokens.push(Token::Op(Operator::Add)), 
                '-' => tokens.push(Token::Op(Operator::Subtract)), 
                '*' => tokens.push(Token::Op(Operator::Multiply)),
                '/' => tokens.push(Token::Op(Operator::Divide)),
                ' ' | '\t' | '\n' => {}
                _ => return Err(Error::BadToken(c)),
                    }
            i += 1;
                }

        if !parens.is_empty() {
                    return Err(Error::MismatchedParens);
                }

                Ok(tokens)
            }

    pub fn to_postfix(mut tokens: Vec<Token>) -> Vec<Token> {
            tokens.reverse(); 

            let mut queue: Vec<Token> = Vec::new(); 
            let mut stack: Vec<Token> = Vec::new(); 

        while let Some(token) = tokens.pop() {
            match token {
                    Token::Number(_) => queue.push(token), 
                Token::Op(ref op) => {
                    while let Some(Token::Op(stack_op)) = stack.last() {
                        if stack_op >= op {
                            queue.push(stack.pop().unwrap());
                        } else {
                            break;
                        }
                        }
                        stack.push(token); 
                }
                    Token::Bracket('(') => stack.push(token),
                    Token::Bracket(')') => {
                    while let Some(top) = stack.last() {
                        if *top == Token::Bracket('(') {
                            break;
                        }
                        queue.push(stack.pop().unwrap());
                        }
                        stack.pop(); 
                }
                _ => {}
            }
        }

        while !stack.is_empty() {
                    queue.push(stack.pop().unwrap());
        }

        queue
    }

    pub fn evaluate(tokens: Vec<Token>) -> Result<f64, Error> {
        let mut stack: Vec<f64> = Vec::new();

        for token in tokens {
            match token {
                Token::Number(n) => stack.push(n),
                Token::Op(op) => {
                    if stack.len() < 2 {
                        return Err(Error::InvalidExpression);
                    }
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    let result = op.apply(left, right)?;
                    stack.push(result);
                }
                _ => return Err(Error::InvalidExpression),
            }
        }

        if stack.len() == 1 {
            Ok(stack.pop().unwrap())
        } else {
            Err(Error::InvalidExpression)
        }
    }

    pub fn calculate<T: AsRef<str>>(expr: T) -> Result<f64, Error> {
        let tokens = Self::parse(expr)?;
        let postfix = Self::to_postfix(tokens);
        Self::evaluate(postfix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_arithmetic() {
        assert_eq!(Calculator::calculate("2 + 3").unwrap(), 5.0);
        assert_eq!(Calculator::calculate("10 - 4").unwrap(), 6.0);
        assert_eq!(Calculator::calculate("3 * 4").unwrap(), 12.0);
        assert_eq!(Calculator::calculate("15 / 3").unwrap(), 5.0);
    }

    #[test]
    fn test_operator_precedence() {
        assert_eq!(Calculator::calculate("2 + 3 * 4").unwrap(), 14.0);
        assert_eq!(Calculator::calculate("10 - 6 / 2").unwrap(), 7.0);
        assert_eq!(Calculator::calculate("2 * 3 + 4 * 5").unwrap(), 26.0);
        assert_eq!(Calculator::calculate("20 / 4 - 2 * 2").unwrap(), 1.0);
    }

    #[test]
    fn test_parentheses() {
        assert_eq!(Calculator::calculate("(2 + 3) * 4").unwrap(), 20.0);
        assert_eq!(Calculator::calculate("2 * (3 + 4)").unwrap(), 14.0);
        assert_eq!(Calculator::calculate("(10 - 6) / 2").unwrap(), 2.0);
        assert_eq!(Calculator::calculate("((2 + 3) * 4) / 5").unwrap(), 4.0);
    }

    #[test]
    fn test_nested_parentheses() {
        assert_eq!(Calculator::calculate("((2 + 3) * (4 + 1))").unwrap(), 25.0);
        assert_eq!(Calculator::calculate("(2 * (3 + 4)) - 1").unwrap(), 13.0);
        assert_eq!(Calculator::calculate("((10 / 2) + 3) * 2").unwrap(), 16.0);
    }

    #[test]
    fn test_decimal_numbers() {
        assert_eq!(Calculator::calculate("2.5 + 3.7").unwrap(), 6.2);
        assert_eq!(Calculator::calculate("10.5 / 2.1").unwrap(), 5.0);
        assert_eq!(Calculator::calculate("3.14 * 2").unwrap(), 6.28);
        assert_eq!(Calculator::calculate("7.5 - 2.25").unwrap(), 5.25);
    }

    #[test]
    fn test_whitespace_handling() {
        assert_eq!(Calculator::calculate("  2   +   3  ").unwrap(), 5.0);
        assert_eq!(Calculator::calculate("2+3").unwrap(), 5.0);
        assert_eq!(Calculator::calculate(" ( 2 + 3 ) * 4 ").unwrap(), 20.0);
        assert_eq!(Calculator::calculate("\t2\n*\t3\n").unwrap(), 6.0);
    }

    #[test]
    fn test_complex_expressions() {
        assert_eq!(Calculator::calculate("1 + 2 * 3 + 4").unwrap(), 11.0);
        assert_eq!(Calculator::calculate("(1 + 2) * (3 + 4)").unwrap(), 21.0);
        assert_eq!(Calculator::calculate("10 + 5 * 2 - 3 / 3").unwrap(), 19.0);
        assert_eq!(Calculator::calculate("100 / 4 / 5 + 2 * 3").unwrap(), 11.0);
    }

    #[test]
    fn test_division_by_zero() {
        match Calculator::calculate("5 / 0") {
            Err(Error::DivisionByZero) => (),
            _ => panic!("Expected DivisionByZero error"),
        }
        
        match Calculator::calculate("10 / (2 - 2)") {
            Err(Error::DivisionByZero) => (),
            _ => panic!("Expected DivisionByZero error"),
        }
    }

    #[test]
    fn test_invalid_tokens() {
        match Calculator::calculate("2 + @") {
            Err(Error::BadToken('@')) => (),
            _ => panic!("Expected BadToken error"),
        }
        
        match Calculator::calculate("5 & 3") {
            Err(Error::BadToken('&')) => (),
            _ => panic!("Expected BadToken error"),
        }
    }

    #[test]
    fn test_mismatched_parentheses() {
        match Calculator::calculate("(2 + 3") {
            Err(Error::MismatchedParens) => (),
            _ => panic!("Expected MismatchedParens error"),
        }
        
        match Calculator::calculate("2 + 3)") {
            Err(Error::MismatchedParens) => (),
            _ => panic!("Expected MismatchedParens error"),
        }
        
        match Calculator::calculate("((2 + 3)") {
            Err(Error::MismatchedParens) => (),
            _ => panic!("Expected MismatchedParens error"),
        }
    }

    #[test]
    fn test_empty_expression() {
        match Calculator::calculate("") {
            Err(Error::InvalidExpression) => (),
            _ => panic!("Expected InvalidExpression error"),
        }
    }

    #[test]
    fn test_operator_precedence_comprehensive() {
        // Test all combinations of operators
        assert_eq!(Calculator::calculate("1 + 2 * 3 - 4 / 2").unwrap(), 5.0);
        assert_eq!(Calculator::calculate("2 * 3 + 4 * 5 - 6 / 2").unwrap(), 23.0);
        assert_eq!(Calculator::calculate("10 / 2 + 3 * 4 - 5").unwrap(), 12.0);
    }

    #[test]
    fn test_large_numbers() {
        assert_eq!(Calculator::calculate("1000000 + 2000000").unwrap(), 3000000.0);
        assert_eq!(Calculator::calculate("999999 * 2").unwrap(), 1999998.0);
        assert_eq!(Calculator::calculate("1000000 / 1000").unwrap(), 1000.0);
    }

    #[test]
    fn test_negative_results() {
        assert_eq!(Calculator::calculate("3 - 5").unwrap(), -2.0);
        assert_eq!(Calculator::calculate("10 / 2 - 8").unwrap(), -3.0);
        assert_eq!(Calculator::calculate("(2 - 5) * 3").unwrap(), -9.0);
    }

    #[test]
    fn test_fractional_results() {
        assert_eq!(Calculator::calculate("1 / 2").unwrap(), 0.5);
        assert_eq!(Calculator::calculate("3 / 4").unwrap(), 0.75);
        assert_eq!(Calculator::calculate("7 / 8").unwrap(), 0.875);
    }

    #[test]
    fn test_parse_function() {
        let tokens = Calculator::parse("2 + 3").unwrap();
        assert_eq!(tokens.len(), 3);
        
        match &tokens[0] {
            Token::Number(n) => assert_eq!(*n, 2.0),
            _ => panic!("Expected number token"),
        }
        
        match &tokens[1] {
            Token::Op(Operator::Add) => (),
            _ => panic!("Expected add operator"),
        }
        
        match &tokens[2] {
            Token::Number(n) => assert_eq!(*n, 3.0),
            _ => panic!("Expected number token"),
        }
    }

    #[test]
    fn test_to_postfix_function() {
        let tokens = Calculator::parse("2 + 3 * 4").unwrap();
        let postfix = Calculator::to_postfix(tokens);
        
        // Should be: 2 3 4 * +
        assert_eq!(postfix.len(), 5);
        
        match &postfix[0] {
            Token::Number(n) => assert_eq!(*n, 2.0),
            _ => panic!("Expected number 2"),
        }
        
        match &postfix[1] {
            Token::Number(n) => assert_eq!(*n, 3.0),
            _ => panic!("Expected number 3"),
        }
        
        match &postfix[2] {
            Token::Number(n) => assert_eq!(*n, 4.0),
            _ => panic!("Expected number 4"),
        }
        
        match &postfix[3] {
            Token::Op(Operator::Multiply) => (),
            _ => panic!("Expected multiply operator"),
        }
        
        match &postfix[4] {
            Token::Op(Operator::Add) => (),
            _ => panic!("Expected add operator"),
        }
    }

    #[test]
    fn test_evaluate_function() {
        let postfix = vec![
            Token::Number(2.0),
            Token::Number(3.0),
            Token::Number(4.0),
            Token::Op(Operator::Multiply),
            Token::Op(Operator::Add),
        ];
        
        let result = Calculator::evaluate(postfix).unwrap();
        assert_eq!(result, 14.0); 
    }
}

fn main() {
    let expression = "2*2 + 48/4";
    println!("Calculating: {}", expression);
    
    match Calculator::calculate(expression) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {:?}", e),
    }
    
    let test_expressions = vec![
        "3 + 4 * 2",
        "(3 + 4) * 2", 
        "10 / 2 - 3",
        "2.5 * 4 + 1.5",
        "((2 + 3) * 4) / 5",
    ];
    
    for expr in test_expressions {
        match Calculator::calculate(expr) {
            Ok(result) => println!("{} = {}", expr, result),
            Err(e) => println!("{} -> Error: {:?}", expr, e),
        }
    }
}
