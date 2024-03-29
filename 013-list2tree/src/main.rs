use core::fmt;
use regex::Regex;
use std::io::{self, BufRead, BufReader};

use clap::Parser;

/// output list style tree to `tree` command-like
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {}

#[derive(Debug)]
struct MultipleListStyleTree {
    tree_list: Vec<ListStyleTree>,
}

impl MultipleListStyleTree {
    pub fn new(lines: Vec<String>) -> Self {
        if lines.is_empty() {
            panic!("lines is empty");
        }
        let mut tree_list = Vec::new();
        let mut line_pos = 0;
        while line_pos < lines.len() {
            let tree = ListStyleTree::new_with_line_pos(&lines, line_pos);
            line_pos = tree.end_line_pos + 1;
            tree_list.push(tree);
        }
        Self { tree_list }
    }
}

impl fmt::Display for MultipleListStyleTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for tree in &self.tree_list {
            write!(f, "{}", tree)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct ListStyleTree {
    content: String,
    children: Vec<ListStyleTree>,
    _start_line_pos: usize,
    end_line_pos: usize,
}

impl ListStyleTree {
    pub fn new_with_line_pos(lines: &[String], line_pos: usize) -> Self {
        if lines.is_empty() {
            panic!("lines is empty");
        }
        Self::from_lines_rec(line_pos, lines)
    }

    fn from_lines_rec(line_pos: usize, lines: &[String]) -> Self {
        let line = lines[line_pos].clone();
        let content = Self::line_content(&line);
        let indent = Self::line_indent(&line);
        let mut next_line_pos = line_pos + 1;
        let mut children = Vec::new();
        while next_line_pos < lines.len() {
            let next_line_indent = Self::line_indent(&lines[next_line_pos]);
            if next_line_indent <= indent {
                break;
            }
            let child = Self::from_lines_rec(next_line_pos, lines);
            next_line_pos = child.end_line_pos + 1;
            children.push(child);
        }
        Self {
            content,
            children,
            _start_line_pos: line_pos,
            end_line_pos: next_line_pos - 1,
        }
    }

    fn output_list(&self) -> Vec<String> {
        let mut result = Vec::new();

        result.push(self.content.clone());

        if self.children.len() > 0 {
            for (i, child) in self.children.iter().enumerate() {
                let is_last_line = i == self.children.len() - 1;

                let lines = child.output_list();
                assert!(!lines.is_empty());
                let (child, descendants) = lines.split_at(1);
                let child = &child[0];

                // ├── <here>
                // │   ├── ...
                // or
                // └── <here>
                //     ├── ...
                let child_prefix = if is_last_line {
                    "└── "
                } else {
                    "├── "
                };
                result.push(format!("{}{}", child_prefix, child));

                // ├── child
                // │   ├── <here>
                // or
                // └── child
                //     ├── <here>
                let prefix = if is_last_line { "    " } else { "│   " };
                for e in descendants {
                    result.push(format!("{}{}", prefix, e));
                }
            }
        }
        result
    }

    fn line_indent(line: &str) -> usize {
        line.chars().take_while(|&ch| ch == ' ').count()
    }

    fn line_content(line: &str) -> String {
        let regex = Regex::new(r"^\s*(-\s*)?").expect("invalid regex");
        regex.replace(line, "").into()
    }
}

impl fmt::Display for ListStyleTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for l in self.output_list() {
            writeln!(f, "{}", l)?;
        }
        Ok(())
    }
}

fn main() {
    let _cli = Cli::parse(); // no args
    let reader = BufReader::new(io::stdin());
    let lines: Vec<String> = reader
        .lines()
        .map(|l| l.expect("failed to read line"))
        .filter(|l| !l.trim().is_empty())
        .collect();
    let tree = MultipleListStyleTree::new(lines);
    println!("{}", tree);
}
