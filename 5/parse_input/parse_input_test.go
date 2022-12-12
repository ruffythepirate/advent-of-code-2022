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


