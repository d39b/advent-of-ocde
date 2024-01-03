package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	lines := readLines()
	hands := make([]HandWithBid, len(lines))
	for i, line := range lines {
		hands[i] = parseHandWithBid(line)
	}
	part1(hands)
	part2(hands)
}

func part1(hands []HandWithBid) {
	sort.Slice(hands, func(i, j int) bool {
		h1, h2 := hands[i].hand, hands[j].hand
		if h1.ty < h2.ty {
			return true
		} else if h1.ty > h2.ty {
			return false
		} else {
			for k := 0; k < 5; k++ {
				if h1.cards[k] < h2.cards[k] {
					return true
				} else if h1.cards[k] > h2.cards[k] {
					return false
				}
			}
		}
		return true
	})

	result := computeWinnings(hands)
	fmt.Println(result)
}

func part2(hands []HandWithBid) {
	sort.Slice(hands, func(i, j int) bool {
		h1, h2 := hands[i].hand, hands[j].hand
		if h1.jokerTy < h2.jokerTy {
			return true
		} else if h1.jokerTy > h2.jokerTy {
			return false
		} else {
			for k := 0; k < 5; k++ {
				if h1.jokerCards[k] < h2.jokerCards[k] {
					return true
				} else if h1.jokerCards[k] > h2.jokerCards[k] {
					return false
				}
			}
		}
		return true
	})

	result := computeWinnings(hands)
	fmt.Println(result)
}

func computeWinnings(hands []HandWithBid) int {
	n := len(hands)
	result := 0
	for i, h := range hands {
		rank := n - i
		result += rank * h.bid
	}
	return result
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

type HandWithBid struct {
	hand Hand
	bid  int
}

func parseHandWithBid(x string) HandWithBid {
	handString, bidString, _ := strings.Cut(x, " ")
	hand := parseHand(handString)
	bid, err := strconv.Atoi(bidString)
	if err != nil {
		panic(err)
	}
	return HandWithBid{
		hand: hand,
		bid:  bid,
	}
}

func parseHand(hand string) Hand {
	cardCount := map[int]int{}
	var cards [5]int
	highCard := -1
	for i := 0; i < 5; i++ {
		card := cardToRank[hand[i]]
		cards[i] = card
		if card > highCard {
			highCard = card
		}
		cardCount[card] += 1
	}
	var ty int
	if len(cardCount) == 1 {
		ty = FIVE
	} else if len(cardCount) == 2 {
		for _, v := range cardCount {
			if v == 4 || v == 1 {
				ty = FOUR
			} else {
				ty = FULLHOUSE
			}
			break
		}
	} else if len(cardCount) == 3 {
		for _, v := range cardCount {
			if v == 3 {
				ty = THREE
				break
			} else if v == 2 {
				ty = TWOPAIR
				break
			}
		}
	} else if len(cardCount) == 4 {
		ty = ONEPAIR
	} else {
		ty = HIGHCARD
	}

	jokerCount := 0
	for _, c := range cards {
		if c == 3 {
			jokerCount += 1
		}
	}

	var jokerCards [5]int
	for i := 0; i < 5; i++ {
		jokerCards[i] = cardToRank2[hand[i]]
	}

	var jokerTy int
	if jokerCount == 0 {
		jokerTy = ty
	} else if len(cardCount) == 1 {
		jokerTy = FIVE
	} else if len(cardCount) == 2 {
		jokerTy = FIVE
	} else if len(cardCount) == 3 {
		if ty == THREE {
			jokerTy = FOUR
		} else if jokerCount == 2 {
			jokerTy = FOUR
		} else {
			jokerTy = FULLHOUSE
		}
	} else if len(cardCount) == 4 {
		jokerTy = THREE
	} else {
		jokerTy = ONEPAIR
	}

	return Hand{
		ty:         ty,
		jokerTy:    jokerTy,
		highCard:   highCard,
		cards:      cards,
		jokerCards: jokerCards,
	}
}

type Hand struct {
	ty         int
	jokerTy    int
	highCard   int
	cards      [5]int
	jokerCards [5]int
}

var cardToRank = map[byte]int{
	'A': 0,
	'K': 1,
	'Q': 2,
	'J': 3,
	'T': 4,
	'9': 5,
	'8': 6,
	'7': 7,
	'6': 8,
	'5': 9,
	'4': 10,
	'3': 11,
	'2': 12,
}

var cardToRank2 = map[byte]int{
	'A': 0,
	'K': 1,
	'Q': 2,
	'T': 3,
	'9': 4,
	'8': 5,
	'7': 6,
	'6': 7,
	'5': 8,
	'4': 9,
	'3': 10,
	'2': 11,
	'J': 12,
}

const (
	FIVE = iota
	FOUR
	FULLHOUSE
	THREE
	TWOPAIR
	ONEPAIR
	HIGHCARD
)
