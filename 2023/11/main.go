package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	lines := readLines()
	// can use the same code for part 1 and 2 if we pass in an expansion multiplier
	part1(lines, 2)
	part1(lines, 1000000)
}

// The shortest path between to galaxies is the Manhattan distance, so easy to calculate.
// We just need to keep track of the empty rows and columns between two galaxies and count those twice.
func part1(grid []string, expansionMultiplier int) {
	m := len(grid)
	n := len(grid[0])

	galaxies := make([][]int, 0)
	// count the number of galaxies in each row and column to find the empty ones
	rows := make([]int, m)
	cols := make([]int, n)

	for r := 0; r < m; r++ {
		for c := 0; c < n; c++ {
			if grid[r][c] == '#' {
				galaxies = append(galaxies, []int{r, c})
				rows[r] += 1
				cols[c] += 1
			}
		}
	}

	// create a prefix sum array for empty rows and cols
	// rows[r] = k means among the rows 0....r there are k empty ones
	// using these we can easily find the number of empty rows/cols in a certain range
	for r := 0; r < m; r++ {
		v := 0
		if rows[r] == 0 {
			v += 1
		}
		if r > 0 {
			v += rows[r-1]
		}
		rows[r] = v
	}
	for c := 0; c < n; c++ {
		v := 0
		if cols[c] == 0 {
			v += 1
		}
		if c > 0 {
			v += cols[c-1]
		}
		cols[c] = v
	}

	getInRange := func(x []int, a, b int) int {
		if b < a {
			return 0
		}
		count := x[b]
		if a > 0 {
			count -= x[a-1]
		}
		return count
	}

	distSum := 0
	for i := 0; i < len(galaxies)-1; i++ {
		r1, c1 := galaxies[i][0], galaxies[i][1]
		for j := i + 1; j < len(galaxies); j++ {
			r2, c2 := galaxies[j][0], galaxies[j][1]
			if r1 <= r2 {
				distSum += r2 - r1 + (expansionMultiplier-1)*getInRange(rows, r1+1, r2-1)
			} else {
				distSum += r1 - r2 + (expansionMultiplier-1)*getInRange(rows, r2+1, r1-1)
			}
			if c1 <= c2 {
				distSum += c2 - c1 + (expansionMultiplier-1)*getInRange(cols, c1+1, c2-1)
			} else {
				distSum += c1 - c2 + (expansionMultiplier-1)*getInRange(cols, c2+1, c1-1)
			}

		}
	}
	fmt.Println(distSum)
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
