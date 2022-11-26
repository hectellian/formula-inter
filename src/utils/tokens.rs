
pub type Token = TokenKind;

/** All toke as the last two arguments (line, col) of start of token */
#[derive(Debug,Clone,PartialEq,Copy)]
pub enum TokenKind {
    /** End of token stream */
    EOF(usize,usize),

    /** The multiplication Operator */
    Multiplier(usize,usize),

    /** The addition Operator */
    Adder(usize,usize),

    /** The equal operator, reserved for asignation */
    Equal(usize,usize),

    /** Variables identifiers (start_offset,end_offset) */
    Identifier(usize,usize,usize,usize),

    /** Real number */
    Real(f64,usize,usize),

    /** Integer number */
    Integer(i64,usize,usize),

    /** ( delimiting the opening of a parenthesis group */
    OpenParenthesis(usize,usize),

    /** ) delimiting the end of a parenthesis group */
    CloseParenthesis(usize,usize),

    /** Semicolon token */
    Semicolon(usize,usize),

    /** Unknown token in place of error (start_offset,end_offset,line,column)*/
    UnknownToken(usize,usize,usize,usize),

    /** inv keywored */
    Inv(usize,usize),

    /** afficher keyword */
    Afficher(usize,usize),

    /** aff_ral keyword */
    AffRal(usize,usize),
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self,f: &mut std::fmt::Formatter) -> std::fmt::Result{
        match *self {
            TokenKind::Afficher(..) => write!(f,"Afficher"),
            TokenKind::EOF(..) => write!(f,"EOF"),
            TokenKind::AffRal(..) => write!(f,"Aff_ral"),
            TokenKind::CloseParenthesis(..) => write!(f,"CloseParenthesis"),
            TokenKind::Equal(..) => write!(f,"Equal"),
            TokenKind::Inv(..) => write!(f,"Inv"),
            TokenKind::OpenParenthesis(..) => write!(f,"OpenParenthesis"),
            TokenKind::Semicolon(..) => write!(f,"Semicolon"),
            TokenKind::Identifier(s, e,..) => write!(f,"Id({},{})",s,e),
            TokenKind::Integer(v,..) => write!(f,"Integer({})",v),
            TokenKind::Real(v,..) => write!(f,"Real({})",v),
            TokenKind::UnknownToken(s, e,..) => write!(f,"Unknown({},{})",s,e),
            TokenKind::Adder(..) => write!(f,"Adder"),
            TokenKind::Multiplier(..) => write!(f,"Multiplier")
        }
    }
}