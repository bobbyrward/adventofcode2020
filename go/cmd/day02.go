package cmd

import (
	"github.com/bobbyrward/adventofcode2020/pkg/day02"
	"github.com/spf13/cobra"
)

var day02Cmd = &cobra.Command{
	Use: "day02",
}
var day02part1Cmd = &cobra.Command{
	Use: "part1",
	Run: day02.Part1,
}

var day02part2Cmd = &cobra.Command{
	Use: "part2",
	Run: day02.Part2,
}

func init() {
	rootCmd.AddCommand(day02Cmd)
	day02Cmd.AddCommand(day02part1Cmd)
	day02Cmd.AddCommand(day02part2Cmd)
}
