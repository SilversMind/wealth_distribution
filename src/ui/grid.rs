use macroquad::prelude::*;

use crate::agent::{Agent, wealth_color, bonus_color};
use crate::config::FontSizes;
use crate::constants::*;
use crate::i18n::Translations;
use crate::layout::Layout;
use super::{draw_button, fmt_wealth};

pub struct CustomSpeedState {
    pub buf:     String,
    pub focused: bool,
    pub mult:    Option<u32>,
}

impl Default for CustomSpeedState {
    fn default() -> Self {
        CustomSpeedState { buf: String::new(), focused: false, mult: None }
    }
}

impl CustomSpeedState {
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
    tr:           &Translations,
    lay:          &Layout,
) -> (Option<usize>, bool, bool) {
    let cs = lay.cell_size;
    let gx = lay.grid_x;
    let gy = lay.grid_y;

    clear_background(Color::new(0.06, 0.06, 0.08, 1.0));

    // grid lines
    let grid_color = Color::new(0.15, 0.15, 0.2, 1.0);
    for x in 0..=GRID_W {
        let px = gx + x as f32 * cs;
        draw_line(px, gy, px, gy + GRID_H as f32 * cs, 0.5, grid_color);
    }
    for y in 0..=GRID_H {
        let py = gy + y as f32 * cs;
        draw_line(gx, py, gx + GRID_W as f32 * cs, py, 0.5, grid_color);
    }

    // agents
    for (i, agent) in agents.iter().enumerate() {
        let cx = gx + agent.pos.0 as f32 * cs + cs / 2.0;
        let cy = gy + agent.pos.1 as f32 * cs + cs / 2.0;
        let r  = (cs / 2.0 - 1.0).max(1.0);

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

    let hud = tr.tf("hud_tick", &[
        ("tick", &tick_count.to_string()),
        ("dev",  &deviance.to_string()),
        ("pct",  &transfer_pct.to_string()),
    ]);

    let max_ln = (total_wealth.max(1) as f64 + 1.0).ln();
    let mut speed_clicked = None;

    let (to_chart, to_config) = if lay.is_mobile {
        draw_mobile_panel(&hud, speeds, speed_idx, fonts, tr, custom, max_ln, total_wealth, lay, &mut speed_clicked)
    } else {
        draw_desktop_panel(&hud, speeds, speed_idx, fonts, tr, custom, max_ln, total_wealth, lay, &mut speed_clicked)
    };

    // tooltip — drawn last, on top of everything
    if let Some(idx) = selected {
        if let Some(agent) = agents.get(idx) {
            let cx = gx + agent.pos.0 as f32 * cs + cs / 2.0;
            let cy = gy + agent.pos.1 as f32 * cs + cs / 2.0;

            let has_cap = agent.capital > 0;
            let tw = 120.0;
            let th = if has_cap { 75.0 } else { 60.0 };
            let tx = (cx + cs + 2.0).min(screen_width() - tw - 4.0);
            let ty = (cy - th / 2.0).clamp(gy, gy + GRID_H as f32 * cs - th);

            draw_rectangle(tx, ty, tw, th, Color::new(0.08, 0.08, 0.14, 0.95));
            draw_rectangle_lines(tx, ty, tw, th, 1.0, Color::new(0.5, 0.5, 0.7, 1.0));

            let sign      = if agent.bonus >= 0 { "+" } else { "" };
            let total_pat = agent.wealth + agent.capital;
            let agent_line = tr.tf("tooltip_agent", &[("id", &agent.id.to_string())]);
            let bonus_line = tr.tf("tooltip_bonus", &[("sign", sign), ("bonus", &agent.bonus.to_string())]);
            draw_text(&agent_line,  tx + 6.0, ty + 16.0, tooltip_font, WHITE);
            draw_text(&bonus_line,  tx + 6.0, ty + 33.0, tooltip_font, bonus_color(agent.bonus));
            if has_cap {
                let cc_val   = if agent.wealth <= 0 { tr.t("tooltip_dead").to_string() } else { fmt_wealth(agent.wealth) };
                let cc_line  = tr.tf("tooltip_cc",      &[("val", &cc_val)]);
                let cap_line = tr.tf("tooltip_capital", &[("val", &fmt_wealth(agent.capital))]);
                draw_text(&cc_line,  tx + 6.0, ty + 50.0, tooltip_font, WHITE);
                draw_text(&cap_line, tx + 6.0, ty + 67.0, tooltip_font, Color::new(1.0, 0.80, 0.20, 1.0));
            } else {
                let w_val = if total_pat <= 0 { tr.t("tooltip_dead").to_string() } else { fmt_wealth(total_pat) };
                draw_text(&w_val, tx + 6.0, ty + 50.0, tooltip_font, wealth_color(total_pat.max(1), total_wealth));
            }
        }
    }

    (speed_clicked, to_chart, to_config)
}

// ── Desktop panel (right side) ────────────────────────────────────────────────

#[allow(clippy::too_many_arguments)]
fn draw_desktop_panel(
    hud:           &str,
    speeds:        &[(f64, &str)],
    speed_idx:     usize,
    fonts:         &FontSizes,
    tr:            &Translations,
    custom:        &mut CustomSpeedState,
    max_ln:        f64,
    total_wealth:  i64,
    lay:           &Layout,
    speed_clicked: &mut Option<usize>,
) -> (bool, bool) {
    let cs = lay.cell_size;
    let gx = lay.grid_x;
    let gy = lay.grid_y;

    draw_text(hud, 10.0, 20.0, fonts.main_title, WHITE);

    let lx    = gx + GRID_W as f32 * cs + 15.0;
    let mut ly = gy;
    let btn_w = lay.panel_w - 10.0;
    let btn_h = 28.0;

    // wealth legend
    draw_text(tr.t("legend_wealth"), lx, ly, fonts.section_title, WHITE);
    ly += 22.0;
    for i in 0..=7 {
        let t = i as f64 / 7.0;
        let w = ((t * max_ln).exp() - 1.0) as i64;
        draw_rectangle(lx, ly, 14.0, 14.0, wealth_color(w, total_wealth));
        draw_text(&fmt_wealth(w), lx + 20.0, ly + 13.0, fonts.legend_value, WHITE);
        ly += 18.0;
    }

    // speed presets
    ly += 12.0;
    draw_text(tr.t("legend_speed"), lx, ly, fonts.section_title, WHITE);
    ly += 22.0;

    let (mx, my) = mouse_position();
    let pressed  = is_mouse_button_pressed(MouseButton::Left);

    for (i, (_, label)) in speeds.iter().enumerate() {
        let active = i == speed_idx && custom.mult.is_none();
        if draw_button(lx, ly, btn_w, btn_h, label, active, fonts.button_text) {
            *speed_clicked = Some(i);
            custom.mult    = None;
            custom.focused = false;
            custom.buf     = String::new();
        }
        ly += btn_h + 4.0;
    }

    // custom speed input
    ly += 4.0;
    draw_text(tr.t("legend_custom"), lx, ly + fonts.legend_value, fonts.legend_value,
              Color::new(0.7, 0.7, 0.7, 1.0));
    ly += fonts.legend_value + 4.0;

    let box_h    = btn_h;
    let over_box = mx >= lx && mx <= lx + btn_w && my >= ly && my <= ly + box_h;

    if pressed {
        if over_box && !custom.focused {
            custom.focused = true;
            custom.buf = custom.mult.map(|m| m.to_string()).unwrap_or_default();
        } else if !over_box && custom.focused {
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
    let to_chart  = draw_button(lx, ly, btn_w, btn_h, tr.t("btn_chart"),  false, fonts.button_text);
    ly += btn_h + 5.0;
    let to_config = draw_button(lx, ly, btn_w, btn_h, tr.t("btn_config"), false, fonts.button_text);

    (to_chart, to_config)
}

// ── Mobile panel (fixed bottom toolbar) ──────────────────────────────────────

#[allow(clippy::too_many_arguments)]
fn draw_mobile_panel(
    hud:           &str,
    speeds:        &[(f64, &str)],
    speed_idx:     usize,
    fonts:         &FontSizes,
    tr:            &Translations,
    custom:        &mut CustomSpeedState,
    max_ln:        f64,
    total_wealth:  i64,
    lay:           &Layout,
    speed_clicked: &mut Option<usize>,
) -> (bool, bool) {
    let sw        = screen_width();
    let sh        = screen_height();
    let dpr       = lay.dpr;
    let margin    = (sw - lay.panel_w) / 2.0;
    let panel_w   = lay.panel_w;
    let btn_h     = 36.0 * dpr;
    let gap       = 4.0 * dpr;
    let toolbar_y = sh - lay.toolbar_h;
    let fv        = fonts.legend_value * dpr;
    let fb        = fonts.button_text  * dpr;

    // toolbar background + separator
    draw_rectangle(0.0, toolbar_y, sw, lay.toolbar_h, Color::new(0.08, 0.08, 0.12, 1.0));
    draw_line(0.0, toolbar_y, sw, toolbar_y, 1.0, Color::new(0.25, 0.25, 0.35, 1.0));

    let mut py = toolbar_y + 6.0 * dpr;

    // Row 1: HUD left, wealth gradient bar right
    let hud_share = panel_w * 0.45;
    draw_text(hud, margin, py + fv * 0.8, fv, Color::new(0.7, 0.7, 0.7, 1.0));
    let bar_x = margin + hud_share + 8.0 * dpr;
    let bar_w = panel_w - hud_share - 8.0 * dpr;
    let n     = 24usize;
    let sw_w  = bar_w / n as f32;
    for i in 0..n {
        let t = i as f64 / (n - 1) as f64;
        let w = ((t * max_ln).exp() - 1.0) as i64;
        draw_rectangle(bar_x + i as f32 * sw_w, py, sw_w, fv, wealth_color(w, total_wealth));
    }
    py += fv + 6.0 * dpr;

    // Row 2: speed buttons in one horizontal row
    let n_speeds = speeds.len() as f32;
    let speed_w  = (panel_w - (n_speeds - 1.0) * gap) / n_speeds;
    for (i, (_, label)) in speeds.iter().enumerate() {
        let bx     = margin + i as f32 * (speed_w + gap);
        let active = i == speed_idx && custom.mult.is_none();
        if draw_button(bx, py, speed_w, btn_h, label, active, fb) {
            *speed_clicked = Some(i);
            custom.mult    = None;
        }
    }
    py += btn_h + gap;

    // Row 3: Chart + Config side by side
    let nav_w     = (panel_w - gap) / 2.0;
    let to_chart  = draw_button(margin,              py, nav_w, btn_h, tr.t("btn_chart"),  false, fb);
    let to_config = draw_button(margin + nav_w + gap, py, nav_w, btn_h, tr.t("btn_config"), false, fb);

    (to_chart, to_config)
}
