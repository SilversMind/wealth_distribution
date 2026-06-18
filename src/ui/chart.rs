use macroquad::prelude::*;

use crate::agent::color_ramp;
use crate::config::FontSizes;
use crate::simulation::{N_PCT, PERCENTILE_RANKS};
use super::{draw_button, fmt_wealth};

const LABELS: &[&str] = &["P10", "P20", "P30", "P40", "P50", "P60", "P70", "P80", "P90", "P99"];

pub fn draw_chart_view(
    pct_history:      &[[i64; N_PCT]],
    gini_history:     &[f32],     // compte_courant gini (white)
    gini_pat_history: &[f32],     // patrimoine gini (yellow)
    total_wealth:     i64,
    tick_count:       u64,
    fonts:            &FontSizes,
) -> bool {
    let sw = screen_width();
    let sh = screen_height();
    clear_background(Color::new(0.06, 0.06, 0.08, 1.0));

    let pad   = 50.0;
    let btn_h = fonts.main_title + 8.0;
    let cx    = pad;
    let cy    = pad + btn_h + 10.0;
    let cw    = sw - pad * 2.0 - 160.0;
    let ch    = sh - cy - pad;

    draw_rectangle(cx, cy, cw, ch, Color::new(0.07, 0.07, 0.11, 1.0));
    draw_rectangle_lines(cx, cy, cw, ch, 1.0, Color::new(0.3, 0.3, 0.4, 1.0));

    let y_max: i64 = pct_history.iter()
        .flat_map(|snap| snap.iter().copied())
        .max()
        .unwrap_or(1)
        .max(1);
    let y_ceil = (y_max as f64 * 1.1) as i64;

    let y_of = |w: i64| -> f32 {
        (w as f32 / y_ceil as f32).clamp(0.0, 1.0)
    };

    let grid_col  = Color::new(0.25, 0.25, 0.35, 1.0);
    let label_col = Color::new(0.55, 0.55, 0.65, 1.0);
    for &frac in &[0.25f32, 0.5, 0.75, 1.0] {
        let w  = (y_ceil as f32 * frac) as i64;
        let gy = cy + ch - frac * ch;
        draw_line(cx, gy, cx + cw, gy, 0.5, grid_col);
        let pct = w as f64 / total_wealth as f64 * 100.0;
        draw_text(
            &format!("{} ({:.1}%)", fmt_wealth(w), pct),
            cx + 4.0, gy - 3.0, fonts.legend_value, label_col,
        );
    }

    // percentile lines
    for pi in 0..N_PCT {
        let t     = PERCENTILE_RANKS[pi] as f32 / 100.0;
        let color = color_ramp(t);
        let n_show = (cw as usize).min(pct_history.len());
        let slice  = &pct_history[pct_history.len() - n_show..];
        if slice.len() < 2 { continue; }
        let n = (slice.len() - 1) as f32;
        for j in 1..slice.len() {
            let x1 = cx + (j - 1) as f32 / n * cw;
            let x2 = cx + j as f32 / n * cw;
            let y1 = cy + ch - y_of(slice[j - 1][pi]) * ch;
            let y2 = cy + ch - y_of(slice[j][pi]) * ch;
            draw_line(x1, y1, x2, y2, 1.5, color);
        }
    }

    // Gini compte_courant — white dashed
    let gini_cc_color  = Color::new(1.0, 1.0, 1.0, 0.75);
    draw_gini_line(cx, cy, cw, ch, gini_history, gini_cc_color);

    // Gini patrimoine — yellow dashed (shown separately only when it diverges)
    let gini_pat_color = Color::new(1.0, 0.80, 0.20, 0.80);
    draw_gini_line(cx, cy, cw, ch, gini_pat_history, gini_pat_color);

    // right-axis labels
    draw_text("1.0", cx + cw + 2.0, cy + fonts.legend_value, fonts.legend_value, gini_cc_color);
    draw_text("0.0", cx + cw + 2.0, cy + ch,                 fonts.legend_value, gini_cc_color);

    let current_cc  = gini_history.last().copied().unwrap_or(0.0);
    let current_pat = gini_pat_history.last().copied().unwrap_or(0.0);
    let gy_cc  = cy + ch - current_cc.clamp(0.0, 1.0) * ch;
    let gy_pat = cy + ch - current_pat.clamp(0.0, 1.0) * ch;
    draw_text(&format!("G.CC  {:.3}", current_cc),  cx + cw + 2.0, gy_cc  - 2.0, fonts.legend_value, gini_cc_color);
    draw_text(&format!("G.Pat {:.3}", current_pat), cx + cw + 2.0, gy_pat + fonts.legend_value, fonts.legend_value, gini_pat_color);

    // legend panel
    let lx  = cx + cw + 15.0;
    let mut ly = cy + fonts.legend_value * 3.0;
    draw_text("Centile", lx, ly, fonts.section_title, WHITE);
    ly += fonts.legend_value * 0.5;
    if let Some(last) = pct_history.last() {
        for pi in (0..N_PCT).rev() {
            ly += fonts.legend_value + 3.0;
            let t     = PERCENTILE_RANKS[pi] as f32 / 100.0;
            let color = color_ramp(t);
            draw_rectangle(lx, ly, 10.0, 10.0, color);
            let txt = if last[pi] <= 0 { "mort".to_string() } else { fmt_wealth(last[pi]) };
            draw_text(&format!("{} {}", LABELS[pi], txt), lx + 14.0, ly + 10.0, fonts.legend_value, color);
        }
    }
    // Gini legend
    ly += fonts.legend_value * 2.0;
    draw_rectangle(lx, ly, 10.0, 10.0, gini_cc_color);
    draw_text("Gini CC",  lx + 14.0, ly + 10.0, fonts.legend_value, gini_cc_color);
    ly += fonts.legend_value + 5.0;
    draw_rectangle(lx, ly, 10.0, 10.0, gini_pat_color);
    draw_text("Gini Pat", lx + 14.0, ly + 10.0, fonts.legend_value, gini_pat_color);

    draw_text(&format!("Tick: {}", tick_count), cx, pad + btn_h - 4.0, fonts.main_title, WHITE);
    draw_button(sw - pad - 110.0, 10.0, 110.0, btn_h, "< Grille", false, fonts.button_text)
}

fn draw_gini_line(cx: f32, cy: f32, cw: f32, ch: f32, history: &[f32], color: Color) {
    let n_show = (cw as usize).min(history.len());
    let slice  = &history[history.len() - n_show..];
    if slice.len() < 2 { return; }
    let n = (slice.len() - 1) as f32;
    for j in 1..slice.len() {
        if j % 2 == 0 { continue; } // dashed
        let x1 = cx + (j - 1) as f32 / n * cw;
        let x2 = cx + j as f32 / n * cw;
        let y1 = cy + ch - slice[j - 1].clamp(0.0, 1.0) * ch;
        let y2 = cy + ch - slice[j].clamp(0.0, 1.0) * ch;
        draw_line(x1, y1, x2, y2, 1.0, color);
    }
}
