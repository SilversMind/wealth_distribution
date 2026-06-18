mod agent;
mod config;
mod constants;
mod simulation;
mod ui;

use macroquad::prelude::*;

use config::{FontSizes, SimConfig};
use constants::{OFFSET, CELL_SIZE, GRID_W, GRID_H};
use simulation::SimState;
use ui::{
    chart::draw_chart_view,
    config_panel::{draw_config_view, ConfigPanelState},
    grid::{draw_world, CustomSpeedState},
    overlay::draw_monopoly_overlay,
};
const SPEEDS: &[(f64, &str)] = &[
    (0.5,    "x1"),
    (0.25,   "x2"),
    (0.1,    "x5"),
    (0.05,   "x10"),
    (0.01,   "x50"),
    (0.005,  "x100"),
    (0.0005, "x1000"),
];

fn window_conf() -> Conf {
    Conf {
        window_title:  "Wealth Distribution".to_string(),
        window_width:  (OFFSET * 2.0 + GRID_W as f32 * CELL_SIZE + constants::PANEL_W + 50.0) as i32,
        window_height: (OFFSET * 2.0 + GRID_H as f32 * CELL_SIZE) as i32,
        ..Default::default()
    }
}

fn agent_at_click(agents: &[crate::agent::Agent], mx: f32, my: f32) -> Option<usize> {
    let r = CELL_SIZE / 2.0;
    agents.iter().position(|a| {
        let cx = OFFSET + a.pos.0 as f32 * CELL_SIZE + CELL_SIZE / 2.0;
        let cy = OFFSET + a.pos.1 as f32 * CELL_SIZE + CELL_SIZE / 2.0;
        (mx - cx).powi(2) + (my - cy).powi(2) <= r.powi(2)
    })
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut sim  = SimConfig::load();
    let mut fonts = FontSizes::from_base(sim.label_font as f32);
    let mut edit = sim.clone();

    let mut state          = SimState::new(&sim);
    let mut last_tick      = get_time();
    let mut speed_idx      = 0usize;
    let mut view           = 0u8; // 0=grid 1=chart 2=config
    let mut selected_agent: Option<usize> = None;
    let mut config_state   = ConfigPanelState::default();
    let mut custom_speed   = CustomSpeedState::default();

    loop {
        // advance simulation — run all ticks due since last frame
        if view != 2 && state.winner.is_none() {
            let now      = get_time();
            let interval = custom_speed.interval().unwrap_or(SPEEDS[speed_idx].0);
            let due      = ((now - last_tick) / interval).floor() as u64;
            if due > 0 {
                let run = due.min(10_000); // cap to avoid freezing
                let seuil_a = (sim.init_wealth as i64 * sim.seuil_a_pct as i64) / 100;
                let seuil_b = (sim.init_wealth as i64 * sim.seuil_b_pct as i64) / 100;
                for _ in 0..run {
                    if state.winner.is_some() { break; }
                    state.step(
                        sim.transfer_pct, sim.tax_type, sim.tax_rate, sim.tax_freq,
                        sim.capital_enabled != 0, seuil_a, seuil_b, sim.capital_rate, sim.capital_freq,
                    );
                }
                last_tick = now;
            }
        }

        // click detection (grid view only, inside grid bounds)
        if view == 0 && is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let in_grid  = mx >= OFFSET
                && mx <= OFFSET + GRID_W as f32 * CELL_SIZE
                && my >= OFFSET
                && my <= OFFSET + GRID_H as f32 * CELL_SIZE;
            if in_grid {
                let hit = agent_at_click(&state.agents, mx, my);
                selected_agent = match (hit, selected_agent) {
                    (Some(i), Some(j)) if i == j => None, // toggle off
                    (Some(i), _)                 => Some(i),
                    (None, _)                    => None,
                };
            }
        }

        // draw
        match view {
            1 => {
                if draw_chart_view(&state.pct_history, &state.gini_history, &state.gini_pat_history, state.total_wealth, state.tick_count, &fonts) {
                    view = 0;
                }
            }
            2 => {
                let (apply, cancel) = draw_config_view(&mut edit, &fonts, &mut config_state);
                if apply {
                    sim            = edit.clone();
                    sim.save();
                    fonts          = FontSizes::from_base(sim.label_font as f32);
                    state          = SimState::new(&sim);
                    selected_agent = None;
                    last_tick      = get_time();
                    config_state.reset();
                    view           = 0;
                } else if cancel {
                    edit = sim.clone();
                    config_state.reset();
                    view = 0;
                }
            }
            _ => {
                let (speed_click, to_chart, to_config) = draw_world(
                    &state.agents, state.tick_count,
                    sim.deviance, sim.transfer_pct, sim.label_font as f32,
                    speed_idx, SPEEDS, &fonts, selected_agent, state.total_wealth,
                    &mut custom_speed,
                );
                if let Some(idx) = speed_click { speed_idx = idx; }
                if to_chart  { view = 1; }
                if to_config { edit = sim.clone(); config_state.reset(); view = 2; }
            }
        }

        if let Some((winner_id, color)) = state.winner {
            draw_monopoly_overlay(winner_id, color, state.tick_count, &fonts);
        }

        next_frame().await;
    }
}
