extern crate regex;

use regex::Regex;
use std::io::BufRead;

const MAX_TIME: u32 = 24;

include!("input.rs");
include!("new_solution.rs");

fn main() {
    let blueprints = read_input();

    let mut total_score = 0;

    println!("{} blueprints", blueprints.len());
    for blueprint in blueprints {
        let print_score = start_iterate(&blueprint, MAX_TIME);
        /*
        let print_score = iterate_solution(&blueprint, 0, MAX_TIME, &Vec::new(), &vec![Resource {
            resource_type: ResourceType::Ore,
            amount: 1,
        }], &mut vec![None; MAX_TIME as usize + 1]);
        */
        println!("blueprint id {} score: {}", blueprint.id, print_score);
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

fn contains_geode_deal(deals: &Vec<&Deal>) -> bool {
    for deal in deals {
        if deal.factory == ResourceType::Geode {
            return true;
        }
    }
    false
}

fn get_resources_for_deal(deal_or_none: &Option<&Deal>, resources: &Vec<Resource>) -> Vec<Resource> {
    match deal_or_none {
        Some(deal) => {
            perform_deal(deal, resources)
        },
        None => {
            return resources.clone();
        }
    }
}

fn get_factories_for_deal(deal_or_none: &Option<&Deal>, factories: &Vec<Resource>) -> Vec<Resource> {
    match deal_or_none {
        Some(deal) => {
            add_factory(factories, deal.factory)
        },
        None => factories.clone(),
    }
}


// get possible deals. if there are none, return None

// how to prune? We could check factory states and see if we have a better score. In that case
// there is no need to continue. Best Score is possible to keep track of. But when do you cut off
// other branches? It's 9 points would only make it possible to cut off at 1/3/6/10 minute 20,
// which still gives roughly 5**20 possibilities. What other rules are applicable? We if last was a
// no deal and we could afford a factory, that branch is also not interesting. This roughly would
// only cut off to 4**20, which is still in the range of 10**12 iterations.
// We could keep a map of best value for factory states. This helps prune the diamond shapes of
// choices. This would speak for a width first search. Here we probably need to go into minute
// 17/18 before we get points and can start to prune. Which is still too much.
// On each level one can have the best score / chosen deal / factory state. if score is better and
// factory state is the same or better, we can prune. That would prune a lot. But it might  prevent
// some states where we would have waited... Well, let's try
//
// how do we decrease the size of the algorithm. 
// 1) Create get_possible_deals function.
// 2) Introduce an array of minute / best state. State is: Score / Factory State.
// 3) Create is_factory_state_better_or_same function. It compares all functions and returns true if they
//    are all better or equal.

#[derive(Clone)]
struct SearchState {
    score: u32,
    factories: Vec<Resource>,
}

/**
 * Checks on the resources from the end of the array until the front. If the resource is higher in
 * vector a it's seen as better. If it's lower it's seen as worse.
 */
fn is_factory_state_better_or_same(a: &Vec<Resource>, b: &Vec<Resource>) -> bool {
    let mut diff = get_resource_diff(ResourceType::Geode, a, b);
    if diff > 0 {
        return true;
    } else if (diff < 0) {
        return false;
    }
    diff = get_resource_diff(ResourceType::Obsidian, a, b);
    if diff > 0 {
        return true;
    } else if (diff < 0) {
        return false;
    }
    diff = get_resource_diff(ResourceType::Clay, a, b);
    if diff > 0 {
        return true;
    } else if (diff < 0) {
        return false;
    }
    diff = get_resource_diff(ResourceType::Ore, a, b);
    if diff > 0 {
        return true;
    } else if (diff < 0) {
        return false;
    }
    true

}

fn get_resource_diff(resource_type: ResourceType, a: &Vec<Resource>, b: &Vec<Resource>) -> i32 {
    get_resource_value(resource_type, a) as i32 - get_resource_value(resource_type, b) as i32
}

fn get_resource_value(resource_type: ResourceType, resources: &Vec<Resource>) -> u32 {
    for resource in resources {
        if resource.resource_type == resource_type {
            return resource.amount;
        }
    }
    0
}

fn get_possible_deals<'a>(blueprint: &'a Blueprint, resources: &Vec<Resource>) -> Vec<Option<&'a Deal>> {
    let mut possible_deals: Vec<Option<&'a Deal>> = Vec::new();
    possible_deals.push(None);
    for deal in &blueprint.deals {
        if is_deal_possible(deal, resources) {
            possible_deals.push(Some(deal));
        }
    }

    // We make sure that the Geode deal is always first, followed by obsidian
    possible_deals.reverse();
    possible_deals
}

// now we search the tree with best options first, but it still takes a lot of time. Perhaps
// allocation off arrays is too expensive. Let's make a measurement.


fn iterate_solution(blueprint: &Blueprint, index: u32, max_index: u32, resources: &Vec<Resource>, factories: &Vec<Resource>, best_states: &mut Vec<Option<SearchState>>) -> u32 {
    let current_score = get_geode_amount(resources);

    let best_state = &best_states[index as usize];
    let previous_best_score = match best_state {
        Some(state) => state.score,
        None => 0,
    };


    if index >= max_index {
        return current_score;
    } else if previous_best_score > current_score && is_factory_state_better_or_same(&best_state.as_ref().unwrap().factories, factories) {
        return current_score;
    }
    let possible_deals = get_possible_deals(blueprint, resources);

    let mut best_score = 0;
    let grown_resources = grow_resources(resources, factories);
    for deal in possible_deals {
        let resources = get_resources_for_deal(&deal, &grown_resources);
        let new_factories = get_factories_for_deal(&deal, &factories);
        let deal_score = iterate_solution(blueprint, index + 1, max_index, &resources, &new_factories, best_states);
        if deal_score > best_score {
            best_score = deal_score;
        }
    }

    if current_score > previous_best_score {
        println!("{}: {}", index, current_score);
        best_states[index as usize] = Some(SearchState { score: current_score, factories: factories.clone() });
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



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_factory_state_better_or_same() {

        let mut result = is_factory_state_better_or_same(&vec![Resource {
            resource_type: ResourceType::Ore,
            amount: 1,
        }], &vec![Resource {
            resource_type: ResourceType::Ore,
            amount: 1,
        }]);
        assert!(result);

        result = is_factory_state_better_or_same(&vec![Resource {
            resource_type: ResourceType::Clay,
            amount: 1,
        }], &vec![Resource {
            resource_type: ResourceType::Ore,
            amount: 2,
        }]);

        assert!(result);

        result = is_factory_state_better_or_same(&vec![Resource {
            resource_type: ResourceType::Ore,
            amount: 1,
        }], &vec![Resource {
            resource_type: ResourceType::Clay,
            amount: 1,
        }]);

        assert!(!result);
    }
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
        let result = iterate_solution(&blueprint, 0, 0, &resources, &factories, &mut vec![None]);
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
    fn should_get_possible_deals() {
        let blueprint = parse_blueprint("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 5 clay. Each geode robot costs 3 ore and 15 obsidian.");
        let resources = vec![
            Resource {
                resource_type: ResourceType::Ore,
                amount: 10,
            },
            Resource {
                resource_type: ResourceType::Clay,
                amount: 10,
            },
            Resource {
                resource_type: ResourceType::Obsidian,
                amount: 15,
            },
        ];

        let deals = get_possible_deals(&blueprint, &resources);
        assert_eq!(deals.get(0).unwrap().unwrap().factory, ResourceType::Geode);
        assert_eq!(deals.get(1).unwrap().unwrap().factory, ResourceType::Obsidian);
        assert_eq!(deals.get(2).unwrap().unwrap().factory, ResourceType::Clay);
        assert_eq!(deals.get(3).unwrap().unwrap().factory, ResourceType::Ore);
        assert_eq!(deals.get(4).unwrap().is_none(), true);
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
