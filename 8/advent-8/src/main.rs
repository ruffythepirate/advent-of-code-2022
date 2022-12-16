use std::cmp;

fn main() {
    // read lines from input and fill vector with them
    let mut lines = Vec::new();
    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        if line.is_empty() {
            break;
        }
        lines.push(line);
    }
    let grid = parse_grid(&lines.iter().map(|l| &**l).collect());
    let min_map = build_min_height_map(&grid);
    let visible_trees = find_visible_tree_amount(&grid, &min_map);

    println!("Visible trees: {}", visible_trees);
}

pub fn parse_grid(input: &Vec<&str>) -> Vec<Vec<i8>> {
    let mut grid = Vec::new();
    for line in input {
        let row = parse_row(line);
        grid.push(row);
    }
    grid
}

pub fn parse_row(row: &str) -> Vec<i8>  {
    let result: Vec<i8> = row.trim().chars().map(|c| c.to_digit(10).unwrap() as i8.collect::<Vec<i8>>();
    result
}

pub fn build_empty_map(grid: &Vec<Vec<i8>>) -> Vec<Vec<i8>> {
    let mut map = Vec::new();
    for row in grid {
        let mut map_row = Vec::new();
        for _ in row {
            map_row.push(-1);
        }
        map.push(map_row);
    }
    map
}

pub fn build_height_map(grid: &Vec<Vec<i8>>, current_map: &mut Vec<Vec<i8>>, pos: (usize, usize), mutate_pos: fn(usize, usize) -> (usize, usize), can_mutate: impl Fn(usize, usize) -> bool) {
    if !can_mutate(pos.0, pos.1) {
        return;
    }
    let new_pos = mutate_pos(pos.0, pos.1);
    current_map[new_pos.1][new_pos.0] = cmp::max(grid[pos.1][pos.0], current_map[pos.1][pos.0]);
    build_height_map(grid, current_map, new_pos, mutate_pos, can_mutate);
}

pub fn find_visible_tree_amount(grid: &Vec<Vec<i8>>, min_height_map: &Vec<Vec<i8>>) -> i32 {
    let mut tree_amount = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] > min_height_map[y][x] {
                tree_amount += 1;
            }
        }
    }
    tree_amount
}

pub fn build_min_height_map(grid: &Vec<Vec<i8>>) -> Vec<Vec<i8>> {
    let mut left_map = build_empty_map(grid);
    let mut right_map = build_empty_map(grid);
    let mut top_map = build_empty_map(grid);
    let mut bottom_map = build_empty_map(grid);
    for i in 0..grid.len() {
        let max = grid.len();
        build_height_map(grid, &mut left_map, (0, i), |x, y| (x + 1, y), |x, _| x < max - 1);
        build_height_map(grid, &mut right_map, (grid.len() - 1, i), |x, y| (x - 1, y), |x, _| x > 0);
        build_height_map(grid, &mut top_map, (i, 0), |x, y| (x, y + 1), |_, y| y < max - 1);
        build_height_map(grid, &mut bottom_map, (i, grid.len() - 1), |x, y| (x, y - 1), |_, y| y > 0);
    }
    let mut min_map = build_empty_map(grid);
    for x in 0..grid[0].len() {
        for y in 0..grid.len() {
            let min_height = cmp::min(cmp::min(left_map[y][x], right_map[y][x]), cmp::min(top_map[y][x], bottom_map[y][x]));
            min_map[y][x] = min_height;
        }
    }
    min_map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_min_height_map() {
        let input = vec![
            "30373",
            "25512",
            "65332",
            "33549",
            "35390",
        ];
        let grid = parse_grid(&input);
        let min_map = build_min_height_map(&grid);
        let expected = vec![
            vec![-1, -1, -1, -1, -1],
            vec![-1, 0, 2, 2, -1],
            vec![-1, 3, 3, 2, -1],
            vec![-1, 3, 3, 5, -1],
            vec![-1, -1, -1, -1, -1],
        ];
        assert_eq!(min_map, expected);
    }

    #[test]
    fn test_find_visible_tree_amount() {
        let input = vec![
            "193",
            "999",
            "799"
        ];
        let grid = parse_grid(&input);
        let min_map = build_min_height_map(&grid);
        let tree_amount = find_visible_tree_amount(&grid, &min_map);
        assert_eq!(tree_amount, 8);
    }
    #[test]
    fn test_parse_row() {
        let row = "123456789";
        let parsed_row = parse_row(row);
        assert_eq!(parsed_row, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_build_height_map() {
        let grid = vec![
            vec![1, 2, 3],
            vec![6, 5, 4],
            vec![7, 5, 9],
        ];
        let mut map = build_empty_map(&grid);
        let pos = (0, 0);
        let max = grid[0].len();
        build_height_map(&grid, &mut map, pos, |x, y| (x+1, y), |x, _| x < max- 1);
        assert_eq!(map, vec![
            vec![-1, 1, 2],
            vec![-1, -1,-1],
            vec![-1, -1,-1],
        ]);
    }

    #[test]
    fn test_build_height_map_2() {
        let grid = vec![
            vec![2, 1, 3],
        ];
        let mut map = build_empty_map(&grid);
        let pos = (0, 0);
        let max = grid[0].len();
        build_height_map(&grid, &mut map, pos, |x, y| (x+1, y), |x, _| x < max- 1);
        assert_eq!(map, vec![
            vec![-1, 2, 2],
        ]);
    }

    #[test]
    fn test_parse_grid() {
        let rows = vec!(
            "123456789",
            "123456789",
            "123456789",
            "123456789",
        );
        let parsed_grid = parse_grid(&rows);
        assert_eq!(parsed_grid, vec![
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        ]);
    }

    #[test]
    fn test_build_empty_map() {
        let grid = vec![
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        ];
        let map = build_empty_map(&grid);

        assert_eq!(map, vec![
            vec![-1, -1, -1, -1, -1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1, -1, -1, -1, -1],
        ]);
    }
}
