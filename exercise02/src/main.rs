use crate::read_input::parse_input;

mod read_input;

fn main() {
    let content = parse_input().content_as_num_vec();
    let mut reports: Vec<Report> = Vec::new();
    for content in content {
        reports.push(Report::from(content));
    }

    let mut safe_count = 0;
    reports.iter_mut().for_each(|report| {
        report.check_safety();
        if report.is_safe.is_some_and(|x|x) {
            safe_count += 1;
        }
    });

    println!("Safe count: {}", safe_count);
}

struct Report {
    content: Vec<i32>,
    is_safe: Option<bool>
}

fn derivative(integral: &Vec<i32>) -> Vec<i32> {
    integral.windows(2).map(|w| w[1] - w[0]).collect()
}

impl Report {
    fn from(content: Vec<i32>) -> Report {
        Report{content, is_safe: None}
    }

    fn check_safety(&mut self) -> () {
        let diffs = derivative(&self.content);
        if diffs[0] > 0 {
            self.is_safe = Option::from(diffs.iter().fold(true, |mut acc, x| {
                if *x > 3 || *x < 1 {
                    acc = false;
                }
                acc
            }))
        } else if diffs[0] < 0  {
            self.is_safe = Option::from(diffs.iter().fold(true, |mut acc, x| {
                if *x < -3 || *x > -1 {
                    acc = false;
                }
                acc
            }))
        } else {
            self.is_safe = Option::from(false)
        }
    }
}
