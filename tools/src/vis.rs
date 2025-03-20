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
}

const VIEW_SIZE: f64 = 600.0;
const VIEW_PADDING: f64 = 10.0;

/// **(CUSTOMIZE IT!)** Visualize the output
pub(super) fn visualize(
    input: &Input,
    outputs: &[Output],
    option: Option<VisOption>,
) -> anyhow::Result<VisResult> {
    let option = option.unwrap_or(VisOption {
        turn: outputs.len(),
    });

    let mut doc = init_svg(VIEW_SIZE*2.0, VIEW_SIZE, VIEW_PADDING);

    // Draw Input
    fn drow(doc: Document, input: &Input, output: Option<&Output>) -> Document {
        let mut doc = doc;

        doc = doc.add(create_rect(0.0, 0.0, VIEW_SIZE, VIEW_SIZE, Some("white".into()), Some(Stroke{0: "black".into(), 1: 1.0})));

        let x = 30.0 * (input.n as f64 + 1.0);
        doc = doc.add(create_rect(x, 10.0, 60.0, 60.0, Some(get_color(0.5)), None));

        // output
        let output_width= 200.0;
        let output_height = 100.0;
        let font_size = 20.0;
        let output_x = VIEW_SIZE + VIEW_PADDING + output_width / 2.0;
        let output_y = VIEW_PADDING + font_size;
        doc = doc.add(create_rect(VIEW_SIZE+VIEW_PADDING, 0.0, output_width, output_height, Some("white".into()), Some(Stroke{0: "black".into(), 1: 1.0})));
        if let Some(output) = output {
            let output_text = format!("{}", output.k);
            doc = doc.add(create_text(output_x, output_y, font_size, output_text));
        } else {
            return doc;
        };

        doc
    }

    doc = drow(doc, input, None);

    todo!("Write code to visualize here.");

    // Draw Output
    if option.turn == 0 {
        return Ok(VisResult { score: 0, svg: doc });
    }

    let output = &outputs[option.turn-1];
    doc = drow(doc, input, Some(output));

    let score = output.calc_score(input)?;

    let y = 10.0 * (output.k as f64 + 1.0);
    doc = doc.add(with_title(
        create_circle(200., y, 20., Some("gray".into()), None),
        "hoge",
    ));

    Ok(VisResult { score, svg: doc })
}
