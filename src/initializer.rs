use typedefs::*;
use simple_ecs::system::System;
use simple_ecs::entity::EntityStore;
use std::mem::replace;

pub enum Initializer<F: FnOnce(&mut EntityStore) -> ()> {
    Uninitialized(F),
    Initialized,
}

impl<F: FnOnce(&mut EntityStore)> Initializer<F> {
    pub fn new(f: F) -> Self {
        Initializer::Uninitialized(f)
    }
}

impl<F: FnOnce(&mut EntityStore)> System<(), UpdateData> for Initializer<F> {
    fn update_all(&mut self, es: &mut EntityStore, _: &UpdateData) {
        if let Initializer::Uninitialized(f) = replace(
            self,
            Initializer::Initialized
        ) {
            f(es);
        }
    }
}
