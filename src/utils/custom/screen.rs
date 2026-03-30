use derivative::Derivative;
use derive_new::new;
use macroquad::prelude::*;
use serde::Serialize;

use crate::utils::{
    custom::draw::{Color as DrawColor, DrawCommand, DrawList},
    renderer::{RenderColor, RenderFrame, RenderMode, Renders},
};

/// Wrapper over macroquad for rendering environments.
#[derive(Serialize, Derivative, new, Clone)]
#[derivative(Debug)]
pub struct Screen {
    height: u32,
    width: u32,
    title: &'static str,
    render_fps: u32,
    mode: RenderMode,
    #[serde(skip_serializing)]
    #[derivative(Debug = "ignore")]
    #[new(default)]
    initialized: bool,
}

impl Screen {
    /// Closes the rendering surface.
    pub fn close(&mut self) {
        self.initialized = false;
    }

    /// Returns whether the screen has been initialized.
    pub fn is_open(&self) -> bool {
        self.initialized
    }

    /// Returns the logical screen width.
    pub fn screen_width(&self) -> u32 {
        self.width
    }

    /// Execute a draw list and return renders based on mode.
    ///
    /// The caller (example main) is responsible for calling `next_frame().await`
    /// after this returns for Human mode.
    pub fn execute(&mut self, draw_list: &DrawList, mode: RenderMode) -> Renders {
        self.initialized = true;

        // Set up camera so (0,0) is top-left with y increasing downward,
        // then flip vertically (matching the old SDL2 behavior where flip_vertical=true).
        let cam = Camera2D {
            zoom: vec2(2.0 / draw_list.width as f32, -2.0 / draw_list.height as f32),
            offset: vec2(-1.0, 1.0),
            ..Default::default()
        };
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
            self.capture_pixels()
        } else {
            Renders::None
        }
    }

    fn capture_pixels(&self) -> Renders {
        let image = get_screen_data();
        let w = image.width() as usize;
        let h = image.height() as usize;
        let bytes = image.bytes;

        // macroquad returns RGBA, we need RGB, and the image is bottom-up
        let mut rows: Vec<Vec<RenderColor>> = Vec::with_capacity(h);
        for y in 0..h {
            let mut row = Vec::with_capacity(w);
            for x in 0..w {
                let idx = (y * w + x) * 4;
                row.push(RenderColor::RGB(bytes[idx], bytes[idx + 1], bytes[idx + 2]));
            }
            rows.push(row);
        }

        Renders::SingleRgbArray(RenderFrame::new(rows))
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
