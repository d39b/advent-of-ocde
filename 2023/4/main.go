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
	cards := parseCards(lines)
	part1(cards)
	part2(cards)
}

func part1(cards []Card) {
	points := 0
	for _, card := range cards {
		m := make(map[int]bool)
		for _, x := range card.winNumbers {
			m[x] = true
		}

		matches := 0
		for _, x := range card.haveNumbers {
			if _, ok := m[x]; ok {
				matches += 1
			}
		}

		if matches > 0 {
			points += 1 << (matches - 1)
		}
	}
	fmt.Println(points)
}

func part2(cards []Card) {
	n := len(cards)
	count := make([]int, n)
	for i := 0; i < n; i++ {
		count[i] = 1
	}
	for i, card := range cards {
		m := make(map[int]bool)
		for _, x := range card.winNumbers {
			m[x] = true
		}

		matches := 0
		for _, x := range card.haveNumbers {
			if _, ok := m[x]; ok {
				matches += 1
			}
		}

		if matches > 0 {
			for j := i + 1; j <= i+matches && j < n; j++ {
				count[j] += count[i]
			}
		}
	}

	total := 0
	for _, x := range count {
		total += x
	}
	fmt.Println(total)
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

func parseCards(x []string) []Card {
	cards := make([]Card, len(x))
	for i, s := range x {
		cards[i] = parseCard(s)
	}
	return cards
}

func parseCard(s string) Card {
	parts := strings.Split(s[strings.IndexByte(s, ':')+2:], "|")
	winNumbers := parseIntList(parts[0])
	haveNumbers := parseIntList(parts[1])
	return Card{winNumbers: winNumbers, haveNumbers: haveNumbers}
}

func parseIntList(s string) []int {
	result := make([]int, 0)
	for _, x := range strings.Fields(strings.TrimSpace(s)) {
		n, err := strconv.Atoi(x)
		if err != nil {
			panic(err)
		}
		result = append(result, n)
	}
	return result
}

type Card struct {
	winNumbers  []int
	haveNumbers []int
}
