mod start;
mod data;
mod size;
mod kind;
use kind::Kind;
use size::Size;
use data::Data;
use start::Start;
use std::error::Error;




#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //let cin: Start = Start::new().await?;
    let data_kind: Kind = Kind::new(&Data::new(&Start::new().await?));
    let data_size: Size = Size::new(&Data::new(&Start::new().await?));

    Ok(())
}