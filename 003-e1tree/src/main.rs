use anyhow::{anyhow, Result};
use std::fs::{self};
use std::path::Path;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    #[structopt(help = "base directory")]
    directory: String,
}

enum TreeItem<'a> {
    Root(&'a Path),
    DirectoryEntry {
        path: &'a Path,
        is_last_entry: bool,
        line_prefix: &'a String,
    },
}

fn print_tree(item: TreeItem) -> Result<()> {
    let (line_string, dir_context): (String, Option<(&Path, String)>) = match item {
        TreeItem::Root(path) => {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            (file_name.to_owned(), Some((path, "".to_owned())))
        }
        TreeItem::DirectoryEntry {
            path,
            is_last_entry,
            line_prefix,
        } => {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            let file_prefix = if is_last_entry {
                "└── "
            } else {
                "├── "
            };
            let line = format!("{}{}{}", line_prefix, file_prefix, file_name);
            let dir_context = if path.is_dir() {
                let new_line_prefix = format!(
                    "{}{}",
                    line_prefix,
                    if is_last_entry { "    " } else { "│   " }
                );
                Some((path, new_line_prefix))
            } else {
                None
            };
            (line, dir_context)
        }
    };

    // print path line
    println!("{}", line_string);

    // print directory entries
    if let Some((dir, line_prefix)) = dir_context {
        let mut entries = fs::read_dir(dir)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()?;
        entries.sort();
        for (i, path) in entries.iter().enumerate() {
            print_tree(TreeItem::DirectoryEntry {
                path: path.as_path(),
                is_last_entry: i == entries.len() - 1,
                line_prefix: &line_prefix,
            })?;
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let metadata = fs::metadata(&opt.directory)?;
    if !metadata.is_dir() {
        return Err(anyhow!("argument is not directory"));
    }
    let path = Path::new(&opt.directory);
    print_tree(TreeItem::Root(path))?;
    Ok(())
}
