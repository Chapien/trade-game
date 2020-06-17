use crate::simulation::{Id, Product};
use std::collections::HashMap;

#[derive(Default)]
pub struct Inventory {
    pub items: HashMap<Id<Product>, u32>,
}
