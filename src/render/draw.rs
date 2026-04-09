//! Drawing primitives that environments emit for rendering.
//!
//! Environments produce a [`DrawList`] describing what to draw.
//! The rendering backend (macroquad) consumes it. These types have
//! **no dependency on any graphics library** — they are always compiled.

/// An RGB color.
#[derive(Debug, Clone, Copy)]
pub struct Color {
    /// Red channel (0–255).
    pub r: u8,
    /// Green channel (0–255).
    pub g: u8,
    /// Blue channel (0–255).
    pub b: u8,
}

impl Color {
    /// White.
    pub const WHITE: Self = Self {
        r: 255,
        g: 255,
        b: 255,
    };
    /// Black.
    pub const BLACK: Self = Self { r: 0, g: 0, b: 0 };

    /// Construct a color from RGB components.
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

/// A single drawing command.
#[derive(Debug, Clone)]
pub enum DrawCommand {
    /// Fill the entire canvas with a color.
    Clear(Color),
    /// Draw a filled convex polygon.
    FilledPolygon {
        /// Vertices in order.
        vertices: Vec<(f32, f32)>,
        /// Fill color.
        color: Color,
    },
    /// Draw a filled circle.
    FilledCircle {
        /// Center x.
        x: f32,
        /// Center y.
        y: f32,
        /// Radius.
        radius: f32,
        /// Fill color.
        color: Color,
    },
    /// Draw a line segment.
    Line {
        /// Start x.
        x1: f32,
        /// Start y.
        y1: f32,
        /// End x.
        x2: f32,
        /// End y.
        y2: f32,
        /// Line color.
        color: Color,
    },
    /// Draw a connected sequence of line segments.
    Polyline {
        /// Points in order.
        points: Vec<(f32, f32)>,
        /// Line color.
        color: Color,
    },
}

/// An ordered list of draw commands with logical canvas dimensions.
#[derive(Debug, Clone)]
pub struct DrawList {
    /// The draw commands to execute in order.
    pub commands: Vec<DrawCommand>,
    /// Logical canvas width in pixels.
    pub width: u32,
    /// Logical canvas height in pixels.
    pub height: u32,
}

impl DrawList {
    /// Create an empty draw list with the given dimensions.
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            commands: Vec::new(),
            width,
            height,
        }
    }

    /// Append a draw command.
    pub fn push(&mut self, cmd: DrawCommand) {
        self.commands.push(cmd);
    }
}

/// Rotates a point `(x, y)` by `angle` radians around the origin.
pub fn rotate_point(x: f64, y: f64, angle: f64) -> (f64, f64) {
    let (sin, cos) = angle.sin_cos();
    (x * cos - y * sin, x * sin + y * cos)
}
