use rocksdb::{DB, Options, Error};

pub struct Storage {
    db: DB,
}

impl Storage {
    /// Initializes a new blockchain storage database
    pub fn new(path: &str) -> Result<Self, Error> {
        let mut options = Options::default();
        options.create_if_missing(true);
        let db = DB::open(&options, path)?;
        Ok(Self { db })
    }

    /// Stores a key-value pair
    pub fn store(&self, key: &str, value: &str) -> Result<(), Error> {
        self.db.put(key, value)?;
        Ok(())
    }

    /// Retrieves a value based on key
    pub fn retrieve(&self, key: &str) -> Result<Option<String>, Error> {
        match self.db.get(key)? {
            Some(value) => Ok(Some(String::from_utf8(value).unwrap())),
            None => Ok(None),
        }
    }
}
