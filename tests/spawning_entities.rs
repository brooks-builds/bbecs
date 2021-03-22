use std::cell::RefCell;
use std::rc::Rc;

use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::world::{World, WorldMethods};
use eyre::Result;

#[test]
fn spawning_entities_includes_id() -> Result<()> {
    let mut world = World::new();
    let location_name = "location";
    let location = Point::new(0.0, 0.0);

    world.register(location_name)?;

    world
        .spawn_entity()?
        .with_component(location_name, location)?;

    world
        .spawn_entity()?
        .with_component(location_name, location)?;

    let entity_ids = world.query_one("entity id")?;

    for (index, wrapped_id) in entity_ids.iter().enumerate() {
        let wrapped_id: &Rc<RefCell<u32>> = wrapped_id.cast()?;
        let id = wrapped_id.borrow();
        assert_eq!(*id, index as u32);
    }
    Ok(())
}

#[test]
fn spawning_entities_includes_deleted_flag() -> Result<()> {
    let mut world = World::new();
    let location_name = "location";
    let location = Point::new(0.0, 0.0);

    world.register(location_name)?;

    world
        .spawn_entity()?
        .with_component(location_name, location)?;

    let is_deleted = world.query_one("to be deleted")?;
    let is_deleted: &Rc<RefCell<bool>> = is_deleted[0].cast()?;
    let is_deleted = is_deleted.borrow();

    assert_eq!(*is_deleted, false);
    Ok(())
}
