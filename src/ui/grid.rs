use macroquad::prelude::*;

use crate::agent::{Agent, wealth_color, bonus_color};
use crate::config::FontSizes;
use crate::constants::*;
use super::{draw_button, fmt_wealth};

pub struct CustomSpeedState {
    pub buf:     String,
    pub focused: bool,
    pub mult:    Option<u32>, // active custom multiplier (1-5000), None = use preset
}

impl Default for CustomSpeedState {
    fn default() -> Self {
        CustomSpeedState { buf: String::new(), focused: false, mult: None }
    }
}

impl CustomSpeedState {
    // seconds per tick for current custom mult
    pub fn interval(&self) -> Option<f64> {
        self.mult.map(|m| 0.5 / m as f64)
    }
    #[allow(dead_code)]
    pub fn reset(&mut self) { *self = Self::default(); }
}

pub fn draw_world(
    agents:       &[Agent],
    tick_count:   u64,
    deviance:     i32,
    transfer_pct: i32,
    tooltip_font: f32,
    speed_idx:    usize,
    speeds:       &[(f64, &str)],
    fonts:        &FontSizes,
    selected:     Option<usize>,
    total_wealth: i64,
    custom:       &mut CustomSpeedState,
) -> (Option<usize>, bool, bool) {
    clear_background(Color::new(0.06, 0.06, 0.08, 1.0));

    // grid lines
    let grid_color = Color::new(0.15, 0.15, 0.2, 1.0);
    for x in 0..=GRID_W {
        let px = OFFSET + x as f32 * CELL_SIZE;
        draw_line(px, OFFSET, px, OFFSET + GRID_H as f32 * CELL_SIZE, 0.5, grid_color);
    }
    for y in 0..=GRID_H {
        let py = OFFSET + y as f32 * CELL_SIZE;
        draw_line(OFFSET, py, OFFSET + GRID_W as f32 * CELL_SIZE, py, 0.5, grid_color);
    }

    // agents
    for (i, agent) in agents.iter().enumerate() {
        let cx = OFFSET + agent.pos.0 as f32 * CELL_SIZE + CELL_SIZE / 2.0;
        let cy = OFFSET + agent.pos.1 as f32 * CELL_SIZE + CELL_SIZE / 2.0;
        let r  = CELL_SIZE / 2.0 - 1.0;

        let total_pat = agent.wealth + agent.capital;
        if total_pat <= 0 {
            draw_circle(cx, cy, r, Color::new(0.12, 0.12, 0.12, 1.0));
            draw_circle_lines(cx, cy, r, 1.0, RED);
            let arm = r * 0.6;
            draw_line(cx - arm, cy - arm, cx + arm, cy + arm, 1.5, RED);
            draw_line(cx + arm, cy - arm, cx - arm, cy + arm, 1.5, RED);
        } else {
            draw_circle(cx, cy, r, wealth_color(total_pat, total_wealth));
            let outline_color = if selected == Some(i) { WHITE } else { agent.color };
            let outline_w     = if selected == Some(i) { 2.5 } else { 1.0 };
            draw_circle_lines(cx, cy, r, outline_w, outline_color);
        }
    }

    // hud
    draw_text(
        &format!("Tick: {}  |  deviance: {}  |  transfer: {}%", tick_count, deviance, transfer_pct),
        10.0, 20.0, fonts.main_title, WHITE,
    );

    // right panel
    let lx    = OFFSET + GRID_W as f32 * CELL_SIZE + 15.0;
    let mut ly = OFFSET;
    let btn_w = PANEL_W - 10.0;
    let btn_h = 28.0;

    // wealth legend
    draw_text("Richesse", lx, ly, fonts.section_title, WHITE);
    ly += 22.0;
    let max_ln = (total_wealth.max(1) as f64 + 1.0).ln();
    for i in 0..=7 {
        let t = i as f64 / 7.0;
        let w = ((t * max_ln).exp() - 1.0) as i64;
        draw_rectangle(lx, ly, 14.0, 14.0, wealth_color(w, total_wealth));
        draw_text(&fmt_wealth(w), lx + 20.0, ly + 13.0, fonts.legend_value, WHITE);
        ly += 18.0;
    }

    // speed presets
    ly += 12.0;
    draw_text("Vitesse", lx, ly, fonts.section_title, WHITE);
    ly += 22.0;

    let (mx, my)  = mouse_position();
    let pressed   = is_mouse_button_pressed(MouseButton::Left);

    let mut speed_clicked = None;
    for (i, (_, label)) in speeds.iter().enumerate() {
        let active = i == speed_idx && custom.mult.is_none();
        if draw_button(lx, ly, btn_w, btn_h, label, active, fonts.button_text) {
            speed_clicked = Some(i);
            custom.mult    = None;
            custom.focused = false;
            custom.buf     = String::new();
        }
        ly += btn_h + 4.0;
    }

    // custom speed input
    ly += 4.0;
    draw_text("Custom (x1–x5000)", lx, ly + fonts.legend_value, fonts.legend_value,
              Color::new(0.7, 0.7, 0.7, 1.0));
    ly += fonts.legend_value + 4.0;

    let box_h = btn_h;
    let over_box = mx >= lx && mx <= lx + btn_w && my >= ly && my <= ly + box_h;

    // focus / unfocus on click
    if pressed {
        if over_box && !custom.focused {
            custom.focused = true;
            custom.buf = custom.mult.map(|m| m.to_string()).unwrap_or_default();
        } else if !over_box && custom.focused {
            // commit on click outside
            if let Ok(v) = custom.buf.parse::<u32>() {
                custom.mult = Some(v.clamp(1, 5000));
            }
            custom.focused = false;
        }
    }

    if custom.focused {
        while let Some(c) = get_char_pressed() {
            if c.is_ascii_digit() && custom.buf.len() < 4 { custom.buf.push(c); }
        }
        if is_key_pressed(KeyCode::Backspace) { custom.buf.pop(); }
        if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::KpEnter) {
            if let Ok(v) = custom.buf.parse::<u32>() {
                custom.mult = Some(v.clamp(1, 5000));
            }
            custom.focused = false;
        }
        if is_key_pressed(KeyCode::Escape) {
            custom.focused = false;
            custom.buf = String::new();
        }
    }

    let is_active = custom.mult.is_some() && !custom.focused;
    let bg = if custom.focused  { Color::new(0.08, 0.18, 0.35, 1.0) }
             else if is_active  { Color::new(0.05, 0.25, 0.10, 1.0) }
             else                { Color::new(0.20, 0.20, 0.28, 1.0) };
    let border = if custom.focused { Color::new(0.30, 0.60, 1.00, 1.0) }
                 else if is_active  { Color::new(0.20, 0.80, 0.30, 1.0) }
                 else                { Color::new(0.60, 0.60, 0.70, 1.0) };
    draw_rectangle(lx, ly, btn_w, box_h, bg);
    draw_rectangle_lines(lx, ly, btn_w, box_h, 1.5, border);

    let text_col = if is_active { Color::new(0.2, 0.9, 0.4, 1.0) } else { WHITE };
    let display = if custom.focused {
        let blink = (get_time() * 2.0) as i32 % 2 == 0;
        if blink { format!("x{}|", custom.buf) } else { format!("x{}", custom.buf) }
    } else if let Some(m) = custom.mult {
        format!("x{}", m)
    } else {
        "x?".to_string()
    };
    draw_text(&display, lx + 6.0, ly + box_h * 0.72, fonts.button_text, text_col);

    ly += box_h + 10.0;
    let to_chart  = draw_button(lx, ly, btn_w, btn_h, "Graphique >", false, fonts.button_text);
    ly += btn_h + 5.0;
    let to_config = draw_button(lx, ly, btn_w, btn_h, "Config...",   false, fonts.button_text);

    // tooltip drawn last — always on top of legend and panel
    if let Some(idx) = selected {
        if let Some(agent) = agents.get(idx) {
            let cx = OFFSET + agent.pos.0 as f32 * CELL_SIZE + CELL_SIZE / 2.0;
            let cy = OFFSET + agent.pos.1 as f32 * CELL_SIZE + CELL_SIZE / 2.0;

            let has_cap = agent.capital > 0;
            let tw = 120.0;
            let th = if has_cap { 75.0 } else { 60.0 };
            let tx = (cx + CELL_SIZE + 2.0).min(screen_width() - tw - 4.0);
            let ty = (cy - th / 2.0).clamp(OFFSET, OFFSET + GRID_H as f32 * CELL_SIZE - th);

            draw_rectangle(tx, ty, tw, th, Color::new(0.08, 0.08, 0.14, 0.95));
            draw_rectangle_lines(tx, ty, tw, th, 1.0, Color::new(0.5, 0.5, 0.7, 1.0));

            let sign      = if agent.bonus >= 0 { "+" } else { "" };
            let total_pat = agent.wealth + agent.capital;
            draw_text(&format!("Agent #{}", agent.id),             tx + 6.0, ty + 16.0, tooltip_font, WHITE);
            draw_text(&format!("Bonus: {}{}%", sign, agent.bonus), tx + 6.0, ty + 33.0, tooltip_font, bonus_color(agent.bonus));
            if has_cap {
                let cc_txt = if agent.wealth <= 0 { "mort".to_string() } else { fmt_wealth(agent.wealth) };
                draw_text(&format!("CC: {}", cc_txt),                    tx + 6.0, ty + 50.0, tooltip_font, WHITE);
                draw_text(&format!("Cap: {}", fmt_wealth(agent.capital)), tx + 6.0, ty + 67.0, tooltip_font, Color::new(1.0, 0.80, 0.20, 1.0));
            } else {
                let wealth_txt = if total_pat <= 0 { "mort".to_string() } else { fmt_wealth(total_pat) };
                draw_text(&wealth_txt, tx + 6.0, ty + 50.0, tooltip_font, wealth_color(total_pat.max(1), total_wealth));
            }
        }
    }

    (speed_clicked, to_chart, to_config)
}
