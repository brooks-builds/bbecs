pub mod bitmap;
mod entity_data;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use entity_data::EntityData;
use eyre::Result;
use ggez::event::KeyCode;
use ggez::graphics::{Color, Mesh, Text};

use crate::components::{CastComponents, ComponentData};
use crate::data_types::point::Point;
use crate::resources::resource::Resource;
use crate::resources::resources_data::ResourcesData;

use self::bitmap::BitMap;
use self::entity_data::EntityDataTraits;

const TO_BE_DELETED: &str = "to be deleted";
pub const ENTITY_ID: &str = "entity id";

pub type DataWrapper<T> = Rc<RefCell<T>>;

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

    pub fn query(&self, names: Vec<&str>) -> Result<HashMap<String, Vec<&ComponentData>>> {
        let bitmap_query = self.bitmap.query(names.clone())?;
        self.entity_data.query(bitmap_query)
    }

    pub fn get_resource<S: Into<String>>(&self, name: S) -> Result<&Rc<RefCell<Resource>>> {
        self.resources.get(&name.into())
    }

    pub fn update(&mut self) -> Result<()> {
        let query_results = self.query(vec![TO_BE_DELETED])?;
        let to_be_deleted_query = query_results.get(TO_BE_DELETED).unwrap();
        let mut bitmap_indexes_to_delete = vec![];

        to_be_deleted_query
            .iter()
            .enumerate()
            .for_each(|(index, to_be_deleted)| {
                let to_be_deleted: &Rc<RefCell<bool>> = to_be_deleted.cast().unwrap();
                let to_be_deleted = to_be_deleted.borrow();
                if *to_be_deleted {
                    bitmap_indexes_to_delete.push(index);
                }
            });

        let component_indexes_to_delete = self
            .bitmap
            .calculate_component_indexes_to_delete(&bitmap_indexes_to_delete)?;

        self.bitmap
            .delete_entities_by_index(bitmap_indexes_to_delete)?;

        self.entity_data
            .delete_entities_by_index(component_indexes_to_delete)?;
        Ok(())
    }

    pub fn delete_by_id(&self, id: u32) -> Result<()> {
        let query_results = self.query(vec![TO_BE_DELETED, ENTITY_ID])?;
        let query_to_be_deleted = query_results.get(TO_BE_DELETED).unwrap();
        let query_ids = query_results.get(ENTITY_ID).unwrap();

        for (index, component_id) in query_ids.iter().enumerate() {
            let wrapped_component_id: &Rc<RefCell<u32>> = component_id.cast()?;
            let component_id = wrapped_component_id.borrow();

            if *component_id == id {
                let wrapped_to_be_deleted: &Rc<RefCell<bool>> =
                    query_to_be_deleted[index].cast()?;
                let mut to_be_deleted = wrapped_to_be_deleted.borrow_mut();
                *to_be_deleted = true;
            }
        }
        Ok(())
    }
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

impl WorldMethods<Color> for World {
    fn with_component(&mut self, name: &str, data: Color) -> Result<&mut Self> {
        self.entity_data.insert(name, data)?;
        self.bitmap.insert(name)?;
        Ok(self)
    }

    fn add_resource(&mut self, name: String, data: Color) {
        self.resources.insert(name, Resource::Color(data));
    }
}

impl WorldMethods<Mesh> for World {
    fn with_component(&mut self, name: &str, data: Mesh) -> Result<&mut Self> {
        self.entity_data.insert(name, data)?;
        self.bitmap.insert(name)?;
        Ok(self)
    }

    fn add_resource(&mut self, name: String, data: Mesh) {
        self.resources.insert(name, Resource::Mesh(data));
    }
}

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

impl WorldMethods<f32> for World {
    fn with_component(&mut self, name: &str, data: f32) -> Result<&mut Self> {
        self.entity_data.insert(name, data)?;
        self.bitmap.insert(name)?;
        Ok(self)
    }

    fn add_resource(&mut self, name: String, data: f32) {
        self.resources.insert(name, Resource::F32(data));
    }
}

impl WorldMethods<usize> for World {
    fn with_component(&mut self, name: &str, data: usize) -> Result<&mut Self> {
        self.entity_data.insert(name, data)?;
        self.bitmap.insert(name)?;
        Ok(self)
    }

    fn add_resource(&mut self, name: String, data: usize) {
        self.resources.insert(name, Resource::Usize(data));
    }
}

impl WorldMethods<bool> for World {
    fn with_component(&mut self, name: &str, data: bool) -> Result<&mut Self> {
        self.entity_data.insert(name, data)?;
        self.bitmap.insert(name)?;
        Ok(self)
    }

    fn add_resource(&mut self, name: String, data: bool) {
        self.resources.insert(name, Resource::Bool(data));
    }
}

impl WorldMethods<KeyCode> for World {
    fn with_component(&mut self, name: &str, data: KeyCode) -> Result<&mut Self> {
        self.entity_data.insert(name, data)?;
        self.bitmap.insert(name)?;
        Ok(self)
    }

    fn add_resource(&mut self, name: String, data: KeyCode) {
        self.resources.insert(name, Resource::GgezKeyCode(data));
    }
}

impl WorldMethods<String> for World {
    fn with_component(&mut self, name: &str, data: String) -> Result<&mut Self> {
        self.entity_data.insert(name, data)?;
        self.bitmap.insert(name)?;
        Ok(self)
    }

    fn add_resource(&mut self, name: String, data: String) {
        self.resources.insert(name, Resource::Marker(data));
    }
}

impl WorldMethods<Text> for World {
    fn with_component(&mut self, name: &str, data: Text) -> Result<&mut Self> {
        self.entity_data.insert(name, data)?;
        self.bitmap.insert(name)?;
        Ok(self)
    }

    fn add_resource(&mut self, name: String, data: Text) {
        self.resources.insert(name, Resource::GgezText(data));
    }
}

impl WorldMethods<ggez::audio::SoundData> for World {
    fn with_component(&mut self, name: &str, data: ggez::audio::SoundData) -> Result<&mut Self> {
        self.entity_data.insert(name, data)?;
        self.bitmap.insert(name)?;
        Ok(self)
    }

    fn add_resource(&mut self, name: String, data: ggez::audio::SoundData) {
        self.resources.insert(name, Resource::GgezSound(data));
    }
}
