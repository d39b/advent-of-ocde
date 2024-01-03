package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

// this code has become kind of a mess
// but it works
// so why fix it?
// ain't nobody got time for that

func main() {
	lines := readLines()
	seeds := parseSeedList(lines[0])
	maps := parseMaps(lines[2:])
	part1(seeds, maps)
	// part2_brute_force(seeds, maps)
	part2_fast(seeds, maps)
}

func part1(seeds []int, maps []Map) {
	min := 1 << 60
	for _, seed := range seeds {
		curr := seed
		for _, m := range maps {
			curr = m.Map(curr)
		}
		if curr < min {
			min = curr
		}
	}
	fmt.Println(min)
}

func part2_brute_force(seeds []int, maps []Map) {
	// this can actually still be brute-forced
	min := 1 << 60
	total := 0
	for i := 1; i < len(seeds); i += 2 {
		total += seeds[i]
	}
	fmt.Println(total)
	checked := 0
	for i := 0; i < len(seeds); i += 2 {
		for seed := seeds[i]; seed < seeds[i]+seeds[i+1]; seed++ {
			curr := seed
			for _, m := range maps {
				// using binary search to find the range for a given source value is a bit faster
				// but this is not obvious since the number of ranges of a map is small (< 20?)
				// brute-force searching through the list of ranges might actually be faster
				curr = m.MapBS(curr)
			}
			if curr < min {
				min = curr
			}
			checked += 1
			if checked%100000 == 0 {
				fmt.Println(checked)
			}
		}
	}
	fmt.Println(min)
}

// the idea is basically function composition, if we have two maps a -> b and b -> c
// each represented by a list of ranges l1 and l2
// we can create an equivalent map a -> c by computing all the non-empty intersections of a range in l1 with a range in l2
func part2_fast(seeds []int, maps []Map) {
	maxEnd := 0
	seedInvs := make([]Inv, 0)
	for i := 0; i < len(seeds); i += 2 {
		start, end := seeds[i], seeds[i]+seeds[i+1]-1
		if end > maxEnd {
			maxEnd = end
		}
		seedInvs = append(seedInvs, Inv{start: start, end: end})
	}
	for i, m := range maps {
		maps[i].ranges = completeRanges(m.ranges, maxEnd)
	}

	curr := maps[0]
	// pruning maps that do not contain any of the relevant seeds in the input
	// does reduce the number of ranges in each step
	// but in terms of runtime for the given problem size it doesn't matter
	curr.Prune(seedInvs)
	for i := 1; i < len(maps); i++ {
		fmt.Println("combining map", i, "with size", len(curr.ranges))
		curr = combineMaps(curr, maps[i])
		curr.Prune(seedInvs)
	}

	minLoc := 1 << 60
	for _, iv := range seedInvs {
		for _, m := range curr.ranges {
			ok, oStart, _ := overlap(m.srcStart, m.srcEnd, iv.start, iv.end)
			if ok {
				z := m.dstStart + oStart - m.srcStart
				if z < minLoc {
					minLoc = z
				}
			}
		}
	}

	fmt.Println(minLoc)
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

func parseSeedList(s string) []int {
	return parseIntList(s[7:])
}

func parseMaps(lines []string) []Map {
	maps := make([]Map, 0)
	start := 0
	for i := 1; i < len(lines); i++ {
		if i == len(lines)-1 || lines[i] == "" {
			end := i - 1
			if i == len(lines)-1 {
				end = len(lines) - 1
			}
			maps = append(maps, parseMap(lines[start:end+1]))
			start = i + 1
		}
	}
	return maps
}

func parseMap(lines []string) Map {
	srcName, dstName := parseMapName(lines[0])
	ranges := make([]MapRange, 0)
	for j := 1; j < len(lines); j++ {
		ranges = append(ranges, parseMapRange(lines[j]))
	}
	sort.Slice(ranges, func(i, j int) bool {
		return ranges[i].srcStart < ranges[j].srcStart
	})
	return Map{
		srcName: srcName,
		dstName: dstName,
		ranges:  ranges,
	}
}

func parseMapName(s string) (string, string) {
	x, _, _ := strings.Cut(s, " ")
	parts := strings.Split(x, "-")
	source := parts[0]
	dest := parts[2]
	return source, dest
}

func parseMapRange(s string) MapRange {
	x := parseIntList(s)
	l := x[2]
	return MapRange{
		srcStart: x[1],
		srcEnd:   x[1] + l - 1,
		dstStart: x[0],
		dstEnd:   x[0] + l - 1,
	}
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

type Map struct {
	srcName string
	dstName string
	ranges  []MapRange
}

type MapRange struct {
	srcStart int
	srcEnd   int
	dstStart int
	dstEnd   int
}

func (m Map) Map(srcVal int) int {
	for _, r := range m.ranges {
		if r.srcStart <= srcVal && (srcVal <= r.srcEnd || r.srcEnd == -1) {
			offset := srcVal - r.srcStart
			return r.dstStart + offset
		} else if r.srcStart > srcVal {
			break
		}
	}
	return srcVal
}

func (m Map) MapBS(srcVal int) int {
	low := 0
	high := len(m.ranges) - 1
	for low <= high {
		mid := (low + high) / 2
		r := m.ranges[mid]
		if r.srcStart <= srcVal && (srcVal <= r.srcEnd || r.srcEnd == -1) {
			offset := srcVal - r.srcStart
			return r.dstStart + offset
		} else if r.srcStart > srcVal {
			high = mid - 1
		} else {
			low = mid + 1
		}
	}
	return srcVal
}

func (m *Map) Prune(ivs []Inv) {
	ranges := make([]MapRange, 0)
	for _, r := range m.ranges {
		overlaps := false
		for _, iv := range ivs {
			ok, _, _ := overlap(r.srcStart, r.srcEnd, iv.start, iv.end)
			if ok {
				overlaps = true
				break
			}
		}
		if overlaps {
			ranges = append(ranges, r)
		}
	}
	m.ranges = ranges
}

type Inv struct {
	start int
	end   int
}

func completeRanges(x []MapRange, end int) []MapRange {
	result := make([]MapRange, 0)
	if x[0].srcStart > 0 {
		result = append(result, MapRange{
			srcStart: 0,
			srcEnd:   x[0].srcStart - 1,
			dstStart: 0,
			dstEnd:   x[0].srcStart - 1,
		})
	}
	for i, r := range x {
		result = append(result, r)
		if i+1 < len(x) {
			r2 := x[i+1]
			if r.srcEnd+1 < r2.srcStart {
				result = append(result, MapRange{
					srcStart: r.srcEnd + 1,
					srcEnd:   r2.srcStart - 1,
					dstStart: r.srcEnd + 1,
					dstEnd:   r2.srcStart - 1,
				})
			}
		} else {
			if r.srcEnd < end {
				result = append(result, MapRange{
					srcStart: r.srcEnd + 1,
					srcEnd:   end,
					dstStart: r.srcEnd + 1,
					dstEnd:   end,
				})
			}
		}
	}

	return result
}

func combineMaps(m1, m2 Map) Map {
	result := Map{srcName: m1.srcName, dstName: m2.dstName}
	ranges := make([]MapRange, 0)
	for _, r1 := range m1.ranges {
		for _, r2 := range m2.ranges {
			ok, start, end := overlap(r1.dstStart, r1.dstEnd, r2.srcStart, r2.srcEnd)
			if ok {
				srcStart, srcEnd := shrink(r1.srcStart, r1.srcEnd, r1.dstStart, r1.dstEnd, start, end)
				dstStart, dstEnd := shrink(r2.dstStart, r2.dstEnd, r2.srcStart, r2.srcEnd, start, end)
				ranges = append(ranges, MapRange{
					srcStart: srcStart,
					srcEnd:   srcEnd,
					dstStart: dstStart,
					dstEnd:   dstEnd,
				})
			}
		}
	}
	result.ranges = ranges
	return result
}

// we have intervals a = [aStart, aEnd] and b = [bStart, bEnd] of the same length
// [bSubStart, bSubEnd] is a subset of b
// return the corresponding subset of a
func shrink(aStart, aEnd, bStart, bEnd, bSubStart, bSubEnd int) (int, int) {
	offsetStart := bSubStart - bStart
	offsetEnd := bEnd - bSubEnd
	return aStart + offsetStart, aEnd - offsetEnd
}

// overlap between [a, b] and [c, d]
func overlap(a, b, c, d int) (bool, int, int) {
	if b < c {
		return false, -1, -1
	} else if d < a {
		return false, -1, -1
	} else {
		return true, max(a, c), min(b, d)
	}
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}
