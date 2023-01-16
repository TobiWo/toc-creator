mod cli;

use cli::create_cli_app;
use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader, Read};
use std::error::Error;
use std::path::Path;

fn main() {
    let cli_matches = create_cli_app().get_matches();
    let file_path = cli_matches.value_of("file").unwrap();
    let file_path_with_toc = format!("{}{}.md", &file_path.replace(".md", "_"), "toc");

    // Open the file in read-only mode (ignoring errors).
    // let file = File::open(file_path);
    // let reader = BufReader::new(file);

    let full_markdown_file = read_file_to_vec(file_path);

    let toc: String = match full_markdown_file {
        Ok(content) => {
            for item in content.iter() {
                println!("{}", item)
            };
            String::from("TST")
        },
        Err(error) => panic!("Problem reading the file: {:?}", error),
    };

    // for (i, item) in full_markdown_file.iter().enumerate() {
    //     println!("{:?}", item);
    // }


    // let mut result: Vec<TocLine> =  reader.lines()
    //     .map(|line| line.unwrap())
    //     .filter(|line_string| line_string.starts_with("#") && line_string.matches("#").count() > 1)
    //     .enumerate()
    //     .map(|(i, line)| TocLine {toc_line: TocLine::new_toc_line(&line, i+1) , hierarchy: TocLine::get_hierarchy(&line), original: line})
    //     .collect();
    
    // let mut toc_lines: Vec<TocLine> =  reader.lines()
    //     .map(|line| line.unwrap())
    //     .filter(|line_string| line_string.starts_with("#") && line_string.matches("#").count() > 1)
    //     .map(|line| TocLine {hierarchy: TocLine::get_hierarchy(&line), toc_line: line})
    //     .collect();

    // let mut hierarchy_count = [0; 6];
    // let mut previous_hierarchy = 0;
    // for mut item in result {
    //     if previous_hierarchy == 0 || previous_hierarchy == item.hierarchy || previous_hierarchy > item.hierarchy {
    //         hierarchy_count[item.hierarchy-1] += 1;
    //     } else if previous_hierarchy < item.hierarchy {
    //         hierarchy_count[item.hierarchy-1] = 1;
    //     }
    //     previous_hierarchy = item.hierarchy;     
    //     item.create_toc_line(hierarchy_count[item.hierarchy-1]);
    //     println!("{}", item.toc_line);
    // }

    // let toc_lines = create_toc_lines(&file);

    // for item in toc_lines {
    //     println!("{}", item.toc_line);
    // }

    // println!("{:?}", result);
    // Read the file line by line using the lines() iterator from std::io::BufRead.
    // for line in reader.lines().filter(|line| line.unwrap().contains("#") ){
    //     let line = line.unwrap(); // Ignore errors.
    //     // Show the line and its number.
    //     println!("{}", line);
    // }
}

pub fn read_file_to_vec<R>(file_path: R) -> Result<Vec<String>, Box<dyn Error>>
where R: AsRef<Path>, {
    let file = File::open(file_path)?;
    let buf = BufReader::new(file);
    let file_content = buf.lines()
        .enumerate()
        .map(|(i, line)| line.expect(format!("{}: {}", "Could not parse line with number", i)))
        .collect();
    Ok(file_content)
}

pub fn create_toc(file_content: &String) -> Vec<TocLine> {
    let mut toc = read_raw_toc_lines(file_content); 
    let mut hierarchy_count = [0; 6];
    let mut previous_hierarchy = 0;
    for item in &mut toc {
        if previous_hierarchy == 0 || previous_hierarchy == item.hierarchy || previous_hierarchy > item.hierarchy {
            hierarchy_count[item.hierarchy-1] += 1;
        } else if previous_hierarchy < item.hierarchy {
            hierarchy_count[item.hierarchy-1] = 1;
        }
        previous_hierarchy = item.hierarchy;     
        item.create_toc_line(hierarchy_count[item.hierarchy-1]);
    };
    toc
}

fn read_raw_toc_lines(file_content: &String) -> Vec<TocLine> {
    file_content.lines()
        .filter(|line_string| line_string.starts_with("#") && line_string.matches("#").count() > 1)
        .map(|line| TocLine {hierarchy: TocLine::get_hierarchy(&line), toc_line: line.to_string()})
        .collect()
}

pub struct TocLine {
    hierarchy: usize,
    toc_line: String,
}

impl TocLine {
    pub fn create_toc_line(&mut self, index: usize) {
        let hierarchy_space = "    ";
        let filtered_string: String = self.toc_line.split("#")
            .filter(|item| item.contains(" "))
            .map(|item| item.trim().to_string())
            .collect();      
            self.toc_line = format!("{}. [{}](#{})", index, filtered_string, filtered_string.to_lowercase().replace(" ", "-"));
            if self.hierarchy > 1 {
                for _i in 1..self.hierarchy {
                    self.toc_line = format!("{}{}", hierarchy_space, self.toc_line);
                }
            };
        }

    pub fn get_hierarchy(raw_line: &str) -> usize {
        raw_line.matches("#").count() - 1 
    } 
}
