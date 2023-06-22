use Allergen::*;
pub struct Allergies {
    allergens: Vec<Allergen>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
    fn push(&mut self, value: u32) {
        match value {
            1 => self.allergens.push(Eggs),
            2 => self.allergens.push(Peanuts),
            4 => self.allergens.push(Shellfish),
            8 => self.allergens.push(Strawberries),
            16 => self.allergens.push(Tomatoes),
            32 => self.allergens.push(Chocolate),
            64 => self.allergens.push(Pollen),
            128 => self.allergens.push(Cats),
            _ => (),
        }
    }
    pub fn new(score: u32) -> Self {
        let allergens_nums = vec![1, 2, 4, 8, 16, 32, 64, 128];
        let mut allergens = Allergies {
            allergens: Vec::new(),
        };

        let mut sum = score % 256; //modulo works on >255 numbers
        for num in allergens_nums.iter().rev() {
            if *num <= sum {
                sum -= num;
                allergens.push(*num);
            }
        }
        allergens
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergens.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergens.clone()
    }
}
