use std::path::Path;
use std::io::{Result as IoResult, Error, ErrorKind};

use std::collections::{HashMap, VecDeque};

pub type Ltsv = Vec<Record>;

#[derive(Debug)]
pub struct Record {
    map: HashMap<String, String>,
    order: VecDeque<String>,
}

impl Record {
    pub fn new() -> Self {
        Record{
            map: HashMap::new(),
            order: VecDeque::new(),
        }
    }
    pub fn insert(&mut self, label: String, value: String) -> Option<String> {
        self.order.push_back(label.clone());
        self.map.insert(label, value)
    }
}

struct LtsvError(String);

impl From<LtsvError> for IoResult<Ltsv> {
    fn from(e: LtsvError) -> Self {
        Result::Err(Error::new(
            ErrorKind::Other,
            e.0,
        ))
    }
}

/// Load ltsv data from path.
pub fn from_path<P: AsRef<Path>>(path: P) -> IoResult<Ltsv> {
    use std::fs::File;
    use std::io::{BufReader, Read};
    use std::iter::FromIterator;

    let mut reader = BufReader::new(File::open(path)?);
    let mut buf = String::new();
    let _ = reader.read_to_string(&mut buf)?;
    let content = buf.trim();

    if content.is_empty() {
        return Ok(vec![]);
    }

    let mut error = None;
    let data: Ltsv = buf.trim().split("\n").enumerate().map(|(l, line)|{
        let mut order = VecDeque::new();
        let iter = line.split("\t").filter_map(|f|{
            let split: Vec<&str> = f.splitn(2, ':').collect();
            if split.len() == 2 {
                let (label, value) = (split[0].to_owned(), split[1].to_owned());
                order.push_back(label.clone());
                return Some((label, value));
            } else if error.is_none() {
                error = Some(LtsvError(format!("Incorrect file format found at line`{}`", l)));
            }
            None
        });
        Record{
            map: HashMap::from_iter(iter),
            order
        }
    }).collect();

    if let Some(error) = error {
        error.into()
    } else {
        Ok(data)
    }
}

pub fn append<P: AsRef<Path>>(path: P, mut data: Record) -> IoResult<()> {
    use std::fs::OpenOptions;
    use std::io::{BufWriter, Write};

    let mut record_str = vec![];
    while let Some(label) = data.order.pop_front() {
        if let Some(value) = data.map.remove(&label) {
            record_str.push(format!("{}:{}", label, value))
        }
    }
    let record = record_str.join("\t") + "\n";

    let mut file = BufWriter::new(
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?
    );
    file.write(record.as_bytes())?;

    Ok(())
}

/// Save as a new ltsv file.
///
/// # Example
///
/// ```
/// let data = Ltsv::new();
/// let record = Record::new();
/// data.push(record);
///
/// save(data, "sample.ltsv").unwrap();
/// ```
pub fn save<P: AsRef<Path>>(path: P, data: Ltsv) -> IoResult<()> {
    use std::fs::OpenOptions;
    use std::io::{BufWriter, Write};

    let mut file = BufWriter::new(
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?
    );

    for mut d in data {
        let mut record_str = vec![];
        while let Some(label) = d.order.pop_front() {
            if let Some(value) = d.map.remove(&label) {
                record_str.push(format!("{}:{}", label, value))
            }
        }
        let record = record_str.join("\t") + "\n";
        file.write(record.as_bytes())?;
    }

    Ok(())
}
