package ast

import "github.com/AsazuTaiga/crafting_interpriters/go/token"

type AssignExpr struct {
	Name  token.Token
	Value Expr
}

func (b AssignExpr) StartLine() int {
	// TODO implement me
	panic("implement me")
}

func (b AssignExpr) EndLine() int {
	// TODO implement me
	panic("implement me")
}

func (b AssignExpr) Accept(visitor Visitor) interface{} {
	return visitor.VisitAssignExpr(b)
}