extern crate regex;

use regex::Regex;
use std::io::BufRead;


fn main() {
    let blueprints = read_input();

    let mut total_score = 0;

    for blueprint in blueprints {
        let print_score = iterate_solution(&blueprint, 0, 23, &Vec::new(), &vec![Resource {
            resource_type: ResourceType::Ore,
            amount: 1
        }]);

        print!("blueprint id {} score: {}", blueprint.id, print_score);
        total_score += print_score * blueprint.id;
    }

    println!("total score: {}", total_score);
}

fn read_input() -> Vec<Blueprint> {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let mut blueprints = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        let blueprint = parse_blueprint(&line.trim());
        blueprints.push(blueprint);

    }
    blueprints
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy, Clone)]
enum ResourceType {
    Ore,
    Clay,
    Obsidian,
    Geode
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy, Clone)]
struct Resource {
    resource_type: ResourceType,
    amount: u32
}

#[derive(Debug)]
#[derive(PartialEq)]
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

fn is_deal_possible(deal: &Deal, resources: &Vec<Resource>) -> bool {
    for price in &deal.price {
        let mut found = false;
        for resource in resources {
            if resource.resource_type == price.resource_type {
                found = true;
                if resource.amount < price.amount {
                    return false;
                }
            }
        }
        if !found {
            return false;
        }
    }
    true
}

fn get_geode_amount(resources: &Vec<Resource>) -> u32 {
    for resource in resources {
        if resource.resource_type == ResourceType::Geode {
            return resource.amount;
        }
    }
    0
}

fn grow_resources(resources: &Vec<Resource>, factories: &Vec<Resource>) -> Vec<Resource> {
    let mut new_resources = resources.into_iter().map(|x| x.clone()).collect::<Vec<Resource>>();
    for factory in factories {
        let mut found = false;
        for resource in &mut new_resources {
            if resource.resource_type == factory.resource_type {
                found = true;
                resource.amount += factory.amount;
                break;
            }
        }
        if !found {
            new_resources.push(factory.clone());
        }
    
    }
    new_resources
}

fn iterate_solution(blueprint: &Blueprint, index: u32, max_index: u32, resources: &Vec<Resource>, factories: &Vec<Resource>) -> u32 {

    if index >= max_index {
        return get_geode_amount(resources);
    }
    let possible_deals = blueprint.deals.iter().filter(|deal| is_deal_possible(deal, resources)).collect::<Vec<&Deal>>();

    println!("index: {}. possible_deals: {}", index, possible_deals.len());
    let mut best_score = iterate_solution(blueprint, index + 1, max_index, &grow_resources(resources, factories), &factories);
    for deal in possible_deals {
        let mut resources = perform_deal(&deal, &resources);
        resources = grow_resources(&resources, &factories);
        let new_factories = add_factory(&resources, deal.factory);
        let deal_score = iterate_solution(blueprint, index + 1, max_index, &resources, &new_factories);
        if deal_score > best_score {
            best_score = deal_score;
        }
    }
    best_score
}

fn add_factory(factories: &Vec<Resource>, factory: ResourceType) -> Vec<Resource> {
    let mut new_factories = factories.into_iter().map(|x| x.clone()).collect::<Vec<Resource>>();
    let mut found = false;
    for f in &mut new_factories {
        if f.resource_type == factory {
            f.amount += 1;
            found = true;
        }
    }
    if !found {
        new_factories.push(Resource {
            resource_type: factory,
            amount: 1,
        });
    }
    new_factories
}

fn perform_deal(deal: &Deal, resources: &Vec<Resource>) -> Vec<Resource> {
    let mut new_resources = resources.into_iter().map(|r| Resource {
        resource_type: r.resource_type,
        amount: r.amount,
    }).collect::<Vec<Resource>>();

    for price in &deal.price {
        for resource in &mut new_resources {
            if resource.resource_type == price.resource_type {
                resource.amount -= price.amount;
            }
        }
    }
    new_resources
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
    fn test_grow_resources() {
        let resources = vec![
            Resource {
                resource_type: ResourceType::Ore,
                amount: 1,
            },
            Resource {
                resource_type: ResourceType::Clay,
                amount: 2,
            },
        ];
        let factories = vec![
            Resource {
                resource_type: ResourceType::Ore,
                amount: 1,
            },
            Resource {
                resource_type: ResourceType::Obsidian,
                amount: 2,
            },
        ];
        let new_resources = grow_resources(&resources, &factories);
        assert_eq!(new_resources, vec![
            Resource {
                resource_type: ResourceType::Ore,
                amount: 2,
            },
            Resource {
                resource_type: ResourceType::Clay,
                amount: 2,
            },
            Resource {
                resource_type: ResourceType::Obsidian,
                amount: 2,
            },
        ]);
    }

    #[test]
    fn iterate_solution_returns_geode_amount_when_max_index_reached() {
        let blueprint = Blueprint {
            id: 1,
            deals: vec![],
        };
        let resources = vec![Resource {
            resource_type: ResourceType::Geode,
            amount: 10,
        }];
        let factories = vec![];
        let result = iterate_solution(&blueprint, 0, 0, &resources, &factories);
        assert_eq!(result, 10);
    }

    #[test]
    fn add_factory_test() {
        let factories = vec![Resource {
            resource_type: ResourceType::Ore,
            amount: 1,
        }];
        let new_factories = add_factory(&factories, ResourceType::Ore);
        assert_eq!(new_factories, vec![Resource {
            resource_type: ResourceType::Ore,
            amount: 2,
        }]);
        assert_eq!(factories, vec![Resource {
            resource_type: ResourceType::Ore,
            amount: 1,
        }]);
    }

    #[test]
    fn perform_deal_should_remove_resources() {
        let deal = Deal {
            price: vec![Resource { resource_type: ResourceType::Ore, amount: 1 }],
            factory: ResourceType::Clay,
        };
        let resources = vec![Resource { resource_type: ResourceType::Ore, amount: 1 }];
        let new_resources = perform_deal(&deal, &resources);
        assert_eq!(new_resources, vec![Resource { resource_type: ResourceType::Ore, amount: 0 }]);
    }

    #[test]
    fn should_say_deal_is_possible_when_enough_resources() {
        let deal = Deal {
            price: vec![Resource {
                resource_type: ResourceType::Ore,
                amount: 1,
            }],
            factory: ResourceType::Clay,
        };

        let resources = vec![Resource {
            resource_type: ResourceType::Ore,
            amount: 1,
        }];

        assert!(is_deal_possible(&deal, &resources));
    }

    #[test]
    fn should_say_deal_is_not_possible_when_not_enough_resources() {
        let deal = Deal {
            price: vec![Resource {
                resource_type: ResourceType::Ore,
                amount: 1,
            }],
            factory: ResourceType::Clay,
        };

        let resources = vec![Resource {
            resource_type: ResourceType::Clay,
            amount: 1,
        }];

        assert!(!is_deal_possible(&deal, &resources));
    }

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
