package day03

import (
	"fmt"
	"io/ioutil"
	"strings"

	"github.com/spf13/cobra"
)

func Part1(cmd *cobra.Command, args []string) {
	input, _ := ioutil.ReadFile("../inputs/day03.txt")

	x := 3
	tree_count := 0

	for _, line := range strings.Split(strings.TrimSpace(string(input)), "\n")[1:] {

		if line[x%len(line)] == '#' {
			tree_count += 1
		}

		x += 3
	}

	fmt.Println(tree_count)
}
