package main

import "testing"

import "github.com/stretchr/testify/assert"

func Test_parse_sensor(t *testing.T) {
  assert := assert.New(t)
  assert.Equal(Sensor{98246, 1908027, 1076513, 2000000}, parse_sensor("Sensor at x=98246, y=1908027: closest beacon is at x=1076513, y=2000000"))
}

func Test_calc_manhattan_dist(t *testing.T) {
  assert := assert.New(t)
  assert.Equal(0, calc_manhattan_dist(0, 0, 0, 0))
  assert.Equal(1, calc_manhattan_dist(0, 0, 0, 1))
  assert.Equal(1, calc_manhattan_dist(0, 0, 1, 0))
  assert.Equal(2, calc_manhattan_dist(0, 0, 1, 1))
  assert.Equal(2, calc_manhattan_dist(0, 0, 1, -1))
  assert.Equal(2, calc_manhattan_dist(0, 0, -1, 1))
  assert.Equal(2, calc_manhattan_dist(0, 0, -1, -1))
}

func Test_prepare_sensor(t *testing.T) {
  assert := assert.New(t)
  assert.Equal(PreparedSensor{1, 1, 10}, prepare_sensor(Sensor{1, 1, 6, 6}))
}

func Test_is_within_range(t *testing.T) {
  assert := assert.New(t)
  assert.True(is_within_range(PreparedSensor{1, 1, 10}, 6, 6))
  assert.False(is_within_range(PreparedSensor{1, 1, 10}, 7, 7))
}
