/// Get a reference to a resource in the world.
///
/// ```
/// use bbecs::world::World;
/// use bbecs::get_resource;
/// use crate::bbecs::world::WorldMethods;
/// use crate::bbecs::resources::resource::ResourceCast;
///
/// let mut world = World::new();
/// world.add_resource("size".to_string(), 15.0_f32);
/// let size: &f32;
/// get_resource!(size, world, "size");
/// assert!(*size == 15.0_f32);
/// ```
#[macro_export]
macro_rules! get_resource {
    ($resource:ident, $world:expr, $name:expr) => {
        let wrapper = $world.get_resource($name).unwrap().borrow();
        $resource = wrapper.cast().unwrap();
    };
}

/// Get a mutable reference to a resource from the provided world so that we can change it. This will
/// probably need to be put into a block or dropped so that other queries can work.
///
/// ```
/// use bbecs::world::World;
/// use bbecs::{get_resource, get_resource_mut};
/// use crate::bbecs::world::WorldMethods;
/// use crate::bbecs::resources::resource::ResourceCast;
///
/// let mut world = World::new();
/// world.add_resource("size".to_string(), 15.0_f32);
///
/// {
///     let size: &mut f32;
///     get_resource_mut!(size, world, "size");
///
///     *size += 1.0;
/// }
///
/// let size: &f32;
/// get_resource!(size, world, "size");
/// assert_eq!(*size, 16.0_f32);
/// ```
#[macro_export]
macro_rules! get_resource_mut {
    ($resource:ident, $world:expr, $name:expr) => {
        let mut wrapper = $world.get_resource($name).unwrap().borrow_mut();
        $resource = wrapper.cast_mut().unwrap();
    };
}
