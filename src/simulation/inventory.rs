use crate::simulation::{Id, Product};
use std::collections::HashMap;

pub struct Inventory {
    pub items: HashMap<Id<Product>, u32>,
}
