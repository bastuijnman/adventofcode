package main

import (
	"bufio"
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

// abs will return the absolute value of an integer
func abs(input int) int {
	if input < 0 {
		return -input
	}
	return input
}

func copyReport(report []int) []int {
	cpy := make([]int, len(report))
	copy(cpy, report)
	return cpy
}

// Check if any given report (array of levels) is valid
func isReportValid(report []int) bool {
	reportSize := len(report)
	for i := 0; i < reportSize; i++ {

		// Check if it stays ascending/descending
		if i > 0 && i < reportSize-1 {
			if !(report[i-1] < report[i] && report[i] < report[i+1]) && !(report[i-1] > report[i] && report[i] > report[i+1]) {
				return false
			}
		}

		// Check if sizes stay in bounds, don't want to check
		// the last iterator value since it was already checked
		if i < reportSize-1 {
			diff := abs(report[i] - report[i+1])
			if diff < 1 || diff > 3 {
				return false
			}
		}
	}

	return true
}

// Parse the report string into an actual integer slice
func parseReport(report string) []int {
	var numbers []int
	fields := strings.Fields(report)
	for _, field := range fields {
		num, _ := strconv.Atoi(field)
		numbers = append(numbers, num)
	}
	return numbers
}

func validReports(useProblemDampener bool) (int, error) {
	args := os.Args[1:]
	file := args[0]

	data, err := os.Open(file)
	if err != nil {
		return -1, err
	}

	scanner := bufio.NewScanner(data)

	valid := 0
	for scanner.Scan() {
		report := parseReport(scanner.Text())

		if isReportValid(report) {
			valid += 1
		} else if useProblemDampener {

			// Bruteforce :(
			for i := 0; i < len(report); i++ {

				cpy := copyReport(report)
				cpy = slices.Delete(cpy, i, i+1)
				if isReportValid(cpy) {
					valid += 1
					break
				}
			}
		}
	}
	return valid, nil
}

func main() {

	validReportsWithoutProblemDampener, errWithoutDampener := validReports(false)
	validReportsWithProblemDampener, errWithDampener := validReports(true)

	if errWithoutDampener != nil || errWithDampener != nil {
		fmt.Println(errWithoutDampener, errWithDampener)
		return
	}

	fmt.Println("Answer part one:", validReportsWithoutProblemDampener)
	fmt.Println("Answer part two:", validReportsWithProblemDampener)
}
