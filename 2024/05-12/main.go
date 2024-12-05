package main

import (
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

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

func updateContainsRuleNumbers(update []int, rule []int) bool {
	for _, ruleNumber := range rule {
		if !slices.Contains(update, ruleNumber) {
			return false
		}
	}
	return true
}

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

	count := 0
	for _, update := range updates {
		if isUpdateValidForRules(update, rules) {
			count += update[len(update)/2]
		}
	}

	fmt.Println("Answer part one", count)

}
