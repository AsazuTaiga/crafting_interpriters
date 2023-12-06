use crate::token::{Token, TokenType};
use crate::Lox;
use std::default;
use std::iter::Peekable;
use std::str::Chars;

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
            _ => {
                // Lox::error(Lox, self.line, "Unexpected character.");
            }
        }
    }

    /// ソースコードの次の文字をConsumeして、現在の位置を進める。
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

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
