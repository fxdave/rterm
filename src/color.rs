// This covers the initial 16 colors, it also support 256 and true color modes
// and a palette of of the colors from 16-255 are generated (see Win::new() in
// win.rs).  The foreground/background and cursor get there own 'slots' after
// the 256 color palette.
pub const COLOR_NAMES: &[&str] = &[
    "#3f3f3f", "#705050", "#60b48a", "#dfaf8f", "#506070", "#dc8cc3", "#8cd0d3", "#dcdccc",
    "#709080", "#dca3a3", "#c3bf9f", "#f0dfaf", "#94bff3", "#ec93d3", "#93e0e3", "#ffffff",
];

pub const FG_COLOR: usize = 258;
pub const BG_COLOR: usize = 259;
pub const CURSOR_COLOR: usize = 256;
pub const CURSOR_REV_COLOR: usize = 257;
pub const FG_COLOR_NAME: &str = "#ffffff";
pub const BG_COLOR_NAME: &str = "#101316";
pub const CURSOR_COLOR_NAME: &str = "#cccccc";
pub const CURSOR_REV_COLOR_NAME: &str = "#555555";
