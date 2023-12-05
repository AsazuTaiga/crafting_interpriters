package logger

import (
	"fmt"
	"os"
)

type Logger struct {
	hadError bool
}

func NewLogger() *Logger {
	return &Logger{}
}

func (l *Logger) HadError() bool {
	return l.hadError
}

func (l *Logger) ResetError() {
	l.hadError = false
}

func (l *Logger) ErrorReport(line int, message string) {
	l.report(line, "", message)
}

func (l *Logger) report(line int, where, message string) {
	fmt.Fprintf(os.Stderr, "[line %d] Error %s: %s\n", line, where, message)
	l.hadError = true
}