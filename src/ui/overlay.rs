use macroquad::prelude::*;
use crate::config::FontSizes;
use crate::i18n::Translations;

pub fn draw_monopoly_overlay(winner_id: usize, color: Color, tick_count: u64, fonts: &FontSizes, tr: &Translations) {
    let sw = screen_width();
    let sh = screen_height();
    draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.55));

    let msg1 = tr.t("overlay_title");
    let msg2 = tr.tf("overlay_winner", &[("id", &winner_id.to_string())]);
    let msg3 = tr.tf("overlay_ticks",  &[("tick", &tick_count.to_string())]);

    let f1 = fonts.main_title * 1.6;
    let f2 = fonts.main_title * 1.1;
    let f3 = fonts.main_title * 0.9;

    let y0 = sh / 2.0;
    draw_text(msg1,  sw / 2.0 - msg1.len() as f32 * f1 * 0.25, y0 - f1 * 0.5,        f1, color);
    draw_text(&msg2, sw / 2.0 - msg2.len() as f32 * f2 * 0.25, y0 + f2 * 0.8,        f2, WHITE);
    draw_text(&msg3, sw / 2.0 - msg3.len() as f32 * f3 * 0.25, y0 + f2 * 0.8 + f3 + 4.0, f3, Color::new(0.7, 0.7, 0.7, 1.0));
}
