package dev.asazutaiga.lox;

public enum TokenType {
  // １文字トークン
  LEFT_PAREN, // (
  RIGHT_PAREN, // )
  LEFT_BRACE, // {
  RIGHT_BRACE, // }
  COMMA, // ,
  DOT, // .
  MINUS, // -
  PLUS, // +
  SEMICOLON, // ;
  SLASH, // /
  STAR, // *

  // 1 or 2 文字トークン
  BANG, // !
  BANG_EQUAL, // !=
  EQUAL, // =
  EQUAL_EQUAL, // ==
  GREATER, // >
  GREATER_EQUAL, // >=
  LESS, // <
  LESS_EQUAL, // <=

  // リテラル
  IDENTIFIER, // 識別子
  STRING, // 文字列
  NUBMER, // 数値

  // 予約後
  AND, // and
  CLASS, // class
  ELSE, // else
  FALSE, // false
  FUN, // fun
  FOR, // for
  IF, // if
  NIL, // nil
  OR, // or
  PRINT, // print
  RETURN, // return
  SUPER, // super
  THIS, // this
  TRUE, // true
  VAR, // var
  WHILE, // while,

  EOF // EOF
}
