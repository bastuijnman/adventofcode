package main

import (
	"fmt"
	"os"
	"slices"
	"strconv"
)

// Refactor
func edges(index int, cols int, rows int) []int {
	var edges []int

	// Top
	if index-cols >= 0 {
		edges = append(edges, index-cols)
	}

	// Right
	if (index+1)%cols != 0 {
		edges = append(edges, index+1)
	}

	// Bottom
	if index+cols < cols*rows {
		edges = append(edges, index+cols)
	}

	// Left
	if index%cols != 0 {
		edges = append(edges, index-1)
	}

	return edges
}

func main() {

	args := os.Args[1:]
	file := args[0]

	data, err := os.ReadFile(file)
	if err != nil {
		fmt.Println("Error while reading input file")
		return
	}

	// Get rid of newlines
	cols := slices.Index(data, 10)
	levels := slices.DeleteFunc(data, func(v byte) bool { return v == 10 })
	rows := len(levels) / cols

	var follow_path func(indices []int, n int, distinct_trails bool) []int
	follow_path = func(indices []int, n int, distinct_trails bool) []int {
		if n == 9 {
			return indices
		}

		var indices_to_test []int
		for _, i := range indices {
			var next []int
			for _, e := range edges(i, cols, rows) {
				if levels[e] == strconv.Itoa(n + 1)[0] {
					next = append(next, e)
				}
			}

			indices_to_test = slices.Concat(indices_to_test, follow_path(next, n+1, distinct_trails))

			// If we don't want the distinct trails we only care about the unique indices
			// of the tested edges. This will give us just the unique destinations.
			if !distinct_trails {
				slices.Sort(indices_to_test)
				indices_to_test = slices.Compact(indices_to_test)
			}
		}

		return indices_to_test
	}

	var starts []int
	for i, n := range levels {
		if n == '0' {
			starts = append(starts, i)
		}
	}

	trails := 0
	for _, idx := range starts {
		trails += len(follow_path([]int{idx}, 0, false))
	}

	distinct_trails := 0
	for _, idx := range starts {
		distinct_trails += len(follow_path([]int{idx}, 0, true))
	}
	fmt.Println("Answer part one:", trails)
	fmt.Println("Answer part two:", distinct_trails)
}
