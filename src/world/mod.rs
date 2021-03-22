pub mod bitmap;
mod entity_data;

use std::cell::RefCell;
use std::rc::Rc;

use entity_data::EntityData;
use eyre::Result;

use crate::components::ComponentData;
use crate::data_types::point::Point;
use crate::resources::resource::Resource;
use crate::resources::resources_data::ResourcesData;

use self::bitmap::BitMap;
use self::entity_data::EntityDataTraits;

const TO_BE_DELETED: &str = "to be deleted";
const ENTITY_ID: &str = "entity id";

pub trait WorldMethods<T> {
    fn with_component(&mut self, name: &str, data: T) -> Result<&mut Self>;
    fn add_resource(&mut self, name: String, data: T);
}

pub struct World {
    pub entity_data: EntityData,
    resources: ResourcesData,
    is_empty: bool,
    next_entity_id: u32,
    bitmap: BitMap,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register<S: ToString>(&mut self, name: S) -> Result<()> {
        self.entity_data.register(name.to_string())?;
        self.bitmap.register(name.to_string());
        Ok(())
    }

    pub fn spawn_entity(&mut self) -> Result<&mut Self> {
        self.entity_data.insert(TO_BE_DELETED, false)?;
        self.entity_data.insert(ENTITY_ID, self.next_entity_id)?;
        self.bitmap.spawn_entity();
        self.bitmap.insert(TO_BE_DELETED)?;
        self.bitmap.insert(ENTITY_ID)?;
        self.is_empty = false;
        self.next_entity_id += 1;

        Ok(self)
    }

    pub fn query_one<S: Into<String>>(&self, name: S) -> Result<&Vec<ComponentData>> {
        self.entity_data.query_one(&name.into())
    }

    pub fn query(&self, names: Vec<&str>) -> Result<Vec<Vec<&ComponentData>>> {
        let bitmap_query = self.bitmap.query(names.clone())?;
        self.entity_data.query(names, bitmap_query)
    }

    pub fn get_resource<S: Into<String>>(&self, name: S) -> Result<&Rc<RefCell<Resource>>> {
        self.resources.get(&name.into())
    }

    // pub fn update(&self) -> Result<()> {
    //     if self.is_empty {
    //         return Ok(());
    //     }

    //     let wrapped_to_be_deleted = self.entity_data.query_one(TO_BE_DELETED)?.borrow();
    //     let to_be_deleted: &Vec<bool> = wrapped_to_be_deleted.cast()?;
    //     let mut indexes_to_delete =
    //         to_be_deleted
    //             .iter()
    //             .enumerate()
    //             .fold(vec![], |mut indexes, (index, is_deleted)| {
    //                 if *is_deleted {
    //                     indexes.push(index);
    //                 }
    //                 indexes
    //             });
    //     drop(wrapped_to_be_deleted);

    //     indexes_to_delete.reverse();
    //     indexes_to_delete
    //         .into_iter()
    //         .try_for_each(|index| self.entity_data.delete_by_index(index))
    // }
}

impl Default for World {
    fn default() -> Self {
        let mut entity_data = EntityData::new();
        let mut bitmap = BitMap::new();

        entity_data.register(TO_BE_DELETED.into()).unwrap();
        bitmap.register(TO_BE_DELETED.into());
        entity_data.register(ENTITY_ID.into()).unwrap();
        bitmap.register(ENTITY_ID.into());

        Self {
            entity_data,
            resources: ResourcesData::new(),
            is_empty: true,
            next_entity_id: 0,
            bitmap,
        }
    }
}

impl WorldMethods<Point> for World {
    fn with_component(&mut self, name: &str, data: Point) -> Result<&mut Self> {
        self.entity_data.insert(name, data)?;
        self.bitmap.insert(name)?;
        Ok(self)
    }

    fn add_resource(&mut self, name: String, data: Point) {
        self.resources.insert(name, Resource::Point(data));
    }
}

// impl WorldMethods<Color> for World {
//     fn with_component(&mut self, name: &str, data: Color) -> Result<&mut Self> {
//         self.entity_data.insert(name, data)?;
//         self.bitmap.insert(name)?;
//         Ok(self)
//     }

//     fn add_resource(&mut self, name: String, data: Color) {
//         self.resources.insert(name, Resource::Color(data));
//     }
// }

// impl WorldMethods<Mesh> for World {
//     fn with_component(&mut self, name: &str, data: Mesh) -> Result<&mut Self> {
//         self.entity_data.insert(name, data)?;
//         self.bitmap.insert(name)?;
//         Ok(self)
//     }

//     fn add_resource(&mut self, name: String, data: Mesh) {
//         self.resources.insert(name, Resource::Mesh(data));
//     }
// }

impl WorldMethods<u32> for World {
    fn with_component(&mut self, name: &str, data: u32) -> Result<&mut Self> {
        self.entity_data.insert(name, data)?;
        self.bitmap.insert(name)?;
        Ok(self)
    }

    fn add_resource(&mut self, name: String, data: u32) {
        self.resources.insert(name, Resource::U32(data));
    }
}

// impl WorldMethods<f32> for World {
//     fn with_component(&mut self, name: &str, data: f32) -> Result<&mut Self> {
//         self.entity_data.insert(name, data)?;
//         self.bitmap.insert(name)?;
//         Ok(self)
//     }

//     fn add_resource(&mut self, name: String, data: f32) {
//         self.resources.insert(name, Resource::F32(data));
//     }
// }

// impl WorldMethods<usize> for World {
//     fn with_component(&mut self, name: &str, data: usize) -> Result<&mut Self> {
//         self.entity_data.insert(name, data)?;
//         self.bitmap.insert(name)?;
//         Ok(self)
//     }

//     fn add_resource(&mut self, name: String, data: usize) {
//         self.resources.insert(name, Resource::Usize(data));
//     }
// }

// impl WorldMethods<bool> for World {
//     fn with_component(&mut self, name: &str, data: bool) -> Result<&mut Self> {
//         self.entity_data.insert(name, data)?;
//         self.bitmap.insert(name)?;
//         Ok(self)
//     }

//     fn add_resource(&mut self, name: String, data: bool) {
//         self.resources.insert(name, Resource::Bool(data));
//     }
// }

// impl WorldMethods<KeyCode> for World {
//     fn with_component(&mut self, name: &str, data: KeyCode) -> Result<&mut Self> {
//         self.entity_data.insert(name, data)?;
//         self.bitmap.insert(name)?;
//         Ok(self)
//     }

//     fn add_resource(&mut self, name: String, data: KeyCode) {
//         self.resources.insert(name, Resource::GgezKeyCode(data));
//     }
// }

// impl WorldMethods<String> for World {
//     fn with_component(&mut self, name: &str, data: String) -> Result<&mut Self> {
//         self.entity_data.insert(name, data)?;
//         self.bitmap.insert(name)?;
//         Ok(self)
//     }

//     fn add_resource(&mut self, name: String, data: String) {
//         self.resources.insert(name, Resource::Marker(data));
//     }
// }

// impl WorldMethods<Text> for World {
//     fn with_component(&mut self, name: &str, data: Text) -> Result<&mut Self> {
//         self.entity_data.insert(name, data)?;
//         self.bitmap.insert(name)?;
//         Ok(self)
//     }

//     fn add_resource(&mut self, name: String, data: Text) {
//         self.resources.insert(name, Resource::GgezText(data));
//     }
// }

// #[cfg(test)]
// mod tests {

//     use super::{Component, Result, World, WorldMethods};
//     use crate::components::CastComponents;
//     use crate::data_types::point::Point;
//     use crate::resources::resource::ResourceCast;
//     use ggez::event::KeyCode;

//     #[test]
//     fn should_get_key_code_resource() -> Result<()> {
//         let mut world = World::new();
//         world.add_resource("keycode".to_owned(), KeyCode::A);
//         let wrapped_keycode = world.get_resource("keycode")?.borrow();
//         let keycode: &KeyCode = wrapped_keycode.cast()?;
//         assert_eq!(*keycode, KeyCode::A);
//         Ok(())
//     }

//     #[test]
//     fn should_get_key_code_component() -> Result<()> {
//         let mut world = World::new();
//         world.register("keycode", Component::GgezKeyCode)?;
//         world
//             .spawn_entity()?
//             .with_component("keycode", KeyCode::A)?;
//         let wrapped_keycodes = world.query_one("keycode").unwrap().borrow();
//         let keycodes: &Vec<KeyCode> = wrapped_keycodes.cast()?;
//         assert_eq!(keycodes[0], KeyCode::A);
//         Ok(())
//     }

//     #[test]
//     fn should_mutably_get_key_code_resource() -> Result<()> {
//         let mut world = World::new();
//         world.add_resource("keycode".to_owned(), KeyCode::A);
//         let mut wrapped_keycode = world.get_resource("keycode")?.borrow_mut();
//         let keycode: &mut KeyCode = wrapped_keycode.cast_mut()?;
//         *keycode = KeyCode::B;
//         let keycode: &KeyCode = wrapped_keycode.cast()?;
//         assert_eq!(*keycode, KeyCode::B);
//         Ok(())
//     }

//     #[test]
//     fn should_mutably_get_key_code_component() -> Result<()> {
//         let mut world = World::new();
//         world.register("keycode", Component::GgezKeyCode)?;
//         world
//             .spawn_entity()?
//             .with_component("keycode", KeyCode::A)?;
//         let mut wrapped_keycodes = world.query_one("keycode").unwrap().borrow_mut();
//         let keycodes: &mut Vec<KeyCode> = wrapped_keycodes.cast_mut()?;
//         keycodes[0] = KeyCode::B;
//         drop(wrapped_keycodes);
//         let wrapped_keycodes = world.query_one("keycode").unwrap().borrow();
//         let keycodes: &Vec<KeyCode> = wrapped_keycodes.cast()?;
//         assert_eq!(keycodes[0], KeyCode::B);
//         Ok(())
//     }

//     #[test]
//     fn should_get_marker_resource() -> Result<()> {
//         let mut world = World::new();
//         world.add_resource("marker".to_owned(), "player".to_owned());
//         let wrapped_marker = world.get_resource("marker")?.borrow();
//         let marker: &String = wrapped_marker.cast()?;
//         assert_eq!(*marker, "player".to_owned());
//         Ok(())
//     }

//     #[test]
//     fn should_get_marker_component() -> Result<()> {
//         let mut world = World::new();
//         world.register("marker", Component::Marker)?;
//         world
//             .spawn_entity()?
//             .with_component("marker", "player".to_owned())?;
//         let wrapped_markers = world.query_one("marker").unwrap().borrow();
//         let markers: &Vec<String> = wrapped_markers.cast()?;
//         assert_eq!(markers[0], "player".to_owned());
//         Ok(())
//     }

//     #[test]
//     fn should_mutably_get_marker_resource() -> Result<()> {
//         let mut world = World::new();
//         world.add_resource("marker".to_owned(), "player".to_owned());
//         let mut wrapped_marker = world.get_resource("marker")?.borrow_mut();
//         let marker: &mut String = wrapped_marker.cast_mut()?;
//         *marker = "asteroid".to_owned();
//         let marker: &String = wrapped_marker.cast()?;
//         assert_eq!(*marker, "asteroid".to_owned());
//         Ok(())
//     }

//     #[test]
//     fn should_mutably_get_marker_component() -> Result<()> {
//         let mut world = World::new();
//         world.register("marker", Component::Marker)?;
//         world
//             .spawn_entity()?
//             .with_component("marker", "player".to_owned())?;
//         let mut wrapped_markers = world.query_one("marker").unwrap().borrow_mut();
//         let markers: &mut Vec<String> = wrapped_markers.cast_mut()?;
//         markers[0] = "asteroid".to_owned();
//         drop(wrapped_markers);
//         let wrapped_markers = world.query_one("marker").unwrap().borrow();
//         let markers: &Vec<String> = wrapped_markers.cast()?;
//         assert_eq!(markers[0], "asteroid".to_owned());
//         Ok(())
//     }

//     // delete entity tests
//     #[test]
//     fn should_be_able_to_delete_an_entity_by_index() -> Result<()> {
//         let mut world = World::new();
//         world.register("location", Component::Point)?;
//         world.register("name", Component::Marker)?;
//         world
//             .spawn_entity()?
//             .with_component("location", Point::new(0.0, 0.0))?
//             .with_component("name", "Player".to_owned())?;
//         world
//             .spawn_entity()?
//             .with_component("location", Point::new(10.0, 10.0))?
//             .with_component("name", "asteroid".to_owned())?;
//         world.delete_entity_by_index(0)?;

//         world.update()?;

//         let wrapped_names = world.query_one("name")?.borrow();
//         let names: &Vec<String> = wrapped_names.cast()?;

//         assert_eq!(names.len(), 1);
//         assert_eq!(names[0], "asteroid");
//         Ok(())
//     }

//     #[test]
//     fn should_be_able_to_delete_all_entities() -> Result<()> {
//         let mut world = World::new();

//         world.register("name", Component::Marker)?;

//         world
//             .spawn_entity()?
//             .with_component("name", "first".to_owned())?;
//         world
//             .spawn_entity()?
//             .with_component("name", "second".to_owned())?;
//         world
//             .spawn_entity()?
//             .with_component("name", "third".to_owned())?;

//         world.delete_entity_by_index(0)?;
//         world.delete_entity_by_index(1)?;
//         world.delete_entity_by_index(2)?;

//         world.update()?;

//         let wrapped_names = world.query_one("name")?.borrow();
//         let names: &Vec<String> = wrapped_names.cast()?;

//         assert_eq!(names.len(), 0);

//         Ok(())
//     }
// }

// #[cfg(test)]
// mod testing_text {
//     use super::{Component, Result, World, WorldMethods};
//     use crate::components::CastComponents;
//     use crate::resources::resource::ResourceCast;
//     use ggez::graphics::Text;

//     #[test]
//     fn should_get_text_resource() -> Result<()> {
//         let mut world = World::new();
//         world.add_resource("text".to_owned(), Text::new("this is a text"));
//         let wrapper_text = world.get_resource("text")?.borrow();
//         let text: &Text = wrapper_text.cast()?;
//         assert_eq!(text.contents(), "this is a text");
//         Ok(())
//     }

//     #[test]
//     fn should_get_text_component() -> Result<()> {
//         let mut world = World::new();
//         world.register("text", Component::GgezText)?;
//         world
//             .spawn_entity()?
//             .with_component("text", Text::new("texting"))?;
//         let wrapper_texts = world.query_one("text").unwrap().borrow();
//         let texts: &Vec<Text> = wrapper_texts.cast()?;
//         assert_eq!(texts[0].contents(), "texting");
//         Ok(())
//     }

//     #[test]
//     fn should_mutably_get_text_resource() -> Result<()> {
//         let mut world = World::new();
//         world.add_resource("text".to_owned(), Text::new("tt"));
//         let mut wrapper_text = world.get_resource("text")?.borrow_mut();
//         let text: &mut Text = wrapper_text.cast_mut()?;
//         *text = Text::new("other text");
//         let text: &Text = wrapper_text.cast()?;
//         assert_eq!(text.contents(), "other text");
//         Ok(())
//     }

//     #[test]
//     fn should_mutably_get_text_component() -> Result<()> {
//         let mut world = World::new();
//         world.register("text", Component::GgezText)?;
//         world
//             .spawn_entity()?
//             .with_component("text", Text::new("meow"))?;
//         let mut wrapper_texts = world.query_one("text").unwrap().borrow_mut();
//         let texts: &mut Vec<Text> = wrapper_texts.cast_mut()?;
//         texts[0] = Text::new("bark");
//         drop(wrapper_texts);
//         let wrapper_texts = world.query_one("text").unwrap().borrow();
//         let texts: &Vec<Text> = wrapper_texts.cast()?;
//         assert_eq!(texts[0].contents(), "bark");
//         Ok(())
//     }
// }

// #[cfg(test)]
// mod creating_world {
//     use super::World;
//     use super::*;
//     use eyre::Result;

//     #[test]
//     fn should_be_able_to_create_a_world() -> Result<()> {
//         let world = World::new();

//         assert_eq!(world.next_entity_id, 0);
//         Ok(())
//     }

//     #[test]
//     fn should_increment_ids_when_inserting_entities() -> Result<()> {
//         let mut world = World::new();

//         world.register("rotation", Component::U32)?;

//         world.spawn_entity()?.with_component("rotation", 42_u32)?;

//         assert_eq!(world.next_entity_id, 1);
//         Ok(())
//     }
// }

// #[cfg(test)]
// mod keeping_bitmap_updated {
//     use super::*;

//     #[test]
//     fn deleting_an_entity_deletes_in_bitmap() {}
// }
