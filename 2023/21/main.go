package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	lines := readLines()
	// part1(lines, 64)
	part2(lines, 1000, 131)
}

// used a hint for this
// 26501365 = v*131 + 65  where z=131 is the width/height of the (quadratic) grid
// the number of cells that can be visited in x, x+z, x+2*z, x+3*z, ...
// forms a quadratic function
// so we calculate the number of steps for 65, 65+131, 65+2*131, ... and so on
// can then fit a quadratic func which yields f(x) = 15286x^2 + 15394x + 3884
// the solution is then f(v) where v=202300
// Note: we could calculate the number of plots more efficiently for a given number of steps by noticing
// that if we can visit a cell in x steps, then also x+2, x+4, x+6, ...
// and it is not possible to reach a cell in both x and x+1 steps
// consider dr, dc = startR - r, startC - c we need to eventually do dr, dc steps
// in towards (r, c), and for every step in the wrong direction we will need to take one back
// so the parity will always be the same as for dr+dc
func part2(grid []string, k int, step int) {
	m := len(grid)
	n := len(grid[0])

	sr, sc := 0, 0
outer:
	for r := 0; r < m; r++ {
		for c := 0; c < n; c++ {
			if grid[r][c] == 'S' {
				sr, sc = r, c
				break outer
			}
		}
	}

	isFree := func(r, c int) bool {
		r = ((r % m) + m) % m
		c = ((c % n) + n) % n
		return grid[r][c] != '#'
	}

	q := [][]int{{sr, sc}}
	for s := 1; s <= k; s++ {
		next := [][]int{}
		seen := map[[2]int]bool{}
		for _, x := range q {
			r, c := x[0], x[1]
			for _, nb := range [][]int{{r + 1, c}, {r - 1, c}, {r, c + 1}, {r, c - 1}} {
				nr, nc := nb[0], nb[1]
				if isFree(nr, nc) && !seen[[2]int{nr, nc}] {
					next = append(next, nb)
					seen[[2]int{nr, nc}] = true
				}
			}
		}
		if s >= 65 && (s-65)%step == 0 {
			fmt.Println(s, len(next))
		}
		q = next
	}
}

func part1(grid []string, k int) {
	m := len(grid)
	n := len(grid[0])

	d := make([][]int, m)
	for r := 0; r < m; r++ {
		d[r] = make([]int, n)
	}

	sr, sc := 0, 0
outer:
	for r := 0; r < m; r++ {
		for c := 0; c < n; c++ {
			if grid[r][c] == 'S' {
				sr, sc = r, c
				break outer
			}
		}
	}

	isValid := func(r, c int) bool {
		return r >= 0 && c >= 0 && r < m && c < m && grid[r][c] != '#'
	}

	q := [][]int{{sr, sc}}
	for s := 1; s <= k; s++ {
		next := [][]int{}
		for _, x := range q {
			r, c := x[0], x[1]
			for _, nb := range [][]int{{r + 1, c}, {r - 1, c}, {r, c + 1}, {r, c - 1}} {
				nr, nc := nb[0], nb[1]
				if isValid(nr, nc) && d[nr][nc] != s {
					d[nr][nc] = s
					next = append(next, nb)
				}
			}
		}
		q = next
	}

	fmt.Println(len(q))
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
