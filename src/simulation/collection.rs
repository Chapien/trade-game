use std::collections::HashMap;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Collection<T> {
    items: HashMap<Id<T>, T>,
    next_id: Id<T>,
}

impl<T> Collection<T> {
    pub fn create(&mut self, item: T) -> Id<T> {
        let id = self.next_id.incr();
        self.items.insert(id, item);
        id
    }

    pub fn destroy(&mut self, id: Id<T>) {
        self.items.remove(&id);
    }

    pub fn get(&self, id: Id<T>) -> Option<&T> {
        self.items.get(&id)
    }

    pub fn get_mut(&mut self, id: Id<T>) -> Option<&mut T> {
        self.items.get_mut(&id)
    }
}

impl <T> Default for Collection<T> {
    fn default() -> Self {
        Collection {
            items: HashMap::default(),
            next_id: Id::default(),
        }
    }
}

#[derive(Debug)]
pub struct Id<T> {
    id: usize,
    _marker: PhantomData<T>,
}

impl<T> Id<T> {
    fn next(self) -> Self {
        Id {
            id: self.id + 1,
            _marker: PhantomData,
        }
    }

    fn incr(&mut self) -> Self {
        let val = *self;
        *self = self.next();
        val
    }
}

impl<T> Copy for Id<T> {}
impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Id {
            id: self.id,
            _marker: PhantomData,
        }
    }
}
impl<T> Default for Id<T> {
    fn default() -> Self {
        Id {
            id: 0,
            _marker: PhantomData,
        }
    }
}
impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl<T> Eq for Id<T> {}
impl<T> std::hash::Hash for Id<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(&self.id, state)
    }
}
