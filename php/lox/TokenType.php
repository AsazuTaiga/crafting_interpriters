<?php

declare(strict_types=1);

namespace Lox;

enum TokenType: string
{
    // Single-character tokens.
    case LEFT_PAREN = 'LEFT_PAREN';
    case RIGHT_PAREN = 'RIGHT_PAREN';
    case LEFT_BRACE = 'LEFT_BRACE';
    case RIGHT_BRACE = 'RIGHT_BRACE';
    case COMMA = 'COMMA';
    case DOT = 'DOT';
    case MINUS = 'MINUS';
    case PLUS = 'PLUS';
    case SEMICOLON = 'SEMICOLON';
    case SLASH = 'SLASH';
    case STAR = 'STAR';

    // One or two character tokens.
    case BANG = 'BANG';
    case BANG_EQUAL = 'BANG_EQUAL';
    case EQUAL = 'EQUAL';
    case EQUAL_EQUAL = 'EQUAL_EQUAL';
    case GREATER = 'GREATER';
    case GREATER_EQUAL = 'GREATER_EQUAL';
    case LESS = 'LESS';
    case LESS_EQUAL = 'LESS_EQUAL';

    // Literals.
    case IDENTIFIER = 'IDENTIFIER';
    case STRING = 'STRING';
    case NUMBER = 'NUMBER';
    case Keywords = 'Keywords';
    case AND = 'AND';
    // case CLASS = 'CLASS'; it's a reserved word in PHP
    case CLAZZ = 'CLAZZ';
    case ELSE = 'ELSE';
    case FALSE = 'FALSE';
    case FUN = 'FUN';
    case FOR = 'FOR';
    case IF = 'IF';
    case NIL = 'NIL';
    case OR = 'OR';
    case PRINT = 'PRINT';
    case RETURN = 'RETURN';
    case SUPER = 'SUPER';
    case THIS = 'THIS';
    case TRUE = 'TRUE';
    case VAR = 'VAR';
    case WHILE = 'WHILE';
    case EOF = 'EOF';
}
