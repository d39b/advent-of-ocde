package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	s := readLine()
	initSeq := parseInitSeq(s)
	part1(initSeq)
	part2(initSeq)
}

func part1(initSeq []string) {
	sum := 0
	for _, s := range initSeq {
		sum += hash(s)
	}
	fmt.Println(sum)
}

func part2(initSeq []string) {
	boxes := make([][]int, 256)
	labelToIndex := map[string]int{}
	for _, s := range initSeq {
		step := parseStep(s)
		box := hash(step.label)
		if step.focalLength == -1 {
			if i, ok := labelToIndex[step.label]; ok {
				boxes[box][i] = -1
				delete(labelToIndex, step.label)
			}
		} else {
			if i, ok := labelToIndex[step.label]; ok {
				boxes[box][i] = step.focalLength
			} else {
				boxes[box] = append(boxes[box], step.focalLength)
				labelToIndex[step.label] = len(boxes[box]) - 1
			}
		}
	}

	sum := 0
	for b, box := range boxes {
		nextIndex := 1
		for _, fl := range box {
			if fl == -1 {
				continue
			}
			focusPower := (b + 1) * nextIndex * fl
			sum += focusPower
			nextIndex += 1
		}
	}
	fmt.Println(sum)
}

func hash(s string) int {
	v := 0
	for i := 0; i < len(s); i++ {
		v = 17 * (v + int(s[i]))
		v = v % 256
	}
	return v
}

type Step struct {
	label       string
	focalLength int
}

func parseStep(s string) Step {
	i := strings.IndexByte(s, '=')
	if i != -1 {
		label := s[:i]
		focalLength := int(s[i+1] - '0')
		return Step{label: label, focalLength: focalLength}
	}
	i = strings.IndexByte(s, '-')
	return Step{label: s[:i], focalLength: -1}
}

func parseInitSeq(s string) []string {
	return strings.Split(s, ",")
}

func readLine() string {
	sc := bufio.NewScanner(os.Stdin)
	sc.Scan()
	line := string(sc.Bytes())
	if err := sc.Err(); err != nil {
		panic(err)
	}
	return line
}
