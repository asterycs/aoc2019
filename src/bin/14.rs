use regex::Regex;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::num::ParseIntError;
use std::ops::Mul;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Component {
    name: String,
    amount: usize,
}

impl Mul<usize> for Component {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self {
        Component {
            name: self.name,
            amount: rhs * self.amount,
        }
    }
}

#[derive(Debug, Clone)]
struct Recipe {
    ingredients: Vec<Component>,
    result: Component,
}

impl FromStr for Recipe {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split("=>").collect::<Vec<_>>();
        let ingredients_in = split[0];
        let results_in = split[1];

        let input_re = Regex::new(r"(\d+)\s(\S+)").unwrap();
        let mut ingredients = Vec::new();

        for ingredient in ingredients_in.split(",") {
            let ingredient = input_re.captures(ingredient).expect("Malformed recipe");

            let amount = ingredient.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let ingredient = ingredient.get(2).unwrap().as_str().to_string();

            ingredients.push(Component {
                name: ingredient,
                amount,
            });
        }

        let result = input_re.captures(results_in).expect("Malformed recipe");
        let result = Component {
            name: result[2].to_string(),
            amount: result[1].parse::<usize>().unwrap(),
        };

        Ok(Recipe {
            ingredients,
            result,
        })
    }
}

fn ore_required(fuel: usize, recipes: &HashMap<String, Recipe>) -> usize {
    let mut requirements = vec![Component {
        name: "FUEL".to_string(),
        amount: fuel,
    }]
    .into_iter()
    .collect::<VecDeque<_>>();

    let mut ore = 0;
    let mut available = HashMap::<String, usize>::new();

    while !requirements.is_empty() {
        let current_requirement = requirements.pop_front().unwrap();

        //println!("{:?}", current_requirement);

        if current_requirement.name == "ORE" {
            ore += current_requirement.amount;
        } else if available.get(&current_requirement.name).unwrap_or(&0)
            >= &current_requirement.amount
        {
            *available.get_mut(&current_requirement.name).unwrap() -= current_requirement.amount;
        } else {
            let recipe = &recipes[&current_requirement.name];

            let avail;

            if available.contains_key(&current_requirement.name) {
                avail = available[&current_requirement.name];
                *available.get_mut(&current_requirement.name).unwrap() = 0;
            } else {
                avail = 0;
            }

            let rounds = (current_requirement.amount - avail + recipe.result.amount - 1)
                / recipe.result.amount;

            *available.entry(current_requirement.name).or_insert(0) =
                rounds * recipe.result.amount + avail - current_requirement.amount;

            for comp in recipe.ingredients.iter() {
                requirements.push_back(comp.clone() * rounds);
            }
        }

        //println!("available: {:?}", available);
    }

    ore
}

fn main() {
    let filename = &mut PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    filename.push("inputs/14.txt");

    println!("Reading {}", filename.display());

    let input = fs::read_to_string(filename).expect("Unable to open file");
    let input = input.lines().collect::<Vec<_>>();

    let recipes = &mut input
        .iter()
        .map(|l| Recipe::from_str(l).unwrap())
        .collect::<Vec<_>>();

    /*for r in recipes.iter().rev() {
        println!("{:?}", r);
    }*/

    let recipes_map = recipes
        .iter()
        .map(|r| (r.result.name.clone(), r.clone()))
        .collect::<HashMap<_, _>>();

    let ore = ore_required(1, &recipes_map);

    println!("Ore required: {}", ore);

    // trillion 10^6 * 10^6
    const STEP: usize = 1000;
    const LIMIT: usize = 1_000_000_000_000;
    let mut fuel = STEP;

    loop {
        let ore = ore_required(fuel, &recipes_map);

        if ore > LIMIT {
            fuel = fuel - STEP;

            break;
        }

        fuel += STEP;
    }

    for i in 0..STEP {
        let ore = ore_required(fuel + i, &recipes_map);

        if ore > LIMIT {
            println!("Part 2: {}", fuel + i - 1);
            break;
        }
    }
}
