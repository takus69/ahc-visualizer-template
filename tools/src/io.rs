mod helpers;

use anyhow::ensure;
use helpers::read;
use itertools::Itertools;
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
    pub k: usize,
    pub a: Vec<usize>,
    pub p: Vec<(i64, i64)>,
}

impl Input {
    pub const CIRCLE_SIZE: i64 = 10000;
    pub const CIRCLE_SIZE2: i64 = Self::CIRCLE_SIZE * Self::CIRCLE_SIZE;

    /// **(CUSTOMIZE IT!)** Generate Input
    pub fn gen(option: GenOption) -> Self {
        let mut rng = ChaCha20Rng::seed_from_u64(option.seed);

        // You shold generate u64 first and then convert it to usize because the size of usize is platform dependent.
        let mut a = vec![];

        for _ in 1..=10 {
            a.push(rng.gen_range(1..=100u64) as usize);
        }

        let n = a.iter().enumerate().map(|(i, ai)| (i + 1) * ai).sum();

        let mut p = vec![];

        for _ in 0..n {
            let (x, y) = loop {
                let x = rng.gen_range(-Self::CIRCLE_SIZE..=Self::CIRCLE_SIZE);
                let y = rng.gen_range(-Self::CIRCLE_SIZE..=Self::CIRCLE_SIZE);

                if x * x + y * y >= Self::CIRCLE_SIZE2 {
                    continue;
                }

                let mut ok = true;

                for (xi, yi) in p.iter() {
                    let dx = x - xi;
                    let dy = y - yi;

                    if dx * dx + dy * dy <= 10 * 10 {
                        ok = false;
                        break;
                    }
                }

                if ok {
                    break (x, y);
                }
            };

            p.push((x, y));
        }

        let k = 100;

        Self { n, k, a, p }
    }

    /// **(CUSTOMIZE IT!)** Parse Input from tokens
    pub(super) fn parse<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> anyhow::Result<Self> {
        let n = read(tokens.next(), 0, 10000)?;
        let k = read(tokens.next(), 100, 100)?;
        let mut a = vec![];

        for _ in 0..10 {
            a.push(read(tokens.next(), 1, 100)?);
        }

        let mut p = vec![];

        for _ in 0..n {
            let x = read(tokens.next(), -Self::CIRCLE_SIZE, Self::CIRCLE_SIZE)?;
            let y = read(tokens.next(), -Self::CIRCLE_SIZE, Self::CIRCLE_SIZE)?;

            p.push((x, y));
        }

        let sum_a = a
            .iter()
            .enumerate()
            .map(|(i, ai)| (i + 1) * ai)
            .sum::<usize>();
        ensure!(sum_a == n, "sum_a = {} but n = {}", sum_a, n);

        Ok(Self { n, k, a, p })
    }
}

impl Display for Input {
    /// **(CUSTOMIZE IT!)** Format Input as string
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {}", self.n, self.k)?;

        let a = self.a.iter().map(usize::to_string).join(" ");
        writeln!(f, "{}", a)?;

        for (x, y) in self.p.iter() {
            writeln!(f, "{} {}", x, y)?;
        }

        Ok(())
    }
}

/// **(CUSTOMIZE IT!)** Output for this problem
#[derive(Debug, Clone)]
pub struct Output {
    pub k: usize,
    pub cut: Vec<(i64, i64, i64, i64)>,
}

impl Output {
    /// **(CUSTOMIZE IT!)** Parse Output from tokens
    pub(super) fn parse<'a>(
        input: &Input,
        tokens: &mut Peekable<impl Iterator<Item = &'a str>>,
    ) -> anyhow::Result<Self> {
        let k = read(tokens.next(), 0, input.k)?;

        let mut cut = vec![];

        const LIMIT: i64 = 100000000;

        for _ in 0..k {
            let x1 = read(tokens.next(), -LIMIT, LIMIT)?;
            let y1 = read(tokens.next(), -LIMIT, LIMIT)?;
            let x2 = read(tokens.next(), -LIMIT, LIMIT)?;
            let y2 = read(tokens.next(), -LIMIT, LIMIT)?;

            cut.push((x1, y1, x2, y2));
        }

        Ok(Self { k, cut })
    }

    /// **(CUSTOMIZE IT!)** Calculate score
    pub(super) fn calc_score(&self, input: &Input) -> anyhow::Result<i64> {
        let b = self.get_b(input);

        let mut ab_sum = 0;
        let mut a_sum = 0;

        for i in 0..10 {
            ab_sum += input.a[i].min(b[i]);
            a_sum += input.a[i];
        }

        let score = (1e6 * ab_sum as f64 / a_sum as f64).round() as i64;

        Ok(score)
    }

    pub fn get_b(&self, input: &Input) -> Vec<usize> {
        let groups = self.div_cakes(input);
        let mut b = vec![0; 10];

        for group in groups {
            if group.len() - 1 < b.len() {
                b[group.len() - 1] += 1;
            }
        }

        b
    }

    pub fn div_cakes(&self, input: &Input) -> Vec<Vec<(i64, i64)>> {
        let mut groups = vec![input.p.clone()];

        for (x1, y1, x2, y2) in self.cut.iter() {
            let mut new_group = vec![];

            let vx1 = x2 - x1;
            let vy1 = y2 - y1;

            for group in &groups {
                let mut g0 = vec![];
                let mut g1 = vec![];

                for &(x, y) in group {
                    let vx2 = x - x1;
                    let vy2 = y - y1;

                    let cross = vx1 * vy2 - vx2 * vy1;

                    if cross < 0 {
                        g0.push((x, y));
                    } else if cross > 0 {
                        g1.push((x, y));
                    }
                }

                if !g0.is_empty() {
                    new_group.push(g0);
                }

                if !g1.is_empty() {
                    new_group.push(g1);
                }
            }

            groups = new_group;
        }

        groups
    }
}
