extern crate ltsv;
use ltsv::*;

fn main() {
    match Ltsv::from_path("sample.ltsv") {
        Ok(map) => {
            println!("{:?}", map);
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
