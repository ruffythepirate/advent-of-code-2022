package main

import "regexp"
import "strconv"

type Move struct {
  from int
  to int
  num int
}

func getMoves(lines []string) []*Move {
  moveStartIndex := findNumberLineIndex(lines) + 2
  var moves []*Move
  for i := moveStartIndex; i < len(lines); i++ {
    move := parseMove(lines[i])
    moves = append(moves, move)
  }
  return moves
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
  reg := regexp.MustCompile(`^move ([0-9]+) from ([0-9]+) to ([0-9]+)$`)
  matches := reg.FindStringSubmatch(asString)
  from, _ := strconv.Atoi(matches[2])
  to, _ := strconv.Atoi(matches[3])
  num, _ := strconv.Atoi(matches[1])
  return &Move{from, to, num}
}

func applyMove(grid [][]byte, move *Move) [][]byte {
  toIndex := move.to - 1
  fromIndex := move.from - 1
  moveItems := grid[fromIndex][len(grid[fromIndex])-move.num:]
  grid[fromIndex] = grid[fromIndex][:len(grid[fromIndex])-move.num]
  arrayLen := len(moveItems)
  for i := 0; i < arrayLen; i++ {
    grid[toIndex] = append(grid[toIndex], moveItems[arrayLen - i - 1])
  }
  return grid
}

func applyMoveUpdated(grid [][]byte, move *Move) [][]byte {
  toIndex := move.to - 1
  fromIndex := move.from - 1
  moveItems := grid[fromIndex][len(grid[fromIndex])-move.num:]
  grid[fromIndex] = grid[fromIndex][:len(grid[fromIndex])-move.num]
  for i := 0; i < len(moveItems); i++ {
    grid[toIndex] = append(grid[toIndex], moveItems[i])
  }
  return grid
}
