// readme_include_mod.rs

#[allow(unused_imports)]
use ansi_term::Colour::{Green, Red, Yellow};
use std::fs;
use unwrap::unwrap;

pub fn readme_include(include_str: &str) {
    let start_delimiter = "[comment]: # (lmake_lines_of_code start)";
    let end_delimiter = "[comment]: # (lmake_lines_of_code end)";
    let file_name = "README.md";

    if let Ok(readme_content) = fs::read_to_string(file_name) {
        let mut new_readme_content = String::with_capacity(readme_content.len());
        if let Some(mut pos_start) = readme_content.find(start_delimiter) {
            pos_start += start_delimiter.len();
            if let Some(pos_end) = readme_content.find(end_delimiter) {
                new_readme_content.push_str(&readme_content[..pos_start]);
                new_readme_content.push_str("\n");
                new_readme_content.push_str(include_str);
                new_readme_content.push_str("\n");
                new_readme_content.push_str(&readme_content[pos_end..]);
                println!("readme_include write file: {}", Green.paint(file_name));
                unwrap!(fs::write(file_name, new_readme_content));
            }
        }
    }
}
