extern crate ltsv;

fn main(){
    let mut record = ltsv::Record::new();
    record.insert(String::from("date"), String::from("2019-01-01"));
    record.insert(String::from("user"), String::from("Alice"));
    record.insert(String::from("age"), String::from("12"));

    ltsv::append("sample.ltsv", record).unwrap();
}
