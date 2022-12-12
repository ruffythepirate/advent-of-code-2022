package main

import "testing"

import "github.com/stretchr/testify/assert"

func Test_identifyIndexesOfColumns(t *testing.T) {
    // given
    numberLine := "  1 2 3 4 5"
    // when
    indexes := identifyIndexesOfColumns(numberLine)
    // then
    assert.Equal(t, []int{2, 4, 6, 8, 10}, indexes)
}

func Test_readCrates(t *testing.T) {
    // given

    lines := []string{
        "    [H][C]",
        "    [G][C]",
        " [C][F][C]",
        " [B][E][C]",
        " [A][D][C]",
    }
    crateIndex := 2
    // when
    crates := readCrates(lines, crateIndex)
    // then
    assert.Equal(t, []byte{'A', 'B', 'C'}, crates)

    crateIndex = 5
    // when
    crates = readCrates(lines, crateIndex)
    // then
    assert.Equal(t, []byte{'D', 'E', 'F', 'G', 'H'}, crates)
}

func Test_findNumberLineIndex(t *testing.T) {
    // given
    lines := []string{
        "    [H][C]",
        "    [G][C]",
        " [C][F][C]",
        " [B][E][C]",
        " [A][D][C]",
        "  1 2 3 4 5",
        "",
    }
    // when
    index := findNumberLineIndex(lines)
    // then
    assert.Equal(t, 5, index)
}

func Test_constructInitialGrid(t *testing.T) {
    // given
    lines := []string{
        "    [H][C]",
        "    [G][C]",
        " [C][F][C]",
        " [B][E][C]",
        " [A][D][C]",
        "  1  2  3 ",
        "",
    }
    // when
    grid := constructInitialGrid(lines)

    // then
    assert.Equal(t, [][]byte{
        {'A', 'B', 'C'},
        {'D', 'E', 'F', 'G', 'H'},
        {'C', 'C', 'C', 'C', 'C'},
    }, grid)
}

func Test_parseMove(t *testing.T) {
    // given
    moveAsString := "move 1 from 2 to 3"
    // when
    move := parseMove(moveAsString)
    // then
    assert.Equal(t, &Move{2, 3, 1}, move)
}

func Test_applyMove(t *testing.T) {
    // given
    grid := [][]byte{
        {'A', 'B', 'C'},
        {'D', 'E', 'F', 'G', 'H'},
        {'C', 'C', 'C', 'C', 'C'},
    }
    move := &Move{2, 3, 1}
    // when
    newGrid := applyMove(grid, move)
    // then
    assert.Equal(t, [][]byte{
        {'A', 'B', 'C'},
        {'D', 'E', 'F', 'G'},
        {'C', 'C', 'C', 'C', 'C', 'H'},
    }, newGrid)
}

func Test_applyMove_multiSteps(t *testing.T) {
    // given
    grid := [][]byte{
        {'A', 'B', 'C'},
        {'D', 'E', 'F', 'G', 'H'},
        {'C', 'C', 'C', 'C', 'C'},
    }
    move := &Move{2, 3, 4}
    // when
    newGrid := applyMove(grid, move)
    // then
    assert.Equal(t, [][]byte{
        {'A', 'B', 'C'},
        {'D'},
        {'C', 'C', 'C', 'C', 'C', 'H', 'G', 'F', 'E'},
    }, newGrid)
}

func Test_applyMoveUpdated(t *testing.T) {
    // given
    grid := [][]byte{
        {'A', 'B', 'C'},
        {'D', 'E', 'F', 'G', 'H'},
        {'C', 'C', 'C', 'C', 'C'},
    }
    move := &Move{2, 3, 3}
    // when
    newGrid := applyMoveUpdated(grid, move)
    // then
    assert.Equal(t, [][]byte{
        {'A', 'B', 'C'},
        {'D', 'E'},
        {'C', 'C', 'C', 'C', 'C', 'F', 'G', 'H'},
    }, newGrid)
}

func Test_getMoves(t *testing.T) {
    // given
    lines := []string{
        "    [H][C]",
        "    [G][C]",
        " [C][F][C]",
        " [B][E][C]",
        " [A][D][C]",
        "  1  2  3 ",
        "",
        "move 1 from 2 to 3",
        "move 2 from 3 to 1",
        "move 3 from 1 to 2",
    }
    // when
    moves := getMoves(lines)
    // then
    assert.Equal(t, []*Move{
        {2, 3, 1},
        {3, 1, 2},
        {1, 2, 3},
    }, moves)
}

