package main

import (
	"fmt"
	"os"
	"strings"
)

// Find all the indices of a character occuring in the haystack string
func findIndicesOfCharInString(haystack string, needle rune) []int {
	var indices []int
	for index, char := range haystack {
		if char == needle {
			indices = append(indices, index)
		}
	}
	return indices
}

// Get all the directions that are valid for a given point within
// the grid. Discards any that are invalid
func getValidDirectionsForPoint(point int, colLen int, rowLen int) [][4]int {

	// Define all the offsets to check
	check := [8][2]int{{3, 0}, {-3, 0}, {0, -3}, {0, 3}, {-3, -3}, {3, 3}, {3, -3}, {-3, 3}}

	// Define all the offsets for any potential valid direction.
	// This should probably be stored in a map or something similar
	dirs := [8][4]int{
		{0, 1, 2, 3},
		{0, -1, -2, -3},
		{0, -colLen, -(colLen * 2), -(colLen * 3)},
		{0, colLen, colLen * 2, colLen * 3},
		{0, -(colLen + 1), -((colLen * 2) + 2), -((colLen * 3) + 3)},
		{0, colLen + 1, (colLen * 2) + 2, ((colLen * 3) + 3)},
		{0, -(colLen - 1), -((colLen * 2) - 2), -((colLen * 3) - 3)},
		{0, colLen - 1, (colLen * 2) - 2, (colLen * 3) - 3},
	}

	pointRow := point / colLen
	pointCol := point - (pointRow * colLen)

	var validDirs [][4]int
	for dirIndex, dir := range dirs {
		colCheck := pointCol + check[dirIndex][0]
		rowCheck := pointRow + check[dirIndex][1]

		// Ignore any invalid direction
		if colCheck < 0 || colCheck > colLen-1 || rowCheck < 0 || rowCheck > rowLen-1 {
			continue
		}

		validDirs = append(validDirs, dir)
	}

	return validDirs

}

// Get the valid directions for a cross, error out if the point is
// not able to make a cross.
func getValidDirectionsForPointWithCross(point int, colLen int, rowLen int) ([2][3]int, error) {
	// Define the checks that we need to run to see if the point
	// is going to be valid anyway.
	checks := [4][2]int{{-1, -1}, {1, -1}, {-1, 1}, {1, 1}}
	defaultReturn := [2][3]int{{0, 0, 0}, {0, 0, 0}}

	pointRow := point / colLen
	pointCol := point - (pointRow * colLen)

	// Check for all potential offsets, error if an invalid one is found
	for _, check := range checks {
		colCheck := pointCol + check[0]
		rowCheck := pointRow + check[1]
		if colCheck < 0 || colCheck > colLen-1 || rowCheck < 0 || rowCheck > rowLen-1 {
			return defaultReturn, fmt.Errorf("Invalid point")
		}
	}

	// Return the cross offsets
	return [2][3]int{
		{-(colLen + 1), 0, colLen + 1},
		{-(colLen - 1), 0, colLen - 1},
	}, nil
}

func main() {

	args := os.Args[1:]
	file := args[0]

	data, err := os.ReadFile(file)
	if err != nil {
		fmt.Println("Error while reading input file")
		return
	}

	letters := string(data)

	colLen := strings.Index(letters, "\n")
	rowLen := strings.Count(letters, "\n")
	letters = strings.ReplaceAll(letters, "\n", "")

	// Grab all of the X chars and see if we can write XMAS (or the reverse)
	// in any of the valid directions for that position
	countPartOne := 0
	points := findIndicesOfCharInString(letters, 'X')
	for _, point := range points {
		directions := getValidDirectionsForPoint(point, colLen, rowLen)
		for _, direction := range directions {
			var wordBuilder strings.Builder
			for _, offset := range direction {
				wordBuilder.WriteByte(letters[point+offset])
			}

			word := wordBuilder.String()

			if word == "XMAS" || word == "SAMX" {
				countPartOne += 1
			}
		}
	}

	// Similar to finding XMAS we now want to find 2 occurances of MAS (or the
	// reverse) from the positions of the A in a cross shape. Code is almost the
	// same with the exception that we check wether the entire point is valid
	// instead of multiple directions.
	//
	// TODO: lot's o code re-use, can probably be abstracted quite a bit
	countPartTwo := 0
	pointsOfA := findIndicesOfCharInString(letters, 'A')
	for _, point := range pointsOfA {
		directions, err := getValidDirectionsForPointWithCross(point, colLen, rowLen)
		if err != nil {
			continue
		}

		isValid := true
		for _, direction := range directions {
			var wordBuilder strings.Builder
			for _, offset := range direction {
				wordBuilder.WriteByte(letters[point+offset])
			}
			word := wordBuilder.String()

			if word != "MAS" && word != "SAM" {
				isValid = false
			}
		}

		if isValid {
			countPartTwo += 1
		}
	}

	fmt.Println("Answer part one:", countPartOne)
	fmt.Println("Answer part two:", countPartTwo)
}
