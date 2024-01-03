package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	lines := readLines()
	hs := make([]HS, len(lines))
	for i, line := range lines {
		hs[i] = parseHS(line)
	}
	// for example
	// part1(hs, 7.0, 27.0)
	part1(hs, 200000000000000.0, 400000000000000.0)
}

func part1(hs []HS, minV, maxV float64) {
	result := 0
	for i, h1 := range hs {
		for _, h2 := range hs[i+1:] {
			// a pair of paths either intersect or they run parallel
			// we have the following lines/paths
			// z1 = vy1/vx1 * (v-x1) + y1 = vy1/vx1*v - (vy1/vx1)*x1 + y1
			// z2 = vy2/vx2 * (v-x2) + y2 = vy2/vx2*v - (vy2/vx2)*x2 + y2
			// find v with z1 == z2 and check that (v, z1) is within bounds
			// v * (vy1/vx1 - vy2/vx2) = -(vy2/vx2)*x2 + y2 + (vy1/vx1)*x1 - y1
			// -> v * k = c
			// need to make sure that k is not zero, and also that vx1 and vx2 are not zero
			// need to find lcm of vx1 and vx2
			vy1, vy2 := h1.vy, h2.vy
			vx1, vx2 := h1.vx, h2.vx
			if vx1 < 0 {
				vx1 *= -1
				vy1 *= -1
			}
			if vx2 < 0 {
				vx2 *= -1
				vy2 *= -1
			}

			l := lcm(vx1, vx2)
			vy1 *= l / vx1
			vy2 *= l / vx2
			nom := vy1 - vy2
			if nom == 0 {
				continue
			}

			k := float64(nom) / float64(l)
			a := -1*float64(h2.vy)/float64(h2.vx)*float64(h2.x) + float64(h2.y)
			b := float64(h1.vy)/float64(h1.vx)*float64(h1.x) - float64(h1.y)
			v := (a + b) / k
			z := (v-float64(h1.x))*float64(h1.vy)/float64(h1.vx) + float64(h1.y)
			// the paths cross in the point (v, z)

			// note that we want future paths to cross
			// i.e. we need to solve v = x1 + t*vx1 and t needs to be > 0, same for h2
			t1 := (v - float64(h1.x)) / float64(h1.vx)
			t2 := (v - float64(h2.x)) / float64(h2.vx)
			if t1 >= 0 && t2 >= 0 && minV <= v && v <= maxV && minV <= z && z <= maxV {
				result += 1
			}
		}
	}

	fmt.Println(result)
}

func lcm(a, b int) int {
	return a * b / gcd(a, b)
}

func gcd(a, b int) int {
	if b == 0 {
		return a
	}
	return gcd(b, a%b)
}

type HS struct {
	x, y, z    int
	vx, vy, vz int
}

func parseHS(s string) HS {
	pos, vel, _ := strings.Cut(s, "@")
	ps := strings.Split(pos, ",")
	vs := strings.Split(vel, ",")
	pi, vi := make([]int, 0), make([]int, 0)
	for _, s := range ps {
		pi = append(pi, parseInt(strings.TrimSpace(s)))
	}
	for _, s := range vs {
		vi = append(vi, parseInt(strings.TrimSpace(s)))
	}
	return HS{pi[0], pi[1], pi[2], vi[0], vi[1], vi[2]}
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
