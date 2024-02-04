mod helpers;

use helpers::read;
use rand::{prelude::*, SeedableRng};
use rand_chacha::ChaCha20Rng;
use std::{
    fmt::Display,
    io::{BufReader, BufWriter, Write as _},
    iter::Peekable,
    process::{ChildStdin, ChildStdout},
};

/// **(CUSTOMIZE IT!)** Option for generating Input
#[derive(Debug, Clone, Copy)]
pub struct GenOption {
    pub seed: u64,
}

/// **(CUSTOMIZE IT!)** Input for this problem
#[derive(Debug, Clone)]
pub struct Input {
    pub n: usize,
}

impl Input {
    /// **(CUSTOMIZE IT!)** Generate Input
    pub fn gen(option: GenOption) -> Self {
        let mut rng = ChaCha20Rng::seed_from_u64(option.seed);

        // You shold generate u64 first and then convert it to usize because the size of usize is platform dependent.
        let n = rng.gen_range(10..=15u64) as usize;

        todo!("Write code to generate Input here.");

        Self { n }
    }

    /// **(CUSTOMIZE IT!)** Parse Input from tokens
    pub(super) fn parse<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> anyhow::Result<Self> {
        let n = read(tokens.next(), 0, 10)?;

        todo!("Write code to parse Input here.");

        Ok(Self { n })
    }
}

impl Display for Input {
    /// **(CUSTOMIZE IT!)** Format Input as string
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.n)?;

        todo!("Write code to format Input here.");

        Ok(())
    }
}

/// **(CUSTOMIZE IT!)** Output for this problem
#[derive(Debug, Clone)]
pub struct Output {
    pub k: usize,
}

impl Output {
    /// **(CUSTOMIZE IT!)** Parse Output from tokens
    pub(super) fn parse<'a>(
        input: &Input,
        tokens: &mut Peekable<impl Iterator<Item = &'a str>>,
    ) -> anyhow::Result<Self> {
        let k = read(tokens.next(), 0, 1000)?;

        todo!("Write code to parse Output here.");

        Ok(Self { k })
    }

    /// **(CUSTOMIZE IT!)** Calculate score
    pub(super) fn calc_score(&self, input: &Input) -> anyhow::Result<i64> {
        let score = 100;

        todo!("Write code to calculate score here.");

        Ok(score)
    }
}

/// **(CUSTOMIZE IT!)** Interactive judge code
pub(super) fn interact(
    process: &mut std::process::Child,
    mut stdin: BufWriter<ChildStdin>,
    mut stdout: BufReader<ChildStdout>,
    input: Input,
) -> Result<i64, anyhow::Error> {
    // Write input to stdin (without secret members)
    todo!("Write code to write input to stdin here.");
    write!(stdin, "{}", input)?;

    stdin.flush()?;

    // Interact with the child process
    let query_count = 100;

    for q in 0..query_count {
        let out = helpers::read_line(&mut stdout)?;
        let mut tokens = out.split_whitespace();

        todo!("Write code to respond to query here.");

        writeln!(stdin, "This is the response to query {}", q)?;
        stdin.flush()?;
    }

    // Calculate score
    let output: Output = todo!("Write code to generate Output here.");
    let score = output.calc_score(&input)?;

    process.wait()?;
    Ok(score)
}
