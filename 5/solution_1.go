package main

import "fmt"
import "os"
import "bufio"

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
    if(crateIndex >= len(lines[i] && lines[i] != " ") {
    crates = append(crates, lines[i][crateIndex])
  }
  return crates
}

func findNumberLineIndex(lines []string) int {
  lastLineIndex := 0
  for i, line := range lines {
    if line == "" {
      return lastLineIndex = i - 1
    }
  }
  return lastLineIndex
}

/**
* Identify the crates and their positions.
*/
func constructInitialGrid(lines []string) [][]byte {
  numberLineIndex := findNumberLineIndex(lines)
  columnIndexes := identifyIndexesOfColumns(lines[numberLineIndex])
  var grid [][]byte
  for i := 0; i < len(columnIndexes); i++ {
    crates := readCrates(lines, columnIndexes[i])
    grid = append(grid, crates)
  }
}


func main() {
  scanner := bufio.NewScanner(os.Stdin)
  for scanner.Scan() {
    fmt.Println(scanner.Text())
    fmt.Println("Hello, World!")
  }
  fmt.Println("Hello, World!")

  // Read lines until empty line. Create slice of strings.
  // Find index based on last string. Read based on this index from bottom upwards and push crates into slices.
  // Finally we have our 2D model.
  // Create a model of the crates. 2D array with chars.

  // Interpret operations into a struct with num, from, to.
  // Pop the arrays and push them back in the right place.
  // Create a function that checks the top item in each array.
}

//Concepts Array / slice.  Array is fixed size. The slice is extendable?
