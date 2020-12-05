package day04

import (
	"fmt"
	"io/ioutil"
	"strings"

	"github.com/spf13/cobra"
)

func Part1(cmd *cobra.Command, args []string) {
	input, _ := ioutil.ReadFile("../inputs/day04.txt")

	required_fields := []string{"byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"}
	count := 0

	for _, line := range strings.Split(strings.TrimSpace(string(input)), "\n\n") {
		if NewPassport(line).HasAllFields(required_fields) {
			count += 1
		}
	}

	fmt.Println(count)
}
