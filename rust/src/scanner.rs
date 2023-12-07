use crate::token::{Token, TokenType};
use crate::Lox;
use std::collections::HashMap;

/**
ライフタイムパラメー<'a> の役割:
Scanner構造体のsourceフィールドのライフタイムを指定している。
Scanner構造体のライフタイムが終わるまで、sourceフィールドのライフタイムも終わらないことを示している。
- 参考: <https://doc.rust-jp.rs/book-ja/ch10-03-lifetime-syntax.html>
 */
pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    // strとStringの違い
    // - strは文字列スライス。文字列リテラルはstr型。
    // - Stringはヒープ上に確保された文字列。String::from("hello") で作成できる。
    // - 参考: <https://doc.rust-jp.rs/book-ja/ch08-02-strings.html>
    pub fn new(source: &'a String) -> Self {
        Scanner {
            source: source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self, lox: &Lox) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token(lox);
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "".to_string(), None, self.line));
        return &self.tokens;
    }

    /// トークンをスキャンして
    pub fn scan_token(&mut self, lox: &Lox) {
        let c = self.advanced().unwrap();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let token_type = if self.match_char('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token_type);
            }
            '=' => {
                let token_type = if self.match_char('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };

                self.add_token(token_type);
            }
            '<' => {
                let token_type = if self.match_char('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token_type);
            }
            '>' => {
                let token_type = if self.match_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token_type)
            }
            '/' => {
                if self.match_char('/') {
                    // コメントは行末まで読み飛ばす。
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advanced();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' | '\r' | '\t' => {
                // 空白文字は無視する。
            }
            '\n' => {
                self.line += 1;
            }
            '"' => {
                self.string(lox).unwrap();
            }
            _ => {
                if self.is_digit(c) {
                    self.number()
                } else if self.is_alpha(c) {
                    self.identifier()
                } else {
                    Lox::error(lox, self.line, "Unexpected character.");
                }
            }
        }
    }

    /// ソースコードの現在の位置の文字を返して、現在の位置を一つ進める。
    fn advanced(&mut self) -> Option<char> {
        if self.is_at_end() {
            None
        } else {
            let c = self.source.chars().nth(self.current);
            self.current += 1;
            return c;
        }
    }

    /// `addToken(TokenType type)` in Java version
    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, None);
    }

    /// `addToken(TokenType type, Object literal)` in Java version
    /// トークンの種類とそのトークンの文字列を受け取り、トークンを生成してtokensフィールドに追加する。
    fn add_token_literal(
        &mut self,
        token_type: TokenType,
        literal: Option<Box<dyn std::fmt::Debug>>,
    ) {
        // to_string()しないとthe size for values of type `str` cannot be known at compilation timeになったからto_string()した
        let text = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(token_type, text, literal, self.line))
    }

    /// ソースコードの最後まで読み込んだかどうかを返す。
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn is_digit(&self, c: char) -> bool {
        return c >= '0' && c <= '9';
    }

    fn is_alpha(&self, c: char) -> bool {
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
    }

    fn is_alphanumeric(&self, c: char) -> bool {
        return self.is_alpha(c) || self.is_digit(c);
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;
        return true;
    }

    // lookahead 先読みするが、現在の位置は進めない。
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0'; // 文字列の終端を表す。
        }
        return self.source.chars().nth(self.current).unwrap();
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        return self.source.chars().nth(self.current + 1).unwrap();
    }

    /// 文字列リテラルをスキャンする。
    fn string(&mut self, lox: &Lox) -> Result<(), String> {
        while self.peek() != '\"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advanced();
        }

        if self.is_at_end() {
            Lox::error(lox, self.line, "Unterminated string.");
        }
        self.advanced();

        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token_literal(TokenType::String, Some(Box::new(value)));

        Ok(())
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advanced();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advanced();
            while self.is_digit(self.peek()) {
                self.advanced();
            }
        }

        let value = self.source[self.start..self.current].to_string();
        self.add_token_literal(TokenType::Number, Some(Box::new(value)));
    }

    fn identifier(&mut self) {
        while self.is_alphanumeric(self.peek()) {
            self.advanced();
        }

        let text = self.source[self.start..self.current].to_string();
        let keyword = self.keywords();
        let token_type: Option<&TokenType> = keyword.get(&text);
        let token_type = match token_type {
            Some(token_type) => token_type,
            None => &TokenType::Identifier,
        };
        self.add_token_literal(token_type.clone(), Some(Box::new(text)));
    }

    fn keywords(&self) -> HashMap<String, TokenType> {
        let mut keywords: HashMap<String, TokenType> = HashMap::new();
        keywords.insert("and".to_string(), TokenType::And);
        keywords.insert("class".to_string(), TokenType::Class);
        keywords.insert("else".to_string(), TokenType::Else);
        keywords.insert("false".to_string(), TokenType::False);
        keywords.insert("for".to_string(), TokenType::For);
        keywords.insert("fun".to_string(), TokenType::Fun);
        keywords.insert("if".to_string(), TokenType::If);
        keywords.insert("nil".to_string(), TokenType::Nil);
        keywords.insert("or".to_string(), TokenType::Or);
        keywords.insert("print".to_string(), TokenType::Print);
        keywords.insert("return".to_string(), TokenType::Return);
        keywords.insert("super".to_string(), TokenType::Super);
        keywords.insert("this".to_string(), TokenType::This);
        keywords.insert("true".to_string(), TokenType::True);
        keywords.insert("var".to_string(), TokenType::Var);
        keywords.insert("while".to_string(), TokenType::While);

        return keywords;
    }
}
