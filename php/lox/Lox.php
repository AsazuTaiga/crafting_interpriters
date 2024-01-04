<?php

declare(strict_types=1);

class Lox
{
    /**
     * @param string[] $args
     * @return void
     */
    public static function main(
        array $args
    ): void
    {
        if (count($args) > 1) {
            echo ('Usage: jlox [script]');
            exit(64);
        } else if (count($args) == 1) {
            self::runFile($args[0]);
        } else {
            self::runPrompt();
        }
    }

    private static function runFile(string $path): void
    {
        self::run(file_get_contents($path));
    }

    private static function runPrompt()
    {
        $input = STDIN;

        for (;;) {
            echo '> ';
            $line = fgets($input);
            if ($line === false) break;
            self::run($line);
        }
    }

    private static function run(string $source): void
    {
        $scanner = new Scanner($source);
        $tokens = $scanner->scanTokens();

        foreach ($tokens as $token) {
            echo $token;
        }
    }
}
