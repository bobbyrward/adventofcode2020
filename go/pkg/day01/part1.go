package day01

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"

	"github.com/spf13/cobra"
)

func Part1(cmd *cobra.Command, args []string) {
	input, _ := ioutil.ReadFile("../inputs/day01.txt")

	expenses := []int{}

	for _, expense_str := range strings.Fields(string(input)) {
		expense, _ := strconv.Atoi(expense_str)
		expenses = append(expenses, expense)
	}

	for i, a := range expenses {
		for _, b := range expenses[i+1:] {
			if a+b == 2020 {
				fmt.Println(a * b)
				return
			}
		}
	}
}
