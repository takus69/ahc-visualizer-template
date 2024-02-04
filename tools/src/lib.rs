pub mod io;
pub mod vis;

use anyhow::Context as _;
use io::Input;
use io::Output;
use std::io::Read as _;
use vis::{VisOption, VisResult};

/// Parse input
pub fn parse_input(s: &str) -> anyhow::Result<Input> {
    let mut s = s.split_whitespace().peekable();
    let input = Input::parse(&mut s).context("Failed to parse Input")?;
    Ok(input)
}

/// Parse single output
pub fn parse_output(input: &Input, s: &str) -> anyhow::Result<Output> {
    let outputs = parse_outputs(input, s)?;
    let last_output = outputs.into_iter().last().context("No output")?;
    Ok(last_output)
}

/// Parse multiple outputs
pub fn parse_outputs(input: &Input, s: &str) -> anyhow::Result<Vec<Output>> {
    let mut s = s.split_whitespace().peekable();
    let mut out = vec![];

    while s.peek().is_some() {
        out.push(Output::parse(input, &mut s).context("Failed to parse Output")?);
    }

    Ok(out)
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
    option: Option<VisOption>,
) -> anyhow::Result<VisResult> {
    vis::visualize(input, outputs, option).context("Failed to visualize")
}

/// Interact with the child process and calculate score
pub fn interact(process: &mut std::process::Child) -> anyhow::Result<i64> {
    let mut stdin_str = String::new();
    std::io::stdin().read_to_string(&mut stdin_str)?;
    let input = parse_input(&stdin_str)?;

    let stdin = std::io::BufWriter::new(
        process
            .stdin
            .take()
            .context("Failed to take stdin of child process")?,
    );
    let stdout = std::io::BufReader::new(
        process
            .stdout
            .take()
            .context("Failed to take stdout of child process")?,
    );

    io::interact(process, stdin, stdout, input).context("Failed to judge")
}
