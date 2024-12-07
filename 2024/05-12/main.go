package main

import (
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

// Parse input into a 2-dimensional int slice
func parse(input string, sep string) ([][]int, error) {
	var rules [][]int

	entries := strings.Split(input, "\n")
	for _, entry := range entries {

		// Entries can contain the EOF newline char that POSIX
		// demands. We can ignore that one
		if entry == "" {
			continue
		}

		// Convert all values found by separating
		var convertedValues []int
		values := strings.Split(entry, sep)
		for _, value := range values {
			number, numberErr := strconv.Atoi(value)
			if numberErr != nil {
				fmt.Println(value)
				return rules, fmt.Errorf("Could not convert one of the numbers")
			}
			convertedValues = append(convertedValues, number)
		}

		rules = append(rules, convertedValues)
	}
	return rules, nil
}

func parseRulesIntoMap(rules [][]int) map[int][]int {
	ruleMap := make(map[int][]int)
	for _, rule := range rules {
		ruleMap[rule[0]] = append(ruleMap[rule[0]], rule[1])
	}
	return ruleMap
}

func makeUpdateCompliant(update []int, rules map[int][]int) []int {
	slices.SortFunc(update, func(a int, b int) int {
		before := rules[a]
		if len(before) == 0 {
			return 0
		}

		if slices.Contains(before, b) {
			return -1
		} else {
			return 1
		}
	})

	return update
}

func main() {

	args := os.Args[1:]
	file := args[0]

	data, err := os.ReadFile(file)
	if err != nil {
		fmt.Println("Error while reading input file")
		return
	}

	input := strings.Split(string(data), "\n\n")
	rules, rulesErr := parse(input[0], "|")
	updates, updatesErr := parse(input[1], ",")

	if rulesErr != nil || updatesErr != nil {
		fmt.Println("Unable to parse input")
		return
	}

	countPartOne := 0
	countPartTwo := 0

	rulesMap := parseRulesIntoMap(rules)
	for _, update := range updates {
		clone := slices.Clone(update)
		fixed := makeUpdateCompliant(update, rulesMap)
		if slices.Compare(clone, fixed) == 0 {
			countPartOne += fixed[len(fixed)/2]
		} else {
			countPartTwo += fixed[len(fixed)/2]
		}
	}

	fmt.Println("Answer part one", countPartOne)
	fmt.Println("Answer part two", countPartTwo)

}
