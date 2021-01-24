use std::collections::HashMap;

use crate::AdventError;

pub fn part1() -> Result<(), AdventError> {
    let ingredients = vec![
        Ingredient::new(2, 0, -2, 0, 3),
        Ingredient::new(0, 5, -3, 0, 3),
        Ingredient::new(0, 0, 5, -1, 8),
        Ingredient::new(0, -1, 0, 5, 8),
    ];
    let mut amounts = vec![0; ingredients.len()];
    let mut cook = Cook::new();
    cook.select_ingredients(&ingredients, &mut amounts, 0, 100, None);
    let best_amount = cook.best_amounts.unwrap();
    println!("Best score: {}", total_score(&ingredients, &best_amount, None));

    let mut cook = Cook::new();
    cook.select_ingredients(&ingredients, &mut amounts, 0, 100, Some(500));
    let best_amount = cook.best_amounts.unwrap();
    println!("Best score with 500 calories: {}", total_score(&ingredients, &best_amount, Some(500)));
    Ok(())
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum MixtureProperty {
    Capacity,
    Durability,
    Flavor,
    Texture,
    Calories
}

struct Ingredient {
    effect: HashMap<MixtureProperty, i32>,
}

impl Ingredient {
    fn new(capacity: i32,
           durability: i32,
           flavor: i32,
           texture: i32,
           calories: i32) -> Ingredient {
        let mut effect = HashMap::new();
        effect.insert(MixtureProperty::Capacity, capacity);
        effect.insert(MixtureProperty::Durability, durability);
        effect.insert(MixtureProperty::Flavor, flavor);
        effect.insert(MixtureProperty::Texture, texture);
        effect.insert(MixtureProperty::Calories, calories);
        Ingredient{ effect }
    }
}

fn total_score(ingredients: &Vec<Ingredient>, amounts: &Vec<u32>, target_calories: Option<i64>) -> i64 {
    let mut totals: HashMap<MixtureProperty, i64> = HashMap::new();
    for (definition, &amount) in itertools::zip(ingredients, amounts) {
        for (effect, &how_much) in &definition.effect {
            let total_so_far = *totals.get(&effect).unwrap_or(&0);
            totals.insert(*effect, total_so_far + ((amount as i32) * how_much) as i64);
        }
    }

    // ignore calories so far
    match target_calories {
        None => {
            totals.remove(&MixtureProperty::Calories);
        },
        Some(t) => {
            if *totals.get(&MixtureProperty::Calories).unwrap() != t {
                totals.clear();
            } else {
                totals.remove(&MixtureProperty::Calories);
            }
        }
    }

    totals.values()
          .copied()
          .filter(|&v| v > 0)
          .product()
}

struct Cook {
    best_score: Option<i64>,
    best_amounts: Option<Vec<u32>>,
}

impl Cook {
    fn new() -> Cook {
        Cook{best_score: None, best_amounts: None}
    }

    fn select_ingredients(
        &mut self,
        ingredients: &Vec<Ingredient>,
        amounts: &mut Vec<u32>,
        ingredient_idx: usize,
        remaining_teaspoons: u32,
        target_calories: Option<i64>) {

        if ingredient_idx + 1 == ingredients.len() {
            // base case
            amounts[ingredient_idx] = remaining_teaspoons;
            let this_score = total_score(ingredients, amounts, target_calories);
            match self.best_score {
                None => {
                    self.best_score = Some(this_score);
                    self.best_amounts = Some(amounts.clone());
                }
                Some(score) => {
                    if this_score > score {
                        self.best_score = Some(this_score);
                        self.best_amounts = Some(amounts.clone());
                    }
                }
            }
        } else {
            // try different amounts
            for this_amount in 0..remaining_teaspoons {
                amounts[ingredient_idx] = this_amount;
                self.select_ingredients(
                    ingredients, amounts,
                    ingredient_idx + 1, remaining_teaspoons - this_amount, target_calories);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_score() {
        let ingredients = vec![
            Ingredient::new(-1, -2, 6, 3, 8),
            Ingredient::new(2, 3, -2, -1, 3),
        ];
        let amounts = vec![44, 56];
        assert_eq!(62842880, total_score(&ingredients, &amounts, None));
    }

    #[test]
    fn test_example() {
        let ingredients = vec![
            Ingredient::new(-1, -2, 6, 3, 8),
            Ingredient::new(2, 3, -2, -1, 3),
        ];
        let mut amounts = vec![0; ingredients.len()];
        let mut subject = Cook::new();
        subject.select_ingredients(&ingredients, &mut amounts, 0, 100, None);

        let best_amount = subject.best_amounts.unwrap();
        assert_eq!(vec![44, 56], best_amount);
    }
}