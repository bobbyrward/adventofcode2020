package day01

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"

	"github.com/spf13/cobra"
)

func Part2(cmd *cobra.Command, args []string) {
	input, _ := ioutil.ReadFile("../inputs/day01.txt")

	expenses := []int{}

	for _, expense_str := range strings.Fields(string(input)) {
		expense, _ := strconv.Atoi(expense_str)
		expenses = append(expenses, expense)
	}

	for i, a := range expenses {
		for j, b := range expenses[i+1:] {
			for _, c := range expenses[j+1:] {
				if a+b+c == 2020 {
					fmt.Println(a * b * c)
					return
				}
			}
		}
	}
}
