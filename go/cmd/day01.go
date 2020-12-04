package cmd

import (
	"github.com/bobbyrward/adventofcode2020/pkg/day01"
	"github.com/spf13/cobra"
)

var day01Cmd = &cobra.Command{
	Use: "day01",
}
var day01part1Cmd = &cobra.Command{
	Use: "part1",
	Run: day01.Part1,
}

var day01part2Cmd = &cobra.Command{
	Use: "part2",
	Run: day01.Part2,
}

func init() {
	rootCmd.AddCommand(day01Cmd)
	day01Cmd.AddCommand(day01part1Cmd)
	day01Cmd.AddCommand(day01part2Cmd)
}
