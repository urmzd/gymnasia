use macroquad::prelude::*;

use super::{
    draw::{Color as DrawColor, DrawCommand, DrawList},
    renderer::{RenderColor, RenderFrame, RenderMode, Renders},
};

/// Macroquad rendering backend.
///
/// Executes [`DrawList`] commands and optionally captures pixel data.
/// The caller is responsible for frame pacing (`next_frame().await`).
#[derive(Debug, Clone)]
pub struct Screen;

impl Screen {
    /// Execute a draw list and return renders based on mode.
    ///
    /// The caller (example main) is responsible for calling `next_frame().await`
    /// after this returns for Human mode.
    pub fn execute(&self, draw_list: &DrawList, mode: RenderMode) -> Renders {
        let cam = Camera2D::from_display_rect(macroquad::math::Rect::new(
            0.0,
            0.0,
            draw_list.width as f32,
            draw_list.height as f32,
        ));
        set_camera(&cam);

        for cmd in &draw_list.commands {
            match cmd {
                DrawCommand::Clear(color) => {
                    clear_background(to_mq_color(color));
                }
                DrawCommand::FilledPolygon { vertices, color } => {
                    draw_convex_polygon(vertices, to_mq_color(color));
                }
                DrawCommand::FilledCircle {
                    x,
                    y,
                    radius,
                    color,
                } => {
                    draw_circle(*x, *y, *radius, to_mq_color(color));
                }
                DrawCommand::Line {
                    x1,
                    y1,
                    x2,
                    y2,
                    color,
                } => {
                    draw_line(*x1, *y1, *x2, *y2, 1.0, to_mq_color(color));
                }
                DrawCommand::Polyline { points, color } => {
                    let c = to_mq_color(color);
                    for pair in points.windows(2) {
                        draw_line(pair[0].0, pair[0].1, pair[1].0, pair[1].1, 1.0, c);
                    }
                }
            }
        }

        if [RenderMode::RgbArray, RenderMode::SingleRgbArray].contains(&mode) {
            Self::capture_pixels()
        } else {
            Renders::None
        }
    }

    /// Advance to the next frame (frame pacing).
    pub async fn next_frame(&self) {
        macroquad::prelude::next_frame().await;
    }

    fn capture_pixels() -> Renders {
        let image = get_screen_data();
        let w = image.width() as usize;
        let h = image.height() as usize;
        let bytes = image.bytes;

        let mut rows: Vec<Vec<RenderColor>> = Vec::with_capacity(h);
        for y in 0..h {
            let mut row = Vec::with_capacity(w);
            for x in 0..w {
                let idx = (y * w + x) * 4;
                row.push(RenderColor::RGB(bytes[idx], bytes[idx + 1], bytes[idx + 2]));
            }
            rows.push(row);
        }

        Renders::SingleRgbArray(RenderFrame(rows))
    }
}

fn to_mq_color(c: &DrawColor) -> Color {
    Color::new(
        c.r as f32 / 255.0,
        c.g as f32 / 255.0,
        c.b as f32 / 255.0,
        1.0,
    )
}

/// Draw a convex polygon as a triangle fan.
fn draw_convex_polygon(vertices: &[(f32, f32)], color: Color) {
    if vertices.len() < 3 {
        return;
    }
    let (ax, ay) = vertices[0];
    for i in 1..vertices.len() - 1 {
        let (bx, by) = vertices[i];
        let (cx, cy) = vertices[i + 1];
        draw_triangle(vec2(ax, ay), vec2(bx, by), vec2(cx, cy), color);
    }
}
