pub use self::{
    city::City,
    collection::{Collection, Id},
    inventory::Inventory,
    merchant::Merchant,
    money::Money,
    product::Product,
};

use std::collections::HashMap;

pub mod city;
pub mod collection;
pub mod inventory;
pub mod merchant;
pub mod money;
pub mod product;

pub struct Simulation {
    pub cities: Collection<City>,
    pub merchants: Collection<Merchant>,
    pub products: Collection<Product>,
}
