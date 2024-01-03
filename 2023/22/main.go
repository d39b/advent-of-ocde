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
	bricks := make([]Brick, len(lines))
	for i, line := range lines {
		bricks[i] = parseBrick(line, i)
	}
	part1And2(bricks)
}

func part1And2(bricks []Brick) {
	// sort blocks by lower z coordinate
	sort.Slice(bricks, func(i, j int) bool {
		a := bricks[i].a.z
		b := bricks[j].a.z
		return a < b
	})

	// adj[i] = the indices of bricks supporting brick i
	adj := make([][]int, len(bricks))
	// bricks fall one by one and hit the ground or another brick or multiple bricks
	g := []Brick{bricks[0].ToZ(1)}
	for i := 1; i < len(bricks); i++ {
		b := bricks[i]
		// find bricks b will land on
		z := -1
		jj := -1
		for j := len(g) - 1; j >= 0; j-- {
			if g[j].b.z < b.a.z {
				if intersectXY(b, g[j]) {
					z = g[j].b.z
					jj = j
					break
				}
			}
		}

		if z == -1 {
			// b falls to the ground
			g = append(g, b.ToZ(1))
		} else {
			adj[b.i] = append(adj[b.i], g[jj].i)
			// find any other that b might land on
			for j := jj - 1; j >= 0; j-- {
				if g[j].b.z == z {
					if intersectXY(b, g[j]) {
						adj[b.i] = append(adj[b.i], g[j].i)
					}
				} else {
					break
				}
			}
			g = append(g, b.ToZ(z+1))
		}

		// keep g sorted by the higher z cooridnate
		sort.Slice(g, func(i, j int) bool {
			a := g[i].b.z
			b := g[j].b.z
			return a < b
		})
	}

	cantDisintegrate := map[int]bool{}
	for _, a := range adj {
		if len(a) == 1 {
			// can't remove a[0] because it is the only one supporting that brick
			cantDisintegrate[a[0]] = true
		}
	}

	fmt.Println(len(bricks) - len(cantDisintegrate))

	// part 2
	// don't have that many blocks, so we can just simulate for each one
	// we now want the bricks sorted again by lower coordinate

	sort.Slice(g, func(i, j int) bool {
		a := g[i].a.z
		b := g[j].a.z
		return a < b
	})

	result := 0
	for i, b := range g {
		// any bricks lower than i can't fall if we disintegrate b
		c := 0
		gone := map[int]bool{b.i: true}
		for j := i + 1; j < len(g); j++ {
			if len(adj[g[j].i]) == 0 {
				// this is a brick on the ground, it will never fall
				continue
			}
			allGone := true
			for _, v := range adj[g[j].i] {
				if !gone[v] {
					allGone = false
					break
				}
			}
			if allGone {
				gone[g[j].i] = true
				c += 1
			}
		}
		result += c
	}

	fmt.Println(result)
}

func intersectXY(br1, br2 Brick) bool {
	// they intersect if x and y range intersect?
	return intersect(br1.a.x, br1.b.x, br2.a.x, br2.b.x) && intersect(br1.a.y, br1.b.y, br2.a.y, br2.b.y)
}

func intersect(a, b, c, d int) bool {
	if a > b {
		a, b = b, a
	}
	if c > d {
		c, d = d, c
	}
	if b < c {
		return false
	}
	if d < a {
		return false
	}
	return true
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

type Brick struct {
	i int
	a Point
	b Point
}

func (br Brick) ToZ(z int) Brick {
	dz := br.a.z - z
	a := br.a
	b := br.b
	a.z -= dz
	b.z -= dz
	return Brick{br.i, a, b}
}

type Point struct {
	x int
	y int
	z int
}

func parseBrick(s string, i int) Brick {
	as, bs, _ := strings.Cut(s, "~")
	ap := strings.Split(as, ",")
	bp := strings.Split(bs, ",")
	a := Point{parseInt(ap[0]), parseInt(ap[1]), parseInt(ap[2])}
	b := Point{parseInt(bp[0]), parseInt(bp[1]), parseInt(bp[2])}
	if a.z > b.z {
		a, b = b, a
	}
	return Brick{i, a, b}
}

func parseInt(s string) int {
	n, err := strconv.Atoi(s)
	if err != nil {
		panic(err)
	}
	return n
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
