use std::fmt::Display;
use std::str::Chars;
use std::iter::Peekable;

#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub enum TokenType {
    // Single char
    TokenLeftParen,
    TokenRightParen,
    TokenLeftBrace,
    TokenRightBrace,
    TokenComma,
    TokenDot,
    TokenMinus,
    TokenPlus,
    TokenSemiColon,
    TokenSlash,
    TokenStar,

    // Keywords
    TokenPrint,
    TokenReturn,
    TokenIf,
    TokenElse,
    TokenTrue,
    TokenFalse,
    TokenAnd,
    TokenOr,
    TokenPipe,
    TokenFunc,
    TokenLet,

    // Literals
    TokenInteger,
    TokenFloat,
    TokenString,
    TokenIdent,

    // Comp
    TokenEqual, // =
    TokenEqEq, // ==
    TokenNotEq, // !=
    TokenNot, // !
    TokenLess, // <
    TokenLessEq, // <=
    TokenGt, // >
    TokenGtEq, // >=

    // misc
    TokenComment,
    TokenError,
    TokenLambda,
    TokenInfix,
}


impl TokenType {
    pub fn is_single(&self)->bool {
        true
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub use TokenType::*;

// start:0, curr:1
// prt
    // start:0, 

#[derive(Debug,Clone, Copy, PartialEq, Eq)]
pub struct Token<'src> {
    pub token_type:TokenType,
    pub content:&'src str,
    pub line:usize
}

impl<'src> Display for Token<'src> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}('{}')", self.token_type, self.content)
    }
}

impl<'src>  Token<'src> {
    pub fn err(line:usize)->Token<'src> {
        Token {
            token_type:TokenError,
            content:"",
            line
        }
    }

    pub fn is_err(&self)->bool {
        match self.token_type {
            TokenError => true,
            _ => false
        }
    }

    pub fn debug_print(&self)->String {
        format!("{}:line {}", self.to_string(), self.line)
    }
}

// store lookahead of one char i.e the Option<char> after peek

#[derive(Debug)]
pub struct LookaheadChars<'src> {
    chars:Peekable<Chars<'src>>,
    peek:Option<char> // current peek (chars always points one step ahead of peek)
}

impl<'src> LookaheadChars<'src> {
    pub fn new<'source>(source:&'source str)->LookaheadChars<'source> {
        let mut chars=source.chars().peekable();
        let peek=chars.next();

        LookaheadChars { chars, peek }
    }

    pub fn peek(&self)->Option<char> {
        self.peek
    }

    pub fn peek_next(&mut self)->Option<char> {
        self.chars.peek().map(|c| c.to_owned())
    }
}

impl<'src> Iterator for LookaheadChars<'src> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        let nxt=self.peek;
        self.peek=self.chars.next();
        nxt
    }
}

#[test]
fn test_lookahead() {
    let inp="23";
    let mut s=LookaheadChars::new(inp);
    assert_eq!(s.peek(), Some('2')); // 2
    assert_eq!(s.peek_next(), Some('3')); // 3
    s.next();
 
    assert_eq!(s.peek(), Some('3')); // 3
    assert_eq!(s.peek_next(), None); // None

    s.next();

    assert_eq!(s.peek(), None); // None
    assert_eq!(s.peek_next(), None); // None


    s.next();
    s.next();

    assert_eq!(s.peek(), None); // None
    assert_eq!(s.peek_next(), None); // None
}