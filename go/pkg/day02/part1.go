package day02

import (
	"fmt"
	"io/ioutil"
	"strings"

	"github.com/spf13/cobra"
)

func Part1(cmd *cobra.Command, args []string) {
	input, _ := ioutil.ReadFile("../inputs/day02.txt")
	total_count := 0

	for _, line := range strings.Split(strings.TrimSpace(string(input)), "\n") {
		var min int
		var max int
		var what byte
		var password []byte

		fmt.Sscanf(line, "%v-%v %c: %v", &min, &max, &what, &password)

		what_count := 0

		for _, c := range password {
			if c == what {
				what_count += 1
			}
		}

		if what_count >= min && what_count <= max {
			total_count += 1
		}
	}

	fmt.Println(total_count)
}
