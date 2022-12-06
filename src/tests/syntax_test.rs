use super::super::*;


#[test]
fn test_simple_variable_syntax() {
    let mut content = String::new();
    content.push_str("a = 1;");
    content.push_str("b = 2;");
    content.push_str("c = 3;");
    content.push_str("d = 4;");

    assert_eq!(syntaxical_analysis(content), true);
}

#[test]
fn test_simple_variable_syntax_with_spaces() {
    let mut content = String::new();
    content.push_str("a = 1; ");
    content.push_str("b = 2; ");
    content.push_str("c = 3; ");
    content.push_str("d = 4; ");

    assert_eq!(syntaxical_analysis(content), true);
}

#[test]
fn test_simple_variable_syntax_with_tabs() {
    let mut content = String::new();
    content.push_str("a = 1;    ");
    content.push_str("b = 2;    ");
    content.push_str("c = 3;    ");
    content.push_str("d = 4;    ");

    assert_eq!(syntaxical_analysis(content), true);
}

#[test]
fn test_simple_variable_syntax_with_newlines() {
    let mut content = String::new();
    content.push_str("a = 1;\n");
    content.push_str("b = 2;\n");
    content.push_str("c = 3;\n");
    content.push_str("d = 4;\n");

    assert_eq!(syntaxical_analysis(content), true);
}

#[test]
fn test_complex_structure() {
    let content: String = String::from("a=2;id=(a+3)*2;re=id+-20;aff_ral;afficher re;");
    assert_eq!(syntaxical_analysis(content), true);
}

#[test]
fn test_syntaxical_error() {
    let mut content = String::new();
    content.push_str("a = 1;\n");
    content.push_str("b = 2;\n");
    content.push_str("c = 3;\n");
    content.push_str("d = 4;\n");
    content.push_str("e = ;\n");

    assert_eq!(syntaxical_analysis(content), false);
}

#[test]
fn test_correct_syntax_wrong_semantics() {
    let mut content = String::new();
    content.push_str("i=12+3*(1+j);\n");
    content.push_str("h=j+1;\n");
    content.push_str("aff_ral;\n");
    content.push_str("afficher z;\n");

    assert_eq!(syntaxical_analysis(content),true);
}