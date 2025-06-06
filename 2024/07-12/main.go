package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Operator int

const (
	Addition Operator = iota
	Multiplication
)

func concatNumbers(a int, b int) int {
	c := strconv.Itoa(a) + strconv.Itoa(b)
	value, _ := strconv.Atoi(c)
	return value
}

func calc(total int, current int, remainder []int, useConcat bool) int {

	validFound := 0

	if len(remainder) > 1 {
		newAdd := current + remainder[0]
		newMul := current * remainder[0]

		validFound += calc(total, newAdd, remainder[1:], useConcat)
		validFound += calc(total, newMul, remainder[1:], useConcat)

		if useConcat {
			newCon := concatNumbers(current, remainder[0])
			validFound += calc(total, newCon, remainder[1:], useConcat)
		}

		return validFound
	}

	if current*remainder[0] == total || current+remainder[0] == total {
		return 1
	} else if useConcat && concatNumbers(current, remainder[0]) == total {
		return 1
	}
	return 0
}

func main() {

	args := os.Args[1:]
	file := args[0]

	data, err := os.Open(file)
	if err != nil {
		fmt.Println("Error opening file")
		return
	}

	calibrationResult := 0
	calibrationResultWithConcat := 0

	scanner := bufio.NewScanner(data)
	for scanner.Scan() {

		line := strings.Split(scanner.Text(), ":")

		total, _ := strconv.Atoi(line[0])
		sequence := strings.Split(strings.TrimSpace(line[1]), " ")
		numbers := make([]int, len(sequence))

		for idx, value := range sequence {
			intval, _ := strconv.Atoi(value)
			numbers[idx] = intval
		}

		if calc(total, 0, numbers, false) > 0 {
			calibrationResult += total
		}

		if calc(total, 0, numbers, true) > 0 {
			calibrationResultWithConcat += total
		}
	}

	fmt.Println("Answer part 1:", calibrationResult)
	fmt.Println("Answer part 2:", calibrationResultWithConcat)
}
