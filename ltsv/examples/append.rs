extern crate ltsv;
use ltsv::*;

fn main(){
    let mut data = Ltsv::from_path("sample.ltsv").unwrap();
    let d = Ltsv::from_path("sample2.ltsv").unwrap();
    data.append(d);

    data.save("sample3.ltsv");
}
