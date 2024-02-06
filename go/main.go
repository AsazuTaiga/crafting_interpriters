package main

import (
	"os"

	"github.com/AsazuTaiga/crafting_interpriters/go/cmd"
	cli "github.com/urfave/cli/v2"
)

func main() {
	app := cli.NewApp()

	app.Commands = []*cli.Command{
		{
			Name:   "ast-printer",
			Aliases: []string{"ast"},
			Usage:  "print ast",
			Action: func(c *cli.Context) error {
				cmd := cmd.NewAstPrinterCmd()
				cmd.Run()
				return nil
			},
		},
		{
			Name:   "glox",
			Aliases: []string{"lox"},
			Usage:  "run glox",
			Action: func(c *cli.Context) error {
				cmd := cmd.NewLoxCmd()
				cmd.Run()
				return nil
			},
		},
	}

	err := app.Run(os.Args)
	if err != nil {
		os.Exit(1)
	}
}