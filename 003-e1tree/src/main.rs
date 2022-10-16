use anyhow::{anyhow, Result};
use std::fs::{self};
use std::path::Path;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    #[structopt(help = "base directory")]
    directory: String,
}

fn build_tree_lines(path: &Path) -> Result<Vec<String>> {
    let mut result = Vec::new();

    let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
    result.push(file_name);

    if path.is_dir() {
        let mut entries = fs::read_dir(path)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()?;
        entries.sort();
        for (i, path) in entries.iter().enumerate() {
            let is_last_entry = i == entries.len() - 1;

            let lines = build_tree_lines(path)?;
            assert!(!lines.is_empty());
            let (child, descendants) = lines.split_at(1);

            // child
            let prefix = if is_last_entry {
                "└── "
            } else {
                "├── "
            };
            result.push(format!("{}{}", prefix, child[0]));

            // descendants (excluded child)
            let prefix = if is_last_entry { "    " } else { "│   " };
            for e in descendants {
                result.push(format!("{}{}", prefix, e));
            }
        }
    }
    Ok(result)
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let dir = Path::new(&opt.directory);
    if !dir.is_dir() {
        return Err(anyhow!("argument is not directory"));
    }
    let lines = build_tree_lines(dir)?;
    for l in lines {
        println!("{}", l);
    }
    Ok(())
}
