mod helpers;

use helpers::read;
use rand::{prelude::*, SeedableRng};
use rand_chacha::ChaCha20Rng;
use std::{fmt::Display, iter::Peekable};

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

        // todo!("Write code to generate Input here.");

        Self { n }
    }

    /// **(CUSTOMIZE IT!)** Parse Input from tokens
    pub(super) fn parse<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> anyhow::Result<Self> {
        let n = read(tokens.next(), 10, 15)?;

        // todo!("Write code to parse Input here.");

        Ok(Self { n })
    }
}

impl Display for Input {
    /// **(CUSTOMIZE IT!)** Format Input as string
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.n)?;

        // todo!("Write code to format Input here.");

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
        lines: &mut Peekable<impl Iterator<Item = &'a str>>,
    ) -> anyhow::Result<(Option<Self>, Vec<String>)> {
        let mut comments: Vec<String> = Vec::new();
        for line in lines {
            if line.starts_with('#') {
                comments.push(line.to_string());
            } else {
                let mut tokens = line.split_whitespace().peekable();
                let k = read(tokens.next(), 0, 1000)?;

                return Ok((Some(Self { k }), comments));
            }
        }

        // todo!("Write code to parse Output here.");
        Ok((None, comments))
    }

    /// **(CUSTOMIZE IT!)** Calculate score
    pub(super) fn calc_score(&self, input: &Input) -> anyhow::Result<i64> {
        let score = 100;

        // todo!("Write code to calculate score here.");

        Ok(score)
    }
}

impl Display for Output {
    /// **(CUSTOMIZE IT!)** Format Input as string
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.k)?;

        // todo!("Write code to format Output here.");

        Ok(())
    }
}
