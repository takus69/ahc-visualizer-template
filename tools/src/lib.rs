pub mod io;
pub mod vis;

use anyhow::Context as _;
use io::Input;
use io::Output;
use vis::{VisOption, VisResult};

/// Parse input
pub fn parse_input(s: &str) -> anyhow::Result<Input> {
    let mut s = s.split_whitespace().peekable();
    let input = Input::parse(&mut s).context("Failed to parse Input")?;
    Ok(input)
}

/// Parse single output
pub fn parse_output(input: &Input, s: &str) -> anyhow::Result<Output> {
    let (outputs, comments) = parse_outputs(input, s)?;
    let last_output = outputs.into_iter().last().context("No output")?;
    Ok(last_output)
}

/// Parse multiple outputs
pub fn parse_outputs(input: &Input, s: &str) -> anyhow::Result<(Vec<Output>, Vec<String>)> {
    // let mut s = s.split_whitespace().peekable();
    let mut s = s.lines().peekable();
    let mut out = vec![];
    let mut comments: Vec<String> = Vec::new();

    while s.peek().is_some() {
        let (o, c) = Output::parse(input, &mut s).context("Failed to parse Output")?;
        if let Some(output) = o {
            out.push(output);
        }
        comments.push(c.join("\n"));
    }

    Ok((out, comments))
}

/// Calculate score
pub fn calc_score(input: &Input, output: &Output) -> anyhow::Result<i64> {
    output
        .calc_score(input)
        .context("Failed to calculate score")
}

/// Visualize the output
pub fn visualize(
    input: &Input,
    outputs: &[Output],
    comments: &[String],
    option: Option<VisOption>,
) -> anyhow::Result<VisResult> {
    vis::visualize(input, outputs, comments, option).context("Failed to visualize")
}
