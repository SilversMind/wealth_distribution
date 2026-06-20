use macroquad::prelude::*;

pub mod chart;
pub mod config_panel;
pub mod grid;
pub mod overlay;
pub mod panel;

pub fn fmt_wealth(v: i64) -> String {
    if v.abs() >= 1_000_000_000 { format!("{:.1}B", v as f64 / 1_000_000_000.0) }
    else if v.abs() >= 1_000_000 { format!("{:.1}M", v as f64 / 1_000_000.0) }
    else if v.abs() >= 1_000     { format!("{:.1}K", v as f64 / 1_000.0) }
    else                          { format!("{}$", v) }
}

pub fn draw_button(x: f32, y: f32, w: f32, h: f32, label: &str, active: bool, font_size: f32) -> bool {
    let (mx, my) = mouse_position();
    let hovered  = mx >= x && mx <= x + w && my >= y && my <= y + h;
    let bg = if active      { YELLOW }
             else if hovered { Color::new(0.4, 0.4, 0.5, 1.0) }
             else            { Color::new(0.2, 0.2, 0.28, 1.0) };
    draw_rectangle(x, y, w, h, bg);
    draw_rectangle_lines(x, y, w, h, 1.0, Color::new(0.6, 0.6, 0.7, 1.0));
    let text_color = if active { BLACK } else { WHITE };
    draw_text(label, x + w / 2.0 - label.len() as f32 * 4.5, y + h * 0.70, font_size, text_color);
    hovered && is_mouse_button_pressed(MouseButton::Left)
}
