package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	lines := readLines()
	instructions := lines[0]
	network := parseNetwork(lines[2:])
	// part1(instructions, network)
	part2(instructions, network)
}

func part1(instructions string, network Network) {
	start := network.nameToIndex["AAA"]
	target := network.nameToIndex["ZZZ"]
	step := 0
	curr := start
	for curr != target {
		inst := instructions[step%len(instructions)]
		if inst == 'L' {
			curr = network.next[curr].left
		} else {
			curr = network.next[curr].right
		}
		step += 1
	}

	fmt.Println(step)
}

/*
Might need to do some math here.
There is a limited number of nodes we can go through, so if we start at some node x
we will pretty soon reach a node we have already visited, this gives a cycle length
and we also have a list of end nodes we can reach, with step numbers
the start of the cycle is some step s, every end node we reached after that will repeat every x+cycleLength steps
there are two ways from here:
we try to solve this set of equations to find a step number where every sequence as at an end point
we "brute-force" generate the elements of this sequence until all are the same
this might be faster because we will do multiple steps at once

Note: it turns out that the problems is generated in such a way that we always end up at the same end
node in regular intervals, i.e. for a given start node we will only ever get to one end node at step
x, 2x, 3x, ....
Let x1, x2, x3, ..., xk be the first step number where we reach an end node for all the start nodes
The result will then be the least common multiple of x1,...,xk
*/
func part2(instructions string, network Network) {
	currNodes := []int{}
	isEndNode := make([]bool, len(network.next))
	for name, index := range network.nameToIndex {
		if name[2] == 'A' {
			currNodes = append(currNodes, index)
		} else if name[2] == 'Z' {
			isEndNode[index] = true
		}
	}

	cycles := make([]int, len(currNodes))
	for i, c := range currNodes {
		cycles[i] = findCycle(instructions, network, c, isEndNode)
	}

	result := cycles[0]
	for i := 1; i < len(cycles); i++ {
		result = lcm(result, cycles[i])
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

func findCycle(instructions string, network Network, start int, isEndNode []bool) int {
	curr := start
	step := 0
	i := 0
	for {
		inst := instructions[i]
		var next int
		if inst == 'L' {
			next = network.next[curr].left
		} else {
			next = network.next[curr].right
		}
		if isEndNode[next] {
			return step + 1
		}
		curr = next
		step += 1
		i = (i + 1) % len(instructions)
	}
}

func parseNetwork(lines []string) Network {
	nameToIndex := map[string]int{}
	next := make([]Node, len(lines))
	toIndex := func(x string) int {
		if _, ok := nameToIndex[x]; !ok {
			i := len(nameToIndex)
			nameToIndex[x] = i
		}
		return nameToIndex[x]
	}

	for _, line := range lines {
		x, left, right := parseNetworkLine(line)
		next[toIndex(x)] = Node{
			left:  toIndex(left),
			right: toIndex(right),
		}
	}

	return Network{
		next:        next,
		nameToIndex: nameToIndex,
	}
}

func parseNetworkLine(l string) (string, string, string) {
	a, b, _ := strings.Cut(l, "=")
	node := strings.TrimSpace(a)
	left, right, _ := strings.Cut(strings.TrimSpace(b), ",")
	left = strings.TrimSpace(left)[1:4]
	right = strings.TrimSpace(right)[0:3]
	return node, left, right
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

type Network struct {
	next        []Node
	nameToIndex map[string]int
}

type Node struct {
	left  int
	right int
}
