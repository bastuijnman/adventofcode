package main

import (
	"fmt"
	"os"
	"time"
)

func calc(one [2]int, two [2]int) [2]int {
	return [2]int{
		one[0] + two[0],
		one[1] + two[1],
	}
}

type Change struct {
	changes     [][2]int
	processable bool
}

func get_changes(room [][]byte, pos [2]int, dir [2]int) Change {
	result := Change{
		changes:     make([][2]int, 0),
		processable: false,
	}

	pos = calc(pos, dir)
	var next = room[pos[0]][pos[1]]
	for next != '#' {
		if next == 'O' {
			result.changes = append(result.changes, pos)
		} else if next == '.' {
			result.processable = true
		}
		pos = calc(pos, dir)
		next = room[pos[0]][pos[1]]
	}

	return result
}

func main() {

	args := os.Args[1:]
	file := args[0]

	data, err := os.ReadFile(file)
	if err != nil {
		fmt.Println("Error reading input")
		return
	}

	// Parse input data
	var room [][]byte
	var currentRow []byte
	var robotIndex int
	var operations []byte

	processingOperations := false
	for i, c := range data {

		// Record robot position when found
		if c == '@' {
			robotIndex = i - len(room)
			c = '.'
		}

		if processingOperations {
			if c != '\n' {
				operations = append(operations, c)
			}
		} else {
			if c != '\n' {
				currentRow = append(currentRow, c)
			} else {

				// If the previous row is empty we have a double newline, and we can start processing the operations
				if len(currentRow) == 0 {
					processingOperations = true
				} else {
					room = append(room, currentRow)
					currentRow = []byte{}
				}
			}
		}
	}

	// Construct room, operations and roboto position
	robot := [2]int{
		robotIndex / len(room[0]),
		robotIndex % len(room[0]),
	}

	var changes [][2]int
	for _, operation := range operations {
		fmt.Print("\033[H\033[2J")
		processable := false
		changes = [][2]int{}
		dx := 0
		dy := 0

		switch operation {
		case '>':
			dx = 1
			test := get_changes(room, robot, [2]int{0, 1})
			changes = test.changes
			processable = test.processable
		case '<':
			dx = -1
			test := get_changes(room, robot, [2]int{0, -1})
			changes = test.changes
			processable = test.processable
		case '^':
			dy = -1

			test := get_changes(room, robot, [2]int{-1, 0})
			changes = test.changes
			processable = test.processable
		case 'v':
			dy = 1
			test := get_changes(room, robot, [2]int{1, 0})
			changes = test.changes
			processable = test.processable
		}

		value := room[robot[0]+dy][robot[1]+dx]
		switch value {
		case '.':
			robot = [2]int{robot[0] + dy, robot[1] + dx}
		case 'O':
			if processable {
				room[robot[0]][robot[1]] = '.'
				robot = [2]int{robot[0] + dy, robot[1] + dx}
				for _, n := range changes {
					room[n[0]+dy][n[1]+dx] = 'O'
				}
			}
		}

		printRoom(room, robot)
		fmt.Println("Operation", string(operation), "Next Was", string(value))
		time.Sleep(2 * time.Second)
	}

	total := 0
	for y, row := range room {
		for x, c := range row {
			if c == 'O' {
				total += (100 * y) + x
			}
		}
	}
	fmt.Println("Answer part one", total)
}

func printRoom(room [][]byte, robot [2]int) {
	for y, row := range room {
		for x, c := range row {
			if x == robot[0] && y == robot[1] {
				fmt.Print("@")
			} else {
				fmt.Print(string(c))
			}
		}
		fmt.Println()
	}
}
