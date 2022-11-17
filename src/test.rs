use super::*;

#[test]
fn int_numerical_values() {
    let expression = String::from("2 * (4^2 + 42/3)");
    let l = Lexer::from(expression);
    let t = l.tokenize().unwrap();
    let p = postfixe(t);
    let e = eval(p);
    match e {
        EvalResult::Integer(s) => assert_eq!(s, 2 * (i32::pow(4, 2) + 42/3)),
        _ => assert!(false)
    }
}

#[test]
fn negative_numbers(){
    let expression = String::from("-2 * (4^2 + 42/3)");
    let l = Lexer::from(expression);
    let t = l.tokenize().unwrap();
    let p = postfixe(t);
    let e = eval(p);
    match e {
        EvalResult::Integer(s) => assert_eq!(s, -2 * (i32::pow(4, 2) + 42/3)),
        _ => assert!(false)
    }
}

#[test]
fn float_numbers(){
    let expression = String::from("256.6 + 95.3");
    let l = Lexer::from(expression);
    let t = l.tokenize().unwrap();
    let p = postfixe(t);
    let e = eval(p);
    match e {
        EvalResult::Float(s) => assert_eq!(s, 256.6 + 95.3),
        _ => assert!(false)
    }
}

#[test]
fn empty_expression() {
    let expression = String::from(" ");
    let l = Lexer::from(expression);
    let t = l.tokenize().ok().unwrap();
    let p = postfixe(t);
    let e = eval(p);
    match e {
        EvalResult::Error(s) => assert_eq!(s, EvalError::InvalidExpression),
        _ => assert!(false)
    }
}

#[test]
fn non_numerical_value() {
    let expression = String::from("a + 6");
    let l = Lexer::from(expression);
    let t = l.tokenize().ok().unwrap();
    let p = postfixe(t);
    let e = eval(p);
    match e {
        EvalResult::Error(err) => assert_eq!(err, EvalError::NonNumericalValue),
        _ => assert!(false)
    }
}