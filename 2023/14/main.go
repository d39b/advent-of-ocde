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
	m := len(grid)
	n := len(grid[0])

	g := make([][]int, m)
	for r := 0; r < m; r++ {
		g[r] = make([]int, n)
		for c := 0; c < n; c++ {
			if grid[r][c] == '#' {
				g[r][c] = 1
			} else if grid[r][c] == 'O' {
				g[r][c] = 2
			}
		}
	}
	tiltNorth(g, m, n)
	fmt.Println(computeLoad(g, m, n))
}

type Hasher struct {
	pow []int
	m   int
	n   int
	p   int
	mod int
}

func NewHasher(m, n, p, mod int) Hasher {
	pow := []int{1}
	for i := 1; i <= m*n; i++ {
		pow = append(pow, (pow[i-1]*p)%mod)
	}
	return Hasher{
		pow: pow,
		m:   m,
		n:   n,
		p:   p,
		mod: mod,
	}
}

func (h Hasher) Hash(grid [][]int) int {
	hash := 0
	next := 0
	for r := 0; r < h.m; r++ {
		for c := 0; c < h.n; c++ {
			if grid[r][c] == 2 {
				i := r*h.n + c
				hash = (hash + h.pow[next]*i) % h.mod
				next += 1
			}
		}
	}
	return hash
}

func cycle(grid [][]int, m, n int) {
	tiltNorth(grid, m, n)
	tiltWest(grid, m, n)
	tiltSouth(grid, m, n)
	tiltEast(grid, m, n)
}

func tiltNorth(grid [][]int, m, n int) {
	for c := 0; c < n; c++ {
		nextFree := 0
		for r := 0; r < m; r++ {
			if grid[r][c] == 1 {
				nextFree = r + 1
			} else if grid[r][c] == 2 {
				grid[r][c] = 0
				grid[nextFree][c] = 2
				nextFree += 1
			}
		}
	}
}

func tiltSouth(grid [][]int, m, n int) {
	for c := 0; c < n; c++ {
		nextFree := m - 1
		for r := m - 1; r >= 0; r-- {
			if grid[r][c] == 1 {
				nextFree = r - 1
			} else if grid[r][c] == 2 {
				grid[r][c] = 0
				grid[nextFree][c] = 2
				nextFree -= 1
			}
		}
	}
}

func tiltWest(grid [][]int, m, n int) {
	for r := 0; r < m; r++ {
		nextFree := 0
		for c := 0; c < n; c++ {
			if grid[r][c] == 1 {
				nextFree = c + 1
			} else if grid[r][c] == 2 {
				grid[r][c] = 0
				grid[r][nextFree] = 2
				nextFree += 1
			}
		}
	}
}

func tiltEast(grid [][]int, m, n int) {
	for r := 0; r < m; r++ {
		nextFree := n - 1
		for c := n - 1; c >= 0; c-- {
			if grid[r][c] == 1 {
				nextFree = c - 1
			} else if grid[r][c] == 2 {
				grid[r][c] = 0
				grid[r][nextFree] = 2
				nextFree -= 1
			}
		}
	}
}

func computeLoad(grid [][]int, m, n int) int {
	result := 0
	for r := 0; r < m; r++ {
		for c := 0; c < n; c++ {
			if grid[r][c] == 2 {
				result += m - r
			}
		}
	}
	return result
}

// run 1B cycles
// there needs to be a faster way than simulation
// eventually the states will repeat, so we just have to find a cycle
func part2(grid []string) {
	m := len(grid)
	n := len(grid[0])
	g := make([][]int, m)
	for r := 0; r < m; r++ {
		g[r] = make([]int, n)
		for c := 0; c < n; c++ {
			if grid[r][c] == '#' {
				g[r][c] = 1
			} else if grid[r][c] == 'O' {
				g[r][c] = 2
			}
		}
	}

	hasher := NewHasher(m, n, 111111, 1000000007)
	seen := map[int]int{hasher.Hash(g): 0}
	for i := 0; i < 1000000000; i++ {
		cycle(g, m, n)
		hash := hasher.Hash(g)
		if v, ok := seen[hash]; ok {
			// cycle found
			// so after v cycles is the same as after i+1 cycles
			// so we have a cycle length of l=i+1-v
			// we want the result after 1B steps
			// we have 1B = v + k*l + z
			// 1B - v = k*l + z
			// z = (1B-v)%l
			// search for a step that is equal to z modulo cycleLength
			cycleLength := i + 1 - v
			z := (1000000000 - v) % cycleLength
			for j := 0; j < cycleLength; j++ {
				if (i+1+j-v)%cycleLength == z {
					fmt.Println(computeLoad(g, m, n))
					return
				}
				cycle(g, m, n)
			}

			fmt.Println(i, v)
			break
		} else {
			seen[hash] = i + 1
		}
	}
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
