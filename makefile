all: 
	javac dev/asazutaiga/lox/Lox.java
	java dev.asazutaiga.lox.Lox $(ARGS)