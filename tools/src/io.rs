mod helpers;

use helpers::read;
use itertools::Itertools;
use rand::{prelude::*, SeedableRng};
use rand_chacha::ChaCha20Rng;
use rand_distr::Normal;
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
    pub d: usize,
    pub c: Vec<i64>,
    pub s: Vec<Vec<i64>>,
}

impl Input {
    const CONTEST_TYPES: usize = 26;

    /// **(CUSTOMIZE IT!)** Generate Input
    pub fn gen(option: GenOption) -> Self {
        let mut rng = ChaCha20Rng::seed_from_u64(option.seed);

        // You shold generate u64 first and then convert it to usize because the size of usize is platform dependent.
        let d = 365;
        let c = (0..Self::CONTEST_TYPES)
            .map(|_| rng.gen_range(0..=100))
            .collect_vec();
        let dists = (0..Self::CONTEST_TYPES)
            .map(|_| {
                let mu = rng.gen_range(10000..=20000) as f64;
                let sigma = rng.gen_range(2000..=8000) as f64;
                Normal::new(mu, sigma).unwrap()
            })
            .collect_vec();

        let mut s = (0..d)
            .map(|_| {
                dists
                    .iter()
                    .map(|d| (d.sample(&mut rng).round() as i64).clamp(0, 100000))
                    .collect_vec()
            })
            .collect_vec();

        Self { d, c, s }
    }

    /// **(CUSTOMIZE IT!)** Parse Input from tokens
    pub(super) fn parse<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> anyhow::Result<Self> {
        let d = read(tokens.next(), 0, 365)?;
        let mut c = vec![];

        for _ in 0..Self::CONTEST_TYPES {
            c.push(read(tokens.next(), 0, 100)?);
        }

        let mut s = vec![];

        for _ in 0..d {
            let mut si = vec![];

            for _ in 0..Self::CONTEST_TYPES {
                si.push(read(tokens.next(), 0, 100000)?);
            }

            s.push(si);
        }

        Ok(Self { d, c, s })
    }
}

impl Display for Input {
    /// **(CUSTOMIZE IT!)** Format Input as string
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.d)?;
        writeln!(f, "{}", self.c.iter().join(" "))?;

        for si in &self.s {
            writeln!(f, "{}", si.iter().join(" "))?;
        }

        Ok(())
    }
}

/// **(CUSTOMIZE IT!)** Output for this problem
#[derive(Debug, Clone)]
pub struct Output {
    pub contest_types: Vec<usize>,
}

impl Output {
    /// **(CUSTOMIZE IT!)** Parse Output from tokens
    pub(super) fn parse<'a>(
        input: &Input,
        tokens: &mut Peekable<impl Iterator<Item = &'a str>>,
    ) -> anyhow::Result<Self> {
        let mut contest_types = vec![];

        for _ in 0..input.d {
            let t = read(tokens.next(), 1, Input::CONTEST_TYPES)?;
            contest_types.push(t - 1);
        }

        Ok(Self { contest_types })
    }

    /// **(CUSTOMIZE IT!)** Calculate score
    pub(super) fn calc_score(&self, input: &Input) -> anyhow::Result<i64> {
        let baseline_output = Output {
            contest_types: (0..input.d).map(|i| i % Input::CONTEST_TYPES).collect(),
        };

        let score =
            (self.calc_satisfaction(input) - baseline_output.calc_satisfaction(input) + 1).max(0);

        Ok(score)
    }

    fn calc_satisfaction(&self, input: &Input) -> i64 {
        let mut last = vec![0; Input::CONTEST_TYPES];
        let mut satisfaction = 0;

        for (d, &t) in self.contest_types.iter().enumerate() {
            last[t] = d + 1;
            satisfaction += input.s[d][t]
                - last
                    .iter()
                    .zip(input.c.iter())
                    .map(|(&l, &c)| c * (d + 1 - l) as i64)
                    .sum::<i64>();
        }

        satisfaction
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
    writeln!(stdin, "{}", input.d)?;
    writeln!(stdin, "{}", input.c.iter().join(" "))?;
    stdin.flush()?;

    // Interact with the child process
    let mut contest_types = vec![];

    for d in 0..input.d {
        writeln!(stdin, "{}", input.s[d].iter().join(" "))?;
        stdin.flush()?;

        let out = helpers::read_line(&mut stdout)?;
        let mut tokens = out.split_whitespace();

        let t = read(tokens.next(), 1, Input::CONTEST_TYPES)?;
        contest_types.push(t - 1);
    }

    // Calculate score
    let output = Output { contest_types };
    let score = output.calc_score(&input)?;

    process.wait()?;
    Ok(score)
}
