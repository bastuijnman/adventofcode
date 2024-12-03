package main

import (
	"fmt"
	"os"
	"regexp"
	"strconv"
)

func calculateTotal(useConditionals bool) (int, error) {

	args := os.Args[1:]
	file := args[0]

	data, err := os.ReadFile(file)
	if err != nil {
		return -1, err
	}

	memory := string(data)
	instructionsEnabled := true
	total := 0

	// Setup regexes by default just look for multiplication functions
	// if we use conditionsals then we also match for do() or don't()
	regex, _ := regexp.Compile(`mul\((\d{1,3}),(\d{1,3})\)`)
	if useConditionals {
		regex, _ = regexp.Compile(`(?:mul\((\d{1,3}),(\d{1,3})\))|(don\'t\(\))|(do\(\))`)
	}
	matches := regex.FindAllStringSubmatch(memory, -1)
	for _, match := range matches {

		if useConditionals && (match[0] == "do()" || match[0] == "don't()") {
			instructionsEnabled = match[0] == "do()"
			continue
		}

		firstMul, firstMulErr := strconv.Atoi(match[1])
		secondMul, secondMulErr := strconv.Atoi(match[2])

		if firstMulErr != nil || secondMulErr != nil {
			return -1, fmt.Errorf("Could not convert one of the multiplication numbers")
		}

		if instructionsEnabled {
			total += firstMul * secondMul
		}
	}

	return total, nil
}

func main() {
	totalWithoutConditionals, _ := calculateTotal(false)
	totalWithConditionals, _ := calculateTotal(true)
	fmt.Println("Answer part one:", totalWithoutConditionals)
	fmt.Println("Answer part two:", totalWithConditionals)
}
