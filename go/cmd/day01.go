package cmd

import (
	"github.com/bobbyrward/adventofcode2020/pkg/day01"
	"github.com/spf13/cobra"
)

var day01Cmd = &cobra.Command{
	Use: "day01",
}
var part1Cmd = &cobra.Command{
	Use: "part1",
	Run: day01.Part1,
}

var part2Cmd = &cobra.Command{
	Use: "part2",
	Run: day01.Part2,
}

func init() {
	rootCmd.AddCommand(day01Cmd)
	day01Cmd.AddCommand(part1Cmd)
	day01Cmd.AddCommand(part2Cmd)
}
