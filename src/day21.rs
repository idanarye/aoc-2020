use hashbrown::{HashMap, HashSet};
use hashbrown::hash_map::Entry;

#[derive(Debug)]
pub struct FoodItem {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

pub fn generator(input: &str) -> Vec<FoodItem> {
    let pattern = regex::Regex::new(r"^(.*) \(contains (.*)\)$").unwrap();
    input.lines().map(|line| {
        let m = pattern.captures(line).unwrap();
        FoodItem {
            ingredients: m[1].split(" ").map(str::to_owned).collect(),
            allergens: m[2].split(", ").map(str::to_owned).collect(),
        }
    }).collect()
}

fn calc_initial_allergens_possibilities(food_items: &[FoodItem]) -> HashMap::<&str, HashSet<&str>> {
    let mut result = HashMap::<&str, HashSet<&str>>::new();
    for food_item in food_items.iter() {
        let ingredients = food_item.ingredients.iter().map(|ing| ing.as_str()).collect::<HashSet<_>>();
        for allergen in food_item.allergens.iter() {
            match result.entry(allergen) {
                Entry::Occupied(mut entry) => {
                    let new_value = entry.get().intersection(&ingredients);
                    *entry.get_mut() = new_value.map(|s| *s).collect();
                }
                Entry::Vacant(entry) => {
                    entry.insert(ingredients.clone());
                }
            }
        }
    }
    result
}

pub fn part_1(food_items: &[FoodItem]) -> usize {
    let allergens_possibilities = calc_initial_allergens_possibilities(food_items);
    let ingredients_with_allergens: HashSet<_> = allergens_possibilities.values().flatten().copied().collect();
    let all_ingredients: HashSet<_> = food_items.iter()
        .flat_map(|fi| fi.ingredients.iter().map(String::as_str))
        .collect();
    let ingredients_with_no_allergens: HashSet<_> = all_ingredients.difference(&ingredients_with_allergens).copied().collect();
    food_items.iter().flat_map(|fi| fi.ingredients.iter()).filter(|ing| ingredients_with_no_allergens.contains(ing.as_str())).count()
}

pub fn part_2(food_items: &[FoodItem]) -> String {
    let mut allergens_possibilities = calc_initial_allergens_possibilities(food_items);
    let mut ingredients_and_their_alergens = Vec::new();
    while let Some((allergen, ingredient)) = allergens_possibilities.iter().find_map(|(allergen, ingredients)| {
        let mut it = ingredients.iter();
        let ingredient = it.next()?;
        if it.next().is_some() {
            None
        } else {
            Some((*allergen, *ingredient))
        }
    }) {
        ingredients_and_their_alergens.push((ingredient, allergen));
        allergens_possibilities.remove(allergen);
        for ingredients in allergens_possibilities.values_mut() {
            ingredients.remove(ingredient);
        }
    }
    ingredients_and_their_alergens.sort_by_key(|&(_, alg)| alg);
    ingredients_and_their_alergens.into_iter().map(|(ing, _)| ing).collect::<Vec<&str>>().join(",")
}
