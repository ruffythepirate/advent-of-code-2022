package parse_input

import "os"
import "bufio"
import "regexp"
import "strconv"

type Move struct {
  from int
  to int
  num int
}

func readLines() []string {
  scanner := bufio.NewScanner(os.Stdin)
  var lines []string
  for scanner.Scan() {
    lines = append(lines, scanner.Text())
  }
  return lines
}


func identifyIndexesOfColumns(numberLine string) []int {
  var indexes []int
  for i, char := range numberLine {
    if char != ' ' {
      indexes = append(indexes, i)
    }
  }
  return indexes
}

func readCrates(lines []string, crateIndex int) []byte {
  var crates []byte
  for i := len(lines)-1; i >= 0 ; i-- {
    if(crateIndex <= len(lines[i]) && lines[i][crateIndex] != ' ') {
      crates = append(crates, lines[i][crateIndex])
    }
  }
  return crates
}

func findNumberLineIndex(lines []string) int {
  for i, line := range lines {
    if line == "" {
      return i - 1
    }
  }
  return -1
}

func constructInitialGrid(lines []string) [][]byte {
  numberLineIndex := findNumberLineIndex(lines)
  columnIndexes := identifyIndexesOfColumns(lines[numberLineIndex])
  var grid [][]byte
  for i := 0; i < len(columnIndexes); i++ {
    crates := readCrates(lines[:numberLineIndex], columnIndexes[i])
    grid = append(grid, crates)
  }
  return grid
}

func parseMove(asString string) *Move {
  // TODO
  reg := regexp.MustCompile(`^move ([0-9]+) from ([0-9]+) to ([0-9]+)$`)
  matches := reg.FindStringSubmatch(asString)
  from, _ := strconv.Atoi(matches[2])
  to, _ := strconv.Atoi(matches[3])
  num, _ := strconv.Atoi(matches[1])
  return &Move{from, to, num}
}

