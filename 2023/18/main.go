package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	lines := readLines()
	plan := make([]Inst, len(lines))
	for i, line := range lines {
		plan[i] = parseInst(line)
	}
	part1(plan)
	part3(plan)
}

// not the most efficient solution but was kinda fast to code
// actually in part 2 this brute force approach won't work anymore
func part1(plan []Inst) {
	m, n := 0, 0
	for _, i := range plan {
		if i.dir == LEFT || i.dir == RIGHT {
			n += i.steps
		} else {
			m += i.steps
		}
	}
	m, n = 2*m, 2*n

	grid := make([][]int, m)
	for r := 0; r < m; r++ {
		grid[r] = make([]int, n)
	}

	r, c := m/2, n/2
	grid[r][c] = 1
	for _, i := range plan {
		dr, dc := 0, 0
		if i.dir == LEFT {
			dc = -1
		} else if i.dir == RIGHT {
			dc = 1
		} else if i.dir == DOWN {
			dr = 1
		} else {
			dr = -1
		}
		for j := 0; j < i.steps; j++ {
			r, c = r+dr, c+dc
			grid[r][c] = 1
		}
	}

	q := [][]int{{0, 0}}
	i := 0
	grid[0][0] = 2
	count := 1
	for i < len(q) {
		r, c := q[i][0], q[i][1]
		i += 1
		for _, nb := range [][]int{{r + 1, c}, {r - 1, c}, {r, c + 1}, {r, c - 1}} {
			nr, nc := nb[0], nb[1]
			if nr < 0 || nc < 0 || nr >= m || nc >= n {
				continue
			}
			if grid[nr][nc] == 0 {
				grid[nr][nc] = 2
				q = append(q, nb)
				count += 1
			}
		}
	}

	fmt.Println(m*n - count)
}

type VertSegment struct {
	c      int
	rStart int
	rEnd   int
}

type HorzSegment struct {
	r      int
	cStart int
	cEnd   int
}

// This is actually not that easy, and the code here definitely became a bit of a mess and could be
// optimized better. Could make it run faster, by e.g. computing intersections more efficiently.
// The basic idea is this:
// Split the loop/edge of the area into vertical and horizontal segments.
// Consider a left most vertical segment (i.e. one without another vertical segment to the left that intersects the same range of rows).
// Find the nearest vertical segments to the right that overlap the row range, the area in between is surrounded.
// Remove those segments from the list of segments and find the next left most segment, continue until you have processed all the segments.
// Need to be careful to handle all the edge cases correctly, e.g. to not count horizontal segments multiple times.
func part3(plan []Inst) {
	for i, t := range plan {
		plan[i] = convertInst(t)
	}

	vs := make([]VertSegment, 0)
	hs := make([]HorzSegment, 0)
	r, c := 0, 0
	area := 0
	for _, i := range plan {
		area += i.steps
		dr, dc := 0, 0
		vert := true
		if i.dir == LEFT {
			dc = -1
			vert = false
		} else if i.dir == RIGHT {
			dc = 1
			vert = false
		} else if i.dir == DOWN {
			dr = 1
		} else {
			dr = -1
		}
		nr, nc := r+i.steps*dr, c+i.steps*dc
		if vert {
			rStart, rEnd := r, nr
			if rStart > rEnd {
				rStart, rEnd = nr, r
			}
			vs = append(vs, VertSegment{c, rStart, rEnd})
		} else {
			cStart, cEnd := c, nc
			if cStart > cEnd {
				cStart, cEnd = nc, c
			}
			hs = append(hs, HorzSegment{r, cStart, cEnd})
		}
		r, c = nr, nc
	}

	sort.Slice(vs, func(i, j int) bool {
		a, b := vs[i], vs[j]
		if a.c < b.c {
			return true
		} else if a.c > b.c {
			return false
		} else if a.rStart < b.rStart {
			return true
		} else {
			return false
		}
	})

	seen := make([]bool, len(vs))
	a := 0
	for i, v := range vs {
		if seen[i] {
			continue
		}
		// TODO do this only if there is an overlapping horz segment
		if doesHorzSegmentExist(v.rStart, v.c, hs) {
			v.rStart += 1
		}
		if doesHorzSegmentExist(v.rEnd, v.c, hs) {
			v.rEnd -= 1
		}
		if v.rStart > v.rEnd {
			continue
		}
		seen[i] = true
		elim := make([][]int, 0)
		for j := i + 1; j < len(vs); j++ {
			z := vs[j]
			if z.c == v.c {
				continue
			}
			l, r, ok := overlaps(z.rStart, z.rEnd, v.rStart, v.rEnd)
			if ok {
				ivs := removeIvs([][]int{{l, r}}, elim)
				for _, iv := range ivs {
					l, r = iv[0], iv[1]
					a += (r - l + 1) * (z.c - v.c - 1)
					elim = append(elim, []int{l, r})
					seen[j] = true
				}
			}
		}
	}

}

func doesHorzSegmentExist(r, c int, hs []HorzSegment) bool {
	for _, h := range hs {
		if h.r == r && h.cStart == c {
			return true
		}
	}
	return false
}

// remove from x any overlaps with y
func removeIvs(x [][]int, y [][]int) [][]int {
	curr := x
	for _, e := range y {
		next := [][]int{}
		for _, f := range curr {
			l, r, ok := overlaps(f[0], f[1], e[0], e[1])
			if ok {
				a, b := f[0], l-1
				if a <= b {
					next = append(next, []int{a, b})
				}
				a, b = r+1, f[1]
				if a <= b {
					next = append(next, []int{a, b})
				}
			} else {
				next = append(next, f)
			}
		}
		curr = next
	}
	return curr
}

func overlaps(a, b, c, d int) (int, int, bool) {
	if b < c {
		return 0, 0, false
	} else if d < a {
		return 0, 0, false
	} else {
		return max(a, c), min(b, d), true
	}
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

const (
	LEFT  = 0
	RIGHT = 1
	DOWN  = 2
	UP    = 3
)

type Inst struct {
	dir   int
	steps int
	color string
}

func convertInst(i Inst) Inst {
	dir := 0
	switch i.color[5] {
	case '0':
		dir = RIGHT
	case '1':
		dir = DOWN
	case '2':
		dir = LEFT
	case '3':
		dir = UP
	}

	steps, err := strconv.ParseInt(i.color[0:5], 16, 0)
	if err != nil {
		panic(err)
	}

	return Inst{dir: dir, steps: int(steps)}
}

func parseInst(s string) Inst {
	p := strings.Split(s, " ")
	dc := p[0][0]
	dir := 0
	switch dc {
	case 'L':
		dir = LEFT
	case 'R':
		dir = RIGHT
	case 'D':
		dir = DOWN
	case 'U':
		dir = UP
	}

	steps, err := strconv.Atoi(p[1])
	if err != nil {
		panic(err)
	}

	color := p[2][2 : len(p[2])-1]
	return Inst{dir, steps, color}
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
