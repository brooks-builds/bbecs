pub mod bitmap;
mod entity_data;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use entity_data::EntityData;
use eyre::Result;
use ggez::audio::SoundData;
use ggez::event::KeyCode;
use ggez::graphics::{Color, Mesh, Text};

use crate::components::{CastComponents, ComponentData};
use crate::data_types::point::Point;
use crate::resources::resource::Resource;
use crate::resources::resources_data::ResourcesData;

use self::bitmap::BitMap;
use self::entity_data::EntityDataTraits;

macro_rules! impl_world_trait {
    ($new_type:ty, $arm:ident) => {
        impl WorldMethods<$new_type> for World {
            fn with_component(&mut self, name: &str, data: $new_type) -> Result<&mut Self> {
                self.entity_data.insert(name, data)?;
                self.bitmap.insert(name)?;
                Ok(self)
            }

            fn add_resource(&mut self, name: String, data: $new_type) {
                self.resources.insert(name, Resource::$arm(data));
            }
        }
    };
}

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

impl_world_trait!(Color, Color);
impl_world_trait!(Mesh, Mesh);
impl_world_trait!(Point, Point);
impl_world_trait!(u32, U32);
impl_world_trait!(f32, F32);
impl_world_trait!(usize, Usize);
impl_world_trait!(bool, Bool);
impl_world_trait!(KeyCode, GgezKeyCode);
impl_world_trait!(String, Marker);
impl_world_trait!(Text, GgezText);
impl_world_trait!(SoundData, GgezSound);
