package main

import (
	"github.com/AsazuTaiga/crafting_interpriters/go/logger"
	"github.com/AsazuTaiga/crafting_interpriters/go/lox"
)

func Run() {
	log := logger.NewLogger()
	l := lox.NewLox(log)
	l.Run()
}