mod helpers;

use std::collections::HashSet;

use crate::{
    io::{Input, Output},
    vis::helpers::{create_circle, create_rect, create_text, get_color, init_svg, Color, Stroke},
};
use svg::Document;

use self::helpers::create_line;

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

const VIEW_OFFSET_Y: f64 = 100.0;
const VIEW_WIDTH: f64 = 600.0;
const VIEW_HEIGHT: f64 = VIEW_WIDTH + VIEW_OFFSET_Y;
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

    let mut doc = init_svg(VIEW_WIDTH, VIEW_HEIGHT, VIEW_PADDING);

    let output = if outputs.len() == 0 {
        Output { k: 0, cut: vec![] }
    } else {
        outputs[option.turn].clone()
    };

    doc = draw_cake(doc);
    doc = draw_lines(&output, doc);
    doc = draw_strawberry(input, &output, doc);
    doc = draw_table(input, &output, doc);

    let score = output.calc_score(&input)?;

    Ok(VisResult { score, svg: doc })
}

fn draw_table(input: &Input, output: &Output, mut doc: Document) -> Document {
    const X0: f64 = 25.0;
    const Y0: f64 = 10.0;
    const W: f64 = 50.0;
    const H: f64 = 30.0;

    let b = output.get_b(input);

    for i in 0..3 {
        for j in 0..=10 {
            let x0 = X0 + W * j as f64;
            let y0 = Y0 + H * i as f64;

            let color = if i == 0 && j > 0 {
                get_color((j - 1) as f64 / 9.0)
            } else {
                Color::Name("white".into())
            };

            doc = doc.add(create_rect(
                x0,
                y0,
                W,
                H,
                Some(color),
                Some(Stroke("black".into(), 1.0)),
            ));

            let text = if j == 0 {
                ["d", "a", "b"][i].to_string()
            } else if i == 0 {
                j.to_string()
            } else if i == 1 {
                input.a[j - 1].to_string()
            } else {
                b[j - 1].to_string()
            };

            let x = x0 + W / 2.0;
            let y = y0 + H * 4.0 / 5.0;

            doc = doc.add(create_text(x, y, 15.0, text));
        }
    }

    doc
}

fn draw_cake(mut doc: Document) -> Document {
    let center_x = VIEW_WIDTH * 0.5;
    let center_y = VIEW_OFFSET_Y + VIEW_WIDTH * 0.5;
    let scale = 280.0 / 1e4;

    doc = doc.add(create_circle(
        center_x,
        center_y,
        1e4 * scale,
        Some("white".into()),
        Some(Stroke("black".into(), 2.0)),
    ));

    doc
}

fn draw_lines(output: &Output, mut doc: Document) -> Document {
    let center_x = VIEW_WIDTH * 0.5;
    let center_y = VIEW_OFFSET_Y + VIEW_WIDTH * 0.5;
    let scale = 280.0 / 1e4;

    for &(x1, y1, x2, y2) in &output.cut {
        let x1 = center_x + x1 as f64 * scale;
        let y1 = center_y + y1 as f64 * scale;
        let x2 = center_x + x2 as f64 * scale;
        let y2 = center_y + y2 as f64 * scale;
        doc = doc.add(create_line(x1, y1, x2, y2, 1.0, "black".into()))
    }

    doc
}

fn draw_strawberry(input: &Input, output: &Output, mut doc: Document) -> Document {
    let center_x = VIEW_WIDTH * 0.5;
    let center_y = VIEW_OFFSET_Y + VIEW_WIDTH * 0.5;
    let scale = 280.0 / 1e4;

    let groups = output.div_cakes(input);
    let mut found = HashSet::new();

    for group in groups {
        let color = if group.len() >= 1 && group.len() <= 10 {
            get_color((group.len() - 1) as f64 / 9.0)
        } else {
            Color::Name("white".into())
        };

        for &(x, y) in &group {
            found.insert((x, y));
        }

        doc = draw_group(&group, center_x, scale, center_y, color, doc);
    }

    for &(x, y) in &input.p {
        if !found.contains(&(x, y)) {
            doc = draw_group(
                &vec![(x, y)],
                center_x,
                scale,
                center_y,
                Color::Name("white".into()),
                doc,
            );
        }
    }

    doc
}

fn draw_group(
    group: &[(i64, i64)],
    center_x: f64,
    scale: f64,
    center_y: f64,
    color: Color,
    mut doc: Document,
) -> Document {
    for &(x, y) in group {
        let x = center_x + x as f64 * scale;
        let y = center_y + y as f64 * scale;

        doc = doc.add(create_circle(
            x,
            y,
            3.0,
            Some(color),
            Some(Stroke("gray".into(), 1.0)),
        ));
    }

    doc
}
