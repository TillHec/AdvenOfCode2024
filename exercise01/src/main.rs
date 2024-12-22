use std::fs;

fn main() {
    let content = FileContent::from("input.txt");
    let mut locations = Locations::from(content.content_as_vec());
    locations.sort();
    let result = locations.distance().iter().fold(0, |acc, x| acc + x);
    print!("The answer is {}\n", result);
}

struct FileContent {
    content: String,
}

impl FileContent {
    fn from(file_path: &str) -> FileContent {
        let file_content = fs::read_to_string(&file_path).expect("Something went wrong reading the file");
        FileContent { content: file_content }
    }

    fn content_as_vec(&self) -> Vec<String> {
        self.content.split("\n").map( |line| line.to_string() ).collect()
    }
}


struct Locations {
    left_side: Vec<i32>,
    right_side: Vec<i32>,
}

fn split_string_to_integers(string: &String, index: usize) -> i32 {
    let vector: Vec<i32> = string.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
    vector[index]
}

impl Locations {
    fn from(file_content: Vec<String>) -> Locations {
        let left_side: Vec<i32> = file_content.iter().map(|line| split_string_to_integers(line, 0)).collect();
        let right_side: Vec<i32> = file_content.iter().map(|line| split_string_to_integers(line, 1)).collect();
        Locations {left_side, right_side}
    }

    fn sort(&mut self) {
        self.left_side.sort_unstable();
        self.right_side.sort_unstable();
    }

    fn distance(&self) -> Vec<u32> {
        self.left_side
            .iter()
            .zip(self.right_side.iter())
            .map(|(left_side, right_side)| (left_side - right_side).abs() as u32)
            .collect()
    }
}
