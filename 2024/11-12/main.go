package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

// Apply some real simple memoization
var cache = make(map[[2]int]int)

func apply(number int, steps int) int {

	// When reaching the end of the steps we can count one stone
	if steps == 0 {
		return 1
	}

	cacheKey := [2]int{number, steps}

	if value, ok := cache[cacheKey]; ok {
		return value
	}

	acc := 0
	num := strconv.Itoa(number) // Convert into a string so we can check the length
	size := len(num)

	if size%2 == 0 {

		// Convert splits back into numbers
		left, _ := strconv.Atoi(num[:size/2])
		right, _ := strconv.Atoi(num[size/2:])

		// Continue for each split stone
		acc += apply(left, steps-1)
		acc += apply(right, steps-1)
	} else if number == 0 {
		acc += apply(1, steps-1)
	} else {
		acc += apply(number*2024, steps-1)
	}
	cache[cacheKey] = acc
	return acc
}

func main() {
	args := os.Args[1:]
	file := args[0]

	data, err := os.ReadFile(file)
	if err != nil {
		fmt.Println("Error reading input")
		return
	}

	// Remove newline entry
	data = data[:len(data)-1]

	part_one := 0
	part_two := 0

	// Loop through all individual numbers for the amount of steps we want
	for entry := range strings.SplitSeq(string(data), " ") {
		number, err := strconv.Atoi(entry)
		if err == nil {
			part_one += apply(number, 25)
			part_two += apply(number, 75)
		}
	}

	fmt.Println("Answer part one:", part_one)
	fmt.Println("Answer part two:", part_two)
}
