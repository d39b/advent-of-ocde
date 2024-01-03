package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	lines := readLines()
	part1(lines)
	part2(lines)
}

func part1(lines []string) {
	sum := 0
	for _, line := range lines {
		// first digit
		for i := 0; i < len(line); i++ {
			if d, ok := isDigit(line[i]); ok {
				// in two-digit number first digit will contribute 10x value
				sum += d * 10
				break
			}
		}
		// last digit
		for i := len(line) - 1; i >= 0; i-- {
			if d, ok := isDigit(line[i]); ok {
				sum += d
				break
			}
		}
	}
	fmt.Println(sum)
}

// digits might be hidden as words "one", "two", ... in a line
func part2(lines []string) {
	sum := 0
	for _, line := range lines {
		c := 0
		// first digit
		for i := 0; i < len(line); i++ {
			if d, ok := isDigit(line[i]); ok {
				// in two-digit number first digit will contribute 10x value
				c += d * 10
				break
			} else if d, ok := isDigitWord(line, i, true); ok {
				c += d * 10
				break
			}
		}
		// last digit
		for i := len(line) - 1; i >= 0; i-- {
			if d, ok := isDigit(line[i]); ok {
				c += d
				break
			} else if d, ok := isDigitWord(line, i, false); ok {
				c += d
				break
			}
		}
		sum += c
	}
	fmt.Println(sum)
}

var digitWords = []string{"one", "two", "three", "four", "five", "six", "seven", "eight", "nine"}

func isDigitWord(s string, i int, forward bool) (int, bool) {
	for j, w := range digitWords {
		var start, end int
		if forward {
			start = i
			end = i + len(w) - 1
		} else {
			end = i
			start = i - len(w) + 1
		}
		if start < 0 || end >= len(s) {
			continue
		}
		if s[start:end+1] == w {
			return j + 1, true
		}
	}
	return 0, false
}

func isDigit(b byte) (int, bool) {
	if '0' <= b && b <= '9' {
		return int(b - '0'), true
	}
	return 0, false
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
