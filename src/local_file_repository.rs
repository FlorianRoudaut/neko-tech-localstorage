use std::fs;
use std::path::{Path, PathBuf};
use neko_tech_persistence::{Key, PersistenceError, Repository};
use crate::ProtoPersisted;

pub struct LocalFileRepository<T: ProtoPersisted> {
    items: Vec<T>,
    file_path: PathBuf,
}

impl<T: ProtoPersisted> LocalFileRepository<T> {
    pub fn load(dir: &Path) -> Result<Self, PersistenceError> {
        let file_path = dir.join(T::collection_filename());
        let items = if file_path.exists() {
            let bytes = fs::read(&file_path)
                .map_err(|e| PersistenceError::StorageError(e.to_string()))?;
            T::from_proto_bytes(&bytes)?
        } else {
            Vec::new()
        };
        Ok(Self { items, file_path })
    }

    fn flush(&self) -> Result<(), PersistenceError> {
        let bytes = T::to_proto_bytes(&self.items);
        fs::write(&self.file_path, bytes)
            .map_err(|e| PersistenceError::StorageError(e.to_string()))
    }
}

impl<T: ProtoPersisted> Repository<T> for LocalFileRepository<T> {
    fn create(&mut self, item: T) -> Result<(), PersistenceError> {
        let id = item.key().id;
        if self.items.iter().any(|i| i.key().id == id) {
            return Err(PersistenceError::AlreadyExists);
        }
        self.items.push(item);
        self.flush()
    }

    fn read_one_by_id(&self, key: &Key) -> Result<&T, PersistenceError> {
        self.items.iter()
            .find(|i| i.key().id == key.id)
            .ok_or(PersistenceError::NotFound)
    }

    fn read_one_by_name(&self, unique_name: &str) -> Result<&T, PersistenceError> {
        self.items.iter()
            .find(|i| i.key().unique_name == unique_name)
            .ok_or(PersistenceError::NotFound)
    }

    fn read_all(&self) -> Vec<&T> {
        self.items.iter().collect()
    }

    fn update(&mut self, item: T) -> Result<(), PersistenceError> {
        let id = item.key().id;
        let slot = self.items.iter_mut()
            .find(|i| i.key().id == id)
            .ok_or(PersistenceError::NotFound)?;
        *slot = item;
        self.flush()
    }

    fn delete(&mut self, key: &Key) -> Result<(), PersistenceError> {
        let pos = self.items.iter()
            .position(|i| i.key().id == key.id)
            .ok_or(PersistenceError::NotFound)?;
        self.items.swap_remove(pos);
        self.flush()
    }
}
