use std::io;

fn main() {
    
    let mut current_command: Command = read_next_command().unwrap();
    let mut x = 1;
    let mut current_cycle = 0;
    let mut current_score = 0;
    loop {
        current_cycle += 1;

        if is_sprite_visible(current_cycle, x) {
            print!("#");
        } else {
            print!(".");
        }

        if current_cycle % 40 == 0 {
            println!("");
        }

        perform_cycle(&mut current_command, &mut x);

        if current_command.cycles_left == 0 {
            // print cycle, x, score
            match read_next_command() {
                Some(command) => current_command = command,
                None => break,
            }
        }
    }
}

fn count_signal_score(cycle: i32, x: i32) -> i32 {
    if (cycle - 20) % 40 == 0 {
        return  cycle * x;
    }
    0
}

fn read_next_command() -> Option<Command> {
    let mut input = String::new();
    let data_read = io::stdin().read_line(&mut input).ok()?;
    if data_read > 0 {
        let command = parse_command(&input);
        return Some(command)
    } else {
        return None;
    }
}

// struct command
struct Command {
    cycles_left: u8,
    x_diff: i32,
}

fn parse_command(line: &str) -> Command {
    let trimmed_line = line.trim();
    if trimmed_line == "noop" {
        return Command { cycles_left: 1, x_diff: 0 };
    } else if trimmed_line.starts_with("addx") {
        let x_diff = trimmed_line[5..].parse::<i32>().unwrap();
        return Command { cycles_left: 2, x_diff: x_diff };
    } else {
        panic!("Unknown command: {}", trimmed_line);
    }
}

fn perform_cycle(command: &mut Command, x: &mut i32) {
    if command.cycles_left > 0 {
        command.cycles_left -= 1;
    }
    if command.cycles_left == 0 {
        *x += command.x_diff;
    }
}

fn is_sprite_visible(cycle: i32, x: i32) -> bool {
    let cycle_modified = (cycle - 1) % 40;
    let diff = cycle_modified - x;

    if diff.abs() <= 1 {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_signal_score() {
        assert_eq!(count_signal_score(0, 0), 0);
        assert_eq!(count_signal_score(20, 5), 5*20);
        assert_eq!(count_signal_score(19, 5), 0);
        assert_eq!(count_signal_score(60, 15), 15*60);
    }

    #[test]
    fn test_is_sprite_visible() {

        assert_eq!(is_sprite_visible(1, 1), true);
        assert_eq!(is_sprite_visible(2, 1), true);
        assert_eq!(is_sprite_visible(3, 1), true);
        assert_eq!(is_sprite_visible(4, 1), false);
        assert_eq!(is_sprite_visible(41, 0), true);
        assert_eq!(is_sprite_visible(3, 0), false);

    }

    #[test]
    fn test_perform_cycle() {
        let mut command = parse_command("addx 5");
        let mut x = 0;
        perform_cycle(&mut command, &mut x);
        assert_eq!(command.cycles_left, 1);
        assert_eq!(x, 0);
        perform_cycle(&mut command, &mut x);
        assert_eq!(command.cycles_left, 0);
        assert_eq!(x, 5);
    }

    #[test]
    fn test_parse_command() {
        let command = parse_command("noop");
        assert_eq!(command.cycles_left, 1);
        assert_eq!(command.x_diff, 0);

        let command = parse_command("addx 15");
        assert_eq!(command.cycles_left, 2);
        assert_eq!(command.x_diff, 15);
    }
}

