use neko_tech_persistence::{Key, PersistenceError, Persisted, Repository};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{LocalFileRepository};

#[derive(Serialize, Deserialize)]
struct TestItem {
    key: Key,
    pub label: String,
}

impl Persisted for TestItem {
    fn key(&self) -> &Key { &self.key }
    fn persisted_type() -> &'static str { "TestItem" }
    fn to_proto_bytes(items: &[Self]) -> Vec<u8> {
        serde_json::to_vec(items).unwrap()
    }

    fn from_proto_bytes(bytes: &[u8]) -> Result<Vec<Self>, PersistenceError> {
        serde_json::from_slice(bytes)
            .map_err(|e| PersistenceError::StorageError(e.to_string()))
    }
}

#[test]
fn round_trip() {
    let dir = tempfile::tempdir().unwrap();
    let id = Uuid::new_v4();

    let item = TestItem {
        key: Key { id, version: 0, unique_name: "item-one".to_string() },
        label: "hello world".to_string(),
    };

    let mut repo = LocalFileRepository::<TestItem>::load(dir.path()).unwrap();
    repo.create(item).unwrap();

    let repo2 = LocalFileRepository::<TestItem>::load(dir.path()).unwrap();
    let loaded = repo2.read_one_by_id(&Key { id, version: 0, unique_name: String::new() }).unwrap();

    assert_eq!(loaded.label, "hello world");
    assert_eq!(loaded.key.unique_name, "item-one");
    assert_eq!(loaded.key.id, id);
}
