// #![allow(dead_code)]
// #![allow(unused_imports)]
use std::path::{Path, PathBuf};

// TODO: pass error to upper floor?
fn gen_path<P: AsRef<str>>(p: P) -> PathBuf
where P: std::convert::AsRef<std::ffi::OsStr> {
    let path = Path::new(&p);
    if !path.exists() && std::fs::create_dir_all(path).is_err() {
        panic!("ERROR: Can not read db path!");
    }

    path.to_path_buf()
}

/// # Trait for database implementation.
///
/// We can implement multi type of db for stql;
pub trait StQLDB {
    fn set(&self, k: Vec<u8>, v: Vec<u8>) -> Vec<u8>;
    fn get(&self, k: Vec<u8>) -> Vec<u8>;
}


/// Sled database model
mod sled_db {
    use super::gen_path;
    use super::StQLDB;
    use sled::Db;

    /// Sled db implemention for StQL
    pub struct Sled(Db);
    
    impl Sled {
        pub fn new(path: &'static str) -> Sled {
            let p = gen_path(path);
            Sled(Db::start_default(p).unwrap())
        }
    }

    impl StQLDB for Sled {
        fn set(&self, k: Vec<u8>, v: Vec<u8>) -> Vec<u8> {
            let r = self.0.set(&k, v).unwrap().unwrap().to_vec();
            self.0.flush().unwrap();
            r
        }

        fn get(&self, k: Vec<u8>) -> Vec<u8> {
            let r = self.0.get(&k).unwrap().unwrap().to_vec();
            self.0.flush().unwrap();
            r
        }
    }
}

pub use sled_db::Sled as StSled;

// type DBModel = StSled;
// struct StQL(DBModel);
// 
// impl std::ops::Deref for StQL {
//     type Target = DBModel;
//     fn deref(&self) -> &Self::Target { &self.0 }
// }
