extern crate regex;

use regex::Regex;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
#[derive(PartialEq)]
enum ResourceType {
    Ore,
    Clay,
    Obsidian,
    Geode
}

struct Resource {
    resource_type: ResourceType,
    amount: u32
}

struct Deal {
    price: Vec<Resource>,
    factory: ResourceType,
}

struct Blueprint {
    id: u32,
    deals: Vec<Deal>,
}

fn parse_blueprint(line: &str) -> Blueprint {
    let total_re = Regex::new(r"Blueprint (\d+): (.*)\. (.*)\. (.*)\. (.*)\.").unwrap();

    let blueprint_id = total_re.captures(line).unwrap().get(1).unwrap().as_str().parse::<u32>().unwrap();


    let mut deals = Vec::new();
    deals.push(parse_deal(total_re.captures(line).unwrap().get(2).unwrap().as_str()));
    deals.push(parse_deal(total_re.captures(line).unwrap().get(3).unwrap().as_str()));
    deals.push(parse_deal(total_re.captures(line).unwrap().get(4).unwrap().as_str()));
    deals.push(parse_deal(total_re.captures(line).unwrap().get(5).unwrap().as_str()));
    Blueprint {
        id: blueprint_id,
        deals: deals,
    }
}

fn parse_deal(line: &str) -> Deal {
    let deal_re = Regex::new(r"Each (\w+) robot costs (.*)").unwrap();

    let caps = deal_re.captures(line).unwrap();
    let factory = parse_resource_type(caps.get(1).unwrap().as_str());
    let parse_str = caps.get(2).unwrap().as_str();
    let price = parse_price(parse_str);
    Deal {
        price: price,
        factory: factory,
    }
}

fn parse_price(line: &str) -> Vec<Resource> {
    line.split(" and ").map(|x| parse_single_price(x)).collect()
}

fn parse_single_price(line: &str) -> Resource {
    let price_re = Regex::new(r"(\d+) (\w+)").unwrap();

    let caps = price_re.captures(line).unwrap();
    let amount = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
    let resource_type = parse_resource_type(caps.get(2).unwrap().as_str());

    Resource {
        resource_type: resource_type,
        amount: amount,
    }
}

fn parse_resource_type(line: &str) -> ResourceType {
    match line {
        "ore" => ResourceType::Ore,
        "clay" => ResourceType::Clay,
        "obsidian" => ResourceType::Obsidian,
        "geode" => ResourceType::Geode,
        _ => panic!("Unknown resource type: {}", line)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_blueprint() {
        let blueprint_strings: Vec<&str> = vec![ "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 5 clay. Each geode robot costs 3 ore and 15 obsidian.",
        ];

        let blueprint = parse_blueprint(blueprint_strings[0]);
        assert_eq!(blueprint.id, 1);
    }

    #[test]
    fn should_parse_simple_deal() {
        let deal_strings: Vec<&str> = vec![ 
            "Each ore robot costs 4 ore",
        ];

        let deal = parse_deal(deal_strings[0]);
        assert_eq!(deal.price.len(), 1);
        assert_eq!(deal.price[0].resource_type, ResourceType::Ore);
        assert_eq!(deal.price[0].amount , 4);
    }

    #[test]
    fn should_parse_complex_deal() {
        let deal_strings: Vec<&str> = vec![ 
            "Each ore robot costs 4 ore and 5 clay",
        ];

        let deal = parse_deal(deal_strings[0]);
        assert_eq!(deal.price.len(), 2);
        assert_eq!(deal.price[0].resource_type, ResourceType::Ore);
        assert_eq!(deal.price[0].amount , 4);
        assert_eq!(deal.price[1].resource_type, ResourceType::Clay);
        assert_eq!(deal.price[1].amount , 5);
    }

    #[test]
    fn should_parse_price() {
        let price_strings: Vec<&str> = vec![ 
            "4 ore",
            "3 ore and 15 obsidian",
        ];

        let mut price = parse_price(price_strings[0]);
        assert_eq!(price.len(), 1);
        assert_eq!(price[0].resource_type, ResourceType::Ore);
        assert_eq!(price[0].amount , 4);

        price = parse_price(price_strings[1]);
        assert_eq!(price.len(), 2);
        assert_eq!(price[0].resource_type, ResourceType::Ore);
        assert_eq!(price[0].amount , 3);
        assert_eq!(price[1].resource_type, ResourceType::Obsidian);
        assert_eq!(price[1].amount , 15);
    }
}
