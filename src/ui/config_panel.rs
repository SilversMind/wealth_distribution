use macroquad::prelude::*;

use crate::config::{SimConfig, FontSizes};
use crate::i18n::Translations;
use super::{draw_button, fmt_wealth};

pub struct ConfigPanelState {
    held:       Option<(usize, i32)>,
    held_since: f64,
    last_fire:  f64,
    focused:    Option<usize>,
    input_buf:  String,
    pub active_tab: usize,
}

impl Default for ConfigPanelState {
    fn default() -> Self {
        ConfigPanelState {
            held: None, held_since: 0.0, last_fire: 0.0,
            focused: None, input_buf: String::new(),
            active_tab: 0,
        }
    }
}

impl ConfigPanelState {
    pub fn reset(&mut self) { *self = Self::default(); }

    fn clear_focus(&mut self) {
        self.focused  = None;
        self.input_buf.clear();
        self.held     = None;
    }
}

fn fmt_param(v: i32, fmt_flag: u8) -> String {
    match fmt_flag {
        1 => fmt_wealth(v as i64),
        2 => format!("{}%", v),
        3 => format!("{}px", v),
        _ => v.to_string(),
    }
}

// Returns (apply, cancel, lang_change)
pub fn draw_config_view(
    edit:  &mut SimConfig,
    fonts: &FontSizes,
    cs:    &mut ConfigPanelState,
    tr:    &Translations,
) -> (bool, bool, Option<String>) {
    let sw = screen_width();
    let sh = screen_height();
    clear_background(Color::new(0.06, 0.06, 0.08, 1.0));

    let row_h  = fonts.section_title + 34.0;
    let btn_sz = fonts.section_title + 12.0;
    let val_w  = 88.0;
    let btn_h  = btn_sz;

    let sec_hdr_h = fonts.section_title + 14.0;
    let sec_sel_h = btn_h + 8.0;
    let sec_h     = sec_hdr_h + sec_sel_h;

    let n_tax_rows = if edit.tax_type > 0        { 2.0f32 } else { 0.0 };
    let n_cap_rows = if edit.capital_enabled > 0  { 4.0f32 } else { 0.0 };

    let tab_h    = btn_sz + 12.0;
    let header_h = fonts.main_title + 20.0;

    let content_h = match cs.active_tab {
        0 => 5.0 * row_h,
        1 => sec_h + n_tax_rows * row_h,
        _ => sec_h + n_cap_rows * row_h,
    };

    let panel_w = 440.0;
    let panel_h = header_h + tab_h + content_h + 70.0;
    let px      = (sw - panel_w) / 2.0;
    let py      = (sh - panel_h) / 2.0;

    draw_rectangle(px, py, panel_w, panel_h, Color::new(0.1, 0.1, 0.15, 1.0));
    draw_rectangle_lines(px, py, panel_w, panel_h, 2.0, Color::new(0.4, 0.4, 0.55, 1.0));

    // header: title + FR/EN toggle
    draw_text(tr.t("config_title"), px + 14.0, py + fonts.main_title + 8.0, fonts.main_title, WHITE);

    let lang_btn_w = 30.0;
    let lang_btn_h = fonts.main_title + 2.0;
    let lang_y     = py + 6.0;
    let en_x       = px + panel_w - 14.0 - lang_btn_w;
    let fr_x       = en_x - lang_btn_w - 4.0;

    let mut lang_change: Option<String> = None;
    if draw_button(fr_x, lang_y, lang_btn_w, lang_btn_h, tr.t("lang_fr"), tr.lang == "fr", fonts.legend_value) {
        if tr.lang != "fr" { lang_change = Some("fr".to_string()); }
    }
    if draw_button(en_x, lang_y, lang_btn_w, lang_btn_h, tr.t("lang_en"), tr.lang == "en", fonts.legend_value) {
        if tr.lang != "en" { lang_change = Some("en".to_string()); }
    }

    let sep_y = py + fonts.main_title + 14.0;
    draw_line(px + 14.0, sep_y, px + panel_w - 14.0, sep_y, 1.0, Color::new(0.3, 0.3, 0.4, 1.0));

    // tab bar
    let tabs = [tr.t("tab_general"), tr.t("tab_tax"), tr.t("tab_capital")];
    let tab_y     = sep_y + 6.0;
    let tab_btn_w = (panel_w - 28.0 - (tabs.len() as f32 - 1.0) * 6.0) / tabs.len() as f32;
    for (ti, tab_label) in tabs.iter().enumerate() {
        let tx = px + 14.0 + ti as f32 * (tab_btn_w + 6.0);
        if draw_button(tx, tab_y, tab_btn_w, btn_sz, tab_label, cs.active_tab == ti, fonts.button_text) {
            if cs.active_tab != ti {
                cs.active_tab = ti;
                cs.clear_focus();
            }
        }
    }

    let content_start = tab_y + btn_sz + 8.0;
    let row_x = px + 14.0;
    let val_x = px + panel_w - 14.0 - btn_sz - val_w - btn_sz - 8.0;
    let num_x = val_x + btn_sz + 4.0;

    let (mx, my)  = mouse_position();
    let pressed   = is_mouse_button_pressed(MouseButton::Left);
    let held_down = is_mouse_button_down(MouseButton::Left);
    let now       = get_time();

    if !held_down { cs.held = None; }

    match cs.active_tab {
        // ── General ──────────────────────────────────────────────────────────
        0 => {
            let param_ry = |i: usize| -> f32 { content_start + i as f32 * row_h };

            let clicked_box: Option<usize> = if pressed {
                (0..5usize).find(|&j| {
                    let ry = param_ry(j);
                    mx >= num_x && mx <= num_x + val_w && my >= ry && my <= ry + btn_sz
                })
            } else { None };

            if pressed && cs.focused.is_some() && clicked_box != cs.focused {
                cs.focused = None; cs.input_buf = String::new();
            }

            let mut num_agents_i32 = edit.num_agents as i32;
            {
                let base: &mut [(&str, &mut i32, i32, i32, i32, u8)] = &mut [
                    (tr.t("param_deviance"),    &mut edit.deviance,     0,    100,               1, 0),
                    (tr.t("param_agents"),       &mut num_agents_i32,    1,   1000,              10, 0),
                    (tr.t("param_init_wealth"),  &mut edit.init_wealth,  1_000, 1_000_000_000, 1_000, 1),
                    (tr.t("param_transfer"),     &mut edit.transfer_pct, 1,    100,               1, 2),
                    (tr.t("param_font"),         &mut edit.label_font,   6,     40,               1, 3),
                ];
                draw_param_rows(base, 0, &param_ry, &mut num_x.clone(), val_x, btn_sz, val_w, btn_h,
                                fonts, cs, mx, my, pressed, held_down, now, clicked_box, row_x);
            }
            edit.num_agents = num_agents_i32 as usize;
        }

        // ── Tax ───────────────────────────────────────────────────────────────
        1 => {
            draw_section_header(px, content_start, panel_w, tr.t("section_tax_type"), fonts);

            let tax_types = [tr.t("opt_none"), tr.t("opt_income")];
            let type_y = content_start + sec_hdr_h;
            let type_w = (panel_w - 28.0 - (tax_types.len() as f32 - 1.0) * 8.0) / tax_types.len() as f32;
            for (ti, label) in tax_types.iter().enumerate() {
                let bx = px + 14.0 + ti as f32 * (type_w + 8.0);
                if draw_button(bx, type_y, type_w, btn_h, label, edit.tax_type == ti as i32, fonts.button_text) {
                    if edit.tax_type != ti as i32 {
                        edit.tax_type = ti as i32;
                        cs.clear_focus();
                    }
                }
            }

            if edit.tax_type > 0 {
                let param_ry = |i: usize| -> f32 { content_start + sec_h + i as f32 * row_h };
                let clicked_box = find_clicked_box(pressed, 2, &param_ry, num_x, val_w, btn_sz, mx, my);
                if pressed && cs.focused.is_some() && clicked_box != cs.focused {
                    cs.focused = None; cs.input_buf = String::new();
                }
                let tax: &mut [(&str, &mut i32, i32, i32, i32, u8)] = &mut [
                    (tr.t("param_tax_rate"), &mut edit.tax_rate, 0,   50,   1, 2),
                    (tr.t("param_tax_freq"), &mut edit.tax_freq, 1, 1000,  10, 0),
                ];
                draw_param_rows(tax, 0, &param_ry, &mut num_x.clone(), val_x, btn_sz, val_w, btn_h,
                                fonts, cs, mx, my, pressed, held_down, now, clicked_box, row_x);
            }
        }

        // ── Capital ───────────────────────────────────────────────────────────
        _ => {
            draw_section_header(px, content_start, panel_w, tr.t("section_capital"), fonts);

            let cap_modes = [tr.t("opt_disabled"), tr.t("opt_enabled")];
            let mode_y = content_start + sec_hdr_h;
            let mode_w = (panel_w - 28.0 - (cap_modes.len() as f32 - 1.0) * 8.0) / cap_modes.len() as f32;
            for (mi, label) in cap_modes.iter().enumerate() {
                let bx = px + 14.0 + mi as f32 * (mode_w + 8.0);
                if draw_button(bx, mode_y, mode_w, btn_h, label, edit.capital_enabled == mi as i32, fonts.button_text) {
                    if edit.capital_enabled != mi as i32 {
                        edit.capital_enabled = mi as i32;
                        cs.clear_focus();
                    }
                }
            }

            if edit.capital_enabled > 0 {
                let param_ry = |i: usize| -> f32 { content_start + sec_h + i as f32 * row_h };
                let clicked_box = find_clicked_box(pressed, 4, &param_ry, num_x, val_w, btn_sz, mx, my);
                if pressed && cs.focused.is_some() && clicked_box != cs.focused {
                    cs.focused = None; cs.input_buf = String::new();
                }
                let cap: &mut [(&str, &mut i32, i32, i32, i32, u8)] = &mut [
                    (tr.t("param_seuil_a"),  &mut edit.seuil_a_pct,   50, 1000,  10, 2),
                    (tr.t("param_seuil_b"),  &mut edit.seuil_b_pct,    0,  200,   5, 2),
                    (tr.t("param_cap_rate"), &mut edit.capital_rate,   0,   20,   1, 2),
                    (tr.t("param_cap_freq"), &mut edit.capital_freq,   1, 5000,  10, 0),
                ];
                draw_param_rows(cap, 0, &param_ry, &mut num_x.clone(), val_x, btn_sz, val_w, btn_h,
                                fonts, cs, mx, my, pressed, held_down, now, clicked_box, row_x);
            }
        }
    }

    let footer_y = panel_h + py - 48.0;
    let apply = draw_button(px + 14.0,                    footer_y, 200.0, 36.0, tr.t("btn_apply"),  false, fonts.button_text);
    let back  = draw_button(px + panel_w - 14.0 - 110.0, footer_y, 110.0, 36.0, tr.t("btn_cancel"), false, fonts.button_text);
    (apply, back, lang_change)
}

fn draw_section_header(px: f32, content_start: f32, panel_w: f32, title: &str, fonts: &FontSizes) {
    draw_line(px + 14.0, content_start, px + panel_w - 14.0, content_start,
              1.0, Color::new(0.3, 0.3, 0.4, 1.0));
    draw_text(title, px + 14.0, content_start + fonts.section_title + 2.0,
              fonts.section_title, Color::new(0.75, 0.75, 0.85, 1.0));
}

fn find_clicked_box(
    pressed:  bool,
    n:        usize,
    param_ry: &dyn Fn(usize) -> f32,
    num_x:    f32,
    val_w:    f32,
    btn_sz:   f32,
    mx:       f32,
    my:       f32,
) -> Option<usize> {
    if !pressed { return None; }
    (0..n).find(|&j| {
        let ry = param_ry(j);
        mx >= num_x && mx <= num_x + val_w && my >= ry && my <= ry + btn_sz
    })
}

#[allow(clippy::too_many_arguments)]
fn draw_param_rows(
    params:       &mut [(&str, &mut i32, i32, i32, i32, u8)],
    index_offset: usize,
    param_ry:     &dyn Fn(usize) -> f32,
    _num_x:       &mut f32,
    val_x:        f32,
    btn_sz:       f32,
    val_w:        f32,
    btn_h:        f32,
    fonts:        &FontSizes,
    cs:           &mut ConfigPanelState,
    mx:           f32,
    my:           f32,
    pressed:      bool,
    held_down:    bool,
    now:          f64,
    clicked_box:  Option<usize>,
    row_x:        f32,
) {
    let num_x  = val_x + btn_sz + 4.0;
    let plus_x = num_x + val_w + 4.0;

    for (j, (label, value, min, max, step, fmt_flag)) in params.iter_mut().enumerate() {
        let i          = index_offset + j;
        let ry         = param_ry(i);
        let is_focused = cs.focused == Some(i);

        if is_focused {
            while let Some(c) = get_char_pressed() {
                if c.is_ascii_digit() { cs.input_buf.push(c); }
            }
            if is_key_pressed(KeyCode::Backspace) { cs.input_buf.pop(); }
            let commit = is_key_pressed(KeyCode::Enter)
                || is_key_pressed(KeyCode::KpEnter)
                || is_key_pressed(KeyCode::Tab);
            let cancel = is_key_pressed(KeyCode::Escape);
            if commit || cancel {
                if commit {
                    if let Ok(v) = cs.input_buf.parse::<i32>() {
                        **value = v.clamp(*min, *max);
                    }
                }
                cs.focused   = None;
                cs.input_buf = String::new();
            }
        }

        if clicked_box == Some(i) && !is_focused {
            cs.focused   = Some(i);
            cs.input_buf = value.to_string();
        }

        draw_text(label, row_x, ry + fonts.section_title, fonts.section_title,
                  Color::new(0.85, 0.85, 0.85, 1.0));

        if cs.focused.is_none() {
            let over_minus = mx >= val_x  && mx <= val_x + btn_sz  && my >= ry && my <= ry + btn_sz;
            let over_plus  = mx >= plus_x && mx <= plus_x + btn_sz && my >= ry && my <= ry + btn_sz;

            if pressed {
                if over_minus && **value - *step >= *min {
                    **value -= *step;
                    cs.held = Some((i, -1)); cs.held_since = now; cs.last_fire = now;
                }
                if over_plus && **value + *step <= *max {
                    **value += *step;
                    cs.held = Some((i, 1)); cs.held_since = now; cs.last_fire = now;
                }
            }
            if held_down {
                if let Some((hi, dir)) = cs.held {
                    if hi == i {
                        let elapsed = now - cs.held_since;
                        if elapsed >= 0.4 {
                            let interval = (0.12 - (elapsed - 0.4) * 0.05).max(0.02);
                            if now - cs.last_fire >= interval {
                                cs.last_fire = now;
                                if dir < 0 && **value - *step >= *min { **value -= *step; }
                                if dir > 0 && **value + *step <= *max { **value += *step; }
                            }
                        }
                    }
                }
            }

            let is_held_minus = cs.held == Some((i, -1)) && held_down;
            let is_held_plus  = cs.held == Some((i,  1)) && held_down;
            draw_button(val_x,  ry, btn_sz, btn_h, "-", is_held_minus, fonts.section_title);
            draw_button(plus_x, ry, btn_sz, btn_h, "+", is_held_plus,  fonts.section_title);
        } else {
            let dim = Color::new(0.2, 0.2, 0.25, 1.0);
            let txt = Color::new(0.4, 0.4, 0.4, 1.0);
            for bx in [val_x, plus_x] {
                draw_rectangle(bx, ry, btn_sz, btn_h, dim);
                draw_rectangle_lines(bx, ry, btn_sz, btn_h, 1.0, txt);
            }
            draw_text("-", val_x  + btn_sz * 0.3, ry + btn_h * 0.72, fonts.section_title, txt);
            draw_text("+", plus_x + btn_sz * 0.3, ry + btn_h * 0.72, fonts.section_title, txt);
        }

        if is_focused {
            let blink   = (get_time() * 2.0) as i32 % 2 == 0;
            let display = if blink { format!("{}|", cs.input_buf) } else { cs.input_buf.clone() };
            draw_rectangle(num_x, ry, val_w, btn_h, Color::new(0.08, 0.18, 0.35, 1.0));
            draw_rectangle_lines(num_x, ry, val_w, btn_h, 1.5, Color::new(0.3, 0.6, 1.0, 1.0));
            let tw = display.len() as f32 * fonts.section_title * 0.5;
            draw_text(&display, (num_x + val_w / 2.0 - tw / 2.0).max(num_x + 3.0),
                      ry + btn_h * 0.72, fonts.section_title, WHITE);
        } else {
            draw_rectangle(num_x, ry, val_w, btn_h, Color::new(0.15, 0.15, 0.2, 1.0));
            draw_rectangle_lines(num_x, ry, val_w, btn_h, 1.0, Color::new(0.4, 0.4, 0.5, 1.0));
            let val_str = fmt_param(**value, *fmt_flag);
            let tw = val_str.len() as f32 * fonts.section_title * 0.5;
            draw_text(&val_str, num_x + val_w / 2.0 - tw / 2.0,
                      ry + btn_h * 0.72, fonts.section_title, WHITE);
        }
    }
}
