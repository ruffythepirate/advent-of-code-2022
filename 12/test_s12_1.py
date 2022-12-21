from s12_1 import parse_height_map, init_fastest_path_map, find_start_position, is_move_allowed, populate_new_moves, find_end_position, populate_fastest_path_map

def test_parse_height_map():
    input_lines = ['..#', '##.', '...']
    expected = [['.', '.', '#'], ['#', '#', '.'], ['.', '.', '.']]
    assert parse_height_map(input_lines) == expected

def test_init_fastest_path_map():
    height_map = [['.', '.', '#'], ['#', '#', '.'], ['.', '.', '.']]
    expected = [[float('inf'), float('inf'), float('inf')], [float('inf'), float('inf'), float('inf')], [float('inf'), float('inf'), float('inf')]]
    assert init_fastest_path_map(height_map) == expected

def test_find_start_position():
    height_map = [['.', '.', '#'], ['#', '#', '.'], ['.', 'S', '.']]
    expected = (1, 2)
    assert find_start_position(height_map) == expected

def test_find_end_position():
    height_map = [['.', '.', '#'], ['#', '#', '.'], ['.', 'E', '.']]
    expected = (1, 2)
    assert find_end_position(height_map) == expected


def test_is_move_allowed():
    # Height diff is one, allowed
    height_map = [['a', 'b']]
    fastest_path_map = init_fastest_path_map(height_map)
    current_pos = (0, 0)
    move = (1, 0, 1)
    assert is_move_allowed(current_pos, move, height_map, fastest_path_map) == True

    # Height diff is two, not allowed
    height_map = [['a', 'c']]
    fastest_path_map = init_fastest_path_map(height_map)
    current_pos = (0, 0)
    move = (1, 0, 1)
    assert is_move_allowed(current_pos, move, height_map, fastest_path_map) == False

    # Height diff is negative, allowed
    height_map = [['e', 'c']]
    fastest_path_map = init_fastest_path_map(height_map)
    current_pos = (0, 0)
    move = (1, 0, 1)
    assert is_move_allowed(current_pos, move, height_map, fastest_path_map) == True

    # Index out of bound, not allowed
    height_map = [['e', 'c']]
    fastest_path_map = init_fastest_path_map(height_map)
    current_pos = (0, 0)
    move = (0, 1, 1)
    assert is_move_allowed(current_pos, move, height_map, fastest_path_map) == False

    # Faster path already exists, not allowed
    height_map = [['e', 'c']]
    fastest_path_map = [[0, 0]]
    current_pos = (0, 0)
    move = (0, 1, 1)
    assert is_move_allowed(current_pos, move, height_map, fastest_path_map) == False

def test_populate_new_moves():
    height_map = [['a', 'b', 'c'], ['d', 'e', 'f']]
    fastest_path_map = init_fastest_path_map(height_map)
    move = (0, 0, 0)
    moves = populate_new_moves(move, height_map, fastest_path_map)
    expected = [(1, 0, 1)]
    assert moves == expected

def test_populate_new_moves_not_diagonal():
    height_map = [['a', 'b', 'c'], ['a', 'b', 'c'], ['d', 'e', 'f']]
    fastest_path_map = init_fastest_path_map(height_map)
    fastest_path_map[0][0] = 0
    move = (0, 0, 0)
    moves = populate_new_moves(move, height_map, fastest_path_map)
    expected = [(0, 1, 1), (1, 0, 1)]
    assert moves == expected

def test_populate_fastest_path_map():
    height_map = [['a', 'b', 'c'], ['f', 'e', 'd']]
    start_pos = (0, 0)
    fastest_path_map = populate_fastest_path_map(start_pos, height_map)
    expected = [[0, 1, 2], [5, 4, 3]]
    assert fastest_path_map == expected
