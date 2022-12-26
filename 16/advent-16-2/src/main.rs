extern crate regex;

use std::io::BufRead;
use std::collections::HashMap;

use regex::Regex;

fn main() {
    let time_used_limit = 26;
    // Read from stdin and parse each line as a vertex.
    let mut vertices = read_input();

    let available_valves = vertices.values().filter(|v| v.valve_value > 0).count();
    println!("Available valves: {}", available_valves);

    let mut journeys = Vec::new();
    let first_journey = Journey {
            current_vertex_id: "AA".to_string(),
            elephant_current_vertex_id: "AA".to_string(),
            released_valves: Vec::new(),
            complete_journey: Vec::new(),
            score: 0,
            time_used: 0
        };
    journeys.push(first_journey);
    let mut current_journey: Journey;
    let mut best_score = 0;
    let mut max_time_used = 0;
    let mut best_scores = HashMap::new();
    while journeys.len() > 0 {
        current_journey = journeys.remove(0);
        let all_valves_released = current_journey.released_valves.len() >= available_valves;
        if current_journey.time_used >= time_used_limit {
            break;
        }
        if all_valves_released {
            continue;
        }

        if current_journey.time_used > max_time_used {
            max_time_used = current_journey.time_used;
            println!("Max time used: {}", max_time_used);
            println!("Best score: {}", best_score);
        }

        let next_journeys = evaluate_journey(&current_journey, &mut vertices, &mut best_scores);
        for next_journey in next_journeys {
            if next_journey.score > best_score {
                best_score = next_journey.score;
                println!("Journey for best score: {:?}", next_journey.complete_journey);
                println!("Released Valves: {:?}", next_journey.released_valves);

            }
            journeys.push(next_journey);
        }
    }
    // Loop until time_used_limit is reached.
    println!("{}", best_score);
}

fn read_input() -> HashMap<String, Vertex> {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let mut vertices = HashMap::new();
    while let Some(Ok(line)) = lines.next() {
        let vertex = parse_vertex(&line);

        vertices.insert(vertex.id.clone(), vertex);
    }
    vertices
}

struct Vertex {
    id: String,
    connected_to: Vec<String>,
    valve_value: i32,
    best_score: i32
}

struct Journey {
    current_vertex_id: String,
    elephant_current_vertex_id: String,
    released_valves: Vec<String>,
    complete_journey: Vec<(String, String)>,
    score: i32,
    time_used: i32
}

/**
 * Parses a string like "Valve BY has flow rate=11; tunnels lead to valves SP, HS, DN, KD, TK" into
 * a Vertex.
 */
fn parse_vertex(line: &str) -> Vertex {
    let re = Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.*)").unwrap();
    let caps = re.captures(line.trim()).unwrap();
    let id = caps.get(1).unwrap().as_str().to_string();
    let valve_value = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
    let connected_to = caps.get(3).unwrap().as_str().split(", ").map(|s| s.to_string()).collect();
    Vertex {
        id: id,
        connected_to: connected_to,
        valve_value: valve_value,
        best_score: 0
    }
}

/**
 * Evalulates which target journeys one might end up in given released valves and current position.
 */
fn get_possible_new_destinations(current_vertex: &Vertex, vertices: &HashMap<String, Vertex>, released_valves: &Vec<String>) -> Vec<String> {
    let mut possible_new_destination = Vec::new();
    let current_vertex_id = &current_vertex.id;
    if current_vertex.valve_value > 0 && !released_valves.contains(current_vertex_id) {
        possible_new_destination.push(current_vertex_id.clone());
    }
    for connected_to in &current_vertex.connected_to {
        possible_new_destination.push(connected_to.clone());
    }
    possible_new_destination
}

fn cartesian_product(a: &Vec<String>, b: &Vec<String>) -> Vec<(String, String)> {
    let mut result = Vec::new();
    for a_item in a {
        for b_item in b {
            result.push((a_item.clone(), b_item.clone()));
        }
    }
    result
}

fn get_key_id(journey: &Journey) -> String {
    if(journey.current_vertex_id < journey.elephant_current_vertex_id) {
        journey.current_vertex_id.clone() + &journey.elephant_current_vertex_id
    } else {
        journey.elephant_current_vertex_id.clone() + &journey.current_vertex_id
    }
}

fn get_possible_journeys(current_journey: &Journey, vertices: &HashMap<String, Vertex>) -> Vec<Journey> {
    let player_destinations = get_possible_new_destinations(&vertices[&current_journey.current_vertex_id], vertices, &current_journey.released_valves);
    let mut extended_released_valves = current_journey.released_valves.clone();
    extended_released_valves.push(current_journey.current_vertex_id.clone());
    let elephant_destinations = get_possible_new_destinations(&vertices[&current_journey.elephant_current_vertex_id], vertices, &extended_released_valves);
    let possible_journeys = cartesian_product(&player_destinations, &elephant_destinations);
    let mut journeys = Vec::new();
    for journey in possible_journeys {
        let mut additional_score = 0;
        let mut new_released_valves = current_journey.released_valves.clone();
        if journey.0 == current_journey.current_vertex_id {
            additional_score += vertices[&journey.0].valve_value * (26 - current_journey.time_used - 1);
            new_released_valves.push(journey.0.clone());
        }
        if journey.1 == current_journey.elephant_current_vertex_id {
            additional_score += vertices[&journey.1].valve_value * (26 - current_journey.time_used - 1);
            new_released_valves.push(journey.1.clone());
        }
        let mut new_complete_journey = current_journey.complete_journey.clone();
        new_complete_journey.push(journey.clone());
        let new_journey = Journey {
            current_vertex_id: journey.0,
            elephant_current_vertex_id: journey.1,
            released_valves: new_released_valves,
            complete_journey: new_complete_journey,
            score: current_journey.score + additional_score,
            time_used: current_journey.time_used + 1
        };
        journeys.push(new_journey);
    }
    journeys
}

fn contains_unreleased_valve(journey: &Journey, vertices: &HashMap<String, Vertex>) -> bool {
    let current_unreleased =  vertices[&journey.current_vertex_id].valve_value > 0 || !journey.released_valves.contains(&journey.current_vertex_id);
    let elephant_unreleased =  vertices[&journey.elephant_current_vertex_id].valve_value > 0 || !journey.released_valves.contains(&journey.elephant_current_vertex_id);

    current_unreleased || elephant_unreleased
}

fn get_unreleased_valve_value(journey: &Journey, vertices: &HashMap<String, Vertex>) -> i32 {
    let mut unreleased_valve_value = 0;
    if vertices[&journey.current_vertex_id].valve_value > 0 && !journey.released_valves.contains(&journey.current_vertex_id) {
        unreleased_valve_value += vertices[&journey.current_vertex_id].valve_value * (26 - journey.time_used - 1);
    }
    if vertices[&journey.elephant_current_vertex_id].valve_value > 0 && !journey.released_valves.contains(&journey.elephant_current_vertex_id) {
        unreleased_valve_value += vertices[&journey.elephant_current_vertex_id].valve_value * (26 - journey.time_used - 1);
    }
    unreleased_valve_value
}


/**
 * Finds possible journeys for the given vertices. Updates the best score of the current vertex if 
 * the current journey has a higher score than the best score of that vertex so far.
 */
fn evaluate_journey(journey: &Journey, vertices: &mut HashMap<String, Vertex>, best_scores: &mut HashMap<String, i32> ) -> Vec<Journey> {
    let current_vertex = vertices.get_mut(&journey.current_vertex_id).unwrap();
    let possible_journeys = get_possible_journeys(journey, vertices);
    let mut filtered_journeys = Vec::new();
    for possible_journey in possible_journeys {
        let key_id = get_key_id(&possible_journey);
        let best_score = best_scores.get(&key_id);
        
        let unreleased_valve_value = get_unreleased_valve_value(&possible_journey, vertices);
        let potential_score = possible_journey.score + unreleased_valve_value;

        if best_score.is_none() || best_score.unwrap() < &potential_score {
            best_scores.insert(key_id, possible_journey.score);
            filtered_journeys.push(possible_journey);
        }
    }
    filtered_journeys
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cartesian_product() {
        let a = vec!["a".to_string(), "b".to_string()];
        let b = vec!["c".to_string(), "d".to_string()];
        let result = cartesian_product(&a, &b);
        assert_eq!(result, vec![
            ("a".to_string(), "c".to_string()),
            ("a".to_string(), "d".to_string()),
            ("b".to_string(), "c".to_string()),
            ("b".to_string(), "d".to_string())
        ]);
    }

    #[test]
    fn test_get_possible_journeys() {
        let mut vertices = HashMap::new();
        let vertex_aa = Vertex {
            id: "AA".to_string(),
            connected_to: vec!["BB".to_string(), "CC".to_string()],
            valve_value: 1,
            best_score: 0
        };
        vertices.insert("AA".to_string(), vertex_aa);
        let vertex_bb = Vertex {
            id: "BB".to_string(),
            connected_to: vec!["AA".to_string(), "CC".to_string()],
            valve_value: 0,
            best_score: 0
        };
        vertices.insert("BB".to_string(), vertex_bb);
        let vertex_cc = Vertex {
            id: "CC".to_string(),
            connected_to: vec!["AA".to_string(), "BB".to_string()],
            valve_value: 0,
            best_score: 0
        };
        vertices.insert("CC".to_string(), vertex_cc);
        let journey = Journey {
            current_vertex_id: "AA".to_string(),
            elephant_current_vertex_id: "AA".to_string(),
            released_valves: Vec::new(),
            score: 0,
            time_used: 0
        };
        let possible_journeys = get_possible_journeys(&journey, &vertices);
        assert_eq!(possible_journeys.len(), 6);
        assert_eq!(possible_journeys[0].current_vertex_id, "AA");
        assert_eq!(possible_journeys[0].elephant_current_vertex_id, "BB");
        assert_eq!(possible_journeys[1].current_vertex_id, "AA");
        assert_eq!(possible_journeys[1].elephant_current_vertex_id, "CC");
        assert_eq!(possible_journeys[2].current_vertex_id, "BB");
        assert_eq!(possible_journeys[2].elephant_current_vertex_id, "BB");
        assert_eq!(possible_journeys[3].current_vertex_id, "BB");
        assert_eq!(possible_journeys[3].elephant_current_vertex_id, "CC");
        assert_eq!(possible_journeys[4].current_vertex_id, "CC");
        assert_eq!(possible_journeys[4].elephant_current_vertex_id, "BB");
        assert_eq!(possible_journeys[5].current_vertex_id, "CC");
        assert_eq!(possible_journeys[5].elephant_current_vertex_id, "CC");
    }

    #[test]
    fn test_get_possible_new_destinations() {
        let mut vertices = HashMap::new();
        vertices.insert("A".to_string(), Vertex {
            id: "A".to_string(),
            connected_to: vec!["B".to_string(), "C".to_string()],
            valve_value: 1,
            best_score: 0
        });
        vertices.insert("B".to_string(), Vertex {
            id: "B".to_string(),
            connected_to: vec!["A".to_string(), "C".to_string()],
            valve_value: 1,
            best_score: 0
        });
        vertices.insert("C".to_string(), Vertex {
            id: "C".to_string(),
            connected_to: vec!["A".to_string(), "B".to_string()],
            valve_value: 1,
            best_score: 0
        });
        let journey = Journey {
            current_vertex_id: "A".to_string(),
            elephant_current_vertex_id: "A".to_string(),
            released_valves: vec![],
            score: 0,
            time_used: 0
        };
        let possible_new_destinations = get_possible_new_destinations(&vertices.get("A").unwrap(), &vertices, &journey.released_valves);
        assert_eq!(possible_new_destinations, vec!["A".to_string(), "B".to_string(), "C".to_string()]);
    }

    #[test]
    fn test_parse_vertex() {
        let line = "Valve BY has flow rate=11; tunnels lead to valves SP, HS, DN, KD, TK";
        let vertex = parse_vertex(line);
        assert_eq!(vertex.id, "BY");
        assert_eq!(vertex.valve_value, 11);
        assert_eq!(vertex.best_score, 0);
        assert_eq!(vertex.connected_to, vec!["SP", "HS", "DN", "KD", "TK"]);

        let line = "Valve BY has flow rate=11; tunnel leads to valve SP";
        let vertex = parse_vertex(line);
        assert_eq!(vertex.id, "BY");
        assert_eq!(vertex.valve_value, 11);
        assert_eq!(vertex.best_score, 0);
        assert_eq!(vertex.connected_to, vec!["SP"]);
    }

}
