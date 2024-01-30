<?php

declare(strict_types=1);

namespace Lox;

class Token
{

    public function __construct(
        private TokenType $type,
        private string $lexeme,
        private $literal,
        private int $line
    ){}

    public function __toString(): string
    {
        $type = $this->type->name;
        return "$type $this->lexeme $this->literal";
    }
}