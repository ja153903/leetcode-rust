#![allow(unused_must_use)]

use std::env;
use std::fs;
use std::io;
use std::io::Write;

use indoc::indoc;

#[derive(Debug)]
struct FilePath {
    pub mod_name: String,
    pub file_name: String,
    pub file_type: String,
}

impl FilePath {
    pub fn new(path: &str) -> Option<Self> {
        let path_components: Vec<&str> = path.split("/").collect();
        if path_components.len() < 2 {
            return None;
        }

        let file_components: Vec<&str> = path_components[1].split(".").collect();

        Some(FilePath {
            mod_name: path_components[0].to_string(),
            file_name: file_components[0].to_string(),
            file_type: file_components[1].to_string(),
        })
    }

    pub fn get_full_file_path(&self) -> String {
        format!(
            "../src/{}/{}.{}",
            self.mod_name, self.file_name, self.file_type
        )
    }

    pub fn get_mod_path(&self) -> String {
        format!("../src/{}/mod.rs", self.mod_name)
    }

    pub fn create_leetcode_stub(&self) -> Result<(), io::Error> {
        let full_file_path = self.get_full_file_path();

        fs::File::create(full_file_path.as_str())?;

        let stub = indoc! {"
            #![allow(dead_code)]

            struct Solution;

            impl Solution {}

            #[cfg(test)]
            mod tests {
                use super::Solution;

                #[test]
                pub fn test_should_pass_basic_case1() {}
            }
        "};

        fs::write(full_file_path.as_str(), stub).expect("unable to write to file");

        // Once the file has been created, we should also add this new file to the appropriate mod
        // folder

        let mod_path = self.get_mod_path();

        let mut mod_file = fs::OpenOptions::new().append(true).open(mod_path).unwrap();

        mod_file.write_all(format!("mod {};", self.file_name).as_bytes())?;

        Ok(())
    }
}

fn main() -> io::Result<()> {
    let path = env::args().nth(1).expect("no file path given");

    let file_path = FilePath::new(&path).expect("invalid file path given");

    file_path.create_leetcode_stub();

    Ok(())
}
