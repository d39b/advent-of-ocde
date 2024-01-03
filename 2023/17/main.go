package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	lines := readLines()
	grid := parseGrid(lines)
	part1(grid)
	part2(grid)
}

func part1(grid [][]int) {
	m := len(grid)
	n := len(grid[0])

	isValid := func(r, c int) bool {
		return r >= 0 && c >= 0 && r < m && c < n
	}

	// we can think of this as a classic shortest path grid search
	// but we have to take 1 to 3 horizontal steps, then 1 to 3 vertical ones and so on
	// h[r][c] = min heat loss to reach cell (r, c) where next step is horizontal
	// v[r][c] = min heat loss to reach cell (r, c) where next step is vertical
	h := make([][]int, m)
	v := make([][]int, m)
	for r := 0; r < m; r++ {
		h[r] = make([]int, n)
		v[r] = make([]int, n)
	}

	q := []E{{0, 0, true}, {0, 0, false}}
	i := 0
	for i < len(q) {
		r, c, horz := q[i].r, q[i].c, q[i].horz
		i += 1
		if horz {
			d := h[r][c]
			hl := 0
			// move right
			for j := 1; j <= 3; j++ {
				nr, nc := r, c+j
				if !isValid(nr, nc) {
					break
				}
				hl += grid[nr][nc]
				z := d + hl
				if v[nr][nc] == 0 || z < v[nr][nc] {
					v[nr][nc] = z
					q = append(q, E{nr, nc, false})
				}
			}
			hl = 0
			// move left
			for j := 1; j <= 3; j++ {
				nr, nc := r, c-j
				if !isValid(nr, nc) {
					break
				}
				hl += grid[nr][nc]
				z := d + hl
				if v[nr][nc] == 0 || z < v[nr][nc] {
					v[nr][nc] = z
					q = append(q, E{nr, nc, false})
				}
			}
		} else {
			d := v[r][c]
			hl := 0
			// move down
			for j := 1; j <= 3; j++ {
				nr, nc := r+j, c
				if !isValid(nr, nc) {
					break
				}
				hl += grid[nr][nc]
				z := d + hl
				if h[nr][nc] == 0 || z < h[nr][nc] {
					h[nr][nc] = z
					q = append(q, E{nr, nc, true})
				}
			}
			hl = 0
			// move up
			for j := 1; j <= 3; j++ {
				nr, nc := r-j, c
				if !isValid(nr, nc) {
					break
				}
				hl += grid[nr][nc]
				z := d + hl
				if h[nr][nc] == 0 || z < h[nr][nc] {
					h[nr][nc] = z
					q = append(q, E{nr, nc, true})
				}
			}
		}
	}

	fmt.Println(h[m-1][n-1], v[m-1][n-1])
}

// basically the same algorithm as part1, just need to up the steps to 10 and keep the minimum of 4 in mind
func part2(grid [][]int) {
	m := len(grid)
	n := len(grid[0])

	isValid := func(r, c int) bool {
		return r >= 0 && c >= 0 && r < m && c < n
	}

	// we can think of this as a classic shortest path grid search
	// but we have to take 1 to 3 horizontal steps, then 1 to 3 vertical ones and so on
	// h[r][c] = min heat loss to reach cell (r, c) where next step is horizontal
	// v[r][c] = min heat loss to reach cell (r, c) where next step is vertical
	h := make([][]int, m)
	v := make([][]int, m)
	for r := 0; r < m; r++ {
		h[r] = make([]int, n)
		v[r] = make([]int, n)
	}

	q := []E{{0, 0, true}, {0, 0, false}}
	i := 0
	for i < len(q) {
		r, c, horz := q[i].r, q[i].c, q[i].horz
		i += 1
		if horz {
			d := h[r][c]
			if isValid(r, c+4) {
				hl := grid[r][c+1] + grid[r][c+2] + grid[r][c+3]
				// move right
				for j := 4; j <= 10; j++ {
					nr, nc := r, c+j
					if !isValid(nr, nc) {
						break
					}
					hl += grid[nr][nc]
					z := d + hl
					if v[nr][nc] == 0 || z < v[nr][nc] {
						v[nr][nc] = z
						q = append(q, E{nr, nc, false})
					}
				}

			}
			if isValid(r, c-4) {
				hl := grid[r][c-1] + grid[r][c-2] + grid[r][c-3]
				// move left
				for j := 4; j <= 10; j++ {
					nr, nc := r, c-j
					if !isValid(nr, nc) {
						break
					}
					hl += grid[nr][nc]
					z := d + hl
					if v[nr][nc] == 0 || z < v[nr][nc] {
						v[nr][nc] = z
						q = append(q, E{nr, nc, false})
					}
				}
			}
		} else {
			d := v[r][c]
			if isValid(r+4, c) {
				hl := grid[r+1][c] + grid[r+2][c] + grid[r+3][c]
				// move down
				for j := 4; j <= 10; j++ {
					nr, nc := r+j, c
					if !isValid(nr, nc) {
						break
					}
					hl += grid[nr][nc]
					z := d + hl
					if h[nr][nc] == 0 || z < h[nr][nc] {
						h[nr][nc] = z
						q = append(q, E{nr, nc, true})
					}
				}
			}
			if isValid(r-4, c) {
				hl := grid[r-1][c] + grid[r-2][c] + grid[r-3][c]
				// move up
				for j := 4; j <= 10; j++ {
					nr, nc := r-j, c
					if !isValid(nr, nc) {
						break
					}
					hl += grid[nr][nc]
					z := d + hl
					if h[nr][nc] == 0 || z < h[nr][nc] {
						h[nr][nc] = z
						q = append(q, E{nr, nc, true})
					}
				}

			}
		}
	}

	fmt.Println(h[m-1][n-1], v[m-1][n-1])
}

type E struct {
	r int
	c int
	// next move is horz
	horz bool
}

func parseGrid(lines []string) [][]int {
	m := len(lines)
	n := len(lines[0])

	result := make([][]int, m)
	for r := 0; r < m; r++ {
		result[r] = make([]int, n)
		for c := 0; c < n; c++ {
			result[r][c] = int(lines[r][c] - '0')
		}
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
