


// Now we want the tree to expand on next possible build step.
// I want to know the highest price in each resource type. This can be expressed by a [u32; 3] array.
// I also want to account for Geode gains direct on purchase.
// For all of this I need a model that contains how many minutes is used for an action, together
// with what factory is built by the action. I then still need to keep track of the resources. and
// the factories. Resources can be tracked by a [u32; 3] array and factories as well.

#[derive(Debug)]
struct Action {
    minutes: u32,
    end_minutes: u32,
    factory: ResourceType,
}

fn max(a: u32, b: u32) -> u32 {
    if a > b {
        a
    } else {
        b
    }
}

fn get_max_factories(blueprint: &Blueprint) -> [u32; 3] {
    let mut max_resources = [0; 3];
    for deal in blueprint.deals.iter() {
        for resource in deal.price.iter() {
            if resource.resource_type != ResourceType::Geode {
                max_resources[resource.resource_type as usize] = max(max_resources[resource.resource_type as usize], resource.amount);
            }
        }
    }
    max_resources
}

fn ceil_div(a: u32, b: u32) -> u32 {
    (a + b - 1) / b
}

fn needed_minutes(price: [u32; 3], resources: [u32; 3], factories: [u32; 3]) -> Option<u32> {
    let mut needed_minutes = 0;
    for i in 0..3 {
        if price[i] > 0 {
            if factories[i] == 0 {
                return None;
            }
            if resources[i] >= price[i] {
                needed_minutes = max(needed_minutes, 1);
            } else {

                needed_minutes = max(needed_minutes, 1 + ceil_div(price[i] - resources[i], factories[i]));
            }
        }
    }
    Some(needed_minutes)
}

fn get_price_as_array(deal: &Deal) -> [u32; 3] {
    let mut price = [0; 3];
    for resource in deal.price.iter() {
        if resource.resource_type != ResourceType::Geode {
            price[resource.resource_type as usize] = resource.amount;
        }
    }
    price
}

fn get_possible_new_actions(blueprint: &Blueprint, resources: [u32; 3], factories: [u32; 3], max_factories: [u32; 3], current_time: u32) -> Vec<Action> {
    let mut actions = Vec::new();
    for deal in blueprint.deals.iter() {
        let index = deal.factory as usize;
        if index < 3 && factories[index] >= max_factories[index] {
            continue;
        }
        if index < 3 && resources[index] >= (2 * max_factories[index]) as u32 {
            continue;
        }
        let price = get_price_as_array(deal);
    
        let needed_minutes = needed_minutes(price, resources, factories);
        if let Some(needed_minutes) = needed_minutes {
            actions.push(Action {
                minutes: needed_minutes,
                end_minutes: current_time + needed_minutes,
                factory: deal.factory,
            });
        }
    }
    actions
}

fn get_price_for_deal(deal: &Deal) -> [u32; 3] {
    let mut price = [0; 3];
    for resource in deal.price.iter() {
        price[resource.resource_type as usize] += resource.amount;
    }
    price
}

fn new_grow_resources(resources: [u32; 3], factories: [u32; 3], minutes: u32) -> [u32; 3] {
    let mut new_resources = resources;
    for i in 0..3 {
        new_resources[i] += factories[i] * minutes;
    }
    new_resources
}

fn withdraw_price(resources: [u32; 3], price: [u32; 3]) -> [u32; 3] {
    let mut new_resources = resources;
    for i in 0..3 {
        new_resources[i] -= price[i];
    }
    new_resources
}

fn add_factory_to_factories(factories: [u32; 3], factory: ResourceType) -> [u32; 3] {
    let mut new_factories = factories;
    new_factories[factory as usize] += 1;
    new_factories
}

fn start_iterate(blueprint: &Blueprint, max_minutes: u32) -> u32 {
    let max_factories = get_max_factories(blueprint);
    let mut resources = [0; 3];
    let mut factories = [1, 0, 0];
    let actions = get_possible_new_actions(blueprint, resources, factories, max_factories, 0);
    // println!("actions: {:?}", actions);
    let mut best_score = 0;
    for action in actions.iter() {
        let score = new_iterate_solution(blueprint, action, max_minutes, resources, factories, max_factories);
        if score > best_score {
            best_score = score;
        }
    }
    best_score
}

fn new_iterate_solution(blueprint: &Blueprint, prev_action: &Action, max_minutes: u32, resources: [u32; 3], factories: [u32; 3], max_factories: [u32; 3]) -> u32 {
    let mut new_resources = new_grow_resources(resources, factories, prev_action.minutes);

    new_resources = withdraw_price(new_resources, get_price_for_deal(&blueprint.deals[prev_action.factory as usize]));
    let new_factories = 
        if prev_action.factory == ResourceType::Geode {
            factories
        } else {
            add_factory_to_factories(factories, prev_action.factory)
        };
    let new_actions = get_possible_new_actions(blueprint, new_resources, new_factories, max_factories, prev_action.end_minutes);
    // println!("actions: {:?}", new_actions);
    let mut best_score = 0;
    for action in new_actions.iter() {
        if action.end_minutes < max_minutes {
            let mut score = new_iterate_solution(blueprint, action, max_minutes, new_resources, new_factories, max_factories);
            if action.factory == ResourceType::Geode {
                // println!("action: {:?}, score: {}", action, score);
                score += MAX_TIME - action.end_minutes;
            }

            if score > best_score {
                best_score = score;
            }
        }
    }
    best_score
}


#[cfg(test)]
mod new_tests {
    use super::*;

    #[test]
    fn test_it_works() {
    }

    #[test]
    fn should_get_needed_minutes_when_possible() {
        let price = [3, 0, 0];
        let resources = [0, 0, 0];
        let factories = [1, 0, 0];
        assert_eq!(needed_minutes(price, resources, factories), Some(4));
    }

    #[test]
    fn should_get_needed_minutes_when_not_possible() {
        let price = [3, 1, 0];
        let resources = [0, 0, 0];
        let factories = [1, 0, 0];
        assert_eq!(needed_minutes(price, resources, factories), None);
    }

    #[test]
    fn should_get_needed_minutes_when_possible_direct() {
        let price = [3, 0, 0];
        let resources = [3, 0, 0];
        let factories = [1, 0, 0];
        assert_eq!(needed_minutes(price, resources, factories), Some(1));
    }

    #[test]
    fn test_get_max_factories() {
        let blueprint = Blueprint {
            id: 1,
            deals: vec![
                Deal {
                    price: vec![
                        Resource {
                            resource_type: ResourceType::Ore,
                            amount: 1,
                        },
                    ],
                    factory: ResourceType::Ore,
                },
                Deal {
                    price: vec![
                        Resource {
                            resource_type: ResourceType::Ore,
                            amount: 5,
                        },
                    ],
                    factory: ResourceType::Clay,
                },
                Deal {
                    price: vec![
                        Resource {
                            resource_type: ResourceType::Ore,
                            amount: 9,
                        },
                        Resource {
                            resource_type: ResourceType::Clay,
                            amount: 10,
                        },
                    ],
                    factory: ResourceType::Obsidian,
                },
                Deal {
                    price: vec![
                        Resource {
                            resource_type: ResourceType::Clay,
                            amount: 14,
                        },
                        Resource {
                            resource_type: ResourceType::Obsidian,
                            amount: 15,
                        },
                    ],
                    factory: ResourceType::Geode,
                },
            ],
        };

        assert_eq!(get_max_factories(&blueprint), [9, 14, 15]);
    }
}

