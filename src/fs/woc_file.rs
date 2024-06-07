use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct WocFile {
    path: String,
    name: Option<String>,
    // The mapping between file line numbers and code lines
    line_num_and_code_map: HashMap<usize, String>,
    current_line: usize,
}

impl WocFile {
    // Create a new WocFile object by only recording the path
    pub fn new(path: String) -> Self {
        let mut wf = Self {
            path,
            name: None,
            line_num_and_code_map: HashMap::new(),
            current_line: 1,
        };

        // Get file object
        let path = Path::new(&wf.path);
        wf.name = Some(path.file_name().unwrap().to_string_lossy().to_string());
        let file = match File::open(path) {
            Ok(f) => f,
            Err(_) => panic!("Failed to open file"),
        };

        // Get file reader
        let mut reader = BufReader::new(file);

        // Read file line by line and store the line number and code line
        let mut line = String::new();
        let mut line_number = 0;
        loop {
            line.clear();
            match reader.read_line(&mut line) {
                Ok(0) => {
                    break;
                }
                Ok(_) => {
                    line_number += 1;
                    wf.line_num_and_code_map
                        .insert(line_number, line.trim().to_string());
                }
                Err(_) => {
                    break;
                }
            }
        }

        wf
    }

    // Read a new line from the file and return the current line number and line content
    pub fn read_line(&mut self) -> Option<(usize, String)> {
        // It's means the file is empty
        if self.line_num_and_code_map.len() <= 0
            || self.current_line > self.line_num_and_code_map.len()
        {
            return None;
        }

        // Get the current line content
        let line = self
            .line_num_and_code_map
            .get(&self.current_line)
            .unwrap()
            .clone();

        // Move the current line number to the next line
        let line_num = self.current_line;
        self.current_line += 1;

        Some((line_num, line))
    }

    // Get the file path
    pub fn get_path(&self) -> &str {
        &self.path
    }

    // Get the file name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }
}
