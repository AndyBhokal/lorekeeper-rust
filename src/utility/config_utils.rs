use std::fs;
use std::collections::HashMap;

pub fn load_config() -> HashMap<String, String> {
    let config_file = fs::read_to_string("Lorekeeper.config")
                                    .expect("No Config File found");
    let mut config_list: HashMap<String, String> = HashMap::new();
    for config_line in config_file.lines(){
        let (key,value) = config_line.split_once(":=").unwrap();
        config_list.insert(key.trim().to_string(),value.trim().to_string());
    }
    return config_list;
}


pub fn get_config_value(config_list : &HashMap<String, String>, value : &str) -> String{
    return config_list.get(value).map_or("", |s| s).to_string();
}