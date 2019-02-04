extern crate ltsv;
use ltsv::*;

fn main() {
    let mut data = Ltsv::new();

    let mut record = Record::new();
    record.insert(String::from("date"), String::from("2019-01-01"));
    record.insert(String::from("user"), String::from("Alice"));
    record.insert(String::from("age"), String::from("12"));
    data.push(record);

    let mut record = Record::new();
    record.insert(String::from("date"), String::from("2019-01-02"));
    record.insert(String::from("user"), String::from("Bob"));
    record.insert(String::from("age"), String::from("15"));
    data.push(record);

    save(data, "sample.ltsv").unwrap();

    // let mut record = Record::new();
    // record.insert(String::from("date"), String::from("2019-01-03"));
    // record.insert(String::from("user"), String::from("Cathy"));
    // record.insert(String::from("age"), String::from("16"));
    // data.push(record);
    //
    // let mut record = Record::new();
    // record.insert(String::from("date"), String::from("2019-01-04"));
    // record.insert(String::from("user"), String::from("Daisy"));
    // record.insert(String::from("age"), String::from("18"));
    // data.push(record);

    // save(data, "sample2.ltsv").unwrap();
}
