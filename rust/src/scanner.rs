use crate::token::{Token, TokenType};
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

    pub fn scan_tokens(&mut self) {
        // TODO
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
