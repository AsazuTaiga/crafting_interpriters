

#[derive(Debug, Clone, PartialEq)]
enum TokenType {
    // Single-character tokens.
    LeftParen,RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // One or two character tokens.
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // Literals.
    Identifier, String, Number,

    // Keywords.
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    Eof,
}

/// Define Token struct with fields for type, lexeme, literal value, and line number
#[derive(Debug)]
struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Box<dyn std::fmt::Debug>>, // JavaではObject型
    line: usize,
}

impl Token {
    fn new(token_type: TokenType, lexeme: String, literal: Option<Box<dyn std::fmt::Debug>>, line: usize) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let literal = match &self.literal {
            Some(literal) => format!("{:?}", literal),
            None => String::from("None"),
        };
        write!(f, "{:?} {} {}", self.token_type, self.lexeme, literal)
    }
}