package main

import (
	"fmt"
	"maps"
	"os"
	"slices"
	"strings"
)

func getAntennaCoordinates(antennaMap [][]rune) map[rune][][2]int {
	chars := make(map[rune][][2]int)

	rows := len(antennaMap)
	cols := len(antennaMap[0])

	for y := 0; y < rows; y++ {
		for x := 0; x < cols; x++ {
			c := antennaMap[y][x]
			if c != '.' {
				chars[c] = append(chars[c], [2]int{y, x})
			}
		}
	}

	return chars
}

// Find all antinodes based on antenna coordinates
func findAntinodes(antennaCoordinates map[rune][][2]int, cols int, rows int, resonance bool) [][2]int {
	var antinodes [][2]int

	for antennaType := range maps.Keys(antennaCoordinates) {

		for _, check := range antennaCoordinates[antennaType] {
			for _, other := range antennaCoordinates[antennaType] {
				if check == other {
					continue
				}

				if resonance && !slices.Contains(antinodes, other) {
					antinodes = append(antinodes, other)
				}

				diff := [2]int{other[0] - check[0], other[1] - check[1]}

				potentialAntinode := [2]int{other[0] + diff[0], other[1] + diff[1]}
				for potentialAntinode[0] >= 0 && potentialAntinode[0] < rows && potentialAntinode[1] >= 0 && potentialAntinode[1] < cols {
					if !slices.Contains(antinodes, potentialAntinode) {
						antinodes = append(antinodes, potentialAntinode)
					}

					if !resonance {
						break
					}
					potentialAntinode = [2]int{potentialAntinode[0] + diff[0], potentialAntinode[1] + diff[1]}
				}
			}
		}

	}

	return antinodes
}

// Parse input into a map
func makeMap(input string) [][]rune {
	var m [][]rune
	lines := strings.Split(input, "\n")

	// Remove last empty newline
	lines = lines[:len(lines)-1]

	for _, line := range lines {
		m = append(m, []rune(line))
	}
	return m
}

func main() {

	args := os.Args[1:]
	file := args[0]

	data, err := os.ReadFile(file)
	if err != nil {
		fmt.Println("Error while reading input file")
		return
	}

	// Parse map & the different antenna coordinates
	antennaMap := makeMap(string(data))
	antennaMapRows := len(antennaMap)
	antennaMapCols := len(antennaMap[0])
	mapped := getAntennaCoordinates(antennaMap)

	antinodes := findAntinodes(mapped, antennaMapCols, antennaMapRows, false)
	resonantAntinodes := findAntinodes(mapped, antennaMapCols, antennaMapRows, true)
	fmt.Println("Answer part one:", len(antinodes))
	fmt.Println("Answer part two:", len(resonantAntinodes))
}
