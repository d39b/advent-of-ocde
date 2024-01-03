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

func part1(grid []string) {
	fmt.Println(numEnergized(grid, S{0, 0, RIGHT}))
}

func part2(grid []string) {
	m := len(grid)
	n := len(grid[0])

	result := 0
	for r := 0; r < m; r++ {
		result = max(result, numEnergized(grid, S{r, 0, RIGHT}))
		result = max(result, numEnergized(grid, S{r, n - 1, LEFT}))
	}
	for c := 0; c < n; c++ {
		result = max(result, numEnergized(grid, S{0, c, DOWN}))
		result = max(result, numEnergized(grid, S{m - 1, c, UP}))
	}
	fmt.Println(result)
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func numEnergized(grid []string, initial S) int {
	m := len(grid)
	n := len(grid[0])

	isValid := func(r, c int) bool {
		return r >= 0 && c >= 0 && r < m && c < n
	}

	seen := map[S]bool{initial: true}
	q := []S{initial}
	i := 0
	for i < len(q) {
		curr := q[i]
		i += 1
		g := grid[curr.r][curr.c]
		nbs := make([]S, 0)
		switch curr.dir {
		case LEFT:
			if g == '.' || g == '-' {
				nbs = append(nbs, S{curr.r, curr.c - 1, LEFT})
			} else if g == '/' {
				nbs = append(nbs, S{curr.r + 1, curr.c, DOWN})
			} else if g == '\\' {
				nbs = append(nbs, S{curr.r - 1, curr.c, UP})
			} else if g == '|' {
				nbs = append(nbs, S{curr.r - 1, curr.c, UP})
				nbs = append(nbs, S{curr.r + 1, curr.c, DOWN})
			}
		case RIGHT:
			if g == '.' || g == '-' {
				nbs = append(nbs, S{curr.r, curr.c + 1, RIGHT})
			} else if g == '/' {
				nbs = append(nbs, S{curr.r - 1, curr.c, UP})
			} else if g == '\\' {
				nbs = append(nbs, S{curr.r + 1, curr.c, DOWN})
			} else if g == '|' {
				nbs = append(nbs, S{curr.r - 1, curr.c, UP})
				nbs = append(nbs, S{curr.r + 1, curr.c, DOWN})
			}
		case UP:
			if g == '.' || g == '|' {
				nbs = append(nbs, S{curr.r - 1, curr.c, UP})
			} else if g == '/' {
				nbs = append(nbs, S{curr.r, curr.c + 1, RIGHT})
			} else if g == '\\' {
				nbs = append(nbs, S{curr.r, curr.c - 1, LEFT})
			} else if g == '-' {
				nbs = append(nbs, S{curr.r, curr.c - 1, LEFT})
				nbs = append(nbs, S{curr.r, curr.c + 1, RIGHT})
			}
		case DOWN:
			if g == '.' || g == '|' {
				nbs = append(nbs, S{curr.r + 1, curr.c, DOWN})
			} else if g == '/' {
				nbs = append(nbs, S{curr.r, curr.c - 1, LEFT})
			} else if g == '\\' {
				nbs = append(nbs, S{curr.r, curr.c + 1, RIGHT})
			} else if g == '-' {
				nbs = append(nbs, S{curr.r, curr.c - 1, LEFT})
				nbs = append(nbs, S{curr.r, curr.c + 1, RIGHT})
			}
		}
		for _, nb := range nbs {
			if !isValid(nb.r, nb.c) {
				continue
			}
			if _, ok := seen[nb]; !ok {
				seen[nb] = true
				q = append(q, nb)
			}
		}
	}

	tiles := map[[2]int]bool{}
	for _, s := range q {
		t := [2]int{s.r, s.c}
		tiles[t] = true
	}
	return len(tiles)
}

const (
	LEFT  = 0
	RIGHT = 1
	UP    = 2
	DOWN  = 3
)

type S struct {
	r   int
	c   int
	dir int
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
