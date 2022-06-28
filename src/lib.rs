pub mod neoconf {
    use std::collections::HashMap;

    const DEFAULT_SECTION: &str = "main";

    pub struct Neoconf {
        file_path: String,
        hash_map: HashMap<String, String>,
    }

    impl Neoconf {
        pub fn new(file_path: String) -> Self {
            Self { file_path, hash_map: HashMap::new() }
        }

        pub fn get(&self, section: Option<&str>, key: &str) -> Option<&String> {
            match section {
                Some(section_) => {
                    return self.hash_map.get(&format!("{section_}.{key}"))
                },
                None => {
                    return self.hash_map.get(&format!("{DEFAULT_SECTION}.{key}"))
                }
            }
        }

        pub fn set(&self, section: &str, key: &str, value: &str) {
            todo!();
        }

        pub fn remove(&mut self, section: Option<&str>, key: &str) {
            let section = match section {
                Some(value) => value,
                None => {
                    DEFAULT_SECTION
                }
            };


            let mut file_contents = self.get_file_contents();
            let file_contents_clone = &file_contents.clone();

            let mut current_section = DEFAULT_SECTION;
            let mut inside_section = false;

            for (index, line) in file_contents_clone.lines().enumerate() {
                if skip_comments_and_empty_lines(line) {
                    continue;
                }

                if line.starts_with("section") && line.ends_with("{") {
                    current_section = get_new_section(line);

                    inside_section = true;
                }

                if inside_section == true && line.starts_with("}") {
                    inside_section = false;
                    current_section = DEFAULT_SECTION;
                }

                if line.contains("=") {
                    let (mut pair_key, _) = line.split_once("=").expect("Corrupt config file!");
                    
                    pair_key = pair_key.trim();
                    
                    if current_section == section && pair_key == key {
                        let mut new_contents: Vec<&str> = file_contents.split("\n").collect();

                        new_contents.remove(index);

                        file_contents = new_contents.join("\n");

                    }
                }
            }

            self.hash_map.remove(&format!("{}.{}", section, key));
            std::fs::write(&self.file_path, file_contents).expect("Failed to write file");
        }

        pub fn load(&mut self) {
            let file_contents = self.get_file_contents();

            self.parse(file_contents);
        }

        /// read config file and return file contents
        fn get_file_contents(&self) -> String {
            if std::path::Path::new(&self.file_path).exists() == false {
                std::fs::write(&self.file_path, "").expect("Failed to write file");
            }

            let contents = std::fs::read_to_string(&self.file_path).expect("Failed to read file");

            return contents;
        }
        
        

        fn parse(&mut self, file_contents: String) {
            let mut current_section = DEFAULT_SECTION;
            let mut inside_section = false;

            for line in file_contents.lines() {
                if skip_comments_and_empty_lines(line) {
                    continue;
                }

                // section NAME {
                // }
                if line.starts_with("section") && line.ends_with("{") {
                    current_section = get_new_section(line);

                    inside_section = true;
                }

                if inside_section == true && line.starts_with("}") {
                    inside_section = false;
                    current_section = DEFAULT_SECTION;
                }

                if line.contains("=") {
                    let (mut key, mut value) = line.split_once("=").expect("Corrupt config file!");
                    
                    key = key.trim();
                    value = value.trim();
                    
                    self.hash_map.insert(format!("{}.{}", current_section, key), value.to_owned());
                }

            }
        }

    }

    // fn remove_first_and_last_chars(value: &str) -> &str {
    //     let mut chars = value.chars();
    //     chars.next();
    //     chars.next_back();
    //     chars.as_str()
    // }

    fn skip_comments_and_empty_lines(line: &str) -> bool {
        line.starts_with(";") || line.is_empty()
    }

    fn get_new_section(line: &str) -> &str {
        let new_line: Vec<&str> = line.split(" ").collect();

        // index 0: "section" 
        // index 1: section name
        // index 2: {
        let new_section = new_line[1];
        
        if new_section.is_empty() {
            return DEFAULT_SECTION
        } 

        return new_section
    }

}

#[cfg(test)]
mod test;
