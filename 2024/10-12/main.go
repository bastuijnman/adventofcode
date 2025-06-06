package main

import (
	"fmt"
	"os"
	"slices"
	"strings"
)

type TrailMap struct {
	contents []int
	height   int
	width    int
}

type TrailPathNode struct {
	index int
	next  []TrailPathNode
}

func parseTrailMap(trailMap string) []int {
	var entries []int
	for _, c := range trailMap {
		entries = append(entries, int(c-'0'))
	}
	fmt.Println(entries)
	return entries
}

func findTrailPositions(trailMap []int, needle int) []int {
	var trailHeads []int
	for idx, c := range trailMap {
		if c == needle {
			trailHeads = append(trailHeads, idx)
		}
	}
	return trailHeads
}

func getValidNeighbours(trailMap TrailMap, index int) []TrailPathNode {
	directions := [4]int{-trailMap.width, 1, trailMap.width, -1}
	var nodes []TrailPathNode

	for _, direction := range directions {
		check := index + direction

		// TODO: these checks can be optimised
		if check < 0 || check >= trailMap.width*trailMap.height {
			continue
		}
		if index%trailMap.width == 0 && direction == -1 {
			continue
		}
		if index+1%trailMap.width == 0 && direction == 1 {
			continue
		}

		if trailMap.contents[check]-trailMap.contents[index] != 1 {
			continue
		}
		nodes = append(nodes, TrailPathNode{
			index: check,
			next:  getValidNeighbours(trailMap, check),
		})
	}

	return nodes
}

func getAllDestinations(start TrailPathNode, trailMap TrailMap) []int {
	var total []int

	if len(start.next) == 0 && trailMap.contents[start.index] == 9 {
		total = append(total, start.index)
	}

	for _, n := range start.next {
		nu := getAllDestinations(n, trailMap)
		total = append(total, nu...)
	}

	return total
}

func unique(slice []int) []int {
	var result []int
	for _, val := range slice {
		if !slices.Contains(result, val) {
			result = append(result, val)
		}
	}
	return result
}

func main() {

	args := os.Args[1:]
	file := args[0]

	data, err := os.ReadFile(file)
	if err != nil {
		fmt.Println("Error while reading input file")
		return
	}

	trailMapData := string(data)
	lines := strings.Split(trailMapData, "\n")
	cols := len(lines[0])
	rows := len(lines) - 1 // Ignore newline at EOF that POSIX demands

	trailMapData = strings.ReplaceAll(trailMapData, "\n", "")
	trailMap := TrailMap{
		contents: parseTrailMap(trailMapData),
		width:    cols,
		height:   rows,
	}

	trailHeads := findTrailPositions(trailMap.contents, 0)
	total := 0
	for _, trailHead := range trailHeads {
		trail := TrailPathNode{
			index: trailHead,
			next:  getValidNeighbours(trailMap, trailHead),
		}

		uniqueDestinations := unique(getAllDestinations(trail, trailMap))
		total += len(uniqueDestinations)
	}

	fmt.Println("Answer part one:", total)

}
