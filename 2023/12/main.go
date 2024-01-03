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
	rows := make([]Row, len(lines))
	for i, x := range lines {
		rows[i] = parseRow(x)
	}
	part1(rows)
	part2(rows)
}

// This is a classic DP problem.
func part1(rows []Row) {
	sum := 0
	for _, row := range rows {
		na := numArrangementsForRow(row)
		sum += na
	}
	fmt.Println(sum)
}

func part2(rows []Row) {
	sum := 0
	for _, row := range rows {
		// unfold input
		springs := row.springs
		for i := 0; i < 4; i++ {
			springs += "?" + row.springs
		}
		groups := make([]int, 0, 5*len(row.groups))
		for i := 0; i < 5; i++ {
			groups = append(groups, row.groups...)
		}

		na := numArrangementsForRow(Row{springs: springs, groups: groups})
		sum += na
	}
	fmt.Println(sum)
}

func numArrangementsForRow(row Row) int {
	return rec(row, 0, 0, map[E]int{})
}

func rec(row Row, i, j int, mem map[E]int) int {
	if v, ok := mem[E{i, j}]; ok {
		return v
	}

	result := 0
	if i == len(row.springs) {
		if j == len(row.groups) {
			result = 1
		}
	} else if j == len(row.groups) {
		// we have matched all the broken springs
		// i.e. everything remaining should be '.' or '?'
		// we will match every ? with a operational spring
		valid := true
		for k := i; k < len(row.springs); k++ {
			if row.springs[k] == '#' {
				valid = false
				break
			}
		}
		if valid {
			result = 1
		}
	} else {
		z := row.groups[j]
		if len(row.springs)-i >= z {
			// we can have 0, 1, 2, ....  operational springs at the start and then match z broken springs
			k := 0
			if i != 0 {
				k = 1
			}
			for {
				if k > 0 {
					if row.springs[i+k-1] == '#' {
						break
					}
				}
				li := i + k - 1 + z
				if li >= len(row.springs) {
					break
				}
				valid := true
				// could optimize this, by counting the number of correct matches
				// when we increase k we need to update that count by removing the first broken spring
				// and adding the last potentially
				for l := i + k; l <= li; l++ {
					if row.springs[l] == '.' {
						valid = false
						break
					}
				}
				if valid {
					result += rec(row, li+1, j+1, mem)
				}
				k += 1
			}
		}
	}

	mem[E{i, j}] = result
	return result
}

type E struct {
	i int
	j int
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

func parseRow(line string) Row {
	springs, s, _ := strings.Cut(line, " ")
	gs := strings.Split(s, ",")
	groups := make([]int, len(gs))
	for i, x := range gs {
		n, err := strconv.Atoi(x)
		if err != nil {
			panic(err)
		}
		groups[i] = n
	}
	return Row{springs: springs, groups: groups}
}

type Row struct {
	springs string
	groups  []int
}
