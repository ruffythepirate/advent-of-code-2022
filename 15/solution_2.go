package main

import "os"
import "bufio"
import "fmt"


func readLines() []string {
  scanner := bufio.NewScanner(os.Stdin)
  var lines []string
  for scanner.Scan() {
    lines = append(lines, scanner.Text())
  }
  return lines
}

func min(a, b int) int {
  if a < b {
    return a
  }
  return b
}

func main() {
  allLines := readLines()
  sensors := make([]PreparedSensor, len(allLines))
  for i, line := range allLines {
    sensors[i] = parse_prepared_sensor(line)
  }

  max_x := 4000001

  for y := 0; y < max_x; y++ {
    for x := 0; x < max_x; x++ {
      is_in_range := false
      for j := 0; j < len(sensors); j++ {
        if is_within_range(sensors[j], x, y) {
          x = sensors[j].x + sensors[j].beacon_dist - abs(y - sensors[j].y)
          is_in_range = true
          break;
        }
      }
      if !is_in_range {
        fmt.Println("x", x, "y", y, "is not in range")
        signal_value := x * 4000000 + y
        fmt.Println("signal value", signal_value)

      }
    }
  }
}



