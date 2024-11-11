use crate::data::Data;


pub struct Size{
    pub size_of_data: usize, 
}
impl Size{
    pub fn new(json_data: &Data)->Self
    {
        let size_tmp: usize = json_data.data.to_string().len();
        println!("The size of json file is: {}", size_tmp);
        Self{
            size_of_data: size_tmp,
        }
    }
}