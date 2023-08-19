use std::io::{self, BufRead, BufReader};

use clap::Parser;

/// hyphen list to tree
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {}

fn main() {
    let _cli = Cli::parse(); // no args
    let reader = BufReader::new(io::stdin());
    let lines: Vec<String> = reader
        .lines()
        .map(|l| l.expect("failed to read line"))
        .collect();
    // TODO: ツリー構造に変換
    // TODO: ツリーを出力
    for l in lines {
        println!("{}", l);
    }
}
