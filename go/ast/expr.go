package ast

import (
	"github.com/AsazuTaiga/crafting_interpriters/go/token"
)

type Expr interface {
	Accept(visitor Visitor) interface{}
}

type Visitor interface {
	VisitBinaryExpr(expr BinaryExpr) interface{}
	VisitGroupingExpr(expr GroupingExpr) interface{}
	VisitLiteralExpr(expr LiteralExpr) interface{}
	VisitUnaryExpr(expr UnaryExpr) interface{}
}

type BinaryExpr struct {
	Left Expr
	Operator token.Token
	Right Expr
}

func (expr BinaryExpr) Accept(visitor Visitor) interface{} {
	return visitor.VisitBinaryExpr(expr)
}

type GroupingExpr struct {
	Expression Expr
}

func (expr GroupingExpr) Accept(visitor Visitor) interface{} {
	return visitor.VisitGroupingExpr(expr)
}

type LiteralExpr struct {
	Value interface{}
}

func (expr LiteralExpr) Accept(visitor Visitor) interface{} {
	return visitor.VisitLiteralExpr(expr)
}

type UnaryExpr struct {
	Operator token.Token
	Right Expr
}

func (expr UnaryExpr) Accept(visitor Visitor) interface{} {
	return visitor.VisitUnaryExpr(expr)
}


