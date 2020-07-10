pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
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

static ALLERGIES: [Allergen; 8] = [
    Allergen::Eggs,
    Allergen::Peanuts,
    Allergen::Shellfish,
    Allergen::Strawberries,
    Allergen::Tomatoes,
    Allergen::Chocolate,
    Allergen::Pollen,
    Allergen::Cats,
];

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let bit = 1 << *allergen as u32;
        self.score & bit == bit
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        ALLERGIES
            .iter()
            .filter(|a| self.is_allergic_to(a))
            .copied()
            .collect()
    }
}
