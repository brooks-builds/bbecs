macro_rules! test_world_by_type {
    ($type:ty, $name:ident) => {
        #[allow(clippy::float_cmp)]
        mod $name {
            use bbecs::components::CastComponents;
            use bbecs::query;
            use bbecs::resources::resource::ResourceCast;
            use bbecs::world::{World, WorldMethods};
            use eyre::Result;
            use std::cell::RefCell;
            use std::rc::Rc;
            #[test]
            fn cast_resource_immutably() -> Result<()> {
                let mut world = World::new();
                let new_resource = <$type>::default();
                world.add_resource("resource".to_owned(), new_resource);

                let wrapped_resource = world.get_resource("resource")?.borrow();
                let resource: &$type = wrapped_resource.cast()?;
                let expected_result = <$type>::default();
                assert_eq!(*resource, expected_result);
                Ok(())
            }

            #[test]
            fn cast_resource_mutably() -> Result<()> {
                let mut world = World::new();
                let new_resource = <$type>::default();
                world.add_resource("resource".to_owned(), new_resource);

                let mut wrapped_resource = world.get_resource("resource")?.borrow_mut();
                let resource: &mut $type = wrapped_resource.cast_mut()?;
                let expected_result = <$type>::default();
                assert_eq!(*resource, expected_result);
                Ok(())
            }

            #[test]
            fn query_component() -> Result<()> {
                let mut world = World::new();
                let data = <$type>::default();
                world.register("component")?;
                world.spawn_entity()?.with_component("component", data)?;

                let query;
                let components = query!(world, query, "component");
                let wrapped_component: &Rc<RefCell<$type>> = components.0[0].cast()?;
                assert_eq!(*wrapped_component.borrow(), <$type>::default());
                Ok(())
            }
        }
    };
}

test_world_by_type!(f32, testing_f32s);
test_world_by_type!(bbecs::data_types::point::Point, testing_points);
test_world_by_type!(u32, testing_u32s);
test_world_by_type!(usize, testing_usizes);
test_world_by_type!(bool, testing_booleans);
test_world_by_type!(String, testing_markers);
// Color(Color), // doesn't implement default
// Mesh(Mesh), // doesn't implement default
// GgezKeyCode(KeyCode), // doesn't implement default
// GgezText(Text), // cannot compare text with each other
// GgezSound(ggez::audio::SoundData), // doesn't implement default
