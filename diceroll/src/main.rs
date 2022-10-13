use anyhow::{anyhow, Error, Result};
use itertools::Itertools;
use rand::prelude::*;
use regex::Regex;
use std::convert::TryFrom;
use std::fmt;
use structopt::StructOpt;

#[derive(Debug, Clone, PartialEq, Eq)]
struct DiceRoll {
    roll_count: u32,
    dice: u32,
}

impl TryFrom<&str> for DiceRoll {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let re = Regex::new(r"\A(\d+)[dD](\d+)\z").unwrap();
        let captures = re.captures(value).ok_or(anyhow!(
            "invalid format: expect '<roll_count>d<dice>' (ex: '26d')"
        ))?;

        // roll_count
        let roll_count = captures.get(1).map(|m| m.as_str()).unwrap();
        let roll_count: u32 = roll_count.parse()?;
        if !(1..=10).contains(&roll_count) {
            return Err(anyhow!("invalid roll count: expect between 1 and 10"));
        }

        // dice
        let dice = captures.get(2).map(|m| m.as_str()).unwrap();
        let dice: u32 = dice.parse()?;
        if !(1..=100).contains(&dice) {
            return Err(anyhow!("invalid dice: expect between 1 and 100"));
        }

        Ok(Self { roll_count, dice })
    }
}

impl fmt::Display for DiceRoll {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}d{}", self.roll_count, self.dice)
    }
}

impl DiceRoll {
    fn roll(&self) -> DiceRollResult {
        let mut rng = rand::thread_rng();
        let result = (0..self.roll_count)
            .map(|_| rng.gen_range(1..=self.dice))
            .collect();
        DiceRollResult::new(result)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DiceRollResult {
    result: Vec<u32>,
}

impl DiceRollResult {
    fn new(result: Vec<u32>) -> Self {
        Self { result }
    }

    fn sum(&self) -> u32 {
        self.result.iter().sum()
    }
}

impl fmt::Display for DiceRollResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.result.iter().join(" "), self.sum())
    }
}

#[derive(StructOpt)]
struct Opt {
    #[structopt(help = "2d6")]
    dice_roll: String,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let dice_roll = DiceRoll::try_from(opt.dice_roll.as_ref())?;
    let roll_result = dice_roll.roll();
    println!("{} = {}", dice_roll, roll_result);
    Ok(())
}
