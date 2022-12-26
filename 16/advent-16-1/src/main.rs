extern crate regex;

use std::io::BufRead;
use std::collections::HashMap;

use regex::Regex;

fn main() {
    let time_used_limit = 30;
    // Read from stdin and parse each line as a vertex.
    let mut vertices = read_input();

    let mut journeys = Vec::new();
    let first_journey = Journey {
            current_vertex_id: "AA".to_string(),
            released_valves: Vec::new(),
            score: 0,
            time_used: 0
        };
    let mut current_journey = first_journey;
    let mut best_score = 0;
    while current_journey.time_used < time_used_limit {
        let next_journeys = evaluate_journey(&current_journey, &mut vertices);
        for next_journey in next_journeys {
            if next_journey.score > best_score {
                best_score = next_journey.score;
            }
            journeys.push(next_journey);
        }
        current_journey = journeys.remove(0);
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
    released_valves: Vec<String>,
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
 * Checks possible journeys from a vertex. Journeys are only added if the target vertex hasn't been
 * opened yet, or if it has been opened but the current journey has a higher score than the best
 * score of that journey so far.
 */
fn get_possible_journeys(journey: &Journey, vertices: &HashMap<String, Vertex>) -> Vec<Journey> {
    let mut possible_journeys = Vec::new();
    let current_vertex = vertices.get(&journey.current_vertex_id).unwrap();
    if current_vertex.valve_value > 0 && !journey.released_valves.contains(&current_vertex.id) {
        let mut released_values = journey.released_valves.clone();
        released_values.push(current_vertex.id.clone());
        let new_journey = Journey {
            current_vertex_id: journey.current_vertex_id.to_string(),
            released_valves: released_values,
            score: journey.score + current_vertex.valve_value * (30 - journey.time_used - 1),
            time_used: journey.time_used + 1
        };
        possible_journeys.push(new_journey);
    }
    for connected_vertex_id in &current_vertex.connected_to {
        let new_journey = Journey {
            current_vertex_id: connected_vertex_id.to_string(),
            released_valves: journey.released_valves.clone(),
            score: journey.score,
            time_used: journey.time_used + 1
        };
        possible_journeys.push(new_journey);
    }
    possible_journeys
}

/**
 * Finds possible journeys for the given vertices. Updates the best score of the current vertex if 
 * the current journey has a higher score than the best score of that vertex so far.
 */
fn evaluate_journey(journey: &Journey, vertices: &mut HashMap<String, Vertex>) -> Vec<Journey> {
    let current_vertex = vertices.get_mut(&journey.current_vertex_id).unwrap();
    if current_vertex.best_score < journey.score {
        current_vertex.best_score = journey.score;
    }
    let possible_journeys = get_possible_journeys(journey, vertices);
    let mut filtered_journeys = Vec::new();
    for possible_journey in possible_journeys {
        let possible_vertex = vertices.get(&possible_journey.current_vertex_id).unwrap();
        let destination_has_unreleased_valve = possible_vertex.valve_value > 0 && !possible_journey.released_valves.contains(&possible_vertex.id);
        let journey_has_higher_score = possible_journey.score >= possible_vertex.best_score;
        if destination_has_unreleased_valve || journey_has_higher_score {
            filtered_journeys.push(possible_journey);
        }
    }
    filtered_journeys
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_possible_journeys() {
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
            released_valves: vec![],
            score: 0,
            time_used: 0
        };
        let possible_journeys = get_possible_journeys(&journey, &vertices);
        assert_eq!(possible_journeys.len(), 3);
        assert_eq!(possible_journeys[0].current_vertex_id, "A");
        assert_eq!(possible_journeys[0].released_valves.len(), 1);
        assert_eq!(possible_journeys[0].score, 30 * 1);
        assert_eq!(possible_journeys[0].time_used, 1);
        assert_eq!(possible_journeys[1].current_vertex_id, "B");
        assert_eq!(possible_journeys[1].released_valves.len(), 0);
        assert_eq!(possible_journeys[1].score, 0);
        assert_eq!(possible_journeys[1].time_used, 1);
        assert_eq!(possible_journeys[2].current_vertex_id, "C");
        assert_eq!(possible_journeys[2].released_valves.len(), 0);
        assert_eq!(possible_journeys[2].score, 0);
        assert_eq!(possible_journeys[2].time_used, 1);
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
