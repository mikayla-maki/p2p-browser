use async_trait::async_trait;
use bytes::*;
use std::path::Path;
use std::collections::HashMap;


#[async_trait]
pub trait Datastore {
    async fn get(key: &Path) -> Option<Bytes>;
    async fn set(key: &Path, value: Bytes) -> bool;
}

// pub struct KeyedStore {
//     storage: Datastore;
// }

// impl KeyedStore {

//     fn init() {
        
//     }

//     fn get(key: &Path) -> Option<(Bytes, Signature)> {

//     }   
// }

#[derive(Debug)]
pub struct MemoryStore {
    map: HashMap<&Path, Bytes>,
}

// #[derive(Copy, Clone, Debug)]
// pub struct FileSystemDriver {
// }

// impl FileSystemDriver {
//     pub fn init() -> Self {
//         FileSystemDriver {}
//     }
// }

// #[async_trait]
// impl Datastore for FileSystemDriver {
//     async fn get(key: &Path) -> Option<Bytes> {
//         println!("GET CALL");

//         Option::from(Bytes::default())
//     }
    
//     async fn set(key: &Path, value: Bytes) -> bool {
//         true
//     }
// }