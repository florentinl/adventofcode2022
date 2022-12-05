package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	dat, err := os.ReadFile("input")
	if err != nil {
		panic(err)
	}
	lines := strings.Split(string(dat), "\n")
	lines = lines[:len(lines)-1]
	value1 := 0
	value2 := 0
	for _, line := range lines {
		ranges := strings.Split(line, ",")
		elf1 := strings.Split(ranges[0], "-")
		elf2 := strings.Split(ranges[1], "-")
		a, _ := strconv.Atoi(elf1[0])
		b, _ := strconv.Atoi(elf1[1])
		c, _ := strconv.Atoi(elf2[0])
		d, _ := strconv.Atoi(elf2[1])
		if (a <= c && d <= b) || (c <= a && b <= d) {
			value1++
		}
		if b >= c && d >= a {
			value2++
		}
	}
	fmt.Println(value1)
	fmt.Println(value2)
}
