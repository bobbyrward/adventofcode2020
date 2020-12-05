package day05

import (
	"fmt"
	"io/ioutil"
	"strings"

	"github.com/spf13/cobra"
)

func Part2(cmd *cobra.Command, args []string) {
	input, _ := ioutil.ReadFile("../inputs/day05.txt")

	seats := make([]bool, 128*8)

	for _, line := range strings.Split(strings.TrimSpace(string(input)), "\n") {
		seats[getBoardingId(line)] = true
	}

	idx := 0

	for ; !seats[idx]; idx++ {
	}
	for ; seats[idx]; idx++ {
	}

	row := idx / 8
	seat := idx % 8

	fmt.Println(row*8 + seat)
}
