package main

// import atoi and regexp.
import "strconv"
import "regexp"

type Sensor struct {
  x int
  y int
  beacon_x int
  beacon_y int
}

type PreparedSensor struct {
  x int
  y int
  beacon_dist int
  covered_coords int
}

func atoi(s string) int {
  i, _ := strconv.Atoi(s)
  return i
}

func abs(x int) int {
  if x < 0 {
    return -x
  }
  return x
}

// regex string that exttracts sensor and beacon coordinates
// Example string: Sensor at x=98246, y=1908027: closest beacon is at x=1076513, y=2000000
var re = regexp.MustCompile(`Sensor at x=(\d+), y=(\d+): closest beacon is at x=(-?\d+), y=(-?\d+)`)

func parse_sensor(input string) Sensor {
  matches := re.FindStringSubmatch(input)
  return Sensor{atoi(matches[1]), atoi(matches[2]), atoi(matches[3]), atoi(matches[4])}
}

func parse_prepared_sensor(input string) PreparedSensor {
  return prepare_sensor(parse_sensor(input))
}

func prepare_sensor(sensor Sensor) PreparedSensor {
  return PreparedSensor{sensor.x, sensor.y, calc_manhattan_dist(sensor.x, sensor.y, sensor.beacon_x, sensor.beacon_y), 0}
}

func calc_manhattan_dist(x1, y1, x2, y2 int) int {
  return abs(x1-x2) + abs(y1-y2)
}

func is_within_range(sensor PreparedSensor, beacon_x, beacon_y int) bool {
  return calc_manhattan_dist(sensor.x, sensor.y, beacon_x, beacon_y) <= sensor.beacon_dist
}
