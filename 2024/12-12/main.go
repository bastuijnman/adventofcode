package main

import (
	"adventofcode/lib"
	"fmt"
	"os"
	"slices"
)

type GardenPlot struct {
	contents byte
	indices  []int
	edges    int
}

func main() {

	args := os.Args[1:]
	file := args[0]

	data, err := os.ReadFile(file)
	if err != nil {
		fmt.Println("Error reading input")
		return
	}

	// Get rid of newlines
	cols := slices.Index(data, 10)
	levels := slices.DeleteFunc(data, func(v byte) bool { return v == 10 })
	rows := len(levels) / cols

	var plots []*GardenPlot

	cost := 0
	fmt.Println("Answer part one:", cost)
}
