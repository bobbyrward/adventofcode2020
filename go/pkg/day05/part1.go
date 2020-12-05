package day05

import (
	"fmt"
	"io/ioutil"
	"strings"

	"github.com/spf13/cobra"
)

func Part1(cmd *cobra.Command, args []string) {
	input, _ := ioutil.ReadFile("../inputs/day05.txt")

	max := 0

	for _, line := range strings.Split(strings.TrimSpace(string(input)), "\n") {
		boarding_id := getBoardingId(line)

		if boarding_id > max {
			max = boarding_id
		}
	}

	fmt.Println(max)
}
