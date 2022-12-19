
fn main() {
    let mut all_moves: Vec<(i32, i32, i32)> = Vec::new();
    let mut current_pos = (0, 0);
    let mut max_pos = (0, 0);
    let mut min_pos = (0, 0);
    // read each line of stdin
    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        if line.is_empty() {
            break;
        }
        //parse command for line
        let command = parse_command(line[..].trim());
        all_moves.push(command);
        //execute command
        current_pos = apply_command(current_pos, command);
        //update max and min
        max_pos = get_max_pos(max_pos, current_pos);
        min_pos = get_min_pos(min_pos, current_pos);
    }

    // create playfield

    let mut playfield = create_playfield(min_pos, max_pos);

    current_pos = get_start_pos(min_pos);
    playfield[current_pos.1 as usize][current_pos.0 as usize] = true;
    let mut knots = create_knots(current_pos);

    // apply_move_to_playfield
    for move_ in all_moves {
        apply_move_to_playfield(&mut playfield, move_, &mut current_pos, &mut knots);
    }

    // count visited tiles
    let result = count_visited_tiles(&playfield);
    //print result
    println!("{}", result);
}

fn create_knots(start_position: (i32, i32)) -> [(i32, i32); 9] {
    let mut knots = [(0,0); 9];
    for i in 0..9 {
        knots[i] = (start_position.0, start_position.1);
    }
    knots
}

fn count_visited_tiles(playfield: &Vec<Vec<bool>>) -> i32 {
    let mut count = 0;
    for row in playfield {
        for tile in row {
            if *tile {
                count += 1;
            }
        }
    }
    count
}

fn create_playfield(min_pos: (i32, i32), max_pos: (i32, i32)) -> Vec<Vec<bool>> {
    let mut playfield = Vec::new();
    for _ in min_pos.1..max_pos.1 + 1{
        let mut row = Vec::new();
        for _ in min_pos.0..max_pos.0 +1 {
            row.push(false);
        }
        playfield.push(row);
    }
    playfield
}

fn apply_move_to_playfield(playfield: &mut Vec<Vec<bool>>, move_: (i32, i32, i32), head_pos: &mut (i32, i32), knots: &mut [(i32, i32);9]) {
    for _ in 0..move_.2 {
        *head_pos = move_head_once(*head_pos, (move_.0, move_.1));
        let mut prev_knot = *head_pos;

        for i in 0..knots.len() {
            let knot = knots[i];
            knots[i] = move_tail(prev_knot, knot);
            prev_knot = knots[i];
        }
        let last_knot = knots[knots.len() - 1];
        playfield[last_knot.1 as usize][last_knot.0 as usize] = true;
    }
}

fn still_adjecent(head_pos: (i32, i32), tail_pos: (i32, i32)) -> bool {
    let x_diff = (head_pos.0 - tail_pos.0).abs();
    let y_diff = (head_pos.1 - tail_pos.1).abs();
    x_diff <= 1 && y_diff <= 1
}

fn move_head_once(head_pos: (i32, i32), dir: (i32, i32)) -> (i32, i32) {
    (head_pos.0 + dir.0, head_pos.1 + dir.1)
}

fn move_tail(head_pos: (i32, i32), tail_pos: (i32, i32)) -> (i32, i32) {
    if still_adjecent(head_pos, tail_pos) {
        return tail_pos;
    }
    // get diff between head and tail
    let x_diff = head_pos.0 - tail_pos.0;
    let y_diff = head_pos.1 - tail_pos.1;


    if x_diff.abs() == y_diff.abs() {
        return (tail_pos.0 + x_diff.signum(), tail_pos.1 + y_diff.signum());
    } else if x_diff.abs() > y_diff.abs() {
        if x_diff > 0 {
            return (head_pos.0 - 1, head_pos.1);
        } else {
            return (head_pos.0 + 1, head_pos.1);
        }
    } else {
        if y_diff > 0 {
            return (head_pos.0, head_pos.1 - 1);
        } else {
            return (head_pos.0, head_pos.1 + 1);
        }
    }
}

fn parse_command(cmd: &str) -> (i32, i32, i32) {
    let mut iter = cmd.split_whitespace();
    let dir = iter.next().unwrap();
    let arg = iter.next().unwrap();
    let arg = arg.parse::<i32>().unwrap();
    if dir == "U" {
        (0, 1, arg)
    } else if dir == "D" {
        (0, -1, arg)
    } else if dir == "L" {
        (-1, 0, arg)
    } else if dir == "R" {
        (1, 0, arg)
    } else {
        panic!("Invalid direction");
    }
}

fn apply_command((x, y): (i32, i32), (dx, dy, size): (i32, i32, i32)) -> (i32, i32) {
    (x + dx * size, y + dy * size)
}

fn get_start_pos(min_pos: (i32, i32)) -> (i32, i32) {
    let x = - min_pos.0;
    let y = - min_pos.1;
    (x, y)
}

fn get_max_pos(current_pos: (i32, i32), previous_max: (i32, i32)) -> (i32, i32) {
    let (x, y) = current_pos;
    let (max_x, max_y) = previous_max;
    (x.max(max_x), y.max(max_y))
}

fn get_min_pos(current_pos: (i32, i32), previous_min: (i32, i32)) -> (i32, i32) {
    let (x, y) = current_pos;
    let (min_x, min_y) = previous_min;
    (x.min(min_x), y.min(min_y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, 1);
    }

    #[test]
    fn test_create_knots() {
        let start_position = (3, 2);
        let knots = create_knots(start_position);
        assert_eq!(knots.len(), 9);
        for knot in knots {
            assert_eq!(knot, start_position);
        }
    }

    #[test]
    fn test_count_visited_tiles() {
        let playfield = vec![
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
        ];
        assert_eq!(count_visited_tiles(&playfield), 9);
    }

    #[test]
    fn test_parse_command() {
        assert_eq!(parse_command("U 5"), (0, 1, 5));
        assert_eq!(parse_command("D 1"), (0, -1, 1));
        assert_eq!(parse_command("R 4"), (1, 0, 4));
        assert_eq!(parse_command("L 4"), (-1, 0, 4));
        assert_eq!(parse_command("L 40"), (-1, 0, 40));
    }

    #[test]
    fn test_apply_move_to_playfield() {
        let mut playfield = create_playfield((0, 0), (3, 3));
        let mut head_pos = (1, 1);
        let mut knots = create_knots(head_pos);
        apply_move_to_playfield(&mut playfield, (1, 0, 2), &mut head_pos, &mut knots);
        assert_eq!(head_pos, (3, 1));
        assert_eq!(knots, [(2, 1), (1, 1), (1, 1), (1, 1), (1, 1), (1, 1), (1, 1), (1, 1), (1, 1)]);
    }

    #[test]
    fn test_apply_move_to_playfield_right_and_down() {
        let mut playfield = create_playfield((0, 0), (12, 12));
        let mut head_pos = (0, 0);
        let mut knots = create_knots(head_pos);
        apply_move_to_playfield(&mut playfield, (1, 0, 2), &mut head_pos, &mut knots);
        apply_move_to_playfield(&mut playfield, (0, 1, 12), &mut head_pos, &mut knots);
        assert_eq!(head_pos, (2, 12));
        assert_eq!(knots, [(2, 11), (2, 10), (2, 9), (2, 8), (2, 7), (2, 6), (2, 5), (2, 4), (2, 3)]);
    }

    #[test]
    fn test_apply_move_to_playfield_right_and_down_2() {
        let mut playfield = create_playfield((0, 0), (12, 12));
        let mut head_pos = (0, 0);
        let mut knots = create_knots(head_pos);
        apply_move_to_playfield(&mut playfield, (1, 0, 4), &mut head_pos, &mut knots);
        apply_move_to_playfield(&mut playfield, (0, 1, 4), &mut head_pos, &mut knots);
        assert_eq!(head_pos, (4, 4));
        assert_eq!(knots[0], (4, 3));
        assert_eq!(knots[1], (4, 2));
        assert_eq!(knots[2], (3, 2));
        assert_eq!(knots[3], (2, 2));
        assert_eq!(knots[4], (1, 1));
        assert_eq!(knots[5], (0, 0));
    }

    #[test]
    fn test_apply_move_to_playfield_updates_field() {
        let mut playfield = create_playfield((0, 0), (20, 20));
        let mut head_pos = (0, 0);
        let mut knots = create_knots(head_pos);
        apply_move_to_playfield(&mut playfield, (1, 0, 10), &mut head_pos, &mut knots);
        assert_eq!(knots[8], (1, 0));
        assert_eq!(true, playfield[0][0]);
        assert_eq!(true, playfield[0][1]);
        assert_eq!(false, playfield[0][2]);
    }

    #[test]
    fn test_get_max_pos() {
        assert_eq!(get_max_pos((1, 1), (0, 0)), (1, 1));
        assert_eq!(get_max_pos((1, 1), (2, 2)), (2, 2));
        assert_eq!(get_max_pos((4, 1), (1, 4)), (4, 4));
        assert_eq!(get_max_pos((1, 1), (1, 2)), (1, 2));
        assert_eq!(get_max_pos((1, 1), (2, 1)), (2, 1));

    }

    #[test]
    fn test_get_min_pos() {
        assert_eq!(get_min_pos((1, 1), (0, 0)), (0, 0));
        assert_eq!(get_min_pos((1, 1), (2, 2)), (1, 1));
        assert_eq!(get_min_pos((4, 1), (1, 4)), (1, 1));
        assert_eq!(get_min_pos((1, 1), (1, 2)), (1, 1));
        assert_eq!(get_min_pos((1, 1), (2, 1)), (1, 1));
    }

    #[test]
    fn test_get_start_pos() {
        assert_eq!(get_start_pos((0, 0)), (0, 0));
        assert_eq!(get_start_pos((-1, -1)), (1, 1));
        assert_eq!(get_start_pos((-1, 0)), (1, 0));
        assert_eq!(get_start_pos((0, -1)), (0, 1));
    }

    #[test]
    fn test_move_tail() {
        assert_eq!(move_tail((1, 0), (0, 0)), (0, 0));
        assert_eq!(move_tail((2, 0), (0, 0)), (1, 0));
        assert_eq!(move_tail((2, 1), (0, 0)), (1, 1));
        assert_eq!(move_tail((1, 2), (0, 0)), (1, 1));
        assert_eq!(move_tail((2, 2), (0, 0)), (1, 1));
    }
}
