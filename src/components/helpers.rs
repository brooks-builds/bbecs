#[macro_export]
/// Helper to query for components in the world. We have to give a query variable that will last longer
/// than the macro for this to work.
///
/// ```
/// use bbecs::query;
/// use bbecs::world::{World, ENTITY_ID, WorldMethods, DataWrapper};
/// use bbecs::components::CastComponents;
///
/// // creating the world and adding an entity for us to query later
/// let mut world = World::new();
///     world.register("size").unwrap();
///     world
///         .spawn_entity()
///         .unwrap()
///         .with_component("size", 15.0_f32)
///         .unwrap();
///
/// // Now we are querying for the size and an entity id which is automatically added to all entities
/// let query;
/// let (sizes, entity_ids) = query!(world, query, "size", ENTITY_ID);
///
/// for (index, size) in sizes.iter().enumerate() {
///     let size: &DataWrapper<f32> = size.cast().unwrap();
///     let entity_id: &DataWrapper<u32> = entity_ids[index].cast().unwrap();
///
///     assert_eq!(*size.borrow(), 15.0);
///     assert_eq!(*entity_id.borrow(), 0);
/// }
/// ```
macro_rules! query {
    ($world:expr, $query:expr, $($name:expr),*) => {{
        $query = $world.query(vec![$($name,)*]).unwrap();

        (
            $($query.get($name).unwrap(),)*
        )
    }};
}
