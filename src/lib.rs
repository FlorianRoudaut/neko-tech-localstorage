mod proto_persisted;
mod local_file_repository;
#[cfg(test)]
mod tests;

pub use proto_persisted::ProtoPersisted;
pub use local_file_repository::LocalFileRepository;
