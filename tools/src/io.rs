mod helpers;

use anyhow::ensure;
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
    pub ads: Vec<Ad>,
}

#[derive(Debug, Clone, Copy)]
pub struct Ad {
    pub x: i64,
    pub y: i64,
    pub r: i64,
}

impl Input {
    pub const SIZE: i64 = 10000;
    pub const SIZE2: i64 = Self::SIZE * Self::SIZE;

    /// **(CUSTOMIZE IT!)** Generate Input
    pub fn gen(option: GenOption) -> Self {
        let mut rng = ChaCha20Rng::seed_from_u64(option.seed);

        // You shold generate u64 first and then convert it to usize because the size of usize is platform dependent.
        let n = (50.0 * 4.0f64.powf(rng.gen_range(0.0..1.0))).round() as usize;

        let mut pos = vec![];

        for _ in 0..n {
            let (x, y) = loop {
                let x = rng.gen_range(0..Input::SIZE);
                let y = rng.gen_range(0..Input::SIZE);

                if !pos.contains(&(x, y)) {
                    break (x, y);
                }
            };

            pos.push((x, y));
        }

        let mut r = vec![0, Input::SIZE2];

        for _ in 0..(n - 1) {
            let ri = loop {
                let ri = rng.gen_range(1..Input::SIZE2);
                if !r.contains(&ri) {
                    break ri;
                }
            };

            r.push(ri);
        }

        r.sort_unstable();

        let mut ad = vec![];

        for i in 0..n {
            ad.push(Ad {
                x: pos[i].0,
                y: pos[i].1,
                r: (r[i + 1] - r[i]),
            });
        }

        Self { n, ads: ad }
    }

    /// **(CUSTOMIZE IT!)** Parse Input from tokens
    pub(super) fn parse<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> anyhow::Result<Self> {
        let n = read(tokens.next(), 50, 200)?;

        let mut ad = vec![];

        for _ in 0..n {
            let x = read(tokens.next(), 0, Input::SIZE - 1)?;
            let y = read(tokens.next(), 0, Input::SIZE - 1)?;
            let r = read(tokens.next(), 1, Input::SIZE2 - 1)?;
            ad.push(Ad { x, y, r });
        }

        let area_sum = ad.iter().map(|ad| ad.r).sum::<i64>();

        ensure!(area_sum == Input::SIZE2, "Invalid area sum: {}", area_sum);

        Ok(Self { n, ads: ad })
    }
}

impl Display for Input {
    /// **(CUSTOMIZE IT!)** Format Input as string
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.n)?;

        for ad in &self.ads {
            writeln!(f, "{} {} {}", ad.x, ad.y, ad.r)?;
        }

        Ok(())
    }
}

/// **(CUSTOMIZE IT!)** Output for this problem
#[derive(Debug, Clone)]
pub struct Output {
    pub rects: Vec<Rect>,
}

#[derive(Debug, Clone)]
pub struct Rect {
    pub x1: i64,
    pub y1: i64,
    pub x2: i64,
    pub y2: i64,
}

impl Rect {
    pub fn area(&self) -> i64 {
        (self.x2 - self.x1) * (self.y2 - self.y1)
    }

    pub fn contains(&self, ad: &Ad) -> bool {
        self.x1 <= ad.x && ad.x < self.x2 && self.y1 <= ad.y && ad.y < self.y2
    }
}

impl Output {
    /// **(CUSTOMIZE IT!)** Parse Output from tokens
    pub(super) fn parse<'a>(
        input: &Input,
        tokens: &mut Peekable<impl Iterator<Item = &'a str>>,
    ) -> anyhow::Result<Self> {
        let mut rects = vec![];

        for _ in 0..input.n {
            let x1 = read(tokens.next(), 0, Input::SIZE)?;
            let y1 = read(tokens.next(), 0, Input::SIZE)?;
            let x2 = read(tokens.next(), x1 + 1, Input::SIZE)?;
            let y2 = read(tokens.next(), y1 + 1, Input::SIZE)?;

            rects.push(Rect { x1, y1, x2, y2 });
        }

        Ok(Self { rects })
    }

    /// **(CUSTOMIZE IT!)** Calculate score
    pub(super) fn calc_score(&self, input: &Input) -> anyhow::Result<i64> {
        let mut p_sum = 0.0;

        for i in 0..input.n {
            if !self.rects[i].contains(&input.ads[i]) {
                continue;
            }

            let r = input.ads[i].r as f64;
            let area = self.rects[i].area() as f64;
            let p = 1.0 - (1.0 - r.min(area) / r.max(area)).powi(2);

            p_sum += p;
        }

        let score = (1e9 * p_sum / input.n as f64).round() as i64;

        Ok(score)
    }
}
