use ::rand::Rng;
use macroquad::prelude::Color;
#[cfg(not(target_arch = "wasm32"))]
use std::fs;

use crate::config::SimConfig;

pub struct AgentConfig {
    pub id:    usize,
    pub bonus: i32,
}

pub struct Agent {
    pub id:      usize,
    pub pos:     (usize, usize),
    pub wealth:  i64, // compte_courant: liquid cash used in transactions
    pub capital: i64, // off-market patrimoine, immune to transaction tax
    pub bonus:   i32,
    pub color:   Color,
}

pub fn agent_color(idx: usize) -> Color {
    const PALETTE: &[(f32, f32, f32)] = &[
        (1.0, 0.45, 0.45), (0.45, 0.65, 1.0), (0.45, 1.0, 0.55),
        (1.0, 1.0, 0.45),  (1.0, 0.65, 0.25), (0.85, 0.45, 1.0),
        (0.45, 1.0, 1.0),  (1.0, 0.45, 0.85), (0.55, 0.85, 0.55),
        (1.0, 0.80, 0.35), (0.70, 0.70, 1.0), (1.0, 0.55, 0.55),
        (0.35, 1.0, 0.85), (1.0, 0.75, 0.75), (0.75, 1.0, 0.45),
    ];
    let (r, g, b) = PALETTE[idx % PALETTE.len()];
    Color::new(r, g, b, 1.0)
}

pub(crate) fn color_ramp(t: f32) -> Color {
    const STOPS: &[(f32, f32, f32)] = &[
        (0.05, 0.05, 0.50), // deep blue  (near 0)
        (0.00, 0.60, 0.90), // cyan        (low)
        (0.10, 0.85, 0.20), // green       (middle)
        (1.00, 0.85, 0.00), // yellow      (above avg)
        (1.00, 0.25, 0.00), // red         (rich)
    ];
    let n  = STOPS.len() - 1;
    let sc = (t * n as f32).clamp(0.0, n as f32);
    let i  = (sc as usize).min(n - 1);
    let f  = sc - i as f32;
    let (r1, g1, b1) = STOPS[i];
    let (r2, g2, b2) = STOPS[i + 1];
    Color::new(r1 + (r2 - r1) * f, g1 + (g2 - g1) * f, b1 + (b2 - b1) * f, 1.0)
}

// logarithmic scale so differences are visible at any wealth magnitude
pub fn wealth_color(wealth: i64, total_wealth: i64) -> Color {
    if wealth <= 0 { return Color::new(0.08, 0.08, 0.12, 1.0); }
    let max = total_wealth.max(1) as f64;
    let t   = ((wealth as f64 + 1.0).ln() / (max + 1.0).ln()) as f32;
    color_ramp(t.clamp(0.0, 1.0))
}

pub fn bonus_color(bonus: i32) -> Color {
    if bonus > 0      { Color::new(0.3, 1.0, 0.4, 1.0) }
    else if bonus < 0 { Color::new(1.0, 0.35, 0.3, 1.0) }
    else              { Color::new(0.7, 0.7, 0.7, 1.0) }
}

pub fn generate_and_save_agents(sim: &SimConfig) -> Vec<AgentConfig> {
    let mut rng = ::rand::thread_rng();
    let configs: Vec<AgentConfig> = (0..sim.num_agents)
        .map(|i| {
            let bonus = if sim.deviance == 0 { 0 } else { rng.gen_range(-sim.deviance..=sim.deviance) };
            AgentConfig { id: i + 1, bonus }
        })
        .collect();

    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut out = format!("# Generated at launch — do not edit manually\n# deviance = {}\n\n", sim.deviance);
        for cfg in &configs {
            let sign = if cfg.bonus >= 0 { "+" } else { "" };
            out.push_str(&format!("[[agents]]\nid    = {}\nbonus = {}  # {}{}\n\n", cfg.id, cfg.bonus, sign, cfg.bonus));
        }
        let _ = fs::write("config/agents.toml", out);
    }
    configs
}

pub fn spawn_agents(configs: Vec<AgentConfig>, init_wealth: i32) -> Vec<Agent> {
    configs.into_iter()
        .map(|cfg| Agent {
            id:      cfg.id,
            pos:     (0, 0),
            wealth:  init_wealth as i64,
            capital: 0,
            bonus:   cfg.bonus,
            color:   agent_color(cfg.id),
        })
        .collect()
}
