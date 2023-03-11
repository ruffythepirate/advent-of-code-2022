use std::io;
use std::io::Read;

fn main() {
    let mut move_list = read_input();
    let mut index_list = Vec::new();
    for i in 0..move_list.len() {
        index_list.push(i);
    }

    for i in 0..move_list.len() {
        move_index(&mut move_list, &mut index_list, i);
    }


    let zero_index = move_list.iter().position(|&r| r == 0).unwrap();
    let mut result = 0;
    let mut add_value = move_list[wrap_index((zero_index + 1000) as i32, move_list.len())];
    println!("first value: {}", add_value);
    result += add_value;
    add_value = move_list[wrap_index((zero_index + 2000) as i32, move_list.len())];
    println!("second value: {}", add_value);
    result += add_value;
    add_value = move_list[wrap_index((zero_index + 3000) as i32, move_list.len())];
    println!("third value: {}", add_value);
    result += add_value;

    println!("{}", result);
}

fn move_index(move_list: &mut Vec<i32>, index_list: &mut Vec<usize>, index: usize) {
    let movement = get_move(&move_list, &index_list, index);
    perform_move(move_list, index_list, movement);
}

// First we'll read the input, one i32 per line from stdin.
fn read_input() -> Vec<i32> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    input.lines().map(|s| s.parse().unwrap()).collect()
}


// We create a corresponding vec containing just the indexes.
// We calculate the move. 
//
// We perform a move from index to index. Perform a remove and then insert.

fn wrap_index(index: i32, len: usize) -> usize {
    if index < len as i32 && index >= 0 {
        index as usize
    } else if index >= len as i32 {
        index as usize % len
    } else {
        len - ((index.abs() as usize) % len)
    }
}

fn perform_move(move_list: &mut Vec<i32>, index_list: &mut Vec<usize>, movement: (usize, usize)) {
    let (from, to) = movement;
    let value = move_list.remove(from);
    move_list.insert(to, value);
    let value = index_list.remove(from);
    index_list.insert(to, value);
}

fn get_move(moves: &Vec<i32>, index_list: &Vec<usize>, index: usize) -> (usize, usize) {
    index_list.iter().position(|&v| v == index).map(|i| {
        let value = moves[i];
        let value_adjusted = value % (index_list.len() - 1) as i32;
        let value_sign = if value_adjusted < 0 { -1 } else { 1 };
        let new_index = wrap_index(i as i32 + value_adjusted, moves.len());
        if value_sign < 0 {
            if new_index == 0 {
                (i, moves.len() - 1)
            } else {
                (i, new_index - 1)
            }
        } else {
            if new_index < i {
                (i, new_index + 1)
            } else {
                (i, new_index)
            }
        }
    }).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perform_move() {
        let mut move_list = vec![1, 2, -3, 3, -2, 0, 4];
        let mut index_list = vec![0, 1, 2, 3, 4, 5, 6];
        move_index(&mut move_list, &mut index_list, 0);
        assert_eq!(move_list, vec![2, 1, -3, 3, -2, 0, 4]);
        assert_eq!(index_list, vec![1, 0, 2, 3, 4, 5, 6]);
        move_index(&mut move_list, &mut index_list, 1);
        assert_eq!(move_list, vec![1, -3, 2, 3, -2, 0, 4]);
        assert_eq!(index_list, vec![0, 2, 1, 3, 4, 5, 6]);
        move_index(&mut move_list, &mut index_list, 2);
        assert_eq!(move_list, vec![1, 2, 3, -2, -3, 0, 4]);
        assert_eq!(index_list, vec![0, 1, 3, 4, 2, 5, 6]);
        move_index(&mut move_list, &mut index_list, 3);
        assert_eq!(move_list, vec![1, 2, -2, -3, 0, 3, 4]);
        assert_eq!(index_list, vec![0, 1, 4, 2, 5, 3, 6]);
        move_index(&mut move_list, &mut index_list, 4);
        assert_eq!(move_list, vec![1, 2, -3, 0, 3, 4, -2]);
        assert_eq!(index_list, vec![0, 1, 2, 5, 3, 6, 4]);
        move_index(&mut move_list, &mut index_list, 6);
        assert_eq!(move_list, vec![1, 2, -3, 4, 0, 3, -2]);
        assert_eq!(index_list, vec![0, 1, 2, 6, 5, 3, 4]);
    }

    #[test]
    fn test_perform_long_move() {
        let mut move_list = vec![10, 2, -10, 3, -2, 0, 4];
        let mut index_list = vec![0, 1, 2, 3, 4, 5, 6];
        move_index(&mut move_list, &mut index_list, 0);
        assert_eq!(move_list, vec![2, -10, 3, -2, 10, 0, 4]);
        assert_eq!(index_list, vec![1, 2, 3, 4, 0, 5, 6]);
        move_index(&mut move_list, &mut index_list, 2);
        assert_eq!(move_list, vec![2, 3, -2,-10, 10, 0, 4]);
        assert_eq!(index_list, vec![1, 3, 4, 2, 0, 5, 6]);
    }

    #[test]
    fn test_perform_wrap_move() {
        let mut move_list = vec![0, -2, 2];
        let mut index_list = vec![0, 1, 2];
        move_index(&mut move_list, &mut index_list, 1);
        assert_eq!(move_list, vec![0, -2, 2]);
        assert_eq!(index_list, vec![0, 1, 2]);
        move_index(&mut move_list, &mut index_list, 2);
        assert_eq!(move_list, vec![0, -2, 2]);
        assert_eq!(index_list, vec![0, 1, 2]);
    }
}

