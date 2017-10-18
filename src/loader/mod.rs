use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

pub struct Loader {
    string_by_path: HashMap<String, String>,
}

impl Loader {
    pub fn new() -> Loader {
        Loader { string_by_path: HashMap::new() }
    }

    pub fn read_file(path: &str) -> Result<String, io::Error> {
        let mut file = File::open(path)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;
        Ok(contents)
    }

    pub fn load(&mut self, name: &str, path: &str) -> Option<&String> {
        let contents = Loader::read_file(path);
        match contents {
            Ok(s) => {
                self.string_by_path.insert(name.into(), s);
                self.string_by_path.get(name)
            }
            Err(_) => None,
        }
    }

    pub fn load_with_path(&mut self, path: &str) -> Option<&String> {
        self.load(path, path)
    }

    pub fn get(&self, name: &str) -> Option<&String> {
        self.string_by_path.get(name)
    }
}
