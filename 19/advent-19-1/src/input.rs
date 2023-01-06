
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
