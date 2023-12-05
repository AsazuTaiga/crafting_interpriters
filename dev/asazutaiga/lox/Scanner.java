package dev.asazutaiga.lox;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

import static dev.asazutaiga.lox.TokenType.*;

class Scanner {
  private static final Map<String, TokenType> keywords;
  static {
    keywords = new HashMap<>();
    keywords.put("and", AND);
    keywords.put("class", CLASS);
    keywords.put("else", ELSE);
    keywords.put("false", FALSE);
    keywords.put("for", FOR);
    keywords.put("fun", FUN);
    keywords.put("if", IF);
    keywords.put("nil", NIL);
    keywords.put("or", OR);
    keywords.put("print", PRINT);
    keywords.put("return", RETURN);
    keywords.put("super", SUPER);
    keywords.put("this", THIS);
    keywords.put("true", TRUE);
    keywords.put("var", VAR);
    keywords.put("while", WHILE);
  }

  public final String source;
  public final List<Token> tokens = new ArrayList<>();

  private int start = 0;
  private int current = 0;
  private int line = 1;

  Scanner(String source) {
    this.source = source;
  }

  List<Token> scanTokens() {
    while (!isAtEnd()) {
      // 次の語彙素から始める
      start = current;
      scanToken();
    }

    tokens.add(new Token(EOF, "", null, line));
    return tokens;
  }

  private void scanToken() {
    // とりあえず全てのトークンが１文字であると想定して書いた
    char c = advance();
    switch (c) {
      case '(':
        addToken(LEFT_PAREN);
        break;
      case ')':
        addToken(RIGHT_PAREN);
        break;
      case '{':
        addToken(LEFT_BRACE);
        break;
      case '}':
        addToken(RIGHT_BRACE);
        break;
      case ',':
        addToken(COMMA);
        break;
      case '.':
        addToken(DOT);
        break;
      case '-':
        addToken(MINUS);
        break;
      case '+':
        addToken(PLUS);
        break;
      case ';':
        addToken(SEMICOLON);
        break;
      case '*':
        addToken(STAR);
        break;
      case '!':
        addToken(match('=') ? BANG_EQUAL : BANG);
        break;
      case '=':
        addToken(match('=') ? EQUAL_EQUAL : EQUAL);
        break;
      case '<':
        addToken(match('=') ? LESS_EQUAL : LESS);
        break;
      case '>':
        addToken(match('=') ? GREATER_EQUAL : GREATER);
        break;
      case '/':
        if (match('/')) {
          // コメントが行末まで続くのそこまで進める
          while (peek() != '\n' && !isAtEnd())
            advance();
        } else {
          addToken(SLASH);
        }
        break;

      // ホワイトスペース
      case ' ':
      case '\r':
      case '\t':
        break;

      // 改行
      case '\n':
        line++;
        break;

      // 文字列リテラル
      case '"':
        string();
        break;

      default:
        if (isDigit(c)) {
          // 数値リテラル
          number();
        } else if (isAlpha(c)) {
          // 識別子（または予約語）
          identifier();
        } else {
          Lox.error(line, "Unexpected character.");
        }
        break;
    }
  }

  private void identifier() {
    while (isAlphaNumeric(peek()))
      advance();

    String text = source.substring(start, current);
    TokenType type = keywords.get(text);
    if (type == null)
      type = IDENTIFIER;

    addToken(type);
  }

  private void number() {
    while (isDigit(peek()))
      advance();

    // 小数部分を探す
    if (peek() == '.' && isDigit(peekNext())) {
      // "."を消費
      advance();

      while (isDigit(peek()))
        advance();
    }

    addToken(NUBMER,
        Double.parseDouble(source.substring(start, current)));
  }

  private void string() {
    while (peek() != '"' && !isAtEnd()) {
      if (peek() == '\n')
        line++; // NOTE: Loxは複数行の文字列をサポートします
      advance();
    }

    if (isAtEnd()) {
      Lox.error(line, "Unterminated string.");
      return;
    }

    // 閉じるほうの "
    advance();

    // クォーテーションを外してトークンとして追加
    String value = source.substring(start + 1, current - 1);
    addToken(STRING, value);
  }

  /**
   * 条件付きの advance() のようなもの。マッチした場合は次にカーソルを動かしつつ、マッチしたことを返却。
   * 
   * @param expected currentがそれであるかどうか確認したい文字
   * @return マッチしたかどうか
   */
  private boolean match(char expected) {
    if (isAtEnd())
      return false;
    if (source.charAt(current) != expected)
      return false;

    current++;
    return true;
  }

  private char peek() {
    if (isAtEnd())
      return '\0'; // 終端文字
    return source.charAt(current);
  }

  private char peekNext() {
    if (current + 1 >= source.length())
      return '\0'; // 終端文字
    return source.charAt(current + 1);
  }

  private boolean isAlpha(char c) {
    return (c >= 'a' && c <= 'z') ||
        (c >= 'A' && c <= 'Z') ||
        c == '_';
  }

  private boolean isAlphaNumeric(char c) {
    return isAlpha(c) || isDigit(c);
  }

  private boolean isDigit(char c) {
    return c >= '0' && c <= '9';
  }

  private boolean isAtEnd() {
    return current >= source.length();
  }

  private char advance() {
    return source.charAt(current++);
  }

  private void addToken(TokenType type) {
    addToken(type, null);
  }

  private void addToken(TokenType type, Object literal) {
    String text = source.substring(start, current);
    tokens.add(new Token(type, text, literal, line));
  }
}
