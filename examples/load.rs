extern crate ltsv;

fn main() {
    match ltsv::from_path("sample.ltsv") {
        Ok(data) => {
            println!("{:?}", data);
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
