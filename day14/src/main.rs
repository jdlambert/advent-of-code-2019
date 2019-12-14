use std::{cmp::max, collections::HashMap, fs};

type Cookbook = HashMap<String, (i64, Vec<(i64, String)>)>;
const TRILLION: i64 = 1_000_000_000_000;

fn part1(cookbook: &Cookbook) -> i64 {
    get_fuel_ore(&cookbook, 1)
}

fn part2(cookbook: &Cookbook) -> i64 {
    let mut low = TRILLION / part1(&cookbook);
    let mut high = low * 2;
    let mut current;
    loop {
        current = (low + high) / 2;
        if get_fuel_ore(&cookbook, current) > TRILLION {
            high = current - 1;
        } else {
            low = current + 1;
        }
        if high < low {
            return high;
        }
    }
}

fn get_fuel_ore(cookbook: &Cookbook, amount: i64) -> i64 {
    get_ingredient_ore(&cookbook, &mut HashMap::new(), "FUEL", amount)
}

fn get_ingredient_ore(
    cookbook: &Cookbook,
    stockpile: &mut HashMap<String, i64>,
    ingredient: &str,
    amount: i64,
) -> i64 {
    let (produced, components) = cookbook.get(ingredient).unwrap();

    let available = stockpile.entry(ingredient.to_string()).or_insert(0);
    let needed = max(0, amount - *available);

    let multiple = needed / produced + (needed % produced != 0) as i64;
    let excess = multiple * produced - amount;
    *available += excess;

    let mut ore = 0;
    for component in components {
        ore += if component.1 == "ORE" {
            component.0 * multiple
        } else {
            get_ingredient_ore(cookbook, stockpile, &component.1, component.0 * multiple)
        };
    }
    ore
}

fn main() {
    let content = fs::read_to_string("./input.txt").unwrap();
    let mut cookbook = Cookbook::new();
    let lines = content.lines();
    for line in lines {
        let mut items = line.split("=>");
        let ingredients = items.next().unwrap().trim();
        let result = items.next().unwrap().trim();
        let mut result_item = result.split_whitespace();
        let result_qty = result_item.next().unwrap().parse::<i64>().unwrap();
        let result_name = result_item.next().unwrap().to_string();

        cookbook.insert(
            result_name,
            (
                result_qty,
                ingredients
                    .split(',')
                    .map(str::trim)
                    .map(str::split_whitespace)
                    .map(|mut ingredient_item| {
                        let ingredient_qty =
                            ingredient_item.next().unwrap().parse::<i64>().unwrap();
                        let ingredient_name = ingredient_item.next().unwrap().to_string();
                        (ingredient_qty, ingredient_name)
                    })
                    .collect(),
            ),
        );
    }

    println!("Part 1: {}", part1(&cookbook));
    println!("Part 2: {}", part2(&cookbook));
}
