use tools::{io::GenOption, parse_input, parse_outputs, vis::VisOption, visualize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn gen(seed: i32) -> String {
    let option = GenOption { seed: seed as u64 };
    let input = tools::io::Input::gen(option);
    input.to_string()
}

#[wasm_bindgen(getter_with_clone)]
pub struct Ret {
    pub score: i64,
    pub err: String,
    pub svg: String,
    pub cmd: String,
    pub before_comment: String,
    pub after_comment: String,
}

#[wasm_bindgen]
pub fn vis(input: String, output: String, turn: usize) -> Ret {
    let option = VisOption { turn };
    match try_vis(&input, &output, option) {
        Ok(ret) => ret,
        Err(err) => Ret {
            score: 0,
            err: format!("{:#}", err),
            svg: "".to_string(),
            cmd: "".to_string(),
            before_comment: "".to_string(),
            after_comment: "".to_string(),
        },
    }
}

fn try_vis(input: &str, output: &str, option: VisOption) -> anyhow::Result<Ret> {
    let input = parse_input(&input)?;
    let (outputs, comments) = parse_outputs(&input, &output)?;
    let vis_result = visualize(&input, &outputs, &comments, Some(option))?;

    Ok(Ret {
        score: vis_result.score,
        err: "".to_string(),
        svg: vis_result.svg.to_string(),
        cmd: vis_result.cmd,
        before_comment: vis_result.before_comment,
        after_comment: vis_result.after_comment,
    })
}

#[wasm_bindgen]
pub fn get_max_turn(input: String, output: String) -> usize {
    try_get_max_turn(&input, &output).unwrap_or(0)
}

fn try_get_max_turn(input: &str, output: &str) -> anyhow::Result<usize> {
    let input = parse_input(&input)?;
    let (outputs, _) = parse_outputs(&input, &output)?;

    Ok(outputs.len())
}
