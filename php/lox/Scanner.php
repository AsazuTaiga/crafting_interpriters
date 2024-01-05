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
        }
    }

    private function isAtEnd(): bool
    {
        return $this->current >= strlen($this->source);
    }

    private function advance(): string
    {
        return $this->source[$this->current++];
    }

    private function addToken(TokenType $type, object $literal = null): void
    {
        $text = substr($this->source, $this->start, $this->current);
        $this->tokens[] = new Token($type, $text, $literal, $this->line);
    }
}
