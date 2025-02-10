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
        turn: if outputs.len() > 0 { outputs[0].s.len()} else { 0 },
    });

    let mut doc = init_svg(VIEW_SIZE, VIEW_SIZE, VIEW_PADDING);

    // Draw Input
    fn drow(doc: Document, input: &Input, prob: &Vec<Vec<f64>>) -> Document {
        let mut doc = doc;
        let n = 20;
        let d = VIEW_SIZE / (n as f64);
        doc = doc.add(create_rect(0.0, 0.0, VIEW_SIZE, VIEW_SIZE, Some("white".into()), Some(Stroke{0: "black".into(), 1: 1.0})));

        // 経路の確率を色付け
        let mut max_p = f64::MIN;
        for i in 0..prob.len() {
            for j in 0..prob[0].len() {
                let p = prob[i][j];
                max_p = max_p.max(p);
            }
        }
        for i in 0..20 {
            for j in 0..20 {
                let p = prob[i][j];
                if p == 0.0 { continue; }
                let color = get_color(p/max_p);
                let x = j as f64 * d;
                let y = i as f64 * d;
                doc = doc.add(create_rect(x, y, d, d, Some(color), Some(Stroke{0: "black".into(), 1: 0.0})));
            }
        }

        // 壁の描写
        for i in 0..20 {
            let hi: Vec<char> = input.h[i].chars().collect();
            for j in 0..19 {
                if hi[j] == '1' {
                    let x1 = (j+1) as f64 * d;
                    let x2 = x1;
                    let y1 = i as f64 * d;
                    let y2 = (i+1) as f64 * d;
                    doc = doc.add(create_line(x1, y1, x2, y2, 1.0, "black".into()));
                }
            }
        }

        for i in 0..19 {
            let vi: Vec<char> = input.v[i].chars().collect();
            for j in 0..20 {
                if vi[j] == '1' {
                    let x1 = j as f64 * d;
                    let x2 = (j+1) as f64 * d;
                    let y1 = (i+1) as f64 * d;
                    let y2 = y1;
                    doc = doc.add(create_line(x1, y1, x2, y2, 1.0, "black".into()));
                }
            }
        }

        // スタートとゴールを描写
        let sy = input.si as f64 * d + d;
        let sx = input.sj as f64 * d + d/2.0;
        doc = doc.add(create_text(sx, sy, 30.0, "s"));

        let sy = input.ti as f64 * d + d;
        let sx = input.tj as f64 * d + d/2.0;
        doc = doc.add(create_text(sx, sy, 30.0, "t"));

        doc
    }

    let mut prob = vec![vec![0.0; 20]; 20];
    prob[input.si][input.sj] = 1.0;
    doc = drow(doc, &input, &prob);

    // Draw Output
    if outputs.len() == 0 {
        return Ok(VisResult { score: 0, svg: doc });
    }

    let output = &outputs[0];
    let (score, probs) = output.calc_score(input)?;
    doc = drow(doc, &input, &probs[option.turn]);

    Ok(VisResult { score, svg: doc })
}
