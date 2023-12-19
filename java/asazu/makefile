all: 
	javac dev/asazutaiga/lox/Lox.java

run:
	java dev.asazutaiga.lox.Lox $(ARGS)

expr:
	javac dev/asazutaiga/tool/GenerateAst.java
	java dev.asazutaiga.tool.GenerateAst dev/asazutaiga/lox/

printAst:
	javac dev/asazutaiga/lox/AstPrinter.java
	java dev.asazutaiga.lox.AstPrinter