use std::cell::RefCell;
use std::rc::Rc;

use bbecs::components::CastComponents;
use bbecs::resources::resource::ResourceCast;
use bbecs::world::{World, WorldMethods};
use eyre::Result;
use ggez::graphics::Color;

const RESOURCE_NAME: &str = "color";
const RESOURCE: Color = Color::new(0.5, 0.2, 0.8, 1.0);

#[test]
fn integration_test_point_resource() -> Result<()> {
    let mut world = World::new();

    world.add_resource(RESOURCE_NAME.to_owned(), RESOURCE);

    let wrapped_retrieved_resource = world.get_resource(RESOURCE_NAME)?.borrow();
    let retrieved_resource: &Color = wrapped_retrieved_resource.cast()?;

    assert_eq!(RESOURCE, *retrieved_resource);

    Ok(())
}

#[test]
#[allow(clippy::float_cmp)]
fn integration_test_point_resource_mut() -> Result<()> {
    let mut world = World::new();

    world.add_resource(RESOURCE_NAME.to_owned(), RESOURCE);

    let mut wrapped_retrieved_resource = world.get_resource(RESOURCE_NAME)?.borrow_mut();
    let retrieved_resource: &mut Color = wrapped_retrieved_resource.cast_mut()?;

    retrieved_resource.r += 0.1;

    drop(wrapped_retrieved_resource);

    let wrapped_retrieved_resource = world.get_resource(RESOURCE_NAME)?.borrow();
    let retrieved_resource: &Color = wrapped_retrieved_resource.cast()?;

    assert_eq!(Color::new(0.6, 0.2, 0.8, 1.0), *retrieved_resource);

    Ok(())
}

#[test]
fn integration_test_point_query() -> Result<()> {
    let mut world = World::new();

    world.register("color")?;

    world
        .spawn_entity()?
        .with_component("color", Color::new(0.5, 0.2, 0.8, 1.0))?;

    let query_results = world.query(vec!["color"])?;
    let colors = query_results.get("color").unwrap();

    let wrapped_colors: &Rc<RefCell<Color>> = colors[0].cast()?;
    let color = wrapped_colors.borrow();

    assert_eq!(*color, Color::new(0.5, 0.2, 0.8, 1.0));
    Ok(())
}

#[test]
fn integration_test_point_query_mut() -> Result<()> {
    let mut world = World::new();

    world.register("color")?;

    world
        .spawn_entity()?
        .with_component("color", Color::new(0.5, 0.2, 0.8, 1.0))?;

    {
        let query_results = world.query(vec!["color"])?;
        let colors = query_results.get("color").unwrap();
        let wrapped_colors: &Rc<RefCell<Color>> = colors[0].cast()?;
        let mut color = wrapped_colors.borrow_mut();
        color.r += 0.1;
    }

    let query_results = world.query(vec!["color"])?;
    let colors = query_results.get("color").unwrap();
    let wrapped_colors: &Rc<RefCell<Color>> = colors[0].cast()?;
    let color = wrapped_colors.borrow();

    assert_eq!(*color, Color::new(0.6, 0.2, 0.8, 1.0));
    Ok(())
}
