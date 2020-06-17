use crate::simulation::{Id, Inventory, Money, Product};

pub struct City {
    pub name: String,
    pub inventory: Inventory,
    pub treasury: Money,
    pub workshops: Vec<Workshop>,
    pub prosperity: f64,
}

impl City {
    pub fn tick(&mut self) {

    }
}

pub struct Workshop {
    pub name: String,
    pub inputs: Vec<(Id<Product>, u32)>,
    pub output: Id<Product>,
    pub amount: u32,
    pub budget: Money,
    pub capacity: u32,
}

impl Workshop {
    pub fn produce()  {
        
    }
}
