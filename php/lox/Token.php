<?php

declare(strict_types=1);

class Token
{

    public function __construct(
        private TokenType $type,
        private string $lexeme,
        private ?object $literal,
        private int $line
    ){}

    public function __toString(): string
    {
        return "$this->type $this->lexeme $this->literal";
    }
}