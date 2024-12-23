use std::{env, fs};

pub struct FileContent {
    content: String,
}

impl FileContent {
    fn from(file_path: &String) -> FileContent {
        let file_content = fs::read_to_string(file_path).expect("Something went wrong reading the file");
        FileContent { content: file_content }
    }

    fn content_as_vec(&self) -> Vec<String> {
        self.content.split("\n").map( |line| line.to_string() ).collect()
    }

    pub fn content_as_num_vec(&self) -> Vec<Vec<i32>> {
        self.content_as_vec()
            .into_iter()
            .map(
                |line| line
                    .split_whitespace()
                    .map(
                        |number| number
                            .parse::<i32>()
                            .expect("Cannot be parsed as an i32"))
                    .collect())
            .collect()
    }
}

pub fn parse_input() -> FileContent {
    let args: Vec<String> = env::args().collect();
    let file_path = Option::from(&args[1]);
    FileContent::from(file_path.expect("No argument given"))
}
