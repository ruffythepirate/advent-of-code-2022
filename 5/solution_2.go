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

func main() {
  allLines := readLines()
  grid := constructInitialGrid(allLines)
  allMoves := getMoves(allLines)
  for _, move := range allMoves {
    grid = applyMoveUpdated(grid, move)
  }

  lastCrates := make([]byte, len(grid))
  for _, row := range grid {
    lastCrate := row[len(row)-1]
    lastCrates = append(lastCrates, lastCrate)
  }
  fmt.Println(string(lastCrates))
}

