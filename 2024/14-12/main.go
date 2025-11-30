package main

import (
	"adventofcode/lib"
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
)

func clip(val int, max int) int {
	if val < 0 {
		return max - lib.AbsInt(val) + 1
	} else if val > max {
		return lib.AbsInt(val-max) - 1
	}
	return val
}

type Robot struct {
	position [2]int
	velocity [2]int
}

func printTree(robots []*Robot, cols int, rows int) {
	grid := make([][]int, rows)
	for row := range rows {
		grid[row] = make([]int, cols)
	}

	for i := range robots {
		grid[robots[i].position[1]][robots[i].position[0]] += 1
	}

	for _, row := range grid {
		for _, col := range row {
			if col > 0 {
				fmt.Print("*")
			} else {
				fmt.Print(".")
			}
		}
		fmt.Println()
	}
}

func calculateShapeScore(robots []*Robot, cols int, rows int) int {
	grid := make([][]int, rows)
	for row := range rows {
		grid[row] = make([]int, cols)
	}

	for i := range robots {
		grid[robots[i].position[1]][robots[i].position[0]] += 1
	}

	score := 0
	for i := range robots {
		position := robots[i].position
		checks := [8][2]int{
			{position[0] - 1, position[1] - 1},
			{position[0], position[1] - 1},
			{position[0] + 1, position[1] - 1},
			{position[0] - 1, position[1]},
			{position[0] + 1, position[1]},
			{position[0] - 1, position[1] + 1},
			{position[0], position[1] + 1},
			{position[0] + 1, position[1] + 1},
		}

		for _, check := range checks {
			if check[0] < 0 || check[0] >= cols || check[1] < 0 || check[1] >= rows {
				continue
			}

			if grid[check[1]][check[0]] > 0 {
				score += 1
			}
		}
	}

	return score
}

func process(robots []*Robot, cols int, rows int) {
	for i := range robots {
		robots[i].position[0] = clip(robots[i].position[0]+robots[i].velocity[0], cols-1)
		robots[i].position[1] = clip(robots[i].position[1]+robots[i].velocity[1], rows-1)
	}

}

func main() {

	args := os.Args[1:]
	file := args[0]
	cols, _ := strconv.Atoi(args[1])
	rows, _ := strconv.Atoi(args[2])

	data, err := os.ReadFile(file)
	if err != nil {
		fmt.Println("Error reading input")
		return
	}

	var robots []*Robot
	robotData := regexp.MustCompile(`p=(\d+),(\d+) v=(-?\d+),(-?\d+)`).FindAllSubmatch(data, -1)
	for _, entry := range robotData {
		pX, _ := strconv.Atoi(string(entry[1]))
		pY, _ := strconv.Atoi(string(entry[2]))
		vX, _ := strconv.Atoi(string(entry[3]))
		vY, _ := strconv.Atoi(string(entry[4]))

		robots = append(robots, &Robot{
			position: [2]int{pX, pY},
			velocity: [2]int{vX, vY},
		})
	}

	for range 100 {
		process(robots, cols, rows)
	}

	quadrants := make(map[[2]int]int)
	for i := range robots {
		if robots[i].position[1]-((rows-1)/2) != 0 && robots[i].position[0]-((cols-1)/2) != 0 {
			hor := 0
			if robots[i].position[0] > ((cols - 1) / 2) {
				hor = 1
			}

			ver := 0
			if robots[i].position[1] > ((rows - 1) / 2) {
				ver = 1
			}

			quadrants[[2]int{hor, ver}] += 1
		}
	}

	total := 0
	for key := range quadrants {
		if total == 0 {
			total = quadrants[key]
		} else {
			total = total * quadrants[key]
		}
	}

	fmt.Println("Answer part one", total)

	// Seconds left over from previous run, assuming we go over 100
	seconds := 100
	for {
		reader := bufio.NewReader(os.Stdin)

		process(robots, cols, rows)
		seconds += 1

		// Arbitrary score of 500 to calculate the chance of a shape forming
		if calculateShapeScore(robots, cols, rows) > 500 {
			fmt.Print("\033[H\033[2J") // Clear terminal screen
			printTree(robots, cols, rows)
			fmt.Println("Number of seconds:", seconds)

			// Just halt for input when we find some shape so the user can check if it's a christmas tree
			reader.ReadString('\n')
		}
	}

}
