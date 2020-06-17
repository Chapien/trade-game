pub use self::{
    city::City,
    collection::{Collection, Id},
    inventory::Inventory,
    merchant::Merchant,
    product::Product,
};

use std::collections::HashMap;

pub mod city;
pub mod collection;
pub mod inventory;
pub mod merchant;
pub mod product;

pub struct Simulation {
    pub cities: Collection<City>,
    pub merchants: Collection<Merchant>,
    pub products: Collection<Product>,

    pub roads: HashMap<Id<City>, Id<City>>,
}
