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
fn integration_test_point_query_one_components() -> Result<()> {
    let mut world = World::new();
    let component_name = "location";
    let component = Point::new(0.0, 0.0);

    world.register(component_name)?;

    world
        .spawn_entity()?
        .with_component(component_name, component)?;

    let components = world.query_one(component_name)?;
    let wrapped_location: &Rc<RefCell<Point>> = components[0].cast()?;
    let queried_location = wrapped_location.borrow();

    assert_eq!(*queried_location, component);
    Ok(())
}

#[test]
fn integration_test_point_query_one_mut_components() -> Result<()> {
    let mut world = World::new();
    let component_name = "location";
    let component = Point::new(0.0, 0.0);

    world.register(component_name)?;

    world
        .spawn_entity()?
        .with_component(component_name, component)?;

    let components = world.query_one(component_name)?;
    let wrapped_location: &Rc<RefCell<Point>> = components[0].cast()?;
    {
        let mut queried_location = wrapped_location.borrow_mut();

        queried_location.x += 10.0;
    }

    let component = Point::new(10.0, 0.0);
    let wrapped_updated_location: &Rc<RefCell<Point>> = components[0].cast()?;
    let updated_location = wrapped_updated_location.borrow();

    assert_eq!(*updated_location, component);
    Ok(())
}
