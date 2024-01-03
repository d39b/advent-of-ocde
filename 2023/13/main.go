package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	lines := readLines()
	patterns := splitInput(lines)
	part1(patterns)
	part2(patterns)
}

func part1(patterns [][]string) {
	sum := 0
	for _, pattern := range patterns {
		sum += findReflection(pattern)
	}
	fmt.Println(sum)
}

func findReflection(pattern []string) int {
	m, n := len(pattern), len(pattern[0])
	// try vertical
	// can set the reflection line after index 0, 1, ... n-2
	for v := 0; v < n-1; v++ {
		valid := true
		left, right := v, v+1
		for left >= 0 && right < n {
			for r := 0; r < m; r++ {
				if pattern[r][left] != pattern[r][right] {
					valid = false
					break
				}
			}

			if !valid {
				break
			}

			left -= 1
			right += 1
		}
		if valid {
			return v + 1
		}
	}

	// try horizontal
	for v := 0; v < m-1; v++ {
		valid := true
		left, right := v, v+1
		for left >= 0 && right < m {
			for c := 0; c < n; c++ {
				if pattern[left][c] != pattern[right][c] {
					valid = false
					break
				}
			}

			if !valid {
				break
			}

			left -= 1
			right += 1
		}
		if valid {
			return (100) * (v + 1)
		}
	}

	return 0
}

// same as before but we want to find a reflection line with exactly one error
func part2(patterns [][]string) {
	sum := 0
	for _, pattern := range patterns {
		sum += findReflectionWithSmudge(pattern)
	}
	fmt.Println(sum)
}

func findReflectionWithSmudge(pattern []string) int {
	m, n := len(pattern), len(pattern[0])
	// try vertical
	// can set the reflection line after index 0, 1, ... n-2
	for v := 0; v < n-1; v++ {
		errors := 0
		left, right := v, v+1
		for left >= 0 && right < n {
			for r := 0; r < m; r++ {
				if pattern[r][left] != pattern[r][right] {
					errors += 1
					if errors > 1 {
						break
					}
				}
			}

			if errors > 1 {
				break
			}

			left -= 1
			right += 1
		}
		if errors == 1 {
			return v + 1
		}
	}

	// try horizontal
	for v := 0; v < m-1; v++ {
		errors := 0
		left, right := v, v+1
		for left >= 0 && right < m {
			for c := 0; c < n; c++ {
				if pattern[left][c] != pattern[right][c] {
					errors += 1
					if errors > 1 {
						break
					}
				}
			}

			if errors > 1 {
				break
			}

			left -= 1
			right += 1
		}
		if errors == 1 {
			return (100) * (v + 1)
		}
	}

	return 0
}

func splitInput(lines []string) [][]string {
	start := 0
	result := make([][]string, 0)
	for i, line := range lines {
		if line == "" {
			result = append(result, lines[start:i])
			start = i + 1
		}
	}
	if start < len(lines)-1 {
		result = append(result, lines[start:])
	}
	return result
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
