use std::cell::RefCell;
use std::rc::Rc;

use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::world::{DataWrapper, World, WorldMethods, ENTITY_ID};
use eyre::Result;

#[test]
fn deleting_an_entity_by_id() -> Result<()> {
    let mut world = World::new();

    world.register("size")?;
    world.register("location")?;

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
        let locations = query_results.get("location").unwrap();
        let ids = query_results.get(ENTITY_ID).unwrap();

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
    let queried_locations = query_results.get("location").unwrap();
    let wrapped_location: &Rc<RefCell<Point>> = queried_locations[1].cast()?;
    let location = wrapped_location.borrow();

    assert_eq!(queried_locations.len(), 2);
    assert_eq!(*location, Point::new(15.0, 15.0));

    Ok(())
}

#[test]
#[allow(clippy::float_cmp)]
fn inserting_an_entity_after_deleting_should_work() -> Result<()> {
    let mut world = World::new();
    world.register("size")?;
    world.spawn_entity()?.with_component("size", 15.0_f32)?;

    let query = world.query(vec!["size", ENTITY_ID])?;
    let _wrapped_size: &DataWrapper<f32> = query.get("size").unwrap()[0].cast()?;
    let wrapped_id: &DataWrapper<u32> = query.get(ENTITY_ID).unwrap()[0].cast()?;

    let id = *wrapped_id.borrow();

    world.delete_by_id(id)?;

    world.update()?;

    world.spawn_entity()?.with_component("size", 30.0_f32)?;

    let query = world.query(vec!["size", ENTITY_ID])?;
    let wrapped_size: &DataWrapper<f32> = query.get("size").unwrap()[0].cast()?;
    let wrapped_id: &DataWrapper<u32> = query.get(ENTITY_ID).unwrap()[0].cast()?;

    assert_eq!(1, *wrapped_id.borrow());
    assert_eq!(*wrapped_size.borrow(), 30.0_f32);

    Ok(())
}
