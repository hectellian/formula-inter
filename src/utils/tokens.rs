pub type Token = TokenKind;

#[derive(Debug,Clone,PartialEq,Copy)]
pub enum TokenKind {
    /** End of token stream */
    EOF,

    /** Operators type such as + or * */
    Operator{raw: char, kind: OperatorKind},

    /** The equal operator, reserved for asignation */
    Equal,

    /** Variables identifiers */
    Identifier,

    /** Real number */
    Real(f64),

    /** Integer number */
    Integer(u64),

    /** ( delimiting the opening of a parenthesis group */
    OpenParenthesis,

    /** ) delimiting the end of a parenthesis group */
    CloseParenthesis,

    /** New line token, a natural separator between inputs */
    NewLine,

    /** Semicolon token */
    Semicolon,

    /** Unknown token in place of error */
    UnknownToken,

    /** inv keywored */
    Inv,

    /** afficher keyword */
    Afficher,
}

#[derive(Debug,Clone, Copy, PartialEq)]
pub enum OperatorKind {
    /** The multiplication Operator */
    Multiplier,

    /** The addition Operator */
    Adder,
}