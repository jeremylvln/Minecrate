use specs::prelude::*;

pub struct ECSWorld<'a, 'b> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> ECSWorld<'a, 'b> {
    pub fn new() -> Self {
        let mut world = World::new();
        let mut dispatcher = DispatcherBuilder::new().build();

        dispatcher.setup(&mut world);

        Self { world, dispatcher }
    }

    pub fn tick(&mut self) {
        self.dispatcher.dispatch(&self.world);
        self.world.maintain();
    }
}

impl<'a, 'b> Default for ECSWorld<'a, 'b> {
    fn default() -> Self {
        Self::new()
    }
}
