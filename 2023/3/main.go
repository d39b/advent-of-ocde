package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	lines := readLines()
	part1_and_2(lines)
}

func part1_and_2(engine []string) {
	m := len(engine)
	n := len(engine[0])

	grid := make([][]int, m)
	for i := 0; i < m; i++ {
		grid[i] = make([]int, n)
	}
	nums := make([]int, 1)
	nextIndex := 1
	// first parse all the numbers
	for r := 0; r < m; r++ {
		curr := -1
		for c := 0; c < n; c++ {
			if isDigit(engine[r][c]) {
				if curr == -1 {
					curr = int(engine[r][c] - '0')
				} else {
					curr = curr*10 + int(engine[r][c]-'0')
				}
				grid[r][c] = nextIndex
			} else {
				if curr != -1 {
					nums = append(nums, curr)
					curr = -1
					nextIndex += 1
				}
			}
		}
		if curr != -1 {
			nums = append(nums, curr)
			nextIndex += 1
		}
	}

	// part 1
	engineParts := make(map[int]bool)
	for r := 0; r < m; r++ {
		for c := 0; c < n; c++ {
			if !isDigit(engine[r][c]) && engine[r][c] != '.' {
				for dr := -1; dr <= 1; dr++ {
					nr := r + dr
					if nr < 0 || nr >= m {
						continue
					}
					for dc := -1; dc <= 1; dc++ {
						nc := c + dc
						if nc < 0 || nc >= n {
							continue
						}
						if grid[nr][nc] > 0 {
							engineParts[grid[nr][nc]] = true
						}
					}
				}
			}
		}
	}

	sum := 0
	for part := range engineParts {
		sum += nums[part]
	}

	fmt.Println(sum)

	// part 2
	gearRatioSum := 0

	for r := 0; r < m; r++ {
		for c := 0; c < n; c++ {
			if engine[r][c] == '*' {
				nbs := make(map[int]bool)
				for dr := -1; dr <= 1; dr++ {
					nr := r + dr
					if nr < 0 || nr >= m {
						continue
					}
					for dc := -1; dc <= 1; dc++ {
						nc := c + dc
						if nc < 0 || nc >= n {
							continue
						}
						if grid[nr][nc] > 0 {
							nbs[grid[nr][nc]] = true
						}
					}
				}
				// this is a gear
				if len(nbs) == 2 {
					ratio := 1
					for v := range nbs {
						ratio *= nums[v]
					}
					gearRatioSum += ratio
				}
			}
		}
	}

	fmt.Println(gearRatioSum)
}

func isDigit(b byte) bool {
	return '0' <= b && b <= '9'
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
