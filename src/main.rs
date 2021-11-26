use std::{
    fmt::{Debug, Error, Formatter},
    fs::{self},
};

mod tool;

struct FolderResult {
    file_number: i32,
    code_file_number: i32,
    comment_lines: i32,
    lines: i32,
    code_lines: i32,
}

impl Debug for FolderResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "1 - Files: {},\n2 - Code files: {},\n3 - Lines: {},\n4 - Code lines: {},\n4 - Comment lines: {}\n",
            self.file_number, self.code_file_number, self.lines, self.code_lines, self.comment_lines,
        )
    }
}

fn count_number_of_lines_and_folders(paths: &Vec<String>, result: &mut FolderResult) {
    for path in paths {
        let file = fs::read_to_string(path);
        if file.is_err() {
            continue;
        }

        let unwraped_file = file.unwrap();
        let lines = unwraped_file.lines();
        result.lines += lines.clone().count() as i32;

        if tool::is_file_a_coding_file(path) {
            result.code_file_number += 1;
            result.code_lines += tool::get_code_lines_number(&lines);
            result.comment_lines += tool::get_comment_lines_number(&lines);
        }
    }
}

fn main() {
    let pattern = std::env::args().nth(1);

    if pattern.is_none() || pattern == Some(String::from("-h")) {
        println!("{}", fs::read_to_string("help.txt").unwrap());
        return;
    }

    let mut result = FolderResult {
        file_number: 0,
        code_file_number: 0,
        comment_lines: 0,
        code_lines: 0,
        lines: 0,
    };

    let paths = tool::get_files_names(&pattern.unwrap());
    result.file_number = paths.len() as i32;
    count_number_of_lines_and_folders(&paths, &mut result);
    println!("{:?}", result);
}
