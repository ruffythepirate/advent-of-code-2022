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

func main() {
  allLines := readLines()
  sensors := make([]PreparedSensor, len(allLines))
  for i, line := range allLines {
    sensors[i] = parse_prepared_sensor(line)
  }
  min_x := sensors[0].x - sensors[0].beacon_dist
  for _, sensor := range sensors {
    if sensor.x - sensor.beacon_dist < min_x {
      min_x = sensor.x - sensor.beacon_dist
    }
  }
  max_x := sensors[0].x + sensors[0].beacon_dist
  for _, sensor := range sensors {
    if sensor.x + sensor.beacon_dist > max_x {
      max_x = sensor.x + sensor.beacon_dist
    }
  }

  fmt.Println("min_x", min_x)
  fmt.Println("max_x", max_x)
  coord_in_range := 0
  for i := min_x; i < max_x; i++ {
    for j := 0; j < len(sensors); j++ {
      y_coord := 2000000
      //y_coord := 10
      if is_within_range(sensors[j], i, y_coord) {
        coord_in_range++
        sensors[j].covered_coords++
        break;
      }
    }
  }

  for _, sensor := range sensors {
    fmt.Println("Sensor", sensor.x, sensor.y, "dist", sensor.beacon_dist, "covered", sensor.covered_coords, "coords")
  }

  fmt.Println("coord_in_range", coord_in_range - 1)
}



