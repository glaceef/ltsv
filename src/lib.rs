use std::path::Path;
use std::io::Result;

pub type Ltsv = Vec<Record>;
pub type Record = std::collections::HashMap<String, String>;

/// Load ltsv data from path.
pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Vec<Record>> {
    use std::fs::File;
    use std::io::{BufReader, Read};
    use std::iter::FromIterator;

    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    let _ = reader.read_to_string(&mut buf)?;

    let data = buf.split("\n").map(|line|{
        let iter = line.trim().split("\t").map(|v|{
            let split: Vec<&str> = v.splitn(2, ':').collect();
            (split[0].to_owned(), split[1].to_owned())
        });
        Record::from_iter(iter)
    }).collect();

    Ok(data)
}

/// Save as a new ltsv file.
///
/// # Eaxmple
///
/// ```
/// let data = Ltsv::new();
///
/// save(data, "sample.ltsv").unwrap();
/// ```
pub fn save<P: AsRef<Path>>(data: Ltsv, path: P) -> Result<()> {
    use std::fs::File;
    use std::io::{BufWriter, Write};

    let records: Vec<String> = data.iter().map(|record|{
        let vec: Vec<String> = record.iter().map(|(k,v)|format!("{}:{}",k,v)).collect();
        vec.join("\t")
    }).collect();
    let data = records.join("\n");

    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    writer.write_all(&data.into_bytes())?;

    Ok(())
}
