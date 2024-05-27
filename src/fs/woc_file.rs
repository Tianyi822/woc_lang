use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub struct WocFile {
    path: String,
    name: Option<String>,
    size: Option<u64>,
    reader: Option<BufReader<File>>,
    current_line: usize,
}

impl WocFile {
    // Create a new WocFile object by only recording the path
    pub fn new(path: String) -> Self {
        Self {
            path,
            name: None,
            size: None,
            reader: None,
            current_line: 0,
        }
    }

    // Initialize the file reader and other fields if not already done
    fn initialize(&mut self) -> io::Result<()> {
        if self.reader.is_none() {
            let path = Path::new(&self.path);
            let file = File::open(path)?;
            self.size = Some(file.metadata()?.len());
            self.name = Some(path.file_name().unwrap().to_string_lossy().to_string());
            self.reader = Some(BufReader::new(file));
        }
        Ok(())
    }

    // Read a new line from the file and return the current line number and line content
    pub fn read_line(&mut self) -> io::Result<(usize, String)> {
        self.initialize()?;

        let reader = self.reader.as_mut().unwrap();
        let mut line = String::new();
        let bytes_read = reader.read_line(&mut line)?;
        if bytes_read == 0 {
            // End of file reached
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "End of file reached",
            ));
        }

        self.current_line += 1;
        Ok((self.current_line, line))
    }

    // Get the file path
    pub fn get_path(&self) -> &str {
        &self.path
    }

    // Get the file name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    // Get the file size
    pub fn get_size(&self) -> Option<u64> {
        self.size
    }
}
