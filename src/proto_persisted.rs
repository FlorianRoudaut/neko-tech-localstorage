use neko_tech_persistence::{Persisted, PersistenceError};

pub trait ProtoPersisted: Persisted + Sized {
    fn collection_filename() -> &'static str;
    fn to_proto_bytes(items: &[Self]) -> Vec<u8>;
    fn from_proto_bytes(bytes: &[u8]) -> Result<Vec<Self>, PersistenceError>;
}
