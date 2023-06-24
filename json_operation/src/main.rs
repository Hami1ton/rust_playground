use serde_json;
use std::fs::File;


fn main() {
    let data_json = File::open("asset/sample.json").unwrap();
    println!("{:?}", &data_json);
    
    // serde_json のValue型にデシリアライズ
    let data: serde_json::Value = serde_json::from_reader(&data_json).unwrap();

    println!("data = {:?}", data);
}
