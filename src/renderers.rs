//! Typical renderers.

use crate::render::render_arguments;
use crate::Renderer;

#[derive(Clone, Copy)]
/// Renders just the `record.args()`.
pub struct ArgumentsRenderer {}

/// The default, minimal renderer.
pub fn default() -> &'static ArgumentsRenderer {
    static RENDERER: ArgumentsRenderer = ArgumentsRenderer {};
    &RENDERER
}


impl Renderer for ArgumentsRenderer {
    fn render<'a>(&self, buf: &'a mut [u8], record: &log::Record) -> &'a [u8] {
        render_arguments(buf, *record.args())
    }
}

unsafe impl Send for ArgumentsRenderer {}
unsafe impl Sync for ArgumentsRenderer {}

#[derive(Clone, Copy)]
/// Renders the `record.args()`, prefixed by level, target, and file, line if they are some.
pub struct RipgrepRenderer {}

impl Renderer for RipgrepRenderer {
    fn render<'a>(&self, buf: &'a mut [u8], record: &log::Record) -> &'a [u8] {
        match (record.file(), record.line()) {
            (Some(file), Some(line)) => render_arguments(buf,
                 format_args!("{}|{}|{}:{}: {}", record.level(), record.target(), file, line, record.args())),
            (Some(file), None) => render_arguments(buf,
                 format_args!("{}|{}|{}: {}", record.level(), record.target(), file, record.args())),
            _ => render_arguments(buf,
                 format_args!("{}|{}: {}", record.level(), record.target(), record.args())),
        }
    }
}
