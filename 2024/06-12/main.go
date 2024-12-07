package main

import (
	"fmt"
	"os"
	"slices"
	"strings"
)

type GuardDirection int

const (
	None GuardDirection = iota
	North
	East
	South
	West
)

func isGuardAtEdge(guardPosition int, cols int, rows int) bool {
	return guardPosition%cols == 0 || guardPosition < cols || guardPosition >= cols*(rows-1) || (guardPosition+1)%cols == 0
}

func getNewGuardDirection(currentDirection GuardDirection) GuardDirection {
	switch currentDirection {
	case North:
		return East
	case East:
		return South
	case South:
		return West
	case West:
		return North
	}

	// Default but should never be reached
	return North
}

func getNextGuardPositionOffset(dir GuardDirection, cols int) int {
	switch dir {
	case North:
		return -cols
	case East:
		return 1
	case South:
		return cols
	case West:
		return -1
	}
	return 0
}

func traverse(guardMap string, guardPosition int, guardDirection GuardDirection, cols int, rows int) int {

	var guardVisitedPositions []int
	for !isGuardAtEdge(guardPosition, cols, rows) {
		nextGuardPosition := guardPosition + getNextGuardPositionOffset(guardDirection, cols)

		if guardMap[nextGuardPosition] == '#' {
			guardDirection = getNewGuardDirection(guardDirection)
		} else {
			if !slices.Contains(guardVisitedPositions, guardPosition) {
				guardVisitedPositions = append(guardVisitedPositions, guardPosition)
			}
			guardPosition = nextGuardPosition
		}
	}

	return len(guardVisitedPositions) + 1
}

func main() {

	args := os.Args[1:]
	file := args[0]

	data, err := os.ReadFile(file)
	if err != nil {
		fmt.Println("Error while reading input file")
		return
	}

	guardMap := string(data)
	lines := strings.Split(guardMap, "\n")

	cols := len(lines[0])
	rows := len(lines) - 1 // Ignore newline at EOF that POSIX demands

	guardMap = strings.ReplaceAll(guardMap, "\n", "")
	guardPosition := strings.Index(guardMap, "^")
	guardDirection := North

	guardVisitedPositions := traverse(guardMap, guardPosition, guardDirection, cols, rows)

	// Append one for the last visited position
	fmt.Println("Answer part one:", guardVisitedPositions)
}
