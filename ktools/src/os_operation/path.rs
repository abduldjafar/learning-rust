use std::fs;

pub struct  FileDescription {
    file_name: String,
}

impl FileDescription {
    pub fn new() -> Self {
        return FileDescription{
            file_name: "".to_string()
        };
    }

    pub fn get_file_name(&self) -> &str{
        return self.file_name.as_str();
    }   

    pub fn list_files(&self,path: &str) -> Vec<FileDescription> {
        let paths = fs::read_dir(path).unwrap();
        let mut  files : Vec<Self>= Vec::new();

        for file in paths{
            let file_name = file.unwrap().file_name().into_string().unwrap();
            files.push(
                FileDescription { file_name: file_name }
                
            )
        }
        
        files
    }


}