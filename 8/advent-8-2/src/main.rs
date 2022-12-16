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

    let mut max_score = -1;
    for y in 0..grid.len() - 1 {
        for x in 0..grid[y].len() - 1 {
            let score = get_score(&grid, (x, y));
            max_score = cmp::max(max_score, score);
        }
    }

    println!("{}", max_score);
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

pub fn count_trees(grid: &Vec<Vec<i8>>, start_value: i8, current_count: i32, current_pos: (usize, usize), mutate_pos: fn(usize, usize) -> (usize, usize), can_mutate: impl Fn(usize, usize) -> bool) -> i32 {
    if !can_mutate(current_pos.0, current_pos.1) {
        return current_count;
    } else if current_count > 0 && grid[current_pos.1][current_pos.0] >= start_value {
        return current_count;
    } else {
        let new_pos = mutate_pos(current_pos.0, current_pos.1);
        return count_trees(grid, start_value, current_count + 1, new_pos, mutate_pos, can_mutate);
    }
}

pub fn get_score(grid: &Vec<Vec<i8>>, pos: (usize, usize)) -> i32 {
    let mut scores: [i32; 4] = [0, 0, 0, 0];
    scores[0] = count_trees(grid, grid[pos.1][pos.0], 0, pos, |x, y| (x + 1, y), |x, _| x < grid[0].len() - 1);
    scores[1] = count_trees(grid, grid[pos.1][pos.0], 0, pos, |x, y| (x - 1, y), |x, _| x > 0);
    scores[2] = count_trees(grid, grid[pos.1][pos.0], 0, pos, |x, y| (x, y + 1), |_, y| y < grid.len() - 1);
    scores[3] = count_trees(grid, grid[pos.1][pos.0], 0, pos, |x, y| (x, y - 1), |_, y| y > 0);
    scores.iter().product::<i32>()
    
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_score() {
        let grid = parse_grid(&vec![
            "7432",
            "7899",
            "7722",
            "7722",
        ]);

        assert_eq!(get_score(&grid, (1, 1)), 2);
        assert_eq!(get_score(&grid, (2, 1)), 4);
    }

    #[test]
    fn test_count_trees() {
        let grid = parse_grid(&vec![
            "74252223471",
        ]);
        let max = grid[0].len();
        let result = count_trees(&grid, grid[0][0], 0, (0, 0), |x, y| (x+1, y), |x, _| x < max - 1);
        assert_eq!(result, 9);

        let result = count_trees(&grid, grid[0][1], 0, (1, 0), |x, y| (x+1, y), |x, _| x < max - 1);
        assert_eq!(result, 2);
    }

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
