use itertools::Itertools;
use std::convert::TryFrom;
use std::fmt;
use structopt::StructOpt;

#[derive(Debug, Clone)]
struct DiceRollCondition {
    dice: u32,
    roll_count: u32,
}

impl TryFrom<&str> for DiceRollCondition {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // TODO: implement
        Ok(Self {
            roll_count: 2,
            dice: 6,
        })
    }
}

impl fmt::Display for DiceRollCondition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}d{}", self.roll_count, self.dice)
    }
}

impl DiceRollCondition {
    fn roll(&self) -> DiceRollResult {
        // TODO: implement
        DiceRollResult::new(vec![2, 5])
    }
}

#[derive(Debug, Clone)]
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
    diceroll: String,
}

fn main() -> Result<(), String> {
    let opt = Opt::from_args();
    let roll_condition = DiceRollCondition::try_from(opt.diceroll.as_ref())?;
    let roll_result = roll_condition.roll();
    println!("{} = {}", roll_condition, roll_result);
    Ok(())
}
