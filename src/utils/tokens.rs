
pub type Token = TokenKind;

#[derive(Debug,Clone,PartialEq,Copy)]
pub enum TokenKind {
    /** End of token stream */
    EOF,

    /** Operators type such as + or * */
    Operator{raw: char, kind: OperatorKind},

    /** The equal operator, reserved for asignation */
    Equal,

    /** Variables identifiers (start_offset,end_offset) */
    Identifier(usize,usize),

    /** Real number */
    Real(f64),

    /** Integer number */
    Integer(i64),

    /** ( delimiting the opening of a parenthesis group */
    OpenParenthesis,

    /** ) delimiting the end of a parenthesis group */
    CloseParenthesis,

    /** New line token, a natural separator between inputs */
    NewLine,

    /** Semicolon token */
    Semicolon,

    /** Unknown token in place of error (start_offset,end_offset,line,column)*/
    UnknownToken(usize,usize,usize,usize),

    /** inv keywored */
    Inv,

    /** afficher keyword */
    Afficher,

    /** aff_ral keyword */
    Aff_ral,
}

#[derive(Debug,Clone, Copy, PartialEq)]
pub enum OperatorKind {
    /** The multiplication Operator */
    Multiplier,

    /** The addition Operator */
    Adder,
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self,f: &mut std::fmt::Formatter) -> std::fmt::Result{
        match *self {
            TokenKind::Afficher => write!(f,"Afficher"),
            TokenKind::EOF => write!(f,"EOF"),
            TokenKind::CloseParenthesis => write!(f,"CloseParenthesis"),
            TokenKind::Equal => write!(f,"Equal"),
            TokenKind::Inv => write!(f,"Inv"),
            TokenKind::NewLine => write!(f,"NewLine"),
            TokenKind::OpenParenthesis => write!(f,"OpenParenthesis"),
            TokenKind::Semicolon => write!(f,"Semicolon"),
            TokenKind::Identifier(s, e) => write!(f,"Id({},{})",s,e),
            TokenKind::Integer(v) => write!(f,"Integer({})",v),
            TokenKind::Real(v) => write!(f,"Real({})",v),
            TokenKind::UnknownToken(s, e,..) => write!(f,"Unknown({},{})",s,e),
            TokenKind::Operator { raw, .. } => write!(f,"Operator({})",raw)
        }
    }
}