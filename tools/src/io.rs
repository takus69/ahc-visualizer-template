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
    pub c: Vec<Vec<char>>,
}

impl Input {
    /// **(CUSTOMIZE IT!)** Generate Input
    pub fn gen(option: GenOption) -> Self {
        let mut rng = ChaCha20Rng::seed_from_u64(option.seed);

        // You shold generate u64 first and then convert it to usize because the size of usize is platform dependent.
        let n = rng.gen_range(20..=20u64) as usize;
        let mut c: Vec<Vec<char>> = Vec::new();
        for i in 0..n {
            let mut ci = Vec::new();
            for j in 0..n {
                ci.push('.');
            }
            c.push(ci);
        }

        // todo!("Write code to generate Input here.");

        Self { n, c }
    }

    /// **(CUSTOMIZE IT!)** Parse Input from tokens
    pub(super) fn parse<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> anyhow::Result<Self> {
        let n = read(tokens.next(), 20, 20)?;
        let mut c = Vec::new();
        for i in 0..n {
            let ci = tokens.next().unwrap().chars().collect::<Vec<char>>();
            c.push(ci);
        }

        // todo!("Write code to parse Input here.");

        Ok(Self { n, c })
    }
}

impl Display for Input {
    /// **(CUSTOMIZE IT!)** Format Input as string
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.n)?;
        for i in 0..self.n {
            writeln!(f, "{}", self.c[i].iter().map(|x| x.to_string()).collect::<Vec<String>>().join(""))?;
        }

        // todo!("Write code to format Input here.");

        Ok(())
    }
}

/// **(CUSTOMIZE IT!)** Output for this problem
#[derive(Debug, Clone)]
pub struct Output {
    pub d: char,
    pub p: usize,
}

impl Output {
    /// **(CUSTOMIZE IT!)** Parse Output from tokens
    pub(super) fn parse<'a>(
        input: &Input,
        tokens: &mut Peekable<impl Iterator<Item = &'a str>>,
    ) -> anyhow::Result<Self> {
        let d = tokens.next().unwrap().chars().collect::<Vec<char>>()[0];
        let p = read(tokens.next(), 0, input.n-1)?;

        // todo!("Write code to parse Output here.");

        Ok(Self { d, p })
    }

    /// **(CUSTOMIZE IT!)** Calculate score
    pub(super) fn calc_score(&self, input: &Input) -> anyhow::Result<i64> {
        let score = 100;

        // todo!("Write code to calculate score here.");

        Ok(score)
    }
}
