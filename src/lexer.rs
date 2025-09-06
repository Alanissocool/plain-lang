use logos::Logos;

#[derive(Logos, Debug, Clone, PartialEq, Eq, Hash)]
#[logos(skip r"[ \t\n\f]+")] // Ignore whitespace
pub enum Token {
    #[regex("set")]
    Set,
    #[regex("add")]
    Add,
    #[regex("subtract")]
    Sub,
    #[regex("multiply")]
    Multiply,
    #[regex("show")]
    Show,
    #[regex("print")]
    Print,
    #[regex("if")]
    If,
    #[regex("then")]
    Then,
    #[regex("is")]
    Is,
    #[regex("greater")]
    Greater,
    #[regex("less")]
    Less,
    #[regex("equal")]
    Equal,
    #[regex("to")]
    To,
    #[regex("by")]
    By,
    #[regex("on")]
    On,
    #[regex("screen")]
    Screen,
    #[regex("from")]
    From,
    #[regex("than")]
    Than,
    #[regex("the")]
    The,
    #[regex("count")]
    Count,
    #[regex("display")]
    Display,
    #[regex("result")]
    Result,
    #[regex("and")]
    And,
    #[regex("when")]
    When,
    #[regex("you")]
    You,
    #[regex("are")]
    Are,
    #[regex("done")]
    Done,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Ident(String),

    #[regex(r"\d+", |lex| lex.slice().to_string())]
    Int(String),

    #[regex(r#""[^"]*""#, |lex| lex.slice().to_string())]
    Str(String),

    #[token(".")]
    Period,
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut lexer = Token::lexer(input);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next() {
        if let Ok(tok) = token {
            tokens.push(tok);
        } else {
            // Skip errors for MVP
        }
    }
    tokens
}
