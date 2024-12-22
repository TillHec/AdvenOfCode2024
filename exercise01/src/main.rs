use std::collections::HashMap;
use std::fs;

fn main() {
    let content = FileContent::from("input.txt");
    let mut locations = Locations::from(content.content_as_vec());
    locations.sort();
    let result1 = locations.distance().iter().fold(0, |acc, x| acc + x);
    print!("The answer to exercise 1 is {}\n", result1);
    locations.set_occurrences_left_and_initialize_right();
    locations.set_occurrences_right();

    println!("The answer to exercise 2 is {}\n", locations.get_similarity_score());
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
    occurrences_left: HashMap<i32, usize>,
    occurrences_right: HashMap<i32, usize>,
}

fn split_string_to_integers(string: &String, index: usize) -> i32 {
    let vector: Vec<i32> = string.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
    vector[index]
}

impl Locations {
    fn from(file_content: Vec<String>) -> Locations {
        let left_side: Vec<i32> = file_content.iter().map(|line| split_string_to_integers(line, 0)).collect();
        let right_side: Vec<i32> = file_content.iter().map(|line| split_string_to_integers(line, 1)).collect();
        Locations {left_side, right_side, occurrences_left: HashMap::new(), occurrences_right: HashMap::new()}
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

    fn set_occurrences_left_and_initialize_right(&mut self) {
        for number in self.left_side.iter() {
            self.occurrences_right.entry(*number).or_insert(0);

            let count = self.occurrences_left.entry(*number).or_insert(0);
            *count += 1;
        }
    }

    fn set_occurrences_right(&mut self) {
        for number in self.right_side.iter() {
            let count = self.occurrences_right.entry(*number).or_insert(0);
            *count += 1;
        }
    }

    fn get_similarity_score(&self) -> usize {
        let mut score = 0;
        self.occurrences_left
            .keys()
            .for_each(
                |key| {
                    score += key.abs() as usize * self.occurrences_left.get(key).unwrap() * self.occurrences_right.get(key).unwrap()
                }
            );
        score
    }
}
