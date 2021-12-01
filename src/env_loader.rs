#![allow(dead_code)]
pub struct EnvLoader;
impl EnvLoader {
    pub fn new<StringLike: AsRef<str>>(path: StringLike) -> Result<std::collections::HashMap<String, String>, String> {
        let _path = path.as_ref();
        let contents = std::fs::read_to_string(_path)
        .expect(
            format!("Failed to read env file! Please check if env file exists.\n--> {}\n", _path).as_str()
        ); 
        
        let splitted = contents
        .split("\r\n")
        .filter(|x| !x.is_empty() && !x.starts_with("#"))
        .map(|x| x
            .split("=")
            .map(|x| x.trim_start().trim_end().to_string())
            .collect::<Vec<String>>()
        )
        .collect::<Vec<Vec<String>>>();
        // Structure 2D: [["key", "value"], ["key2", "value2"]]

        let mut vars = std::collections::HashMap::new();
        for i in splitted {
            vars.insert(i[0].to_string(), i[1].to_string());
        }

        Ok(vars)
    }
}

fn trim_str(string: String) -> String {
    string.chars().filter(|c| !c.is_whitespace()).collect()
} 