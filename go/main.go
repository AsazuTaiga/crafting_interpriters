package main

import (
	"github.com/AsazuTaiga/crafting_interpriters/go/logger"
	"github.com/AsazuTaiga/crafting_interpriters/go/lox"
)

func main() {
	log := logger.NewLogger()
	l := lox.NewLox(log)
	l.Run()
}


