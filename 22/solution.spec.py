import unittest
from solution import parse_map, parse_moves, parse_input, get_new_direction, get_direction_score

class TestSolution(unittest.TestCase):

    def test_parse_map(self):
        parsed_map = parse_map([
            '.....',
            '.....',
            '.....',
        ])

        self.assertEqual(parsed_map.width, 5)
        self.assertEqual(parsed_map.height, 3)

    def test_parse_moves(self):
        parsed_moves = parse_moves('1R20L5')

        self.assertEqual(parsed_moves[0].turn, None)
        self.assertEqual(parsed_moves[0].steps, 1)
        self.assertEqual(parsed_moves[1].turn, 'R')
        self.assertEqual(parsed_moves[1].steps, None)
        self.assertEqual(parsed_moves[2].turn, None)
        self.assertEqual(parsed_moves[2].steps, 20)

    def test_parse_input(self):
        map, moves = parse_input([
            '        ...#    ',
            '        .#..    ',
            '        #...    ',
            '        ....    ',
            '...#.......#    ',
            '........#...    ',
            '..#....#....    ',
            '..........#.    ',
            '        ...#....',
            '        .....#..',
            '        .#......',
            '        ......#.',
            '',
            '10R5L5R10L4R5L5'])
        self.assertEqual(map.width, 16)
        self.assertEqual(map.height, 12)
        self.assertEqual(moves[0].turn, None)
        self.assertEqual(moves[0].steps, 10)
        self.assertEqual(moves[1].turn, 'R')
        self.assertEqual(moves[1].steps, None)

    def test_map_get_start_coordinates(self):
        map = parse_map([
            '        ...#    ',
            '        .#..    ',
            '        #...    ',
            '        ....    ',
            '...#.......#    ',
            '........#...    ',
            '..#....#....    ',
            '..........#.    ',
            '        ...#....',
            '        .....#..',
            '        .#......',
            '        ......#.'])

        self.assertEqual(map.get_start_coordinates(), (8, 0))

    def test_map_get_coord_value(self):
        map = parse_map([
            '        ...#    ',
            '        .#..    ',
            '        #...    ',
            '        ....    ',
            '...#.......#    ',
            '........#...    ',
            '..#....#....    ',
            '..........#.    ',
            '        ...#....',
            '        .....#..',
            '        .#......',
            '        ......#.'])

        self.assertEqual(map.get_coord_value(8, 0), '.')

    def test_map_perform_move(self):
        map = parse_map([
            '        ...#    ',
            '        .#..    ',
            '        #...    ',
            '        ....    ',
            '...#.......#    ',
            '........#...    ',
            '..#....#....    ',
            '..........#.    ',
            '        ...#....',
            '        .....#..',
            '        .#......',
            '        ......#.'])
        x,y = map.get_start_coordinates()

        new_x, new_y = map.perform_move(x, y, 'S', 1)
        self.assertEqual(new_x, 8)
        self.assertEqual(new_y, 1)

    def test_map_perform_move_with_block(self):
        map = parse_map([
            '        ...#    ',
            '        .#..    ',
            '        #...    ',
            '        ....    ',
            '...#.......#    ',
            '........#...    ',
            '..#....#....    ',
            '..........#.    ',
            '        ...#....',
            '        .....#..',
            '        .#......',
            '        ......#.'])
        x,y = map.get_start_coordinates()

        new_x, new_y = map.perform_move(x, y, 'S', 5)

        self.assertEqual(new_x, 8)
        self.assertEqual(new_y, 1)

    def test_map_perform_move_with_wrapping(self):
        map = parse_map([
            '        ...#    ',
            '        .#..    ',
            '        #...    ',
            '        ....    ',
            '...#.......#    ',
            '........#...    ',
            '..#....#....    ',
            '..........#.    ',
            '        ...#....',
            '        .....#..',
            '        .#......',
            '        ......#.'])
        x,y = map.get_start_coordinates()

        new_x, new_y = map.perform_move(x, y, 'N', 5)

        self.assertEqual(new_x, 8)
        self.assertEqual(new_y, 7)

    def test_get_new_direction(self):
        self.assertEqual(get_new_direction('N', 'R'), 'E')
        self.assertEqual(get_new_direction('N', 'L'), 'W')
        self.assertEqual(get_new_direction('E', 'R'), 'S')
        self.assertEqual(get_new_direction('E', 'L'), 'N')
        self.assertEqual(get_new_direction('S', 'R'), 'W')
        self.assertEqual(get_new_direction('S', 'L'), 'E')
        self.assertEqual(get_new_direction('W', 'R'), 'N')
        self.assertEqual(get_new_direction('W', 'L'), 'S')

    def test_get_direction_score(self):
        self.assertEqual(get_direction_score('E'), 0)
        self.assertEqual(get_direction_score('S'), 1)
        self.assertEqual(get_direction_score('W'), 2)
        self.assertEqual(get_direction_score('N'), 3)

if __name__ == '__main__':
    unittest.main()

