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
	games := make([]Game, len(lines))
	for i, line := range lines {
		games[i] = parseGame(line)
	}
	part1(games)
	part2(games)
}

func part1(games []Game) {
	maxRed, maxGreen, maxBlue := 12, 13, 14
	sumOfIds := 0
	for _, game := range games {
		valid := true
		for _, subset := range game.subsets {
			if subset.red > maxRed {
				valid = false
				break
			}
			if subset.green > maxGreen {
				valid = false
				break
			}
			if subset.blue > maxBlue {
				valid = false
				break
			}
		}
		if valid {
			sumOfIds += game.id
		}
	}
	fmt.Println(sumOfIds)
}

func part2(games []Game) {
	// for each color find the max count among all subsets
	// that is the minimum number of cubes of a color that must be in the bag
	// power = #green * #red * #blue
	// return sum of powers of all games
	sum := 0
	for _, game := range games {
		r, g, b := 0, 0, 0
		for _, subset := range game.subsets {
			r = max(r, subset.red)
			g = max(g, subset.green)
			b = max(b, subset.blue)
		}
		power := r * g * b
		sum += power
	}
	fmt.Println(sum)
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func parseGame(s string) Game {
	sepI := strings.IndexByte(s, ':')
	id, err := strconv.Atoi(s[5:sepI])
	if err != nil {
		panic(err)
	}

	var subsets []Subset
	for _, x := range strings.Split(s[sepI+1:], ";") {
		subset := Subset{}
		for _, y := range strings.Split(x, ",") {
			parts := strings.Split(strings.TrimSpace(y), " ")
			count, err := strconv.Atoi(parts[0])
			if err != nil {
				panic(err)
			}
			switch parts[1] {
			case "green":
				subset.green += count
			case "red":
				subset.red += count
			case "blue":
				subset.blue += count
			default:
				panic(fmt.Sprintf("unkown color: %v", parts[1]))
			}
		}
		subsets = append(subsets, subset)
	}

	return Game{id: id, subsets: subsets}
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

type Game struct {
	id      int
	subsets []Subset
}

type Subset struct {
	red   int
	green int
	blue  int
}
