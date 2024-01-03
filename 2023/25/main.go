package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	lines := readLines()
	graph := parseGraph(lines)
	part1(graph)
}

// there is a decent number of nodes and edges
// say we find an MST, are the three edges necessarily in the MST?
// suppose none of the edges where in the MST, that can't be because their removal should
// leave graph disconnected
// at least one has to be in the MST by definition, let it be e
// how can we find it? every edge in MST disconnects the graph
// but suppose we removed e, then only f or g would reconnect the graph
// any other edges couldn't, otherwise the three edges removed would not disconnect the graph
// so we can test every edge in the MST, remove it and see which edges would reconnect the graph
// how to do this?
// can do the classic union find MST algorithm but we make sure that the nodes x, y with e=(x,y)
// are the representatives of their components
// we then count the number of edges that would connect the components of x and y, if there are only 2
// that edge is our solution
// we can count the number of nodes in both components directly in the MST
//
// Another approach that might also work:
// for every edge e = (x, y) start a simultaneous dfs at both x and y
// mark every node found by x with c1 and every node found by y with c2
// if we encounter an edge that would connect both components increase a counter by 1
// at the end if counter is 2, e was one of the three edges that disconnect the graph
func part1(graph [][]int) {
	edges := make([][]int, 0)
	for a, x := range graph {
		for _, b := range x {
			if a < b {
				edges = append(edges, []int{a, b})
			}
		}
	}

	for _, edge := range edges {
		x, y := edge[0], edge[1]
		c1, c2, ok := findEdges(edges, len(graph), x, y)
		if ok {
			fmt.Println(c1 * c2)
			break
		}
	}
}

func findEdges(edges [][]int, n int, x, y int) (int, int, bool) {
	uf := make([]int, n)
	for i := 0; i < n; i++ {
		uf[i] = i
	}

	count := 0
	for _, edge := range edges {
		a, b := edge[0], edge[1]
		if a == x && b == y {
			continue
		}
		u1, u2 := find(a, uf), find(b, uf)
		if u1 != u2 {
			if (u1 == x && u2 == y) || (u1 == y && u2 == x) {
				count += 1
			} else {
				union(u1, u2, uf, x, y)
			}
		}
	}

	if count == 2 {
		cx, cy := 0, 0
		for i := 0; i < n; i++ {
			if find(i, uf) == x {
				cx += 1
			} else {
				cy += 1
			}
		}
		return cx, cy, true
	}
	return 0, 0, false
}

func find(x int, uf []int) int {
	if uf[x] == x {
		return x
	}
	uf[x] = find(uf[x], uf)
	return uf[x]
}

func union(a, b int, uf []int, x, y int) {
	if a == x || a == y {
		uf[b] = a
	} else {
		uf[a] = b
	}
}

func parseGraph(lines []string) [][]int {
	adj := make([][]int, 0)
	nameToIndex := map[string]int{}

	getIndex := func(x string) int {
		if _, ok := nameToIndex[x]; !ok {
			nameToIndex[x] = len(nameToIndex)
			adj = append(adj, nil)
		}
		return nameToIndex[x]
	}

	for _, line := range lines {
		node, nbs := parseLine(line)
		a := getIndex(node)
		for _, nb := range nbs {
			b := getIndex(nb)
			adj[a] = append(adj[a], b)
			adj[b] = append(adj[b], a)
		}
	}

	return adj
}

func parseLine(s string) (string, []string) {
	node, x, _ := strings.Cut(s, ":")
	nbs := strings.Split(strings.TrimSpace(x), " ")
	return node, nbs
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
