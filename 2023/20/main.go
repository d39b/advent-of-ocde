package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	lines := readLines()
	modules, nameToIndex, bc := parseModules(lines)
	part1(modules, bc)
	part2(modules, bc, nameToIndex["rs"])
}

func part1(modules []*Module, bc int) {
	low := 0
	high := 0
	for j := 0; j < 1000; j++ {
		pulses := []Pulse{{false, -1, bc}}
		i := 0
		for i < len(pulses) {
			curr := pulses[i]
			if curr.high {
				high += 1
			} else {
				low += 1
			}
			i += 1
			if curr.to == -1 {
				continue
			}
			next := modules[curr.to].Process(curr)
			pulses = append(pulses, next...)
		}
	}

	fmt.Println(low * high)
}

// there seem to be a lot of these cycle detection problems this year
// also note that: we often need to consider the structure of our particular input
// i.e. the input is a special case that is easier to solve
func part2(modules []*Module, bc int, conjIdx int) {
	// reset state
	for i := 0; i < len(modules); i++ {
		if modules[i] != nil {
			modules[i].on = false
			modules[i].highPulses = make([]bool, len(modules))
			modules[i].numHighPulses = 0
		}
	}

	cycles := map[int][]int{}
	j := 1
	for {
		pulses := []Pulse{{false, -1, bc}}
		i := 0
		for i < len(pulses) {
			curr := pulses[i]
			i += 1
			if curr.to == -1 {
				continue
			}
			if curr.to == conjIdx {
				if curr.high {
					cycles[curr.from] = append(cycles[curr.from], j)
				}
			}
			next := modules[curr.to].Process(curr)
			pulses = append(pulses, next...)
		}
		j += 1
		if j > 100000 {
			break
		}
	}

	cycleLengths := make([]int, 0)
	for _, v := range cycles {
		cycleLengths = append(cycleLengths, v[0])
	}

	r := cycleLengths[0]
	for i := 1; i < len(cycleLengths); i++ {
		r = lcm(r, cycleLengths[i])
	}
	fmt.Println(r)
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

const (
	FLIPFLOP    = 0
	CONJUNCTION = 1
	BROADCAST   = 2
)

type Pulse struct {
	high bool
	from int
	to   int
}

type Module struct {
	t         int
	i         int
	dest      []int
	inputs    []int
	numInputs int
	// state of flip flop module
	on bool
	// state of conjunction module
	numHighPulses int
	highPulses    []bool
}

func (m *Module) Process(p Pulse) []Pulse {
	out := make([]Pulse, 0)
	if m.t == BROADCAST {
		for _, d := range m.dest {
			out = append(out, Pulse{p.high, m.i, d})
		}
	} else if m.t == FLIPFLOP {
		if !p.high {
			outHigh := true
			if m.on {
				outHigh = false
			}
			for _, d := range m.dest {
				out = append(out, Pulse{outHigh, m.i, d})
			}
			m.on = !m.on
		}
	} else {
		if p.high {
			if !m.highPulses[p.from] {
				m.highPulses[p.from] = true
				m.numHighPulses += 1
			}
		} else {
			if m.highPulses[p.from] {
				m.highPulses[p.from] = false
				m.numHighPulses -= 1
			}
		}
		outHigh := true
		if m.numHighPulses == m.numInputs {
			outHigh = false
		}
		for _, d := range m.dest {
			out = append(out, Pulse{outHigh, m.i, d})
		}
	}
	return out
}

func parseModules(lines []string) ([]*Module, map[string]int, int) {
	n := len(lines)
	modules := make([]*Module, n)
	nameToIndex := map[string]int{}
	nextIndex := 0
	getIndex := func(name string) int {
		if name == "rx" {
			return -1
		}
		if _, ok := nameToIndex[name]; !ok {
			nameToIndex[name] = nextIndex
			nextIndex += 1
		}
		return nameToIndex[name]
	}
	for _, s := range lines {
		t, name, dest := parseModule(s)
		m := Module{t: t, i: getIndex(name)}
		for _, d := range dest {
			m.dest = append(m.dest, getIndex(d))
		}
		m.on = false
		m.numHighPulses = 0
		m.highPulses = make([]bool, n)
		modules[m.i] = &m
	}

	for _, m := range modules {
		for _, d := range m.dest {
			if d != -1 {
				modules[d].numInputs += 1
				modules[d].inputs = append(modules[d].inputs, d)
			}
		}
	}

	bc := nameToIndex["broadcaster"]

	return modules, nameToIndex, bc
}

func parseModule(s string) (int, string, []string) {
	a, b, ok := strings.Cut(s, "->")
	if !ok {
		panic("sep not found")
	}
	a = strings.TrimSpace(a)
	b = strings.TrimSpace(b)

	var t int
	var name string
	if a == "broadcaster" {
		t = BROADCAST
		name = a
	} else if a[0] == '%' {
		t = FLIPFLOP
		name = a[1:]
	} else {
		t = CONJUNCTION
		name = a[1:]
	}

	dest := strings.Split(b, ", ")
	return t, name, dest
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
