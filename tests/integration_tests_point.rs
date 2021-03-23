use std::cell::RefCell;
use std::rc::Rc;

use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::resources::resource::ResourceCast;
use bbecs::world::{World, WorldMethods};
use eyre::Result;

// insert and get out resources
// insert and query for components
// delete an entity

#[test]
fn integration_test_point_resource() -> Result<()> {
    let mut world = World::new();
    let location = Point::new(0.0, 0.0);
    let resource_name = "location";

    world.add_resource(resource_name.to_owned(), location);

    let wrapped_retrieved_location = world.get_resource(resource_name)?.borrow();
    let retrieved_location: &Point = wrapped_retrieved_location.cast()?;

    assert_eq!(location, *retrieved_location);

    Ok(())
}

#[test]
#[allow(clippy::float_cmp)]
fn integration_test_point_resource_mut() -> Result<()> {
    let mut world = World::new();
    let location = Point::new(0.0, 0.0);
    let resource_name = "location";

    world.add_resource(resource_name.to_owned(), location);

    let mut wrapped_retrieved_location = world.get_resource(resource_name)?.borrow_mut();
    let retrieved_location: &mut Point = wrapped_retrieved_location.cast_mut()?;

    retrieved_location.x += 10.0;

    drop(wrapped_retrieved_location);

    let wrapped_retrieved_location = world.get_resource(resource_name)?.borrow();
    let retrieved_location: &Point = wrapped_retrieved_location.cast()?;

    assert_eq!(location.x + 10.0, retrieved_location.x);
    assert_eq!(location.y, retrieved_location.y);

    Ok(())
}

#[test]
fn integration_test_point_query() -> Result<()> {
    let mut world = World::new();

    world.register("location")?;

    world
        .spawn_entity()?
        .with_component("location", Point::new(0.0, 0.0))?;

    let query_results = world.query(vec!["location"])?;
    let locations = &query_results[0];

    let wrapped_location: &Rc<RefCell<Point>> = locations[0].cast()?;
    let location = wrapped_location.borrow();

    assert_eq!(*location, Point::new(0.0, 0.0));
    Ok(())
}

#[test]
fn integration_test_point_query_mut() -> Result<()> {
    let mut world = World::new();

    world.register("location")?;

    world
        .spawn_entity()?
        .with_component("location", Point::new(0.0, 0.0))?;

    {
        let query_results = world.query(vec!["location"])?;
        let locations = &query_results[0];
        let wrapped_location: &Rc<RefCell<Point>> = locations[0].cast()?;
        let mut location = wrapped_location.borrow_mut();
        location.x += 10.0;
    }

    let query_results = world.query(vec!["location"])?;
    let locations = &query_results[0];
    let wrapped_location: &Rc<RefCell<Point>> = locations[0].cast()?;
    let location = wrapped_location.borrow();

    assert_eq!(*location, Point::new(10.0, 0.0));
    Ok(())
}
