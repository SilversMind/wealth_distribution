use macroquad::prelude::*;
use crate::config::FontSizes;
use crate::i18n::Translations;
use crate::layout::Layout;
use super::panel::{Panel, PanelItem};

const BG:     Color = Color { r: 0.08, g: 0.08, b: 0.14, a: 0.97 };
const BORDER: Color = Color { r: 0.40, g: 0.40, b: 0.60, a: 1.00 };
const GRAY:   Color = Color { r: 0.75, g: 0.75, b: 0.75, a: 1.00 };

fn full_dim(alpha: f32) {
    let sw = screen_width();
    let sh = screen_height();
    draw_rectangle(0.0, 0.0, sw, sh, Color { r: 0.0, g: 0.0, b: 0.0, a: alpha });
}

fn panel_max_w(lay: &Layout) -> f32 {
    let sw = screen_width();
    (520.0 * lay.dpr).min(sw - 40.0 * lay.dpr)
}

pub fn draw_intro_overlay(fonts: &FontSizes, tr: &Translations, lay: &Layout) -> bool {
    let sw = screen_width();
    let sh = screen_height();
    full_dim(0.75);

    let items = vec![
        PanelItem::Title  { text: tr.t("intro_title"), size: fonts.main_title,    color: WHITE },
        PanelItem::Gap    { px: 4.0 * lay.dpr },
        PanelItem::Body   { text: tr.t("intro_line1"), size: fonts.section_title, color: GRAY },
        PanelItem::Body   { text: tr.t("intro_line2"), size: fonts.section_title, color: GRAY },
        PanelItem::Body   { text: tr.t("intro_line3"), size: fonts.section_title, color: GRAY },
        PanelItem::Button { label: tr.t("intro_btn"),  size: fonts.button_text },
    ];

    Panel::build(items, panel_max_w(lay), 20.0 * lay.dpr, lay.dpr, BG, BORDER)
        .centered_on(sw / 2.0, sh / 2.0)
        .draw()
        .is_some()
}

pub fn draw_chart_intro_overlay(fonts: &FontSizes, tr: &Translations, lay: &Layout) -> bool {
    let sw = screen_width();
    let sh = screen_height();
    full_dim(0.75);

    let items = vec![
        PanelItem::Title  { text: tr.t("chart_intro_title"), size: fonts.main_title,    color: WHITE },
        PanelItem::Gap    { px: 4.0 * lay.dpr },
        PanelItem::Body   { text: tr.t("chart_intro_body1"), size: fonts.section_title, color: GRAY },
        PanelItem::Gap    { px: 4.0 * lay.dpr },
        PanelItem::Body   { text: tr.t("chart_intro_body2"), size: fonts.section_title, color: GRAY },
        PanelItem::Gap    { px: 4.0 * lay.dpr },
        PanelItem::Body   { text: tr.t("chart_intro_body3"), size: fonts.section_title, color: GRAY },
        PanelItem::Button { label: tr.t("chart_intro_btn"),  size: fonts.button_text },
    ];

    Panel::build(items, panel_max_w(lay), 20.0 * lay.dpr, lay.dpr, BG, BORDER)
        .centered_on(sw / 2.0, sh / 2.0)
        .draw()
        .is_some()
}

pub fn draw_monopoly_overlay(
    winner_id:  usize,
    color:      Color,
    tick_count: u64,
    fonts:      &FontSizes,
    tr:         &Translations,
    lay:        &Layout,
) -> bool {
    let sw   = screen_width();
    let sh   = screen_height();
    full_dim(0.55);

    let msg2 = tr.tf("overlay_winner", &[("id",   &winner_id.to_string())]);
    let msg3 = tr.tf("overlay_ticks",  &[("tick", &tick_count.to_string())]);

    let items = vec![
        PanelItem::Title  { text: tr.t("overlay_title"), size: fonts.main_title * 1.5, color },
        PanelItem::Gap    { px: 8.0 * lay.dpr },
        PanelItem::Body   { text: &msg2, size: fonts.main_title,    color: WHITE },
        PanelItem::Body   { text: &msg3, size: fonts.section_title, color: Color { r: 0.7, g: 0.7, b: 0.7, a: 1.0 } },
        PanelItem::Button { label: tr.t("btn_new_sim"), size: fonts.button_text },
    ];

    Panel::build(items, panel_max_w(lay), 20.0 * lay.dpr, lay.dpr, BG, BORDER)
        .centered_on(sw / 2.0, sh / 2.0)
        .draw()
        .is_some()
}
