package cmd

import (
	"github.com/bobbyrward/adventofcode2020/pkg/day04"
	"github.com/spf13/cobra"
)

var day04Cmd = &cobra.Command{
	Use: "day04",
}
var day04part1Cmd = &cobra.Command{
	Use: "part1",
	Run: day04.Part1,
}

var day04part2Cmd = &cobra.Command{
	Use: "part2",
	Run: day04.Part2,
}

func init() {
	rootCmd.AddCommand(day04Cmd)
	day04Cmd.AddCommand(day04part1Cmd)
	day04Cmd.AddCommand(day04part2Cmd)
}
