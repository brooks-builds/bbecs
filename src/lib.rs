// The equivalent of an import / require from other languages. This allows us to reference the imported thing directly
// Instead of having to put it's full path in the code directly. For example we can use `Any` instead of `std::any::Any`.
// We can also bring in local modules as well as any core system or external crates using this same syntax.
use entities::Entities;
use query_builder::Query;
use resources::Resources;
use std::any::{Any, TypeId};

/// Tell Rust that the following modules are available and we want to access code in them. In this case look for files
/// names `entities`, `errors`, and so on. However we could also define a folder named `entities` and put a mod.rs file
/// inside it and this syntax would still work.
///
/// By making these public, we are essentially exporting them to the users of this library.
pub mod entities;
pub mod errors;
pub mod query_builder;
pub mod resources;

/// The world is the main entry to the library. Through this the developer can get access to the resources (one-off pieces
/// of data that are not owned by any one entity). Examples of resources include location that the mouse was clicked,
/// the fps, and so on.
///
/// The entities are the objects in the game world. Examples of entities would be the main player, enemies, background
/// sprites, and any cameras. Entities are almost always made up of multiple components, which are discreet pieces of
/// data.
///
/// By deriving Debug for this we are forcing the user of the library to derive or implement Debug for all of their
/// resources and components. This isn't necessarily a bad thing, but it is something to be aware of.
#[derive(Debug, Default)]
pub struct World {
    resources: Resources,
    entities: Entities,
}

impl World {
    /// Create a new empty default world. Since we are deriving Default for the World then we can use the default static
    /// method that was added for us.
    pub fn new() -> Self {
        Self::default()
    }

    /// We want to store a resource into the world. We are using dynamic dispatch which allows us to take in anything
    /// that has the provided trait. In this case we are using the Any trait, which allows us to pass in _almost_
    /// anything.
    pub fn add_resource(&mut self, resource: impl Any) {
        self.resources.add(resource);
    }

    /// Get resource function takes in a generic type. In order to tell the compiler as much information about what kind
    /// of type to expect we are constraining the incoming types with the `: Any + 'static` syntax. In this case our
    /// generic type doesn't have anything to do with what we are passing into the function, but rather what is going to
    /// be returned. We are telling the compiler to handle any type that implements the Any and 'static traits.
    pub fn get_resource<T: Any + 'static>(&self) -> Option<&T> {
        self.resources.get::<T>()
    }

    /// Because we are storing the resources normally, then we have the option of getting them immutably or mutably.
    /// It can be a good idea to give the ability to get mutable and immutably separately so the user of the library
    /// can decide what they need.
    pub fn get_resource_mut<T: Any + 'static>(&mut self) -> Option<&mut T> {
        self.resources.get_mut::<T>()
    }

    /// We are using the builder pattern here which allows a user to create an entity all in one
    /// go using . syntax.
    ///
    /// ```
    /// use bbecs::World;
    ///
    /// let mut world = World::new();
    /// world.register_entity() // This returns a mutable reference to the entities struct which will allow us to add all of the components for the entity.
    ///     .with_component(15.0_f32) // this assumes that only one f32 component exists in the entire world
    ///     .with_component(42_i32); // we can chain adding the components together for this entity
    /// ```
    pub fn register_entity(&mut self) -> &mut Entities {
        self.entities.register_entity()
    }

    /// The self tells the compile that this function is a method available to instantiated 'objects' of the parent struct
    /// The only way this can be called is by first creating an instance of the world.
    ///
    /// ```
    /// use bbecs::World;
    ///
    /// // This won't work because we can only call query after it's been instantiated. This is because the query has a 'self' in the method signature.
    /// // World.query()
    ///
    /// // This also won't work because the method query has the self parameter, which tells the compiler that it is a method available for instances of World.
    /// // World::query();
    ///
    /// let world = World::new();
    /// let query = world.query();
    /// ```
    pub fn query(&self) -> Query {
        self.entities.new_query()
    }

    /// Since we are storing the components in a HashMap using the type id as the key we need to get that type id if we
    /// want to find that list of components again. We can get the type id off of any generic using the TypeId struct.
    /// Otherwise we can call .type_id() on something.
    pub fn remove_component<T: Any + 'static>(&mut self, index: usize) {
        self.entities.remove_component(&TypeId::of::<T>(), index);
    }

    /// This will add any component to an existing entity, event if the component type hasn't been seen before.
    /// The entity index is the index of the entity in components vectors.
    pub fn add_component_to_entity(&mut self, component: impl Any, entity_index: usize) {
        self.entities
            .add_component_to_entity(component, entity_index);
    }
}
