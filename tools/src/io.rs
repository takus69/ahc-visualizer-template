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
    pub si: usize,
    pub sj: usize,
    pub ti: usize,
    pub tj: usize,
    pub p: f64,
    pub h: Vec<String>,
    pub v: Vec<String>, 
}

impl Input {
    /// **(CUSTOMIZE IT!)** Generate Input
    pub fn gen(option: GenOption) -> Self {
        let mut rng = ChaCha20Rng::seed_from_u64(option.seed);

        // You shold generate u64 first and then convert it to usize because the size of usize is platform dependent.
        let si = rng.gen_range(0..=4u64) as usize;
        let sj = rng.gen_range(0..=4u64) as usize;
        let ti = rng.gen_range(15..=19u64) as usize;
        let tj = rng.gen_range(15..=19u64) as usize;
        let tj = rng.gen_range(15..=19u64) as usize;
        let p = rng.gen_range(10..=50u64) as usize;
        let p = p as f64 / 100.0;

        let mut h = vec![
            "0010000100110011100".to_string(),
            "1000001000001000001".to_string(),
            "0000000000000000010".to_string(),
            "0100000000000110100".to_string(),
            "0000000000000000100".to_string(),
            "1000101100000101010".to_string(),
            "0010001011000110000".to_string(),
            "0000001001000000000".to_string(),
            "0000000100010001001".to_string(),
            "0010010000100000001".to_string(),
            "0001000010000100000".to_string(),
            "0011010000000001000".to_string(),
            "0000000101010100000".to_string(),
            "0000001000000100010".to_string(),
            "0110100010000000000".to_string(),
            "0010011101000101000".to_string(),
            "0000100110010000000".to_string(),
            "0010000101101000010".to_string(),
            "1001000000000000000".to_string(),
            "1000110000000000000".to_string(),
        ];
        let mut v = vec![
            "00000001000000000100".to_string(),
            "00001000010001000000".to_string(),
            "00010001010000010000".to_string(),
            "01110010101000010100".to_string(),
            "00000000000001100000".to_string(),
            "00001000010000000100".to_string(),
            "00101000000010110011".to_string(),
            "01010100000000000000".to_string(),
            "00001101010010010010".to_string(),
            "10000000000000010100".to_string(),
            "01011010000001100100".to_string(),
            "00000000000000010011".to_string(),
            "00001100111000110100".to_string(),
            "00000010000000000000".to_string(),
            "00010000100111000000".to_string(),
            "11010000001001010100".to_string(),
            "01100010011001011001".to_string(),
            "00000101000010101010".to_string(),
            "00100000000000000001".to_string(),
        ];

        Self { si, sj, ti, tj, p, h, v }
    }

    /// **(CUSTOMIZE IT!)** Parse Input from tokens
    pub(super) fn parse<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> anyhow::Result<Self> {
        let si = read(tokens.next(), 0, 4)?;
        let sj = read(tokens.next(), 0, 4)?;
        let ti = read(tokens.next(), 15, 19)?;
        let tj = read(tokens.next(), 15, 19)?;
        let p = read(tokens.next(), 0.1, 0.5)?;

        let mut h = vec![];
        for i in 0..20 {
            let hi = tokens.next().unwrap();
            h.push(hi.to_string());
        }

        let mut v = vec![];
        for i in 0..19 {
            let vi = tokens.next().unwrap();
            v.push(vi.to_string());
        }

        // todo!("Write code to parse Input here.");

        Ok(Self { si, sj, ti, tj, p, h, v })
    }
}

impl Display for Input {
    /// **(CUSTOMIZE IT!)** Format Input as string
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {} {} {} {}", self.si, self.sj, self.ti, self.tj, self.p)?;
        for hi in self.h.iter() {
            writeln!(f, "{}", hi)?;
        }
        for vi in self.v.iter() {
            writeln!(f, "{}", vi)?;
        }

        Ok(())
    }
}

/// **(CUSTOMIZE IT!)** Output for this problem
#[derive(Debug, Clone)]
pub struct Output {
    pub s: String,
}

impl Output {
    /// **(CUSTOMIZE IT!)** Parse Output from tokens
    pub(super) fn parse<'a>(
        input: &Input,
        tokens: &mut Peekable<impl Iterator<Item = &'a str>>,
    ) -> anyhow::Result<Self> {
        let s = tokens.next().unwrap();
        let s = s.to_string();

        Ok(Self { s })
    }

    /// **(CUSTOMIZE IT!)** Calculate score
    pub(super) fn calc_score(&self, input: &Input) -> anyhow::Result<(i64, Vec<Vec<Vec<f64>>>)> {
        let score = 0;

        let s_chars = self.s.chars().collect::<Vec<char>>();
        let mut probs: Vec<Vec<Vec<f64>>> = vec![vec![vec![0.0; 20]; 20]; self.s.len()+1];
        probs[0][input.si][input.sj] = 1.0;
        let mut h: Vec<Vec<char>> = Vec::new();
        for hi in input.h.iter() {
            h.push(hi.chars().collect());
        }
        let mut v: Vec<Vec<char>> = Vec::new();
        for vi in input.v.iter() {
            v.push(vi.chars().collect());
        }
        for t in 0..self.s.len() {
            let dir = s_chars[t];
            for i in 0..20 {
                for j in 0..20 {
                    let (i2, j2) = match dir {
                        'L' => { if j==0 || h[i][j-1]=='1' { (i, j) } else { (i, j-1) }},
                        'R' => { if j==19 || h[i][j]=='1' { (i, j) } else { (i, j+1) }},
                        'U' => { if i==0 || v[i-1][j]=='1' { (i, j) } else { (i-1, j) }},
                        'D' => { if i==19 || v[i][j]=='1' { (i, j) } else { (i+1, j) }},
                        _ => { (i, j) },
                    };
                    probs[t+1][i2][j2] += probs[t][i][j] * input.p;
                    probs[t+1][i][j] += probs[t][i][j] * (1.0-input.p);
                }
            }
        }

        Ok((score, probs))
    }
}
