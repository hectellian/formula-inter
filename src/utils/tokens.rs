
pub type Token = TokenKind;

/** All toke as the last two arguments (line, col) of start of token */
#[derive(Debug,Clone,PartialEq,Copy)]
pub enum TokenKind {
    /** End of token stream */
    EOF(Option<(usize,usize)>),

    /** The multiplication Operator */
    Multiplier(Option<(usize,usize)>),

    /** The addition Operator */
    Adder(Option<(usize,usize)>),

    /** The equal operator, reserved for asignation */
    Equal(Option<(usize,usize)>),

    /** Variables identifiers (start_offset,end_offset) */
    Identifier(usize,usize,Option<(usize,usize)>),

    /** Real number */
    Real(f64,Option<(usize,usize)>),

    /** Integer number */
    Integer(i64,Option<(usize,usize)>),

    /** ( delimiting the opening of a parenthesis group */
    OpenParenthesis(Option<(usize,usize)>),

    /** ) delimiting the end of a parenthesis group */
    CloseParenthesis(Option<(usize,usize)>),

    /** { delimiting the opening of a curly group */
    OpenCurly(Option<(usize,usize)>),

    /** loop tokne initialising a loop */
    Loop(Option<(usize,usize)>),

    /** } delimiting the end of a curly group */
    CloseCurly(Option<(usize,usize)>),

    /** Semicolon token */
    Semicolon(Option<(usize,usize)>),

    /** Unknown token in place of error (start_offset,end_offset,line,column)*/
    UnknownToken(usize,usize,Option<(usize,usize)>),

    /** inv keywored */
    Inv(Option<(usize,usize)>),

    /** afficher keyword */
    Afficher(Option<(usize,usize)>),

    /** aff_ral keyword */
    AffRal(Option<(usize,usize)>),

    /** racine keyword */
    Sqrt(Option<(usize,usize)>)
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self,f: &mut std::fmt::Formatter) -> std::fmt::Result{
        match *self {
            TokenKind::Afficher(..) => write!(f,"Afficher"),
            TokenKind::EOF(..) => write!(f,"EOF"),
            TokenKind::AffRal(..) => write!(f,"Aff_ral"),
            TokenKind::CloseParenthesis(..) => write!(f,"CloseParenthesis"),
            TokenKind::CloseCurly(..) => write!(f,"CloseCurly"),
            TokenKind::Equal(..) => write!(f,"Equal"),
            TokenKind::Inv(..) => write!(f,"Inv"),
            TokenKind::Sqrt(..) => write!(f,"Sqrt"),
            TokenKind::OpenParenthesis(..) => write!(f,"OpenParenthesis"),
            TokenKind::OpenCurly(..) => write!(f,"OpenCurly"),
            TokenKind::Semicolon(..) => write!(f,"Semicolon"),
            TokenKind::Identifier(s, e,..) => write!(f,"Id({},{})",s,e),
            TokenKind::Integer(v,..) => write!(f,"Integer({})",v),
            TokenKind::Real(v,..) => write!(f,"Real({})",v),
            TokenKind::UnknownToken(s, e,..) => write!(f,"Unknown({},{})",s,e),
            TokenKind::Adder(..) => write!(f,"Adder"),
            TokenKind::Multiplier(..) => write!(f,"Multiplier"),
            TokenKind::Loop(..) => write!(f,"Loop")
        }
    }
}

impl Token {
    pub fn pos(&self) -> Option<(usize,usize)>{
        match *self {
            TokenKind::Afficher(p) => return p,
            TokenKind::EOF(p) => return p,
            TokenKind::AffRal(p) => return p,
            TokenKind::CloseParenthesis(p) => return p,
            TokenKind::CloseCurly(p) => return p,
            TokenKind::Equal(p) => return p,
            TokenKind::Inv(p) => return p,
            TokenKind::Sqrt(p) => return p,
            TokenKind::OpenParenthesis(p) => return p,
            TokenKind::OpenCurly(p) => return p,
            TokenKind::Semicolon(p) => return p,
            TokenKind::Identifier(_s, _e,p) => return p,
            TokenKind::Integer(_v,p) => return p,
            TokenKind::Real(_v,p) => return p,
            TokenKind::UnknownToken(_s, _e,p) => return p,
            TokenKind::Adder(p) => return p,
            TokenKind::Multiplier(p) => return p,
            TokenKind::Loop(p) => return p,
        }
    }
}