use super::super::utils::evalution::evaluation;

#[test]
fn test_uninitialized_value(){
    let mut content = String::new();
    content.push_str("i=j+1;");
    content.push_str("afficher j;");

    assert!(matches!(evaluation(content),Err(..)));
}

#[test]
fn test_assignation() {
    let mut content = String::new();
    content.push_str("j=1+1;");
    content.push_str("j2=j+1;");
    content.push_str("j=j+j2;");

    assert!(matches!(evaluation(content),Ok(..)));
}

#[test]
fn tes_double_parenthesis(){
    let mut content = String::new();
    content.push_str("j=(1+2)*(2+1);");
    content.push_str("afficher j;");
    content.push_str("aff_ral;");

    assert!(matches!(evaluation(content),Ok(..)));
}