package day03

import (
	"fmt"
	"io/ioutil"
	"strings"

	"github.com/spf13/cobra"
)

func countTreesOnPath(lines []string, right int, down int) int {
	tree_count := 0
	x := right

	for y, line := range lines[down:] {
		if y%down != 0 {
			continue
		}

		if line[x%len(line)] == '#' {
			tree_count += 1
		}

		x += right
	}

	return tree_count
}

type Slope struct {
	x int
	y int
}

func Part2(cmd *cobra.Command, args []string) {
	input, _ := ioutil.ReadFile("../inputs/day03.txt")

	lines := strings.Split(strings.TrimSpace(string(input)), "\n")
	slopes := []Slope{
		{1, 1},
		{3, 1},
		{5, 1},
		{7, 1},
		{1, 2},
	}

	tree_count := 1

	for _, slope := range slopes {
		tree_count *= countTreesOnPath(lines, slope.x, slope.y)
	}

	fmt.Println(tree_count)

}
