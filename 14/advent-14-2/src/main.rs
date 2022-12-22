use std::io;
use std::io::BufRead;

/**
 * Reads all the shapes of rocks. These are baically i32 tuples that are separated by an arrow (->).
 */
fn read_all_rock_shapes(all_lines: Vec<&str>) -> Vec<Vec<(i32, i32)>> {
    let mut all_rock_shapes = Vec::new();
    for line in all_lines {
        let mut rock_shape = Vec::new();
        for point in line.split("->") {
            let mut point = point.trim().split(",");
            let x = point.next().unwrap().parse::<i32>().unwrap();
            let y = point.next().unwrap().parse::<i32>().unwrap();
            rock_shape.push((x, y));
        }
        all_rock_shapes.push(rock_shape);
    }
    all_rock_shapes
}

fn parse_rock_shape(line: &str) -> Vec<(i32, i32)> {
    let mut rock_shape = Vec::new();
    for point in line.split("->") {
        let mut point = point.trim().split(",");
        let x = point.next().unwrap().parse::<i32>().unwrap();
        let y = point.next().unwrap().parse::<i32>().unwrap();
        rock_shape.push((x, y));
    }
    rock_shape
}

/**
 * Identifies the max point of the shape.
 */
fn find_max_point(rock_shape: &Vec<(i32, i32)>) -> (i32, i32) {
    let mut max_x = 0;
    let mut max_y = 0;
    for &(x, y) in rock_shape {
        if x > max_x {
            max_x = x;
        }
        if y > max_y {
            max_y = y;
        }
    }
    (max_x, max_y)
}

fn find_max_point_for_shapes(rock_shapes: &Vec<Vec<(i32, i32)>>) -> (i32, i32) {
    let mut max_x = 0;
    let mut max_y = 0;
    for rock_shape in rock_shapes {
        let (x, y) = find_max_point(rock_shape);
        if x > max_x {
            max_x = x;
        }
        if y > max_y {
            max_y = y;
        }
    }
    (max_x, max_y)
}

/**
 * Creates a map for rock and air and sand. The shape has the minimum size needed, but included
 * coord 500,0 as well as to max and min points of the shape. The map is initially filled with air
 * which is represented by a dot (.).
 */
fn create_empty_rock_map(x_size: i32, y_size: i32) -> Vec<Vec<char>> {
    let mut rock_map = Vec::new();
    for _ in 0..y_size {
        let mut row = Vec::new();
        for _ in 0..x_size {
            row.push('.');
        }
        rock_map.push(row);
    }
    rock_map
}

/**
 * Fills the map with rock that is represented by a hash (#). The map is filled according to given
 * rock shapes.
 */
fn fill_rock_map(rock_map: &mut Vec<Vec<char>>, x_index_offset: i32, rock_shape: &Vec<(i32, i32)>) {
    let mut prev_coord = rock_shape[0];
    for i in 1..rock_shape.len() {
        let adjusted_prev_coord = (prev_coord.0 - x_index_offset, prev_coord.1);
        let adjusted_next_coord = (rock_shape[i].0 - x_index_offset, rock_shape[i].1);
        fill_rock_between_coords(rock_map, adjusted_prev_coord, adjusted_next_coord);
        prev_coord = rock_shape[i];
    }
}

/**
 * Fills the coordinates between these coordinates with rock.
 */
fn fill_rock_between_coords(rock_map: &mut Vec<Vec<char>>, (x1, y1): (i32, i32), (x2, y2): (i32, i32)) {
    let x_diff = x2 - x1;
    let y_diff = y2 - y1;

    let max_diff = if x_diff.abs() > y_diff.abs() {
        x_diff.abs()
    } else {
        y_diff.abs()
    };

    for i in 0..max_diff+1 {
        let x = x1 + (x_diff * i) / max_diff;
        let y = y1 + (y_diff * i) / max_diff;
        rock_map[y as usize][x as usize] = '#';
    }
}

/**
 * Prints the map.
 */
fn print_rock_map(rock_map: &Vec<Vec<char>>) {
    for row in rock_map {
        for c in row {
            print!("{}", c);
        }
        println!("");
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
enum SandStatus {
    Falling(i32, i32),
    Settled(i32, i32),
    OutOfBounds
}

/**
 * Drops a grain of sand. If it settles the function returns Some((x, y)) where x and y are the
 * coordinates where it settled. If it does not settle the function returns None.
 */
fn drop_sand_grain(rock_map: &mut Vec<Vec<char>>, x: i32, y: i32) -> Option<(i32, i32)> {
    let mut current_state = SandStatus::Falling(x as i32, y as i32);
    loop {
        match current_state {
            SandStatus::Falling(x, y) => { current_state = find_next_pos_for_sand_grain(rock_map, x, y); },
            SandStatus::Settled(x, y) => return Some((x, y)),
            SandStatus::OutOfBounds => return None,
        }
    }
}


/**
 * Checks where the next position is for a falling grain of sand. If it is out of bounds the function
 * returns SandStatus::OutOfBounds. If it is settled the function returns SandStatus::Settled. If it
 * is still falling the function returns SandStatus::Falling.
 */
fn find_next_pos_for_sand_grain(rock_map: &Vec<Vec<char>>, x: i32, y: i32) -> SandStatus {
    let x_usize = x as usize;
    let y_usize = y as usize;
    if y_usize == rock_map.len() - 1 {
        SandStatus::OutOfBounds
    } else if rock_map[y_usize +1][x_usize] != '.' {
        // check left
        if x == 0 {
            SandStatus::OutOfBounds
        } else if rock_map[y_usize+1][x_usize-1] == '.' {
            SandStatus::Falling(x-1, y+1)
        } 
        // check right
        else if x_usize == rock_map[0].len() - 1 {
            SandStatus::OutOfBounds
        } else if rock_map[y_usize+1][x_usize+1] == '.' {
            SandStatus::Falling(x+1, y+1)
        } else {
            SandStatus::Settled(x, y)
        }
    } else {
        SandStatus::Falling(x, y + 1)
    }
}

/**
 * Identifies the min point of the shape.
 */
fn find_min_point(rock_shape: &Vec<(i32, i32)>) -> (i32, i32) {
    let mut min_x = i32::max_value();
    let mut min_y = i32::max_value();
    for &(x, y) in rock_shape {
        if x < min_x {
            min_x = x;
        }
        if y < min_y {
            min_y = y;
        }
    }
    (min_x, min_y)
}

fn find_min_point_for_shapes(rock_shapes: &Vec<Vec<(i32, i32)>>) -> (i32, i32) {
    let mut min_x = i32::max_value();
    let mut min_y = i32::max_value();
    for rock_shape in rock_shapes {
        let (x, y) = find_min_point(rock_shape);
        if x < min_x {
            min_x = x;
        }
        if y < min_y {
            min_y = y;
        }
    }
    (min_x, min_y)
}

fn main() {
    // read input from stdin. line for line and parse it as rock shapes.
    let mut rock_shapes = Vec::new();
    let mut line = String::new();
    let stdin = io::stdin();
    for line_result in stdin.lock().lines() {
        let line = line_result.unwrap();
        let rock_shape = parse_rock_shape(&line);
        rock_shapes.push(rock_shape);
    }

    // find the min point of all rock shapes
    let min_point = find_min_point_for_shapes(&rock_shapes);
    let max_point = find_max_point_for_shapes(&rock_shapes);

    // We now include the floor here, hence + 2
    let y_diff = 1+max_point.1 + 2;
    // create map
    let x_diff = 3 + 2 * y_diff;

    // We now make the x_offset to be y_diff. This is because this is the max amount of space
    // possibly needed.
    let x_offset = 500 - y_diff;


    // This adds the floor to the map.
    rock_shapes.push(vec![(x_offset, y_diff - 1), (x_offset + x_diff - 1, y_diff - 1)]);

    let mut rock_map = create_empty_rock_map(x_diff, y_diff);

    // fill map with rock
    for rock_shape in rock_shapes {
        fill_rock_map(&mut rock_map, x_offset, &rock_shape);
    }

    let sand_drop_coord = (500 - x_offset, 0);
    let mut sand_grains = 0;
    loop {
        let pos = drop_sand_grain(&mut rock_map, sand_drop_coord.0, sand_drop_coord.1);
        match pos {
            Some((x, y)) => {
                rock_map[y as usize][x as usize] = '0';
                if x == sand_drop_coord.0 && y == sand_drop_coord.1 {
                    sand_grains += 1;
                    break;
                }
            },
            None => {
                print_rock_map(&rock_map);
                panic!("Sand grain fell out of bounds!");
            }
        }
        sand_grains += 1;
    }

    println!("Number of sand grains: {}", sand_grains);
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drop_sand_grain() {
        let mut rock_map = create_empty_rock_map(10, 10);
        fill_rock_map(&mut rock_map, 0, &vec![(0, 0), (9, 0), (9, 9), (0, 9)]);
        print_rock_map(&rock_map);
        let result = drop_sand_grain(&mut rock_map, 5, 0);
        assert_eq!(result, Some((5, 8)));
    }
    #[test]
    fn test_find_next_pos_for_sand_grain() {
        let mut rock_map = create_empty_rock_map(10, 10);
        rock_map[9][2] = '#';
        rock_map[9][1] = '#';
        rock_map[9][0] = '#';

        assert_eq!(find_next_pos_for_sand_grain(&rock_map, 1, 7), SandStatus::Falling(1, 8));
        assert_eq!(find_next_pos_for_sand_grain(&rock_map, 1, 8), SandStatus::Settled(1, 8));
        assert_eq!(find_next_pos_for_sand_grain(&rock_map, 3, 9), SandStatus::OutOfBounds);

        // test falling diagonally left
        rock_map[8][1] = '#';
        assert_eq!(find_next_pos_for_sand_grain(&rock_map, 1, 7), SandStatus::Falling(0, 8));
        rock_map[8][0] = '#';
        assert_eq!(find_next_pos_for_sand_grain(&rock_map, 1, 7), SandStatus::Falling(2, 8));
        rock_map[8][2] = '#';
        assert_eq!(find_next_pos_for_sand_grain(&rock_map, 1, 7), SandStatus::Settled(1, 7));
    }

    #[test]
    fn test_fill_rock_between_coords() {
        // given positive x diff
        let mut rock_map = create_empty_rock_map(10, 10);
        fill_rock_between_coords(&mut rock_map, (0, 0), (9, 0));
        assert_eq!(rock_map[0], vec!['#', '#', '#', '#', '#', '#', '#', '#', '#', '#']);

        // given negative x diff
        let mut rock_map = create_empty_rock_map(10, 10);
        fill_rock_between_coords(&mut rock_map, (9, 0), (0, 0));
        assert_eq!(rock_map[0], vec!['#', '#', '#', '#', '#', '#', '#', '#', '#', '#']);

        // given positive y diff
        let mut rock_map = create_empty_rock_map(10, 10);
        fill_rock_between_coords(&mut rock_map, (0, 0), (0, 9));
        assert_eq!(rock_map[0], vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '.']);
        assert_eq!(rock_map[9], vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '.']);

        // given negative y diff
        let mut rock_map = create_empty_rock_map(10, 10);
        fill_rock_between_coords(&mut rock_map, (0, 9), (0, 0));
        assert_eq!(rock_map[0], vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '.']);
        assert_eq!(rock_map[9], vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '.']);
    }

    #[test]
    fn test_fill_rock_map() {
        let mut rock_map = create_empty_rock_map(10, 10);
        let rock_shape = vec![(0, 0), (2, 0), (2, 2), (0, 2), (0,0)];
        fill_rock_map(&mut rock_map, 0, &rock_shape);
        assert_eq!(rock_map[0][0], '#');
        assert_eq!(rock_map[0][1], '#');
        assert_eq!(rock_map[0][2], '#');
        assert_eq!(rock_map[1][0], '#');
        assert_eq!(rock_map[2][0], '#');
        assert_eq!(rock_map[2][2], '#');
    }

    #[test]
    fn test_create_empty_rock_map() {
        let rock_map = create_empty_rock_map(10, 10);
        assert_eq!(rock_map.len(), 10);
        for row in rock_map {
            assert_eq!(row.len(), 10);
            for c in row {
                assert_eq!(c, '.');
            }
        }
    }

    #[test]
    fn test_read_all_rock_shapes() {
        let all_lines = vec![
            "1,24 -> 2,12 -> 3,12 -> 4,1",
            "2,12 -> 3,12 -> 4,1",
        ];
        let expected = vec![
            vec![(1, 24), (2, 12), (3, 12), (4, 1)],
            vec![(2, 12), (3, 12), (4, 1)],
        ];
        assert_eq!(read_all_rock_shapes(all_lines), expected);
    }

    #[test]
    fn test_find_max_point() {
        let rock_shape = vec![(1, 24), (2, 12), (3, 12), (4, 1)];
        assert_eq!(find_max_point(&rock_shape), (4, 24));
    }

    #[test]
    fn test_find_min_point() {
        let rock_shape = vec![(1, 24), (2, 12), (3, 12), (4, 1)];
        assert_eq!(find_min_point(&rock_shape), (1, 1));
    }
}
