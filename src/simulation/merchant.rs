use crate::simulation::{City, Id, Inventory, Money};

pub struct Merchant {
    pub name: String,
    pub inventory: Inventory,
    pub location: Location,
    pub cash: Money,
}

pub enum Location {
    City(Id<City>),
    Traveling {
        origin: Id<City>,
        destination: Id<City>,
        progress: f64,
    },
}
