use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::io;
// use std::io::{Read, Write};
use std::env;
// use std::thread;
// use std::sync::mpsc;
// use std::sync::{Arc, RwLock, RwLockWriteGuard};
use std::result;

extern crate byteorder;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    ByteOrderError(byteorder::Error),
    PoisonError,
    ChunkReadError,
    InvalidEntryPrefixError,
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

impl From<byteorder::Error> for Error {
    fn from(err: byteorder::Error) -> Error {
        Error::ByteOrderError(err)
    }
}

pub type Result<T> = result::Result<T, Error>;

type Entries = HashMap<Vec<u8>, Vec<u8>>;

pub struct Database {
    collection: Entries,
    dbPath: PathBuf,
}

impl Database {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        Self::ensure_path(path).and_then(Self::create)
    }

    pub fn put<S: Into<Vec<u8>>, T: Into<Vec<u8>>>(&mut self, key: S, value: T) -> Result<()> {
        let k = key.into();
        let v = value.into();

        Self::memory_put(&mut self.collection, k, v);

        Ok(())
    }

    pub fn get<S: Into<Vec<u8>>>(&self, key: S) -> Option<Vec<u8>> {
        return self.collection.get(&key.into()).map(|val| val.clone());
    }

    fn ensure_path<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
        let mut buf = try!(env::current_dir());
        buf = buf.join(path);
        try!(fs::create_dir_all(buf.as_path()));
        Ok(buf)
    }

    fn create(path: PathBuf) -> Result<Self> {
        assert!(fs::metadata(path.as_path()).unwrap().is_dir());

        let mut db = Database {
            collection: HashMap::new(),
            dbPath: path,
        };
        Ok(db)
    }

    fn memory_put(collection: &mut Entries, k: Vec<u8>, v: Vec<u8>) {
        collection.insert(k, v);
    }
}

fn main() {
    let db = &mut Database::open("testdb").unwrap();

    db.put("molim", "lijepo");

    let res = db.get("molim");

    let res2 = String::from_utf8(res.unwrap());

    println!("{:?}", res2);
}
