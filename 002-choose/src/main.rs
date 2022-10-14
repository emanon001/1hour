use anyhow::{anyhow, Result};
use rand::prelude::*;
use std::io::{self, BufRead, BufReader};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    #[structopt(help = "Number of rows to select", short, default_value = "1")]
    n: usize,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let n = opt.n;
    if n == 0 {
        return Err(anyhow!("illegal row count"));
    }

    let reader = BufReader::new(io::stdin());
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let mut rng = rand::thread_rng();
    let res = lines.choose_multiple(&mut rng, n);
    for l in res {
        println!("{}", l);
    }
    Ok(())
}
