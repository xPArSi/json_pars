use serde_json::Value;
use crate::data::Data;

pub struct Kind{
    pub kind_of_data: Vec<String>,
}
impl Kind{
    pub fn new(json_data: &Data)->Self{
        let mut kind_of_data_tmp: Vec<String> = Vec::new();
        if let Some(array) = json_data.data.as_array()
        {
            for item in array{
                if let Some(object) = item.as_object(){
                    
                    for (_key, value) in object{
                        match value {    
                            Value::Null => kind_of_data_tmp.push("null".to_string()),
                            Value::Bool(_) => kind_of_data_tmp.push("bool".to_string()),
                            Value::Number(_) => kind_of_data_tmp.push("number".to_string()),
                            Value::String(_) => kind_of_data_tmp.push("string".to_string()),
                            Value::Array(_) => kind_of_data_tmp.push("array".to_string()),
                            Value::Object(_) => kind_of_data_tmp.push("object".to_string()),
                        }
                    }
                }
            }
        }
        if let Some(object) = json_data.data.as_object()
        {     
            for (_key, value) in object{
                match value {    
                    Value::Null => kind_of_data_tmp.push("null".to_string()),
                    Value::Bool(_) => kind_of_data_tmp.push("bool".to_string()),
                    Value::Number(_) => kind_of_data_tmp.push("number".to_string()),
                    Value::String(_) => kind_of_data_tmp.push("string".to_string()),
                    Value::Array(_) => kind_of_data_tmp.push("array".to_string()),
                    Value::Object(_) => kind_of_data_tmp.push("object".to_string()),
                }
            }
        }
        println!("The types of data in json file is: ");
        for data_type in &kind_of_data_tmp{
            println!("{}", data_type);
        }
        Self { 
            kind_of_data: kind_of_data_tmp,
        }
    }
}
