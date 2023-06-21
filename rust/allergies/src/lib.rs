pub struct Allergies {
    allergens: Vec<Allergen>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        let allergens_nums = vec![1, 2, 4, 8, 16, 32, 64, 128];
        let mut allergens = Vec::new();

        push_summing_numbers(allergens_nums, score, &mut allergens);
        let allergens = allergens
            .iter()
            .map(|a| match a {
                1 => Allergen::Eggs,
                2 => Allergen::Peanuts,
                4 => Allergen::Shellfish,
                8 => Allergen::Strawberries,
                16 => Allergen::Tomatoes,
                32 => Allergen::Chocolate,
                64 => Allergen::Pollen,
                _ => Allergen::Cats,
            })
            .collect::<Vec<Allergen>>();
        Allergies {
            allergens: allergens,
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergens.contains(allergen)
    }

    pub fn allergies(self) -> Vec<Allergen> {
        self.allergens
    }
}

fn push_summing_numbers(set: Vec<u32>, sum: u32, final_vec: &mut Vec<u32>) -> () {
    let mut tmp = sum % 256; //modulo works on >255 numbers
    for num in set.iter().rev() {
        if *num <= tmp {
            tmp -= num;
            final_vec.push(*num);
        }
        if final_vec.iter().sum::<u32>() == sum % 256 {
            break;
        }
    }
    if final_vec.iter().sum::<u32>() != sum % 256 {
        final_vec.clear();
    }
}
//TO DO!
//have to refactor this code
