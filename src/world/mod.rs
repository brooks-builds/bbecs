mod entity_data;

use std::cell::RefCell;
use std::rc::Rc;

use entity_data::EntityData;
use eyre::Result;
use ggez::event::KeyCode;
use ggez::graphics::{Color, Mesh, Text};

use crate::components::{CastComponents, Component, Components};
use crate::data_types::point::Point;
use crate::resources::resource::Resource;
use crate::resources::resources_data::ResourcesData;

use self::entity_data::EntityDataTraits;

const TO_BE_DELETED: &str = "to be deleted";

pub trait WorldMethods<T> {
    fn with_component<S: Into<String>>(&mut self, name: S, data: T) -> Result<&mut Self>;
    fn add_resource<S: Into<String>>(&mut self, name: S, data: T);
}

pub struct World {
    pub entity_data: EntityData,
    resources: ResourcesData,
    is_empty: bool,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register<S: Into<String>>(&mut self, name: S, component_type: Component) -> Result<()> {
        self.entity_data.register(name.into(), component_type)
    }

    pub fn spawn_entity(&mut self) -> Result<&mut Self> {
        if self.is_empty {
            self.entity_data
                .register(TO_BE_DELETED.into(), Component::Bool)?;
            self.is_empty = false;
        }
        self.entity_data.insert(TO_BE_DELETED, false)?;
        Ok(self)
    }

    pub fn query_one<S: Into<String>>(&self, name: S) -> Result<&Rc<RefCell<Components>>> {
        self.entity_data.query_one(&name.into())
    }

    pub fn get_resource<S: Into<String>>(&self, name: S) -> Result<&Rc<RefCell<Resource>>> {
        self.resources.get(&name.into())
    }

    pub fn delete_entity_by_index(&self, index: usize) -> Result<()> {
        let mut wrapped_to_be_deleted = self.entity_data.query_one(TO_BE_DELETED)?.borrow_mut();
        let to_be_deleted: &mut Vec<bool> = wrapped_to_be_deleted.cast_mut()?;
        to_be_deleted[index] = true;

        Ok(())
    }

    pub fn update(&self) -> Result<()> {
        if self.is_empty {
            return Ok(());
        }

        let wrapped_to_be_deleted = self.entity_data.query_one(TO_BE_DELETED)?.borrow();
        let to_be_deleted: &Vec<bool> = wrapped_to_be_deleted.cast()?;
        let mut indexes_to_delete =
            to_be_deleted
                .iter()
                .enumerate()
                .fold(vec![], |mut indexes, (index, is_deleted)| {
                    if *is_deleted {
                        indexes.push(index);
                    }
                    indexes
                });
        drop(wrapped_to_be_deleted);

        indexes_to_delete.reverse();
        indexes_to_delete
            .into_iter()
            .try_for_each(|index| self.entity_data.delete_by_index(index))
    }
}

impl Default for World {
    fn default() -> Self {
        Self {
            entity_data: EntityData::new(),
            resources: ResourcesData::new(),
            is_empty: true,
        }
    }
}

impl WorldMethods<Point> for World {
    fn with_component<S: Into<String>>(&mut self, name: S, data: Point) -> Result<&mut Self> {
        self.entity_data.insert(&name.into(), data)?;
        Ok(self)
    }

    fn add_resource<S: Into<String>>(&mut self, name: S, data: Point) {
        self.resources.insert(name.into(), Resource::Point(data));
    }
}

impl WorldMethods<Color> for World {
    fn with_component<S: Into<String>>(&mut self, name: S, data: Color) -> Result<&mut Self> {
        self.entity_data.insert(&name.into(), data)?;
        Ok(self)
    }

    fn add_resource<S: Into<String>>(&mut self, name: S, data: Color) {
        self.resources.insert(name.into(), Resource::Color(data));
    }
}

impl WorldMethods<Mesh> for World {
    fn with_component<S: Into<String>>(&mut self, name: S, data: Mesh) -> Result<&mut Self> {
        self.entity_data.insert(&name.into(), data)?;
        Ok(self)
    }

    fn add_resource<S: Into<String>>(&mut self, name: S, data: Mesh) {
        self.resources.insert(name.into(), Resource::Mesh(data));
    }
}

impl WorldMethods<u32> for World {
    fn with_component<S: Into<String>>(&mut self, name: S, data: u32) -> Result<&mut Self> {
        self.entity_data.insert(&name.into(), data)?;
        Ok(self)
    }

    fn add_resource<S: Into<String>>(&mut self, name: S, data: u32) {
        self.resources.insert(name.into(), Resource::U32(data));
    }
}

impl WorldMethods<f32> for World {
    fn with_component<S: Into<String>>(&mut self, name: S, data: f32) -> Result<&mut Self> {
        self.entity_data.insert(&name.into(), data)?;
        Ok(self)
    }

    fn add_resource<S: Into<String>>(&mut self, name: S, data: f32) {
        self.resources.insert(name.into(), Resource::F32(data));
    }
}

impl WorldMethods<usize> for World {
    fn with_component<S: Into<String>>(&mut self, name: S, data: usize) -> Result<&mut Self> {
        self.entity_data.insert(&name.into(), data)?;
        Ok(self)
    }

    fn add_resource<S: Into<String>>(&mut self, name: S, data: usize) {
        self.resources.insert(name.into(), Resource::Usize(data));
    }
}

impl WorldMethods<bool> for World {
    fn with_component<S: Into<String>>(&mut self, name: S, data: bool) -> Result<&mut Self> {
        self.entity_data.insert(&name.into(), data)?;
        Ok(self)
    }

    fn add_resource<S: Into<String>>(&mut self, name: S, data: bool) {
        self.resources.insert(name.into(), Resource::Bool(data));
    }
}

impl WorldMethods<KeyCode> for World {
    fn with_component<S: Into<String>>(&mut self, name: S, data: KeyCode) -> Result<&mut Self> {
        self.entity_data.insert(&name.into(), data)?;
        Ok(self)
    }

    fn add_resource<S: Into<String>>(&mut self, name: S, data: KeyCode) {
        self.resources
            .insert(name.into(), Resource::GgezKeyCode(data));
    }
}

impl WorldMethods<String> for World {
    fn with_component<S: Into<String>>(&mut self, name: S, data: String) -> Result<&mut Self> {
        self.entity_data.insert(&name.into(), data)?;
        Ok(self)
    }

    fn add_resource<S: Into<String>>(&mut self, name: S, data: String) {
        self.resources.insert(name.into(), Resource::Marker(data));
    }
}

impl WorldMethods<Text> for World {
    fn with_component<S: Into<String>>(&mut self, name: S, data: Text) -> Result<&mut Self> {
        self.entity_data.insert(&name.into(), data)?;
        Ok(self)
    }

    fn add_resource<S: Into<String>>(&mut self, name: S, data: Text) {
        self.resources.insert(name.into(), Resource::GgezText(data));
    }
}

#[cfg(test)]
mod tests {

    use super::{Component, Result, World, WorldMethods};
    use crate::components::CastComponents;
    use crate::data_types::point::Point;
    use crate::resources::resource::ResourceCast;
    use ggez::event::KeyCode;

    #[test]
    fn should_get_key_code_resource() -> Result<()> {
        let mut world = World::new();
        world.add_resource("keycode", KeyCode::A);
        let wrapped_keycode = world.get_resource("keycode")?.borrow();
        let keycode: &KeyCode = wrapped_keycode.cast()?;
        assert_eq!(*keycode, KeyCode::A);
        Ok(())
    }

    #[test]
    fn should_get_key_code_component() -> Result<()> {
        let mut world = World::new();
        world.register("keycode", Component::GgezKeyCode)?;
        world
            .spawn_entity()?
            .with_component("keycode", KeyCode::A)?;
        let wrapped_keycodes = world.query_one("keycode").unwrap().borrow();
        let keycodes: &Vec<KeyCode> = wrapped_keycodes.cast()?;
        assert_eq!(keycodes[0], KeyCode::A);
        Ok(())
    }

    #[test]
    fn should_mutably_get_key_code_resource() -> Result<()> {
        let mut world = World::new();
        world.add_resource("keycode", KeyCode::A);
        let mut wrapped_keycode = world.get_resource("keycode")?.borrow_mut();
        let keycode: &mut KeyCode = wrapped_keycode.cast_mut()?;
        *keycode = KeyCode::B;
        let keycode: &KeyCode = wrapped_keycode.cast()?;
        assert_eq!(*keycode, KeyCode::B);
        Ok(())
    }

    #[test]
    fn should_mutably_get_key_code_component() -> Result<()> {
        let mut world = World::new();
        world.register("keycode", Component::GgezKeyCode)?;
        world
            .spawn_entity()?
            .with_component("keycode", KeyCode::A)?;
        let mut wrapped_keycodes = world.query_one("keycode").unwrap().borrow_mut();
        let keycodes: &mut Vec<KeyCode> = wrapped_keycodes.cast_mut()?;
        keycodes[0] = KeyCode::B;
        drop(wrapped_keycodes);
        let wrapped_keycodes = world.query_one("keycode").unwrap().borrow();
        let keycodes: &Vec<KeyCode> = wrapped_keycodes.cast()?;
        assert_eq!(keycodes[0], KeyCode::B);
        Ok(())
    }

    #[test]
    fn should_get_marker_resource() -> Result<()> {
        let mut world = World::new();
        world.add_resource("marker", "player".to_owned());
        let wrapped_marker = world.get_resource("marker")?.borrow();
        let marker: &String = wrapped_marker.cast()?;
        assert_eq!(*marker, "player".to_owned());
        Ok(())
    }

    #[test]
    fn should_get_marker_component() -> Result<()> {
        let mut world = World::new();
        world.register("marker", Component::Marker)?;
        world
            .spawn_entity()?
            .with_component("marker", "player".to_owned())?;
        let wrapped_markers = world.query_one("marker").unwrap().borrow();
        let markers: &Vec<String> = wrapped_markers.cast()?;
        assert_eq!(markers[0], "player".to_owned());
        Ok(())
    }

    #[test]
    fn should_mutably_get_marker_resource() -> Result<()> {
        let mut world = World::new();
        world.add_resource("marker", "player".to_owned());
        let mut wrapped_marker = world.get_resource("marker")?.borrow_mut();
        let marker: &mut String = wrapped_marker.cast_mut()?;
        *marker = "asteroid".to_owned();
        let marker: &String = wrapped_marker.cast()?;
        assert_eq!(*marker, "asteroid".to_owned());
        Ok(())
    }

    #[test]
    fn should_mutably_get_marker_component() -> Result<()> {
        let mut world = World::new();
        world.register("marker", Component::Marker)?;
        world
            .spawn_entity()?
            .with_component("marker", "player".to_owned())?;
        let mut wrapped_markers = world.query_one("marker").unwrap().borrow_mut();
        let markers: &mut Vec<String> = wrapped_markers.cast_mut()?;
        markers[0] = "asteroid".to_owned();
        drop(wrapped_markers);
        let wrapped_markers = world.query_one("marker").unwrap().borrow();
        let markers: &Vec<String> = wrapped_markers.cast()?;
        assert_eq!(markers[0], "asteroid".to_owned());
        Ok(())
    }

    // delete entity tests
    #[test]
    fn should_be_able_to_delete_an_entity_by_index() -> Result<()> {
        let mut world = World::new();
        world.register("location", Component::Point)?;
        world.register("name", Component::Marker)?;
        world
            .spawn_entity()?
            .with_component("location", Point::new(0.0, 0.0))?
            .with_component("name", "Player".to_owned())?;
        world
            .spawn_entity()?
            .with_component("location", Point::new(10.0, 10.0))?
            .with_component("name", "asteroid".to_owned())?;
        world.delete_entity_by_index(0)?;

        world.update()?;

        let wrapped_names = world.query_one("name")?.borrow();
        let names: &Vec<String> = wrapped_names.cast()?;

        assert_eq!(names.len(), 1);
        assert_eq!(names[0], "asteroid");
        Ok(())
    }

    #[test]
    fn should_be_able_to_delete_all_entities() -> Result<()> {
        let mut world = World::new();

        world.register("name", Component::Marker)?;

        world
            .spawn_entity()?
            .with_component("name", "first".to_owned())?;
        world
            .spawn_entity()?
            .with_component("name", "second".to_owned())?;
        world
            .spawn_entity()?
            .with_component("name", "third".to_owned())?;

        world.delete_entity_by_index(0)?;
        world.delete_entity_by_index(1)?;
        world.delete_entity_by_index(2)?;

        world.update()?;

        let wrapped_names = world.query_one("name")?.borrow();
        let names: &Vec<String> = wrapped_names.cast()?;

        assert_eq!(names.len(), 0);

        Ok(())
    }
}

#[cfg(test)]
mod testing_text {
    use super::{Component, Result, World, WorldMethods};
    use crate::components::CastComponents;
    use crate::resources::resource::ResourceCast;
    use ggez::graphics::Text;

    #[test]
    fn should_get_text_resource() -> Result<()> {
        let mut world = World::new();
        world.add_resource("text", Text::new("this is a text"));
        let wrapper_text = world.get_resource("text")?.borrow();
        let text: &Text = wrapper_text.cast()?;
        assert_eq!(text.contents(), "this is a text");
        Ok(())
    }

    #[test]
    fn should_get_text_component() -> Result<()> {
        let mut world = World::new();
        world.register("text", Component::GgezText)?;
        world
            .spawn_entity()?
            .with_component("text", Text::new("texting"))?;
        let wrapper_texts = world.query_one("text").unwrap().borrow();
        let texts: &Vec<Text> = wrapper_texts.cast()?;
        assert_eq!(texts[0].contents(), "texting");
        Ok(())
    }

    #[test]
    fn should_mutably_get_text_resource() -> Result<()> {
        let mut world = World::new();
        world.add_resource("text", Text::new("tt"));
        let mut wrapper_text = world.get_resource("text")?.borrow_mut();
        let text: &mut Text = wrapper_text.cast_mut()?;
        *text = Text::new("other text");
        let text: &Text = wrapper_text.cast()?;
        assert_eq!(text.contents(), "other text");
        Ok(())
    }

    #[test]
    fn should_mutably_get_text_component() -> Result<()> {
        let mut world = World::new();
        world.register("text", Component::GgezText)?;
        world
            .spawn_entity()?
            .with_component("text", Text::new("meow"))?;
        let mut wrapper_texts = world.query_one("text").unwrap().borrow_mut();
        let texts: &mut Vec<Text> = wrapper_texts.cast_mut()?;
        texts[0] = Text::new("bark");
        drop(wrapper_texts);
        let wrapper_texts = world.query_one("text").unwrap().borrow();
        let texts: &Vec<Text> = wrapper_texts.cast()?;
        assert_eq!(texts[0].contents(), "bark");
        Ok(())
    }
}
