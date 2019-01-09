extern crate ltsv;

fn main(){
    let mut data = ltsv::from_path("sample.ltsv").unwrap();
    let mut d = ltsv::from_path("sample2.ltsv").unwrap();
    data.append(&mut d);

    ltsv::save(data, "sample3.ltsv").unwrap();
}
