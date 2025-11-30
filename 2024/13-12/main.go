package main

import (
	"fmt"
	"os"
	"regexp"
	"strconv"
)

func findPath(position [2]int, increments [2][2]int, maxRange int) [2]int {
	// Solve the system of linear equations using Cramer's rule:
	// i * ax + j * bx = px
	// i * ay + j * by = py

	ax, ay := increments[0][0], increments[0][1] // Button A increments
	bx, by := increments[1][0], increments[1][1] // Button B increments
	px, py := position[0], position[1]           // Target position

	// Calculate determinant
	det := ax*by - ay*bx
	if det == 0 {
		// No unique solution (parallel lines)
		return [2]int{}
	}

	// Calculate solutions using Cramer's rule
	numeratorI := px*by - py*bx
	numeratorJ := py*ax - px*ay

	// Check if solutions are integers
	if numeratorI%det != 0 || numeratorJ%det != 0 {
		return [2]int{}
	}

	i := numeratorI / det
	j := numeratorJ / det

	// Check if solutions are non-negative
	if i < 0 || j < 0 {
		return [2]int{}
	}

	// For part 1, check maxRange constraint (part 2 doesn't have this constraint)
	if maxRange > 0 && (i >= maxRange || j >= maxRange) {
		return [2]int{}
	}

	return [2]int{i, j}
}

type ClawMachine struct {
	a [2]int
	b [2]int
	p [2]int
}

func main() {
	args := os.Args[1:]
	file := args[0]

	data, err := os.ReadFile(file)
	if err != nil {
		fmt.Println("Error reading input")
		return
	}

	// parse all input data into claw machine objects
	var machines []ClawMachine
	machineData := regexp.MustCompile(`Button A\: X\+(\d+), Y\+(\d+)\nButton B\: X\+(\d+), Y\+(\d+)\nPrize\: X\=(\d+), Y\=(\d+)`).FindAllSubmatch(data, -1)
	for _, machine := range machineData {
		aX, _ := strconv.Atoi(string(machine[1]))
		aY, _ := strconv.Atoi(string(machine[2]))
		bX, _ := strconv.Atoi(string(machine[3]))
		bY, _ := strconv.Atoi(string(machine[4]))
		pX, _ := strconv.Atoi(string(machine[5]))
		pY, _ := strconv.Atoi(string(machine[6]))

		machines = append(machines, ClawMachine{
			a: [2]int{aX, aY},
			b: [2]int{bX, bY},
			p: [2]int{pX, pY},
		})
	}

	partOne := 0
	for _, machine := range machines {
		// partOne += getCheapestTokensForClawMachine(machine)
		path := findPath(machine.p, [2][2]int{machine.a, machine.b}, 100)
		partOne += (path[0] * 3) + path[1]
	}
	fmt.Println("Answer part one:", partOne)

	partTwo := 0
	for _, machine := range machines {
		machine.p = [2]int{
			machine.p[0] + 10000000000000,
			machine.p[1] + 10000000000000,
		}
		path := findPath(machine.p, [2][2]int{machine.a, machine.b}, 0)
		partTwo += (path[0] * 3) + path[1]
	}
	fmt.Println("Answer part two:", partTwo)

}
