use std::path::Path;

use std::io::Result;

use std::collections::HashMap;

pub struct Ltsv;
impl Ltsv {
    pub fn new() -> Map {
        Map{ map: HashMap::<String, String>::new() }
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Map> {
        use std::fs::File;
        use std::io::{BufReader, Read};
        use std::iter::FromIterator;

        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut buf = String::new();
        let _ = reader.read_to_string(&mut buf)?;

        let iter = buf.trim().split("\t").map(|v|{
            let split: Vec<&str> = v.splitn(2, ':').collect();
            (split[0].to_owned(), split[1].to_owned())
        });
        Ok( Map{ map: HashMap::from_iter(iter) } )
    }

    pub fn over_write<P: AsRef<Path>>(path: P, map: Map) -> Result<()> {
        let mut file = Self::from_path(path)?;
        file.append(map);
        Ok(())
    }
}

#[derive(Clone)]
pub struct Map {
    map: HashMap<String, String>,
}

impl Map {
    pub fn insert<S: Into<String>>(&mut self, tag: S, value: S) {
        self.map.insert(tag.into(), value.into());
    }

    pub fn get<'a, S: Into<&'a str>>(&self, key: S) -> Option<&String> {
        self.map.get(key.into())
    }

    pub fn append(&mut self, map: Self) {
        for (k, v) in map.map.into_iter() {
            self.map.insert(k, v);
        }
    }

    pub fn save<P: AsRef<Path>>(self, path: P) -> Result<()> {
        use std::fs::File;
        use std::io::{BufWriter, Write};

        let vec: Vec<String> = self.map.iter().map(|(k,v)|format!("{}:{}",k,v)).collect();
        let content = vec.join("\t");

        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(&content.into_bytes())?;

        Ok(())
    }
}

use std::fmt;
impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.map)
    }
}
