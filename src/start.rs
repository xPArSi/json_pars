use reqwest;
use std::error::Error;

pub struct Start{
    pub text: String,
}
impl Start{
    pub async fn new()->Result<Self, Box<dyn Error>>{
        println!("Start of file reading");
        Ok(Self{
            text: reqwest::get("http://127.0.0.1:3030/get_json_file")
            .await?
            .text()
            .await?,
        })
        
    }
}