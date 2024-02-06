package lox

import (
	"bufio"
	"fmt"
	"os"

	"github.com/AsazuTaiga/crafting_interpriters/go/interpreter"
	"github.com/AsazuTaiga/crafting_interpriters/go/logger"
	"github.com/AsazuTaiga/crafting_interpriters/go/parser"
	"github.com/AsazuTaiga/crafting_interpriters/go/scanner"
)

type Lox struct {
	logger *logger.Logger
	interpreter *interpreter.Interpreter
}

func NewLox(
	logger *logger.Logger,
) *Lox {
	i := interpreter.NewInterpreter()
	return &Lox{
		logger: logger,
		interpreter: i,
	}
}

func (l *Lox) Run() {
	args := os.Args[2:]

	if len(args) > 1 {
		fmt.Println("Usage: golox [script]")
		os.Exit(64)
	} else if len(args) == 1 {
		l.runFile(args[0])
	} else {
		l.runPrompt()
	}
}

func (l *Lox) runFile(path string) error {
	bytes, err := os.ReadFile(path)
	if err != nil {
		return err
	}

	l.run(string(bytes))
	if(l.logger.HadError()) {
		os.Exit(65)
	}
	return nil
}

func (l *Lox) runPrompt() {
	reader := bufio.NewReader(os.Stdin)

	for {
		fmt.Print("> ")
		line, err := reader.ReadString('\n')
		if err != nil {
			break
		}

		l.run(line)
		l.logger.ResetError()
	}
}

func (l *Lox) run(source string) {
	s := scanner.NewScanner(source)
	tokens := s.ScanTokens(l.logger)
	parser := parser.NewParser(tokens)
	expr := parser.Parse()

	if l.logger.HadError() {
		return
	}


	l.interpreter.Interpret(expr)
}
