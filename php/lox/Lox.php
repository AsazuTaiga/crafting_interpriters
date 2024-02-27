<?php

declare(strict_types=1);

namespace Lox;

class Lox
{
    private static $hadError = false;

    /**
     * @param string[] $args
     * @return void
     */
    public static function main(
        array $args
    ): void
    {
        if (count($args) > 2) {
            echo ('Usage: jlox [script]');
            exit(64);
        } else if (count($args) == 2) {
            self::runFile($args[0]);
        } else {
            self::runPrompt();
        }
    }

    private static function runFile(string $path): void
    {
        self::run(file_get_contents($path));
        if (self::$hadError) exit(65);
    }

    private static function runPrompt()
    {
        $input = STDIN;

        for (;;) {
            echo '> ';
            $line = fgets($input);
            if ($line === false) break;
            self::run($line);
            self::$hadError = false;
        }
    }

    private static function run(string $source): void
    {
        $scanner = new Scanner($source);
        $tokens = $scanner->scanTokens();

        foreach ($tokens as $token) {
            echo $token . "\n";
        }
    }

    static function error(int $line, string $message): void
    {
        self::report($line, "", $message);
    }

    private static function report(
        int $line,
        string $where,
        string $message
    ): void
    {
        fputs(STDERR, "[line $line] Error$where: $message");
        self::$hadError = true;
    }
}
