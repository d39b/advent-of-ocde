package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	lines := readLines()
	graph := parseGraph(lines)
	part1(graph)
	part2(graph)
}

/*
We can imagine adding extra cells between each of the original ones.
That is a cell (r, c) is then (2r, 2c).
Consider two neighboring cells on the cycle e.g. (r, c) (r+1, c)
those will be (2r, 2c) (2r+2, 2c), we will also consider (2r+1, 2c) as part of the cycle.
Initially all cells on the cycle are considered inside, all cells on the outside edge of the grid
(that are not already on the cycle) will be considered outside.
If a cell has a neighbor that is outside, it is outside as well.
Repeat until all the outside cells have been found, the result will be the number of outside cells
that are also in the original grid, i.e. those where both row and column number are even.
*/
func part2(g Graph) {
	// First find all the cells in the cycle.
	var cycle []int
	for _, nb := range g.adj[g.start] {
		hasStartNb := false
		for _, x := range g.adj[nb] {
			if x == g.start {
				hasStartNb = true
				break
			}
		}
		if !hasStartNb {
			continue
		}
		cycle = []int{g.start}
		visited := make([]bool, g.nNodes)
		visited[g.start] = true
		visited[nb] = true
		prev := g.start
		curr := nb
		found := false
	Outer:
		for {
			next := curr
			cycle = append(cycle, curr)
			for _, x := range g.adj[curr] {
				if !visited[x] {
					visited[x] = true
					next = x
				} else {
					if x != prev {
						if x == g.start {
							found = true
						}
						break Outer
					}
				}
			}
			if next == curr {
				break
			}
			prev = curr
			curr = next
		}

		if found {
			break
		}
	}

	m, n := g.m, g.n
	mm, nn := 2*m-1, 2*n-1
	grid := make([][]int, mm)
	for r := 0; r < mm; r++ {
		grid[r] = make([]int, nn)
	}

	for i, x := range cycle {
		r, c := x/n, x%n
		var nr, nc int
		if i+1 < len(cycle) {
			nr, nc = cycle[i+1]/n, cycle[i+1]%n
		} else {
			nr, nc = cycle[0]/n, cycle[0]%n
		}
		grid[2*r][2*c] = 1
		if nr != r {
			if r < nr {
				grid[2*r+1][2*c] = 1
			} else {
				grid[2*nr+1][2*c] = 1
			}
		} else {
			if c < nc {
				grid[2*r][2*c+1] = 1
			} else {
				grid[2*r][2*nc+1] = 1
			}
		}
	}

	outside := 0
	q := make([][]int, 0)
	for r := 0; r < mm; r++ {
		if grid[r][0] == 0 {
			grid[r][0] = 2
			q = append(q, []int{r, 0})
			if r%2 == 0 {
				outside += 1
			}
		}
		if grid[r][nn-1] == 0 {
			grid[r][nn-1] = 2
			q = append(q, []int{r, nn - 1})
			if r%2 == 0 {
				outside += 1
			}
		}
	}
	for c := 0; c < nn; c++ {
		if grid[0][c] == 0 {
			grid[0][c] = 2
			q = append(q, []int{0, c})
			if c%2 == 0 {
				outside += 1
			}
		}
		if grid[mm-1][c] == 0 {
			grid[mm-1][c] = 2
			q = append(q, []int{mm - 1, c})
			if c%2 == 0 {
				outside += 1
			}
		}
	}
	i := 0
	for i < len(q) {
		r, c := q[i][0], q[i][1]
		i += 1
		for _, nb := range [][]int{{r + 1, c}, {r - 1, c}, {r, c + 1}, {r, c - 1}} {
			nr, nc := nb[0], nb[1]
			if nr < 0 || nc < 0 || nr >= mm || nc >= nn {
				continue
			}
			if grid[nr][nc] == 0 {
				grid[nr][nc] = 2
				q = append(q, nb)
				if nr%2 == 0 && nc%2 == 0 {
					outside += 1
				}
			}
		}
	}

	inside := m*n - outside - len(cycle)
	fmt.Println(inside)
}

func part1(g Graph) {
	for _, nb := range g.adj[g.start] {
		hasStartNb := false
		for _, x := range g.adj[nb] {
			if x == g.start {
				hasStartNb = true
				break
			}
		}
		if !hasStartNb {
			continue
		}
		visited := make([]bool, g.nNodes)
		visited[g.start] = true
		visited[nb] = true
		prev := g.start
		curr := nb
		l := 1
		found := false
	Outer:
		for {
			next := curr
			for _, x := range g.adj[curr] {
				if !visited[x] {
					visited[x] = true
					next = x
					l += 1
				} else {
					if x != prev {
						if x == g.start {
							l += 1
							found = true
						}
						break Outer
					}
				}
			}
			if next == curr {
				break
			}
			prev = curr
			curr = next
		}

		if found {
			fmt.Println(l, l/2)
			return
		}
	}
}

type Graph struct {
	m      int
	n      int
	nNodes int
	adj    [][]int
	start  int
}

// return number of nodes, adjacency lists and starting node
func parseGraph(lines []string) Graph {
	m, n := len(lines), len(lines[0])
	adj := make([][]int, m*n)
	getIndex := func(r, c int) int {
		return r*n + c
	}
	isValid := func(r, c int) bool {
		return r >= 0 && r < m && c >= 0 && c < n
	}

	startingNode := -1
	for r := 0; r < m; r++ {
		for c := 0; c < n; c++ {
			i := getIndex(r, c)
			var nbs [][]int
			switch lines[r][c] {
			case '|':
				nbs = [][]int{{r + 1, c}, {r - 1, c}}
			case '-':
				nbs = [][]int{{r, c + 1}, {r, c - 1}}
			case 'L':
				nbs = [][]int{{r - 1, c}, {r, c + 1}}
			case 'J':
				nbs = [][]int{{r - 1, c}, {r, c - 1}}
			case '7':
				nbs = [][]int{{r + 1, c}, {r, c - 1}}
			case 'F':
				nbs = [][]int{{r + 1, c}, {r, c + 1}}
			case 'S':
				startingNode = i
				nbs = [][]int{{r + 1, c}, {r - 1, c}, {r, c + 1}, {r, c - 1}}
				// default:
			}
			for _, nb := range nbs {
				if isValid(nb[0], nb[1]) {
					j := getIndex(nb[0], nb[1])
					adj[i] = append(adj[i], j)
				}
			}
		}
	}
	return Graph{
		m:      m,
		n:      n,
		nNodes: m * n,
		adj:    adj,
		start:  startingNode,
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
