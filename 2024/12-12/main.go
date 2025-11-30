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

	// Find plot for any given starting index & value
	var processed []int
	var rec func(index int, value byte) []int
	rec = func(index int, value byte) []int {
		edges := lib.Edges(index, cols, rows)

		result := []int{index}
		processed = append(processed, index)
		for _, edge := range edges {
			if levels[edge.Index] == levels[index] && !slices.Contains(processed, edge.Index) {
				result = slices.Concat(result, rec(edge.Index, value))
			}
		}
		return result
	}

	// Calculate edges for any given garden plot,
	var calcEdges func(plot GardenPlot) int
	calcEdges = func(plot GardenPlot) int {
		total := 0
		for _, index := range plot.indices {
			edges := lib.Edges(index, cols, rows)
			count := 4 - len(edges)
			for _, edge := range edges {
				if levels[edge.Index] != levels[index] {
					count += 1
				}
			}
			total += count
		}
		return total
	}

	calcEdgeCount := func(indices []int, edge lib.EdgeLocation) int {
		var result []int

		diff := 1
		if edge == lib.LeftEdge || edge == lib.RightEdge {
			diff = cols
		}
		for _, index := range indices {
			topEdge, err := lib.GetEdge(lib.Edges(index, cols, rows), edge)
			if err != nil || levels[index] != levels[topEdge.Index] {
				i := slices.Index(result, index-diff)
				if i == -1 {
					result = append(result, index)
				} else {
					result[i] = index
				}
			}
		}
		return len(result)
	}

	// Calculate sides for any given garden plot
	calcSides := func(plot GardenPlot) int {
		var edges = []lib.EdgeLocation{
			lib.TopEdge,
			lib.RightEdge,
			lib.BottomEdge,
			lib.LeftEdge,
		}
		var indices []int
		indices = make([]int, len(plot.indices))

		copy(indices, plot.indices)
		slices.Sort(indices)

		var total int
		for _, edge := range edges {
			total += calcEdgeCount(indices, edge)
		}
		return total
	}

	for index, value := range levels {
		plotIndex := slices.IndexFunc(plots, func(plot *GardenPlot) bool {
			return plot.contents == value && slices.Contains(plot.indices, index)
		})

		// We only care about non-existing plots in case we're encountering a new value
		if plotIndex == -1 {
			plots = append(plots, &GardenPlot{
				contents: value,
				indices:  rec(index, value),
			})
		}
	}

	partOne := 0
	partTwo := 0
	for _, plot := range plots {
		partOne += calcEdges(*plot) * len(plot.indices)
		partTwo += calcSides(*plot) * len(plot.indices)
	}

	fmt.Println("Answer part one:", partOne)
	fmt.Println("Answer part two:", partTwo)
}
