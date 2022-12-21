from typing import List, Tuple


def parse_height_map(input_lines: List[str]) -> List[List[str]]:
    """
    Parses each line of the input into a list of characters.
    """
    return [list(line) for line in input_lines]

def init_fastest_path_map(height_map: List[List[str]]) -> List[List[int]]:
    """
    Initializes a map with the same dimensions as the height map with Max Int values in each place.
    """
    return [[float('inf') for _ in row] for row in height_map]

def find_start_position(height_map: List[List[str]]) -> Tuple[int, int]:
    """
    Finds the starting position of the map. This is where 'S' is located.
    """
    for y, row in enumerate(height_map):
        for x, char in enumerate(row):
            if char == 'S':
                return (x, y)

def find_end_position(height_map: List[List[str]]) -> Tuple[int, int]:
    """
    Finds the end position of the map. This is where 'E' is located.
    """
    for y, row in enumerate(height_map):
        for x, char in enumerate(row):
            if char == 'E':
                return (x, y)

def populate_new_moves(move: Tuple[int, int, int], height_map: List[List[str]], fastest_path_map: List[List[int]]) -> List[Tuple[int, int, int]]:
    """
    Populates the new moves from the current move. New moves will be adjecent squares that have a char value that is not higher than the current move + 1. They must also be within the playfield, and are only accepted if the distance part of the the tuple (the third value) is lower than the value in the fastest path map.
    """
    new_moves = []
    for x in range(-1, 2):
        for y in range(-1, 2):
            if (x == 0 and y == 0) or x * y != 0:
                continue
            new_move = (move[0] + x, move[1] + y, move[2] + 1)
            if is_move_allowed(move, new_move, height_map, fastest_path_map):
                fastest_path_map[new_move[1]][new_move[0]] = min(new_move[2], fastest_path_map[new_move[1]][new_move[0]])
                new_moves.append(new_move)
    return new_moves

def is_move_allowed(current_pos: Tuple[int, int], move: Tuple[int, int, int], height_map: List[List[str]], fastest_path_map: List[List[int]]) -> bool:
    """
    Checks if the move is allowed. A move is allowed if the move is within the playfield, and the height of the move is not higher than the current position + 1 and the move is lower than the already available fastest path.
    """
    if move[0] < 0 or move[1] < 0:
        return False
    elif move[0] >= len(height_map[0]) or move[1] >= len(height_map):
        return False
    elif ord(height_map[move[1]][move[0]]) - ord(height_map[current_pos[1]][current_pos[0]]) > 1:
        return False
    elif move[2] >= fastest_path_map[move[1]][move[0]]:
        return False
    else:
        return True

def populate_fastest_path_map(start_pos: Tuple[int, int], height_map: List[List[str]]) -> List[List[int]]:
    """
    Populates the fastest path map with the shortest path from the start position to any given position.
    """
    moves = [(start_pos[0], start_pos[1], 0)]
    fastest_path_map = init_fastest_path_map(height_map)
    fastest_path_map[start_pos[1]][start_pos[0]] = 0
    while len(moves) > 0:
        move = moves.pop(0)
        new_moves = populate_new_moves(move, height_map, fastest_path_map)
        moves.extend(new_moves)
    return fastest_path_map

def read_input_lines_from_stdin() -> List[str]:
    """
    Reads input lines from stdin.
    """
    input_lines = []
    while True:
        try:
            input_lines.append(input())
        except EOFError:
            break
    return input_lines

def main():
    """
    Main function.
    """
    print("Reading input..")
    input_lines = read_input_lines_from_stdin()
    height_map = parse_height_map(input_lines)
    start_pos = find_start_position(height_map)
    end_pos = find_end_position(height_map)
    height_map[start_pos[1]][start_pos[0]] = 'a'
    height_map[end_pos[1]][end_pos[0]] = 'z'
    print("Populating fastest path map..")
    fastest_path_map = populate_fastest_path_map(start_pos, height_map)
    print(fastest_path_map[end_pos[1]][end_pos[0]])

if __name__ == '__main__':
    main()



