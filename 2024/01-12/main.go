package main

import (
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

func absInt(input int) int {
	if input < 0 {
		return -input
	}
	return input
}

func main() {

	args := os.Args[1:]
	file := args[0]

	data, err := os.ReadFile(file)
	if err != nil {
		fmt.Println("File not found")
		return
	}

	lines := strings.Split(string(data), "\n")
	lineCount := len(lines)

	leftList := make([]int, lineCount)
	rightList := make([]int, lineCount)
	similarityMap := make(map[int]int)

	// Account for the EOL newline that POSIX demands
	// this shoul've been a buffer scanner
	for i := 0; i < len(lines)-1; i++ {
		values := strings.Split(lines[i], "   ")

		leftValue, leftErr := strconv.Atoi(values[0])
		rightValue, rightErr := strconv.Atoi(values[1])
		if leftErr != nil || rightErr != nil {
			fmt.Println("A value could not be converted")
			return
		}

		leftList[i] = leftValue
		rightList[i] = rightValue
		similarityMap[rightValue] += 1
	}

	sort.Sort(sort.IntSlice(leftList))
	sort.Sort(sort.IntSlice(rightList))

	total := 0
	for i := 0; i < lineCount; i++ {
		total += absInt(leftList[i] - rightList[i])
	}
	fmt.Println("Answer part 1:", total)

	totalSimilarity := 0
	for i := 0; i < lineCount; i++ {
		totalSimilarity += leftList[i] * similarityMap[leftList[i]]
	}
	fmt.Println("Answer part 2:", totalSimilarity)

}
