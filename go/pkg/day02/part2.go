package day02

import (
	"fmt"
	"io/ioutil"
	"strings"

	"github.com/spf13/cobra"
)

func Part2(cmd *cobra.Command, args []string) {
	input, _ := ioutil.ReadFile("../inputs/day02.txt")
	total_count := 0

	for _, line := range strings.Split(strings.TrimSpace(string(input)), "\n") {
		var pos1 int
		var pos2 int
		var what byte
		var password []byte

		fmt.Sscanf(line, "%v-%v %c: %v", &pos1, &pos2, &what, &password)

		if (password[pos1-1] == what) != (password[pos2-1] == what) {
			total_count += 1
		}
	}

	fmt.Println(total_count)
}
