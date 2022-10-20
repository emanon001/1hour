use anyhow::Result;
use jq_finder::jq::execute_jq;
use std::io::{BufRead, Stdin};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    #[structopt(help = "jq path", long = "jq-path", default_value = "jq")]
    jq_path: String,
}

fn get_json_from_stdin(stdin: Stdin) -> Result<String> {
    let lines = stdin.lock().lines();
    Ok(lines.map(|l| l.unwrap()).collect::<Vec<_>>().join("\n"))
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    // get json from stdio
    let json = get_json_from_stdin(std::io::stdin())?;

    // TODO: setup UI

    // TODO: remove test code
    let output = execute_jq(&opt.jq_path, &json, ".")?;
    println!("{:?}", output);

    Ok(())
}
