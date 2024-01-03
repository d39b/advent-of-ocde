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
	workflows, parts := parseInput(lines)
	part1(workflows, parts)
	part2(workflows, parts)
}

func part1(workflows map[string]Workflow, parts []Part) {
	sum := 0

	for _, part := range parts {
		wfName := "in"
		for !(wfName == "A" || wfName == "R") {
			wf := workflows[wfName]
			for _, rule := range wf.rules {
				if rule.cat == 0 {
					wfName = rule.next
					break
				}
				var v int
				switch rule.cat {
				case 'x':
					v = part.x
				case 'm':
					v = part.m
				case 'a':
					v = part.a
				case 's':
					v = part.s
				}

				if rule.op == '>' {
					if v > rule.v {
						wfName = rule.next
						break
					}
				} else {
					if v < rule.v {
						wfName = rule.next
						break
					}
				}
			}
		}

		if wfName == "A" {
			sum += part.x + part.m + part.a + part.s
		}
	}

	fmt.Println(sum)
}

// can solve this recursively by keeping track of the currently possible interval for each rating
// for every rule we branch two times, i.e. we constrain the relevent rating interval to either satisfy the rule or not
func part2(workflows map[string]Workflow, parts []Part) {
	// can probably do this recursively, might be slow?
	r := IV{left: 1, right: 4000}
	fmt.Println(rec("in", IVPart{x: r, m: r, a: r, s: r}, workflows))
}

func rec(wfName string, part IVPart, workflows map[string]Workflow) int {
	if wfName == "A" {
		return part.x.Size() * part.m.Size() * part.a.Size() * part.s.Size()
	} else if wfName == "R" {
		return 0
	}

	result := 0
	wf := workflows[wfName]
	for _, rule := range wf.rules {
		if rule.cat == 0 {
			result += rec(rule.next, part, workflows)
			break
		}
		var v IV
		switch rule.cat {
		case 'x':
			v = part.x
		case 'm':
			v = part.m
		case 'a':
			v = part.a
		case 's':
			v = part.s
		}

		if rule.op == '>' {
			z := IV{rule.v + 1, v.right}
			if z.left <= z.right {
				result += rec(rule.next, part.Copy(rule.cat, z), workflows)
			}
			z = IV{v.left, rule.v}
			if z.left > z.right {
				break
			}
			part = part.Copy(rule.cat, z)
		} else {
			z := IV{v.left, rule.v - 1}
			if z.left <= z.right {
				result += rec(rule.next, part.Copy(rule.cat, z), workflows)
			}
			z = IV{rule.v, v.right}
			if z.left > z.right {
				break
			}
			part = part.Copy(rule.cat, z)
		}
	}
	return result
}

type IVPart struct {
	x IV
	m IV
	a IV
	s IV
}

func (i IVPart) Copy(cat byte, iv IV) IVPart {
	r := i
	switch cat {
	case 'x':
		r.x = iv
	case 'm':
		r.m = iv
	case 'a':
		r.a = iv
	case 's':
		r.s = iv
	}
	return r
}

type IV struct {
	left  int
	right int
}

func (i IV) Size() int {
	return i.right - i.left + 1
}

func parseInput(lines []string) (map[string]Workflow, []Part) {
	workflows := map[string]Workflow{}

	i := 0
	for lines[i] != "" {
		wf := parseWorkflow(lines[i])
		workflows[wf.name] = wf
		i += 1
	}

	parts := make([]Part, 0)

	i += 1
	for i < len(lines) {
		parts = append(parts, parsePart(lines[i]))
		i += 1
	}

	return workflows, parts
}

func parseWorkflow(s string) Workflow {
	name, rs, _ := strings.Cut(s, "{")
	// remove trailing }
	rs = rs[0 : len(rs)-1]

	rules := make([]Rule, 0)
	for _, r := range strings.Split(rs, ",") {
		if i := strings.IndexByte(r, ':'); i != -1 {
			next := r[i+1:]
			cat := r[0]
			op := r[1]
			v := parseInt(r[2:i])
			rules = append(rules, Rule{cat: cat, op: op, v: v, next: next})
		} else {
			rules = append(rules, Rule{cat: 0, next: r})
		}
	}

	return Workflow{name: name, rules: rules}
}

func parsePart(s string) Part {
	v := strings.Split(s[1:len(s)-1], ",")
	return Part{
		x: parseInt(v[0][2:]),
		m: parseInt(v[1][2:]),
		a: parseInt(v[2][2:]),
		s: parseInt(v[3][2:]),
	}
}

func parseInt(s string) int {
	n, err := strconv.Atoi(s)
	if err != nil {
		panic(err)
	}
	return n
}

type Workflow struct {
	name  string
	rules []Rule
}

type Rule struct {
	cat  byte
	op   byte
	v    int
	next string
}

type Part struct {
	x int
	m int
	a int
	s int
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
