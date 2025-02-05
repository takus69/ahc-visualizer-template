mod helpers;

use crate::{
    io::{Input, Output},
    vis::helpers::{create_circle, create_rect, create_text, get_color, init_svg, with_title, Stroke},
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

    let mut doc = init_svg(VIEW_SIZE, VIEW_SIZE, VIEW_PADDING);

    // Draw Input
    fn drow_board(doc: Document, input: &Input) -> Document {
        let mut doc = doc;
        doc = doc.add(create_rect(0.0, 0.0, VIEW_SIZE, VIEW_SIZE, Some("white".into()), Some(Stroke{0: "black".into(), 1: 1.0})));
        let d = VIEW_SIZE / input.n as f64;
        for i in 0..input.n {
            for j in 0..input.n {
                let x = j as f64 * d;
                let y = i as f64 * d;
                doc = doc.add(create_rect(x, y, d, d, Some("white".into()), Some(Stroke{0: "black".into(), 1: 1.0})));
                if input.c[i][j] == 'o' {
                    doc = doc.add(create_text(x+d/2.0, y+d, 30.0, "o"));
                } else if input.c[i][j] == 'x' {
                    doc = doc.add(create_text(x+d/2.0, y+d, 30.0, "x"));
                }
            }
        }

        doc
    }
    doc = drow_board(doc, input);
    let mut first_x_cnt = 0;
    for i in 0..input.n {
        for j in 0..input.n {
            if input.c[i][j] == 'x' {
                first_x_cnt += 1;
            }
        }
    }
    // todo!("Write code to visualize here.");

    // Draw Output
    if option.turn == 0 {
        return Ok(VisResult { score: (4*input.n*input.n - input.n*first_x_cnt) as i64, svg: doc });
    }

    let mut c = input.c.clone();
    let mut o_cnt = 0;
    let mut x_cnt = 0;
    for t in 0..option.turn {
        let output = outputs[t].clone();
        let p = output.p;
        match output.d {
            'L' => {
                match c[p][0] {
                    'o' => { o_cnt += 1; },
                    'x' => { x_cnt += 1; },
                    _ => {},
                }
                for j in 0..(input.n-1) {
                    c[p][j] = c[p][j+1];
                }
                c[p][input.n-1] = '.';
            },
            'R' => {
                match c[p][input.n-1] {
                    'o' => { o_cnt += 1; },
                    'x' => { x_cnt += 1; },
                    _ => {},
                }
                for j in (1..input.n).rev() {
                    c[p][j] = c[p][j-1];
                }
                c[p][0] = '.';
            },
            'U' => {
                match c[0][p] {
                    'o' => { o_cnt += 1; },
                    'x' => { x_cnt += 1; },
                    _ => {},
                }
                for i in 0..(input.n-1) {
                    c[i][p] = c[i+1][p];
                }
                c[input.n-1][p] = '.';
            },
            'D' => {
                match c[input.n-1][p] {
                    'o' => { o_cnt += 1; },
                    'x' => { x_cnt += 1; },
                    _ => {},
                }
                for i in (1..input.n).rev() {
                    c[i][p] = c[i-1][p];
                }
                c[0][p] = '.';
            },
            _ => {},
        }
    }
    let input = Input { n: input.n, c };
    doc = drow_board(doc, &input);
    let score = if (first_x_cnt-x_cnt) == 0 && o_cnt == 0 {
        (8*input.n*input.n - option.turn) as i64
    } else {
        (4*input.n*input.n - input.n*(first_x_cnt-x_cnt+o_cnt)) as i64
    };

    Ok(VisResult { score, svg: doc })
}
