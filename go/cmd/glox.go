package cmd

import (
	"github.com/AsazuTaiga/crafting_interpriters/go/logger"
	"github.com/AsazuTaiga/crafting_interpriters/go/lox"
)

type LoxCmd struct {
}

func NewLoxCmd() *LoxCmd {
	return &LoxCmd{}
}

func (cmd *LoxCmd) Run() {
	log := logger.NewLogger()
	l := lox.NewLox(log)
	l.Run()
}