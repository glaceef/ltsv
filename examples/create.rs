extern crate ltsv;
use ltsv::*;

fn main() {
    let mut map = Ltsv::new();
    map.insert("tag-foo", "foo");
    map.insert("tag-bar", "bar");

    map.save("sample.ltsv").unwrap();
}
