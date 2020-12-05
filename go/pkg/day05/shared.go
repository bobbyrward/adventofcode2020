package day05

import (
	"strconv"
	"strings"
)

func getBoardingId(boarding_pass string) int {
	replaced := strings.ReplaceAll(
		strings.ReplaceAll(
			strings.ReplaceAll(
				strings.ReplaceAll(boarding_pass, "F", "0"),
				"L", "0"),
			"B", "1"),
		"R", "1")

	boarding_id, _ := strconv.ParseUint(replaced, 2, 32)

	return int(boarding_id)
}
