package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	lines := readLines()
	times, distances := parseInput(lines)
	part1(times, distances)
	part2(times, distances)
}

func part1(times, distances []int) {
	n := len(times)
	result := 1
	for i := 0; i < n; i++ {
		time, recordDistance := times[i], distances[i]
		count := 0
		for chargeDuration := 1; chargeDuration < time; chargeDuration++ {
			if chargeDuration*(time-chargeDuration) > recordDistance {
				count += 1
			}
		}
		result *= count
	}
	fmt.Println(result)
}

// numbers are small enough to just brute-force check
// all the possible durations to charge the boat in both part 1 and 2 of the problem
// for a faster solution we could use binary search
// since the charging durations that beat the record are probably all
// in a contiguous interval [x, y], so we would just have to find the start and end point
func part2(times, distances []int) {
	t, d := combineNums(times), combineNums(distances)
	part1([]int{t}, []int{d})
}

func combineNums(x []int) int {
	result := 0
	for _, v := range x {
		e := 1
		for e*10 < v {
			e *= 10
		}
		for e > 0 {
			result = result*10 + (v / e)
			v = v % e
			e = e / 10
		}
	}
	return result
}

func parseInput(lines []string) ([]int, []int) {
	_, x, _ := strings.Cut(lines[0], ":")
	times := parseIntList(x)
	_, x, _ = strings.Cut(lines[1], ":")
	distances := parseIntList(x)
	return times, distances
}

func readLines() []string {
	sc := bufio.NewScanner(os.Stdin)
	lines := make([]string, 0)
	for sc.Scan() {
		lines = append(lines, string(sc.Bytes()))
	}
	if err := sc.Err(); err != nil {
		panic(err)
	}
	return lines
}

func parseIntList(s string) []int {
	result := make([]int, 0)
	for _, x := range strings.Fields(strings.TrimSpace(s)) {
		n, err := strconv.Atoi(x)
		if err != nil {
			panic(err)
		}
		result = append(result, n)
	}
	return result
}
