use std::cell::RefCell;
use std::rc::Rc;

use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::world::{World, WorldMethods};
use eyre::Result;

#[test]
fn querying_for_multiple_components() -> Result<()> {
    let mut world = World::new();
    let location_name = "location";
    let size_name = "size";
    let location = Point::new(0.0, 0.0);
    let size = 15_u32;

    world.register(location_name)?;
    world.register(size_name)?;

    world
        .spawn_entity()?
        .with_component(location_name, location)?
        .with_component(size_name, size)?;

    let components = world.query(vec![location_name, size_name])?;
    let locations = &components[0];
    let sizes = &components[1];

    let wrapped_location: &Rc<RefCell<Point>> = locations[0].cast()?;
    let wrapped_size: &Rc<RefCell<u32>> = sizes[0].cast()?;

    let queried_location = wrapped_location.borrow();
    let queried_size = wrapped_size.borrow();

    assert_eq!(*queried_location, location);
    assert_eq!(*queried_size, size);
    Ok(())
}

#[test]
fn query_for_entities_when_not_all_entities_have_same_number_of_components() -> Result<()> {
    let mut world = World::new();
    let location_name = "location";
    let size_name = "size";
    let first_location = Point::new(0.0, 0.0);
    let second_location = Point::new(10.0, 10.0);
    let third_location = Point::new(15.0, 15.0);
    let size = 15_u32;
    let third_size = 30_u32;

    world.register(location_name)?;
    world.register(size_name)?;

    world
        .spawn_entity()?
        .with_component(location_name, first_location)?
        .with_component(size_name, size)?;

    world
        .spawn_entity()?
        .with_component(location_name, second_location)?;

    world
        .spawn_entity()?
        .with_component(location_name, third_location)?
        .with_component(size_name, third_size)?;

    let components = world.query(vec![location_name, size_name])?;
    let locations = &components[0];
    let sizes = &components[1];

    assert_eq!(locations.len(), sizes.len());
    assert_eq!(locations.len(), 2);
    let wrapped_queried_first_location: &Rc<RefCell<Point>> = locations[0].cast()?;
    let queried_first_location = wrapped_queried_first_location.borrow();
    assert_eq!(*queried_first_location, first_location);
    let wrapped_queried_second_location: &Rc<RefCell<Point>> = locations[1].cast()?;
    let queried_second_location = wrapped_queried_second_location.borrow();
    assert_eq!(*queried_second_location, third_location);
    Ok(())
}
