pub use self::{
    city::City,
    city::Workshop,
    collection::{Collection, Id},
    inventory::Inventory,
    merchant::Merchant,
    money::Money,
    product::Product,
};

use decimal::d128;
use std::collections::HashMap;

pub mod city;
pub mod collection;
pub mod inventory;
pub mod merchant;
pub mod money;
pub mod product;

const MINUTE_LENGTH: f64 = 1.0;
const MINUTES_PER_DAY: u32 = 60*24;

pub struct Simulation {
    pub time: f64,
    pub minute: u32,
    pub day: u32,

    pub cities: Collection<City>,
    pub merchants: Collection<Merchant>,
    pub products: Collection<Product>,
}

impl Simulation {
    pub fn new() -> Self {
        Simulation {
            time: 0.0,
            minute: 0,
            day: 0,
            cities: Collection::default(),
            merchants: Collection::default(),
            products: Collection::default(),
        }
    }
    pub fn tick(&mut self, dt: f64) {
        // Keep track of the day
        // Determine if we are in the next day
        // Each day has 24 hours
        // Each day is in a week
        // Each week is in a month
        self.time += dt;
        // If a minute hasn't passed yet, nothing should update.
        if (self.time < MINUTE_LENGTH) {
            return;
        }

        self.time -= MINUTE_LENGTH;
        self.minute += 1;

        let new_day = if (self.minute >= MINUTES_PER_DAY) {
            // Do daily stuff like updating workshops
            // Also, reset minute counter to 0
            self.minute = 0;
            self.day += 1;
            true
        } else {
            false
        };
    }

    pub fn create_test() -> Simulation {
        let mut sim = Simulation::new();

        let grain = sim.products.create(Product {
            name: "grain".into(),
            base_price: d128!(1),
        });

        let town_sample = sim.cities.create(City {
            name: "Townsville".into(),
            inventory: Inventory::default(),
            treasury: d128!(1),
            workshops: vec![Workshop {
                name: "Farm".into(),
                inputs: vec![],
                output: grain,
                amount: 10,
                budget: d128!(0),
                capacity: 1,
            }],
            prosperity: 1.0,
        });

        sim.merchants.create(Merchant {
            name: "Bob".into(),
            inventory: Inventory::default(),
            location: merchant::Location::City(town_sample),
            cash: d128!(10000000.69),
        });

        sim
    }
}
