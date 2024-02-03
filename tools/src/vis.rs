mod helpers;

use crate::{
    io::{Input, Output},
    vis::helpers::{circle, color, init_svg, rect, with_title},
};
use svg::Document;

use self::helpers::line;

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
        turn: outputs.len() - 1,
    });

    let mut doc = init_svg(VIEW_SIZE, VIEW_PADDING);

    if outputs.len() == 0 {
        doc = draw_points(doc, &input);
        return Ok(VisResult { score: 0, svg: doc });
    }

    let output = &outputs[option.turn];
    let score = output.calc_score(input)?;

    doc = draw_rects(doc, &input, &output);
    doc = draw_lines(doc, &input, &output);
    doc = draw_points(doc, input);

    Ok(VisResult { score, svg: doc })
}

fn draw_points(doc: Document, input: &Input) -> Document {
    let mut doc = doc;

    for ad in &input.ads {
        let x = ad.x as f64 * VIEW_SIZE / Input::SIZE as f64;
        let y = ad.y as f64 * VIEW_SIZE / Input::SIZE as f64;
        doc = doc.add(circle(x, y, 3.0, "black".into()));
    }

    doc
}

fn draw_rects(doc: Document, input: &Input, output: &Output) -> Document {
    let mut doc = doc;

    for (i, (ad, rec)) in input.ads.iter().zip(output.rects.iter()).enumerate() {
        let xr = rec.x1 as f64 * VIEW_SIZE / Input::SIZE as f64;
        let yr = rec.y1 as f64 * VIEW_SIZE / Input::SIZE as f64;
        let width = (rec.x2 - rec.x1) as f64 * VIEW_SIZE / Input::SIZE as f64;
        let height = (rec.y2 - rec.y1) as f64 * VIEW_SIZE / Input::SIZE as f64;
        let r = ad.r as f64;
        let area = rec.area() as f64;
        let p = r.min(area) / r.max(area);

        let color = if rec.contains(ad) {
            color(p / 2.0)
        } else {
            "white".into()
        };

        let rect = rect(xr, yr, width, height, color, Some("gray".into()));
        let group = with_title(rect, format!("[rect {i}]\nr={r}, s={area}, p={p:.03}"));

        doc = doc.add(group);
    }

    doc
}

fn draw_lines(doc: Document, input: &Input, output: &Output) -> Document {
    let mut doc = doc;

    for (ad, rec) in input.ads.iter().zip(output.rects.iter()) {
        let xp = ad.x as f64 * VIEW_SIZE / Input::SIZE as f64;
        let yp = ad.y as f64 * VIEW_SIZE / Input::SIZE as f64;
        let xr = (rec.x1 + rec.x2) as f64 * 0.5 * VIEW_SIZE / Input::SIZE as f64;
        let yr = (rec.y1 + rec.y2) as f64 * 0.5 * VIEW_SIZE / Input::SIZE as f64;

        doc = doc.add(line(xp, yp, xr, yr, 1.0, "black".into()))
    }

    doc
}
