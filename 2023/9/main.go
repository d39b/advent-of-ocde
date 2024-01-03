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
	histories := make([][]int, len(lines))
	for i, line := range lines {
		histories[i] = parseIntList(line)
	}
	part1(histories)
	part2(histories)
}

func part1(histories [][]int) {
	sum := 0
	for _, h := range histories {
		sum += predictNextValue(h)
	}
	fmt.Println(sum)
}

func part2(histories [][]int) {
	sum := 0
	for _, h := range histories {
		sum += predictPrevValue(h)
	}
	fmt.Println(sum)
}

func predictPrevValue(history []int) int {
	seq := history
	first := []int{seq[0]}
	for {
		nextSeq, allZero := computeDiffSequence(seq)
		if allZero {
			break
		}
		first = append(first, nextSeq[0])
		seq = nextSeq
	}
	v := first[len(first)-1]
	for i := len(first) - 2; i >= 0; i-- {
		v = first[i] - v
	}
	return v
}

// Note: to speed this up we could override the values in the current sequence with the new one
// instead of creating a new slice every time
func predictNextValue(history []int) int {
	seq := history
	v := seq[len(seq)-1]
	for {
		nextSeq, allZero := computeDiffSequence(seq)
		if allZero {
			break
		}
		v += nextSeq[len(nextSeq)-1]
		seq = nextSeq
	}
	return v
}

func computeDiffSequence(seq []int) ([]int, bool) {
	ds := make([]int, len(seq)-1)
	allZero := true
	for i := 1; i < len(seq); i++ {
		ds[i-1] = seq[i] - seq[i-1]
		if ds[i-1] != 0 {
			allZero = false
		}
	}
	return ds, allZero
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

func parseIntList(x string) []int {
	e := strings.Split(strings.TrimSpace(x), " ")
	r := make([]int, len(e))
	for i, sn := range e {
		n, err := strconv.Atoi(sn)
		if err != nil {
			panic(err)
		}
		r[i] = n
	}
	return r
}
