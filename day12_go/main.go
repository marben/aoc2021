package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
	"unicode"
)

type connectionsMap map[string][]string

func newConnections() connectionsMap {
	return map[string][]string{}
}

func (c connectionsMap) add(a, b string) {
	cs := c[a]
	c[a] = append(cs, b)
}

func loadInput(filename string) (connectionsMap, error) {
	f, err := os.Open(filename)
	if err != nil {
		return nil, fmt.Errorf("failed to open file '%s': %w", filename, err)
	}
	defer f.Close()

	conns := newConnections()

	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		l := scanner.Text()
		split := strings.Split(l, "-")
		if len(split) != 2 {
			return connectionsMap{}, fmt.Errorf("unexpected number of elementes on input line %q", l)
		}

		from, to := split[0], split[1]

		conns.add(from, to)
		conns.add(to, from)
	}
	if scanner.Err() != nil {
		return connectionsMap{}, fmt.Errorf("scanner failed with error %w", err)
	}

	return conns, nil
}

type path struct {
	pathSlice     []string
	doubleVisited bool // for part 2
}

func (p *path) add(cave string) {
	p.pathSlice = append(p.pathSlice, cave)
}

func (p path) last() string {
	return p.pathSlice[len(p.pathSlice)-1]
}

// we need to do a deep copy because of the slice inside
func (p path) copy() path {
	return path{pathSlice: append([]string{}, p.pathSlice...), doubleVisited: p.doubleVisited}
}

func (p path) contains(what string) bool {
	for _, v := range p.pathSlice {
		if v == what {
			return true
		}
	}
	return false
}

func findPathsPart1(conns connectionsMap) ([]path, error) {
	todo := []path{{pathSlice: []string{"start"}}}

	paths := []path{}

	for len(todo) > 0 {
		// pop last
		path := todo[len(todo)-1]
		todo = todo[:len(todo)-1]

		currCave := path.last()
		if currCave == "end" {
			paths = append(paths, path)
			continue
		}

		for _, conn := range conns[currCave] {
			if unicode.IsUpper(rune(conn[0])) || !path.contains(conn) {
				// we need to make a copy. append modifies the underlying slice
				pathCopy := path.copy()
				pathCopy.add(conn)
				todo = append(todo, pathCopy)
			}
		}
	}

	return paths, nil
}

func findPathsPart2(conns connectionsMap) ([]path, error) {
	todo := []path{{pathSlice: []string{"start"}}}

	paths := []path{}

	for len(todo) > 0 {
		// pop last
		path := todo[len(todo)-1]
		todo = todo[:len(todo)-1]

		currCave := path.last()
		if currCave == "end" {
			paths = append(paths, path)
			continue
		}

		for _, conn := range conns[currCave] {
			if conn == "start" {
				continue
			}
			if unicode.IsUpper(rune(conn[0])) || !path.contains(conn) {
				// we need to make a copy. append modifies the underlying slice
				pathCopy := path.copy()
				pathCopy.add(conn)
				todo = append(todo, pathCopy)
			} else if !path.doubleVisited {
				pathCopy := path.copy()
				pathCopy.add(conn)
				pathCopy.doubleVisited = true
				todo = append(todo, pathCopy)
			}
		}
	}

	return paths, nil
}

func main() {
	if err := run(); err != nil {
		log.Fatalf("run function failed with error: %v", err)
	}
}

func run() error {
	conns, err := loadInput("input")
	if err != nil {
		return fmt.Errorf("failed to load input %w", err)
	}

	// fmt.Printf("loaded input: %+v", conns)

	paths1, err := findPathsPart1(conns)
	if err != nil {
		return fmt.Errorf("find path 1 failed: %w", err)
	}

	// fmt.Printf("found paths: %+v\n", paths)
	fmt.Printf("number of paths part 1: %d\n", len(paths1))

	paths2, err := findPathsPart2(conns)
	if err != nil {
		return fmt.Errorf("find path 2 failed: %w", err)
	}

	// fmt.Printf("found paths: %+v\n", paths)
	fmt.Printf("number of paths part 2: %d\n", len(paths2))

	return nil
}
