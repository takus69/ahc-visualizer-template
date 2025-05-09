mod helpers;

use crate::{
    io::{Input, Output},
    vis::helpers::{create_circle, create_rect, get_color, init_svg, with_title, Stroke, create_line, create_text},
};
use svg::Document;

/// **(CUSTOMIZE IT!)**  Option for visualization
#[derive(Debug, Clone)]
pub struct VisOption {
    pub turn: usize,
}

/// **(CUSTOMIZE IT!)** Result of visualization
#[derive(Debug, Clone)]
pub struct VisResult {
    pub score: i64,
    pub svg: Document,
    pub cmd: String,
    pub before_comment: String,
    pub after_comment: String,
}

const VIEW_SIZE: f64 = 600.0;
const VIEW_PADDING: f64 = 10.0;

/// **(CUSTOMIZE IT!)** Visualize the output
pub(super) fn visualize(
    input: &Input,
    outputs: &[Output],
    comments: &[String],
    option: Option<VisOption>,
) -> anyhow::Result<VisResult> {
    let option = option.unwrap_or(VisOption {
        turn: outputs.len(),
    });

    let mut doc = init_svg(VIEW_SIZE, VIEW_SIZE, VIEW_PADDING);

    // Draw Input
    fn drow(doc: Document, input: &Input, output: Option<&Output>) -> Document {
        let mut doc = doc;

        doc = doc.add(create_rect(0.0, 0.0, VIEW_SIZE, VIEW_SIZE, Some("white".into()), Some(Stroke{0: "black".into(), 1: 1.0})));

        let x = 30.0 * (input.n as f64 + 1.0);
        doc = doc.add(create_rect(x, 10.0, 60.0, 60.0, Some(get_color(0.5)), None));

        // 色
        for i in 0..=100 {
            let x = i as f64 * 5.0;
            let c = i as f64 / 100.0;
            doc = doc.add(with_title(
                create_rect(x, 100.0, 5.0, 5.0, Some(get_color(c)), None),
                format!("color: {}", c),
            ));
        }

        doc
    }

    doc = drow(doc, input, None);

    // todo!("Write code to visualize here.");

    // Draw Output
    if option.turn == 0 {
        return Ok(VisResult { score: 0, svg: doc, cmd: String::new(), before_comment: String::new(), after_comment: String::new() });
    }

    let output = &outputs[option.turn-1];
    let before_comment = comments[option.turn-1].clone();
    let after_comment = if comments.len() > option.turn { comments[option.turn].clone() } else { String::new() };
    doc = drow(doc, input, Some(output));

    let score = output.calc_score(input)?;

    // outputの描画例
    // 円
    let y = 10.0 * (output.k as f64 + 1.0);
    doc = doc.add(with_title(
        create_circle(200., y, 20., Some("gray".into()), None),
        "hoge",
    ));
    // 直線
    let x = 30.0 * (input.n as f64 + 1.0);
    doc = doc.add(create_line(200., y, x, 10., 1.0, "red".into()));
    // テキスト
    doc = doc.add(create_text(x, 10.0, 20.0,format!("{}", output.k)));

    Ok(VisResult { score, svg: doc, cmd: output.to_string(), before_comment, after_comment })
}
