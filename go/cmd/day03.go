package cmd

import (
	"github.com/bobbyrward/adventofcode2020/pkg/day03"
	"github.com/spf13/cobra"
)

var day03Cmd = &cobra.Command{
	Use: "day03",
}
var day03part1Cmd = &cobra.Command{
	Use: "part1",
	Run: day03.Part1,
}

var day03part2Cmd = &cobra.Command{
	Use: "part2",
	Run: day03.Part2,
}

func init() {
	rootCmd.AddCommand(day03Cmd)
	day03Cmd.AddCommand(day03part1Cmd)
	day03Cmd.AddCommand(day03part2Cmd)
}
