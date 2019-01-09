extern crate ltsv;
use ltsv::*;

fn main() {
    let mut data = Ltsv::new();

    let mut record = Record::new();
    record.insert(String::from("date"), String::from("2019-01-01"));
    record.insert(String::from("user"), String::from("Alice"));
    data.push(record);

    let mut record = Record::new();
    record.insert(String::from("date"), String::from("2019-01-02"));
    record.insert(String::from("user"), String::from("Bob"));
    data.push(record);

    save(data, "sample.ltsv").unwrap();
}
