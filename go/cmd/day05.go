package cmd

import (
	"github.com/bobbyrward/adventofcode2020/pkg/day05"
	"github.com/spf13/cobra"
)

var day05Cmd = &cobra.Command{
	Use: "day05",
}
var day05part1Cmd = &cobra.Command{
	Use: "part1",
	Run: day05.Part1,
}

var day05part2Cmd = &cobra.Command{
	Use: "part2",
	Run: day05.Part2,
}

func init() {
	rootCmd.AddCommand(day05Cmd)
	day05Cmd.AddCommand(day05part1Cmd)
	day05Cmd.AddCommand(day05part2Cmd)
}
