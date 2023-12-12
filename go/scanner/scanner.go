package scanner

import (
	"strconv"

	"github.com/AsazuTaiga/crafting_interpriters/go/logger"
	"github.com/AsazuTaiga/crafting_interpriters/go/token"
)

var keywords = map[string]token.TokenType{
	"and":    token.AND,
	"class":  token.CLASS,
	"else":   token.ELSE,
	"false":  token.FALSE,
	"for":    token.FOR,
	"fun":    token.FUN,
	"if":     token.IF,
	"nil":    token.NIL,
	"or":     token.OR,
	"print":  token.PRINT,
	"return": token.RETURN,
	"super":  token.SUPER,
	"this":   token.THIS,
	"true":   token.TRUE,
	"var":    token.VAR,
	"while":  token.WHILE,
}

type Scanner struct {
	source  string
	tokens  []*token.Token
	start   int
	current int
	line    int
}

func NewScanner(source string) *Scanner {
	return &Scanner{
		source: source,
		tokens: make([]*token.Token, 0),
		start:  0,
		current: 0,
		line: 1,
	}
}

func (s *Scanner) ScanTokens(logger *logger.Logger) []*token.Token {
	for !s.isAtEnd() {
		s.start = s.current
		s.scanToken(logger)
	}

	s.tokens = append(s.tokens, token.NewToken(token.EOF, "", nil, s.line))
	return s.tokens
}

func (s *Scanner) scanToken(logger *logger.Logger) {
	c := s.advance();
	switch c {
		case '(':
				s.addToken(token.LEFT_PAREN);
				break;
		case ')':
			s.addToken(token.RIGHT_PAREN);
			break;
		case '{':
			s.addToken(token.LEFT_BRACE);
			break;
		case '}':
			s.addToken(token.RIGHT_BRACE);
			break;
		case ',':
			s.addToken(token.COMMA);
			break;
		case '.':
			s.addToken(token.DOT);
			break;
		case '-':
			s.addToken(token.MINUS);
			break;
		case '+':
			s.addToken(token.PLUS);
			break;
		case ';':
			s.addToken(token.SEMICOLON);
			break;
		case '*':
			s.addToken(token.STAR);
			break;
		case '!':
			if s.match('=') {
				s.addToken(token.BANG_EQUAL);
			} else {
				s.addToken(token.BANG);
			}
			break;
		case '=':
			if s.match('=') {
				s.addToken(token.EQUAL_EQUAL)
			} else {
				s.addToken(token.EQUAL)
			}
			break;
		case '<':
			if s.match('=') {
				s.addToken(token.LESS_EQUAL)
			} else {
				s.addToken(token.LESS)
			}
			break;
		case '>':
			if s.match('=') {
				s.addToken(token.GREATER_EQUAL)
			} else {
				s.addToken(token.GREATER)
			}
			break;
		case '/':
			if s.match('/') {
				for s.peek() != '\n' && !s.isAtEnd() {
					s.advance()
				}
			} else {
				s.addToken(token.SLASH)
			}
			break;
		case ' ':
		case '\r':
		case '\t':
			break;
		case '\n':
			s.line++
			break;
		case '"':
			s.string(logger)
			break;
		case 'o':
			if s.match('r') {
				s.addToken(token.OR)
			}
			break;
		default:
			if s.isDigit(c) {
				s.number()
			} else if s.isAlpha(c) {
				s.identifier()
			} else {
				logger.ErrorReport(s.line, "Unexpected character.");
			}
			break;
	}
}

func (s *Scanner) identifier() {
	for s.isAlphaNumeric(s.peek()) {
		s.advance()
	}

	text := s.source[s.start:s.current]
	if t, ok := keywords[text]; ok {
		s.addToken(t)
	} else {
		s.addToken(token.IDENTIFIER)
	}
}

func (s *Scanner) number() {
	for s.isDigit(s.peek()) {
		s.advance()
	}

	if  s.peek() == '.' && s.isDigit(s.peekNext()) {
		s.advance()

		for s.isDigit(s.peek()) {
			s.advance()
		}
	}

	val, err := strconv.ParseFloat(s.source[s.start:s.current], 64)
	if err != nil {
		panic(err) // NOTE: panicがいいのかはわかってない
	}
	s.addTokenWithLiteral(token.NUMBER, val)
}

func (s *Scanner) string(logger *logger.Logger) {
	for s.peek() != '"' && !s.isAtEnd() {
		if s.peek() == '\n' {
			s.line++
		}
		s.advance()
	}

	if s.isAtEnd() {
		logger.ErrorReport(s.line, "Unterminated string.")
		return
	}

	s.advance()

	value := s.source[s.start+1:s.current-1]
	s.addTokenWithLiteral(token.STRING, value)
}

func (s *Scanner) match(expected byte) bool {
	if s.isAtEnd() {
		return false
	}
	if s.source[s.current] != expected {
		return false
	}

	s.current++
	return true
}

func (s *Scanner) peek() byte {
	if s.isAtEnd() {
		return '\u0000'
	} else {
		return s.source[s.current]
	}
}

func (s *Scanner) peekNext() byte {
	if s.current + 1 >= len(s.source) {
		return '\u0000'
	} else {
		return s.source[s.current+1]
	}
}

func (s *Scanner) isAlpha(c byte) bool {
	return (c >= 'a' && c <= 'z') ||
		(c >= 'A' && c <= 'Z') ||
		c == '_'
}

func (s *Scanner) isAlphaNumeric(c byte) bool {
	return s.isAlpha(c) || s.isDigit(c)
}

func (s *Scanner) isDigit(c byte) bool {
	return c >= '0' && c <= '9'
}

func (s *Scanner) isAtEnd() bool {
	return s.current >= len(s.source)
}

func (s *Scanner) advance() byte {
	s.current++
	return s.source[s.current-1]
}

func (s *Scanner) addToken(t token.TokenType)   {
	s.addTokenWithLiteral(t, nil)
}

func (s *Scanner) addTokenWithLiteral(t token.TokenType, literal interface{}) {
	text := s.source[s.start:s.current]
	s.tokens = append(s.tokens, token.NewToken(t, text, literal, s.line))
}