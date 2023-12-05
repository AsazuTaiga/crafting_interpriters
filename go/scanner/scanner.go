package scanner

import (
	"github.com/AsazuTaiga/crafting_interpriters/go/logger"
	"github.com/AsazuTaiga/crafting_interpriters/go/token"
)

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
		default:
			logger.ErrorReport(s.line, "Unexpected character.");
			break;
	}
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