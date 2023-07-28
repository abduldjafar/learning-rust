use super::data_sources::DataSourceTypes;




pub struct Initialization {

}

impl Initialization {
    
    pub fn get(name : &str) -> DataSourceTypes {
        match name.as_ref() {
            "mongo" => DataSourceTypes::MongoSource(super::mongo_source::MongoSource  {} ),
            &_ =>{
                eprintln!("Error in DataSourceFactory processing get for name of '{}'", name);
                panic!("Error in DataSourceFactory processing get request")}
            ,
        }
    }
}