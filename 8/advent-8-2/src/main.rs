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

    println!("To be done");
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
    let result: Vec<i8> = row.trim().chars().map(|c| c.to_digit(10).unwrap() as i8).collect::<Vec<i8>>();
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

pub fn build_visible_trees_map(grid: &Vec<Vec<i8>>, current_map: &mut Vec<Vec<i8>>, pos: (usize, usize), mutate_pos: fn(usize, usize) -> (usize, usize), can_mutate: impl Fn(usize, usize) -> bool) {
    panic!("To be done");
}

pub fn build_best_height_map(grid: &Vec<Vec<i8>>) -> Vec<Vec<i8>> {
    panic!("To be done");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_row() {
        let row = "123456789";
        let parsed_row = parse_row(row);
        assert_eq!(parsed_row, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
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
