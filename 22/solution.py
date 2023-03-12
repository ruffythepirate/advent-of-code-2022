import fileinput

def parse_input(all_lines):
    map_lines = []
    for line in all_lines:
        if line.strip() == '':
            break
        map_lines.append(line)
    map = parse_map(map_lines)
    moves = parse_moves(all_lines[len(all_lines) - 1])

    return map, moves


def parse_map(map_lines):
    return Map(map_lines)

def parse_moves(moves_line):
    accumulated_string = ''
    moves = []
    for char in moves_line:
        if char.isdigit():
            accumulated_string += char
        elif char in ['L', 'R']:
            moves.append(Move(None, int(accumulated_string) if accumulated_string else None))
            accumulated_string = ''
            moves.append(Move(char, int(accumulated_string) if accumulated_string else None))

    if accumulated_string.strip() != '':
        moves.append(Move(None, int(accumulated_string) if accumulated_string else None))
    return moves

class Move:
    def __init__(self, turn, steps):
        self.turn = turn
        self.steps = steps

class Robot:
    def __init__(self, x, y, direction):
        self.x = x
        self.y = y
        self.direction = direction

    def move(self, move):
        raise NotImplementedError

class Map:
    def __init__(self, map_lines):
        self.map_lines = map_lines
        self.width = len(map_lines[0])
        self.height = len(map_lines)

    def perform_move(self, x, y, direction, steps):
        last_x, last_y = x, y
        while steps > 0:
            next_x, next_y = self.get_next_coordinate_safe(x, y, direction)
            print('getting coord value for', next_x, next_y, self.width, self.height)
            coord_value = self.get_coord_value(next_x, next_y)
            if  coord_value == '#':
                return last_x, last_y
            elif coord_value == '.':
                x, y = next_x, next_y
                last_x, last_y = x, y
                steps -= 1
            else:
                x, y = next_x, next_y
        return  last_x, last_y

    def get_next_coordinate_safe(self, x, y, direction):
        new_x, new_y = self.get_next_coordinate(x, y, direction)
        if new_x < 0:
            return new_x + self.width, new_y
        elif new_x >= self.width:
            return new_x - self.width, new_y
        elif new_y < 0:
            return new_x, new_y + self.height
        elif new_y >= self.height:
            return new_x, new_y - self.height
        return new_x, new_y

    def get_next_coordinate(self, x, y, direction):
        if direction == 'N':
            return x, y - 1
        elif direction == 'S':
            return x, y + 1
        elif direction == 'E':
            return x + 1, y
        elif direction == 'W':
            return x - 1, y
        else:
            raise ValueError('Invalid direction')

    def get_coord_value(self, x, y):
        if len(self.map_lines[y]) <= x:
            return ' '
        return self.map_lines[y][x]

    def get_start_coordinates(self):
        for y, line in enumerate(self.map_lines):
            for x, char in enumerate(line):
                if char == '.':
                    return x, y

def get_new_direction(old_direction, turn):
    directions = ['N', 'E', 'S', 'W']
    direction_index = directions.index(old_direction)
    new_index = direction_index + 1 if turn == 'R' else direction_index - 1
    if new_index < 0:
        new_index = len(directions) - 1
    elif new_index >= len(directions):
        new_index = 0
    return directions[new_index]

def get_direction_score(direction):
    if direction == 'E':
        return 0
    elif direction == 'S':
        return 1
    elif direction == 'W':
        return 2
    elif direction == 'N':
        return 3
    else:
        raise ValueError('Invalid direction')

if __name__ == '__main__':
    all_lines = []
    for line in fileinput.input():
        all_lines.append(line)

    map, moves = parse_input(all_lines)


    x, y = map.get_start_coordinates()
    direction = 'E'
    for move in moves:
        if move.turn:
            direction = get_new_direction(direction, move.turn)
        else:
            x, y = map.perform_move(x, y, direction, move.steps)

    final_score = 1000 * (y + 1) + 4 * (x + 1) + get_direction_score(direction)
    print('final_score', final_score)



