use crate::start::Start;
use serde_json::Value;
pub struct Data{
    pub data: Value,
}
impl Data{
    pub fn new(json_file: &Start)->Self
    {
        let data_tmp: Value = serde_json::from_str(&json_file.text).unwrap();
        println!("The data in json file is: {}", data_tmp);
        Self { data: data_tmp }
        
    }
}