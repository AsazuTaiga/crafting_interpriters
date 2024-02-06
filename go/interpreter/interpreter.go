package interpreter

import (
	"errors"
	"fmt"
	"strings"

	"github.com/AsazuTaiga/crafting_interpriters/go/ast"
	"github.com/AsazuTaiga/crafting_interpriters/go/token"
)

type Interpreter struct {}

func NewInterpreter() *Interpreter {
	return &Interpreter{}
}

func(i *Interpreter) Interpret(expression ast.Expr) interface{} {
	value:= i.evaluate(expression)
	return fmt.Sprintf("%s", stringify(value))
}

func (i *Interpreter) VisitLiteralExpr(expr ast.LiteralExpr) interface{} {
	return expr.Value
}

func (i *Interpreter) VisitGroupingExpr(expr ast.GroupingExpr) interface{} {
	return expr.Accept(i)
}

func (i *Interpreter) VisitUnaryExpr(expr ast.UnaryExpr) interface{} {
	right := i.evaluate(expr.Right)

	switch expr.Operator.Type {
	case token.BANG:
		return !isTruthy(right);
	case token.MINUS:
		err := checkNumberOperand(expr.Operator, right)
		if err != nil {
			fmt.Printf("%s\n", err)
			return nil
		}
		return -right.(float64)
	}

	return nil
}

func (i *Interpreter) VisitBinaryExpr(expr ast.BinaryExpr) interface{} {
	left := i.evaluate(expr.Left)
	right := i.evaluate(expr.Right)

  switch expr.Operator.Type {
	case token.GREATER:
		err := checkNumberOperands(expr.Operator, left, right)
		if err != nil {
			fmt.Printf("%s\n", err)
			return nil
		}
		return left.(float64) > right.(float64)
	case token.GREATER_EQUAL:
		err := checkNumberOperands(expr.Operator, left, right)
		if err != nil {
			fmt.Printf("%s\n", err)
			return nil
		}
		return left.(float64) >= right.(float64)
	case token.LESS:
		err := checkNumberOperands(expr.Operator, left, right)
		if err != nil {
			fmt.Printf("%s\n", err)
			return nil
		}
		return left.(float64) < right.(float64)
	case token.LESS_EQUAL:
		err := checkNumberOperands(expr.Operator, left, right)
		if err != nil {
			fmt.Printf("%s\n", err)
			return nil
		}
		return left.(float64) <= right.(float64)
	case token.MINUS:
		err := checkNumberOperands(expr.Operator, left, right)
		if err != nil {
			fmt.Printf("%s\n", err)
			return nil
		}
		return left.(float64) - right.(float64)
	case token.PLUS:
		switch left.(type) {
		case float64:
			switch right.(type) {
			case float64:
				return left.(float64) + right.(float64)
			}
		case string:
			switch right.(type) {
			case string:
				return left.(string) + right.(string)
			}
		}
		err := errors.New(fmt.Sprintf("Error: %s", "Operands must be two numbers or two strings."))
		fmt.Printf("%s\n", err)
		return nil
	case token.SLASH:
		err := checkNumberOperands(expr.Operator, left, right)
		if err != nil {
			fmt.Printf("%s\n", err)
			return nil
		}
		return left.(float64) / right.(float64)
	case token.STAR:
		err := checkNumberOperands(expr.Operator, left, right)
		if err != nil {
			fmt.Printf("%s\n", err)
			return nil
		}
		return left.(float64) * right.(float64)
	case token.BANG_EQUAL:
		return !isEqual(left, right)
	case token.EQUAL_EQUAL:
		return isEqual(left, right)
	}

	return nil
}

func(i *Interpreter) evaluate(expr ast.Expr) interface{} {
	return expr.Accept(i)
}

func  isTruthy(object interface{}) bool {
	if object == nil {
		return false
	}
	switch object.(type) {
	case bool:
		return object.(bool)
	}
	return true
}

func  isEqual(a interface{}, b interface{}) bool {
	if a == nil && b == nil {
		return true
	}
	if a == nil {
		return false
	}

	return a == b
}

func  checkNumberOperand(operator token.Token, operand interface{}) error {
	switch operand.(type) {
	case float64:
		return nil
	}
	return errors.New(fmt.Sprintf("Error: %s", "Operand must be a number."))
}

func  checkNumberOperands(operator token.Token, left interface{}, right interface{}) error {
	switch left.(type) {
	case float64:
		switch right.(type) {
		case float64:
			return nil
		}
	}
	return errors.New(fmt.Sprintf("Error: %s", "Operands must be numbers."))
}

func stringify(object interface{}) string {
	if object == nil {
		return "nil"
	}
	switch object.(type) {
	case float64:
		text := fmt.Sprintf("%g", object)
		if strings.HasSuffix(text, ".0") {
			text = text[:len(text)-2]
		}
		return text
	}
	return object.(string)
}

