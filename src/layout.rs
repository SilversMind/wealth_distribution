use crate::constants::{CELL_SIZE, OFFSET, PANEL_W, GRID_W, GRID_H};

const TOOLBAR_CSS_H: f32 = 120.0;
const MOBILE_THRESHOLD: f32 = 1100.0;

#[cfg(target_arch = "wasm32")]
fn get_dpr() -> f32 {
    extern "C" { fn device_pixel_ratio() -> f32; }
    (unsafe { device_pixel_ratio() }).max(1.0)
}

#[cfg(not(target_arch = "wasm32"))]
fn get_dpr() -> f32 { 1.0 }

pub struct Layout {
    pub cell_size:  f32,
    pub grid_x:     f32,
    pub grid_y:     f32,
    pub panel_w:    f32,
    pub toolbar_h:  f32,
    pub is_mobile:  bool,
    pub dpr:        f32,
}

impl Layout {
    pub fn from_screen(sw: f32, sh: f32) -> Self {
        if sw < MOBILE_THRESHOLD {
            let dpr       = get_dpr();
            let toolbar_h = TOOLBAR_CSS_H * dpr;
            let margin    = 8.0 * dpr;
            let avail_w   = sw - 2.0 * margin;
            let avail_h   = sh - toolbar_h;
            let cell_size = (avail_w / GRID_W as f32).min(avail_h / GRID_H as f32);
            let grid_w    = cell_size * GRID_W as f32;
            let grid_h    = cell_size * GRID_H as f32;
            let grid_x    = (sw - grid_w) / 2.0;
            let grid_y    = ((avail_h - grid_h) / 2.0).max(margin);
            Layout { cell_size, grid_x, grid_y, panel_w: sw - 2.0 * margin, toolbar_h, is_mobile: true, dpr }
        } else {
            Layout { cell_size: CELL_SIZE, grid_x: OFFSET, grid_y: OFFSET, panel_w: PANEL_W, toolbar_h: 0.0, is_mobile: false, dpr: 1.0 }
        }
    }
}
