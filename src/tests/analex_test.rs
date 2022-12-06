use super::super::*;

#[test]
fn test_tokens() {
    let mut content = String::new();
    content.push_str("a = 1;\n");
    content.push_str("b = 2;\n");
    content.push_str("c = 3;\n");
    content.push_str("d = 4;\n");

    assert!(matches!(lexical_analysis(content),Ok(true)));
}

#[test]
fn test_tokens_with_spaces() {
    let mut content = String::new();
    content.push_str("a = 1; ");
    content.push_str("b = 2; ");
    content.push_str("c = 3; ");
    content.push_str("d = 4; ");

    assert!(matches!(lexical_analysis(content),Ok(true)));
}

#[test]
fn test_tokens_with_tabs() {
    let mut content = String::new();
    content.push_str("a = 1;    ");
    content.push_str("b = 2;    ");
    content.push_str("c = 3;    ");
    content.push_str("d = 4;    ");

    assert!(matches!(lexical_analysis(content),Ok(true)));
}

#[test]
fn test_tokens_with_newlines() {
    let mut content = String::new();
    content.push_str("a = 1;\n");
    content.push_str("b = 2;\n");
    content.push_str("c = 3;\n");
    content.push_str("d = 4;\n");

    assert!(matches!(lexical_analysis(content),Ok(true)));
}

#[test]
fn test_unkown_tokens() {
    let mut tokens = String::new();
    tokens.push_str("/\n");
    tokens.push_str("^");
    tokens.push_str("!");
    tokens.push_str("?");

    assert!(matches!(lexical_analysis(tokens),Err(..)));
}
