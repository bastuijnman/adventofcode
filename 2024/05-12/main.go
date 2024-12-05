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

		// Check for pesky added newlines from neovim
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

// Check wether the update contains the rule numbers. Basically
// syntatic sugar so I didn't have to write 2 contains statements
// in an if statement... (what a win I know...)
func updateContainsRuleNumbers(update []int, rule []int) bool {
	for _, ruleNumber := range rule {
		if !slices.Contains(update, ruleNumber) {
			return false
		}
	}
	return true
}

// Check if an update complies with the rules
func isUpdateValidForRules(update []int, rules [][]int) bool {

	for _, rule := range rules {

		// Skip if the rule does not apply
		if !updateContainsRuleNumbers(update, rule) {
			continue
		}

		if slices.Index(update, rule[0]) > slices.Index(update, rule[1]) {
			return false
		}
	}
	return true
}

// Take an update and rules and make them compliant.
func makeUpdateCompliant(update []int, rules [][]int) []int {
	for i := 0; i < len(update); i++ {

		// By default we assume the numbers are not found in the rules
		lowest := -1
		value := update[i]

		// Loop over all rules, if we find the
		for _, rule := range rules {

			// Ignore rules that don't apply anyway
			if rule[0] != update[i] {
				continue
			}

			// Find the index of the second part of the rule
			// if found and lower than the current lowest we
			// treat it as the new lowest index
			l := slices.Index(update, rule[1])
			if l != -1 && (lowest == -1 || l < lowest) {
				lowest = l
			}
		}

		// If we have a new lowest index move the current value
		// into that index instead, making it comply with the
		// processed rule
		if lowest != -1 && lowest < i {
			update = slices.Delete(update, i, i+1)
			update = slices.Insert(update, lowest, value)
		}

	}

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
	for _, update := range updates {
		if isUpdateValidForRules(update, rules) {
			countPartOne += update[len(update)/2]
		} else {
			fixedUpdate := makeUpdateCompliant(update, rules)
			countPartTwo += fixedUpdate[len(fixedUpdate)/2]
		}
	}

	fmt.Println("Answer part one", countPartOne)
	fmt.Println("Answer part two", countPartTwo)

}
