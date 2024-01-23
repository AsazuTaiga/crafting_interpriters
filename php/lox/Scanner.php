<?php

declare(strict_types=1);

class Scanner
{
    /** @var $tokens Token[] */
    private array $tokens = [];
    private int $start = 0;
    private int $current = 0;
    private int $line = 1;
    public function __construct(
        private string $source,
    ){}

    /**
     * @return Token[]
     */
    public function scanTokens(): array {
        while (!$this->isAtEnd()) {
            // We are at the beginning of the next lexeme.
            $this->start = $this->current;
            $this->scanToken();
        }

        $this->tokens[] = new Token(TokenType::EOF, "", null, $this->line);
        return $this->tokens;
    }

    private function scanToken(): void
    {
        $c = $this->advance();
        switch ($c) {
            case '(': $this->addToken(TokenType::LEFT_PAREN);
                break;
            case ')': $this->addToken(TokenType::RIGHT_PAREN);
                break;
            case '{': $this->addToken(TokenType::LEFT_BRACE);
                break;
            case '}': $this->addToken(TokenType::RIGHT_BRACE);
                break;
            case ',': $this->addToken(TokenType::COMMA);
                break;
            case '.': $this->addToken(TokenType::DOT);
                break;
            case '-': $this->addToken(TokenType::MINUS);
                break;
            case '+': $this->addToken(TokenType::PLUS);
                break;
            case ';': $this->addToken(TokenType::SEMICOLON);
                break;
            case '*': $this->addToken(TokenType::STAR);
                break;
            case '!':
                $this->addToken(
                    $this->match('=')
                        ? TokenType::BANG_EQUAL
                        : TokenType::BANG
                );
                break;
            case '=':
                $this->addToken(
                    $this->match('=')
                        ? TokenType::EQUAL_EQUAL
                        : TokenType::EQUAL
                );
                break;
            case '<':
                $this->addToken(
                    $this->match('=')
                        ? TokenType::LESS_EQUAL
                        : TokenType::LESS
                );
                break;
            case '>':
                $this->addToken(
                    $this->match('=')
                        ? TokenType::GREATER_EQUAL
                        : TokenType::GREATER
                );
                break;
            case '/':
                if ($this->match('/')) {
                    // A comment goes until the end of the line.
                    while ($this->peek() != '\n' && !$this->isAtEnd()) $this->advance();
                } else {
                    $this->addToken(TokenType::SLASH);
                }
                break;

            case ' ':
            case '\r':
            case '\t':
                // Ignore whitespace.
                break;

            case '\n':
                $this->line++;
                break;

            case '"': $this->string(); break;
            default:
                if ($this->isDigit($c)) {
                    $this->number();
                } else {
                    Lox::error($this->line, 'Unexpected character.');
                }
                break;
        }
    }

    private function number(): void
    {
        while ($this->isDigit($this->peek())) $this->advance();

        // Look for a fractional part.
        if ($this->peek() == '.' && $this->isDigit($this->peekNext())) {
            // Consume the "."
            $this->advance();

            while ($this->isDigit($this->peek())) $this->advance();
        }

        $this->addToken(
            TokenType::NUMBER,
            floatval(substr($this->source, $this->start, $this->current))
        );
    }

    private function string(): void
    {
        while ($this->peek() != '"' && !$this->isAtEnd()) {
            if ($this->peek() == '\n') $this->line++;
            $this->advance();
        }

        if ($this->isAtEnd()) {
            Lox::error($this->line, "Unterminated string.");
            return;
        }

        // The closing ".
        $this->advance();

        // Trim the surrounding quotes.
        $value = substr($this->source, $this->start + 1, $this->current - 1);
        $this->addToken(TokenType::STRING, $value);
    }

    private function match(string $expected): bool
    {
        if ($this->isAtEnd()) return false;
        if ($this->source[$this->current] !== $expected) return false;

        $this->current++;
        return true;
    }

    private function peek(): string
    {
        if ($this->isAtEnd()) return '\0';
        return $this->source[$this->current];
    }

    private function peekNext(): string
    {
        if ($this->current + 1 >= $this->source) return '\0';
        return $this->source[$this->current + 1];
    }

    private function isDigit(string $c): bool
    {
        return $c >= '0' && $c <= '9';
    }

    private function isAtEnd(): bool
    {
        return $this->current >= strlen($this->source);
    }

    private function advance(): string
    {
        return $this->source[$this->current++];
    }

    private function addToken(TokenType $type, $literal = null): void
    {
        $text = substr($this->source, $this->start, $this->current);
        $this->tokens[] = new Token($type, $text, $literal, $this->line);
    }
}
