use serde::de::DeserializeOwned;
use std::fs;

const RESOURCES_PATH: &str = "./assets/yaml/";

pub fn load_yaml_from_all_files<T>(path: impl AsRef<str>) -> Vec<T>
where T: DeserializeOwned {
    let mut vec: Vec<T> = Vec::new();
    let path = String::from(RESOURCES_PATH) + path.as_ref();
    let paths = fs::read_dir(path).unwrap();
    for path in paths {
        let path = path.expect(&format!("Unable to read file")).path();
        let s: String = fs::read_to_string(&path).expect(&format!("Unable to read {}", path.display()));
        let target: T = serde_yaml::from_str(&s).expect(&format!("Unable to parse {}", path.display()));
        vec.push(target);
    }
    vec
}

pub fn load_yaml_from_file<T>(path: impl AsRef<str>) -> T
where T: DeserializeOwned {
    let path = String::from(RESOURCES_PATH) + path.as_ref();
    let s = fs::read_to_string(&path).expect(&format!("Unable to read {}", path));
    let target: T = serde_yaml::from_str(&s).expect(&format!("Unable to parse {}", path));
    target
}


pub fn load_yaml_from_string<T>(str: impl AsRef<str>) -> T
where T: DeserializeOwned {
    let target: T = serde_yaml::from_str(str.as_ref()).expect(&format!("Unable to parse {}", str.as_ref()));
    target
}