package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	lines := readLines()
	// part1(lines)
	part2(lines)
}

// this seems to be a bit of a troll problem
// finding the longest path in any grid might even be NP-complete
// but if we analyze the input, we can see that most cell have at most two neighbors
// since we must reach such a cell from one cell and leave it at the other there is only ever one possible path for those
// there is only a small number of cells where we can move to 2 or 3 neighbors, and those cells also contain arrows
// this makes backtracking possible
func part1(grid []string) {
	m, n := len(grid), len(grid[0])
	visited := make([][]bool, m)
	for r := 0; r < m; r++ {
		visited[r] = make([]bool, n)
	}
	fmt.Println(backtrack(0, 1, grid, visited))
}

func backtrack(r, c int, grid []string, visited [][]bool) int {
	m, n := len(grid), len(grid[0])
	if r == m-1 && c == n-2 {
		return 0
	}

	z := grid[r][c]
	if z == '#' {
		return -1
	}
	if visited[r][c] {
		return -1
	}

	visited[r][c] = true

	if z == '^' {
		s := backtrack(r-1, c, grid, visited)
		if s != -1 {
			s += 1
		}
		visited[r][c] = false
		return s
	} else if z == 'v' {
		s := backtrack(r+1, c, grid, visited)
		if s != -1 {
			s += 1
		}
		visited[r][c] = false
		return s
	} else if z == '>' {
		s := backtrack(r, c+1, grid, visited)
		if s != -1 {
			s += 1
		}
		visited[r][c] = false
		return s
	} else if z == '<' {
		s := backtrack(r, c-1, grid, visited)
		if s != -1 {
			s += 1
		}
		visited[r][c] = false
		return s
	}

	result := -1
	for _, nb := range [][]int{{r - 1, c}, {r + 1, c}, {r, c - 1}, {r, c + 1}} {
		nr, nc := nb[0], nb[1]
		if nr < 0 || nr >= m || nc < 0 || nc >= n {
			continue
		}
		if isMovePossible(r, c, nr, nc, grid) {
			s := backtrack(nr, nc, grid, visited)
			if s != -1 {
				if result == -1 || s+1 > result {
					result = s + 1
				}
			}
		}
	}

	visited[r][c] = false
	return result
}

// backtracking approach still works, but there is too much overhead
// let a cell (r, c) be a choice cell if is has more than 2 neighbors that are not forest #
// we can simplify the problem by "contracting" all the paths between two choice nodes
// this yields a graph whose nodes are the choice nodes, backtrack on that
func part2(grid []string) {
	m, n := len(grid), len(grid[0])

	choiceCells := make([][2]int, 0)
	cellToIndex := make(map[[2]int]int)
	for r := 1; r < m-1; r++ {
		for c := 1; c < n-1; c++ {
			if grid[r][c] == '#' {
				continue
			}

			count := 0
			for _, nb := range [][]int{{r - 1, c}, {r + 1, c}, {r, c - 1}, {r, c + 1}} {
				nr, nc := nb[0], nb[1]
				if grid[nr][nc] != '#' {
					count += 1
				}
			}

			if count > 2 {
				cell := [2]int{r, c}
				choiceCells = append(choiceCells, cell)
				i := len(choiceCells) - 1
				cellToIndex[cell] = i
			}
		}
	}

	// build the graph
	adj := make([][]Nb, len(choiceCells))
	startEndSteps := 0
	startCell, endCell := -1, -1
	for i, cell := range choiceCells {
		cr, cc := cell[0], cell[1]
		// for each neighbor start a search until we reach a choice cell again or the starting
		// or the ending cell
		for _, nb := range [][]int{{cr + 1, cc}, {cr - 1, cc}, {cr, cc - 1}, {cr, cc + 1}} {
			prevR, prevC := cr, cc
			r, c := nb[0], nb[1]
			if grid[r][c] == '#' {
				continue
			}
			steps := 1
			for {
				if j, ok := cellToIndex[[2]int{r, c}]; ok {
					adj[i] = append(adj[i], Nb{j, steps})
					break
				} else if r == 0 && c == 1 {
					startCell = i
					startEndSteps += steps
					break
				} else if r == m-1 && c == n-2 {
					endCell = i
					startEndSteps += steps
					break
				}
				for _, nnb := range [][]int{{r + 1, c}, {r - 1, c}, {r, c - 1}, {r, c + 1}} {
					nr, nc := nnb[0], nnb[1]
					if grid[nr][nc] == '#' {
						continue
					}
					if prevR == nr && prevC == nc {
						continue
					}
					prevR, prevC = r, c
					r, c = nr, nc
					steps += 1
					break
				}
			}
		}

	}

	nc := len(choiceCells)
	visited := make([]bool, nc)
	fmt.Println(startEndSteps + backtrack2(startCell, adj, endCell, visited))
}

type Nb struct {
	i int
	w int
}

func backtrack2(i int, adj [][]Nb, end int, visited []bool) int {
	if i == end {
		return 0
	}

	visited[i] = true

	result := -1
	for _, nb := range adj[i] {
		if visited[nb.i] {
			continue
		}
		s := backtrack2(nb.i, adj, end, visited)
		if s != -1 {
			if result == -1 || s+nb.w > result {
				result = s + nb.w
			}
		}
	}

	visited[i] = false
	return result
}

func isMovePossible(r, c, nr, nc int, grid []string) bool {
	z := grid[nr][nc]
	if z == '#' {
		return false
	}

	if z == '.' {
		return true
	}

	if z == 'v' && r > nr {
		return false
	}

	if z == '^' && r < nr {
		return false
	}

	if z == '>' && c > nc {
		return false
	}

	if z == '<' && c < nc {
		return false
	}

	return true
}

func isArrow(b byte) bool {
	return !(b == '#' || b == '.')
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
