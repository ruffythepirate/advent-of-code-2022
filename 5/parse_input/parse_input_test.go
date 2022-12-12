package parse_input

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



