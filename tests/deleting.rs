use std::cell::RefCell;
use std::rc::Rc;

use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::world::{World, WorldMethods, ENTITY_ID};
use eyre::Result;

#[test]
fn deleting_an_entity_by_id() -> Result<()> {
    let mut world = World::new();

    world.register("location")?;
    world.register("size")?;

    world
        .spawn_entity()?
        .with_component("location", Point::new(0.0, 0.0))?
        .with_component("size", 25.0_f32)?;

    world
        .spawn_entity()?
        .with_component("location", Point::new(10.0, 10.0))?;

    world
        .spawn_entity()?
        .with_component("location", Point::new(15.0, 15.0))?
        .with_component("size", 50.0_f32)?;
    {
        let query_results = world.query(vec!["location", ENTITY_ID])?;
        let locations = &query_results[0];
        let ids = &query_results[1];

        for (index, location) in locations.iter().enumerate() {
            let wrapped_location: &Rc<RefCell<Point>> = location.cast()?;
            let location = wrapped_location.borrow();
            if *location == Point::new(10.0, 10.0) {
                let wrapped_id: &Rc<RefCell<u32>> = ids[index].cast()?;
                let id = wrapped_id.borrow();
                world.delete_by_id(*id)?;
            }
        }
    }

    world.update()?;

    let query_results = world.query(vec!["location"])?;
    let queried_locations = &query_results[0];
    let wrapped_location: &Rc<RefCell<Point>> = queried_locations[1].cast()?;
    let location = wrapped_location.borrow();

    assert_eq!(queried_locations.len(), 2);
    assert_eq!(*location, Point::new(15.0, 15.0));

    Ok(())
}
