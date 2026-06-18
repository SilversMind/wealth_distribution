#[cfg(not(target_arch = "wasm32"))]
use std::fs;
#[cfg(not(target_arch = "wasm32"))]
use toml::Value;

#[derive(Clone)]
pub struct SimConfig {
    pub deviance:        i32,
    pub num_agents:      usize,
    pub init_wealth:     i32,
    pub transfer_pct:    i32,
    pub label_font:      i32,
    pub tax_type:        i32,
    pub tax_rate:        i32,
    pub tax_freq:        i32,
    pub capital_enabled: i32,
    pub seuil_a_pct:     i32,
    pub seuil_b_pct:     i32,
    pub capital_rate:    i32,
    pub capital_freq:    i32,
}

impl SimConfig {
    pub fn load() -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let content = fs::read_to_string("config/simulation.toml").unwrap_or_default();
            if let Ok(parsed) = content.parse::<Value>() {
                let get = |key, def: i64| parsed.get(key).and_then(|v| v.as_integer()).unwrap_or(def);
                return SimConfig {
                    deviance:        get("deviance",        0)   as i32,
                    num_agents:      get("num_agents",      10)  as usize,
                    init_wealth:     get("init_wealth",     100) as i32,
                    transfer_pct:    get("transfer_pct",    10)  as i32,
                    label_font:      get("label_font",      20)  as i32,
                    tax_type:        get("tax_type",        0)   as i32,
                    tax_rate:        get("tax_rate",        2)   as i32,
                    tax_freq:        get("tax_freq",        100) as i32,
                    capital_enabled: get("capital_enabled", 0)   as i32,
                    seuil_a_pct:     get("seuil_a_pct",    150) as i32,
                    seuil_b_pct:     get("seuil_b_pct",    50)  as i32,
                    capital_rate:    get("capital_rate",    2)   as i32,
                    capital_freq:    get("capital_freq",    100) as i32,
                };
            }
        }
        SimConfig::default()
    }

    pub fn save(&self) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let content = format!(
                "# Simulation parameters\n\
                 deviance        = {}\n\
                 num_agents      = {}\n\
                 init_wealth     = {}\n\
                 transfer_pct    = {}\n\
                 label_font      = {}\n\
                 tax_type        = {}\n\
                 tax_rate        = {}\n\
                 tax_freq        = {}\n\
                 capital_enabled = {}\n\
                 seuil_a_pct     = {}\n\
                 seuil_b_pct     = {}\n\
                 capital_rate    = {}\n\
                 capital_freq    = {}\n",
                self.deviance, self.num_agents, self.init_wealth,
                self.transfer_pct, self.label_font, self.tax_type, self.tax_rate, self.tax_freq,
                self.capital_enabled, self.seuil_a_pct, self.seuil_b_pct, self.capital_rate, self.capital_freq,
            );
            let _ = fs::write("config/simulation.toml", content);
        }
    }
}

impl Default for SimConfig {
    fn default() -> Self {
        SimConfig {
            deviance: 0, num_agents: 10, init_wealth: 100, transfer_pct: 10, label_font: 20,
            tax_type: 0, tax_rate: 2, tax_freq: 100,
            capital_enabled: 0, seuil_a_pct: 150, seuil_b_pct: 50, capital_rate: 2, capital_freq: 100,
        }
    }
}

// ── Font sizes ────────────────────────────────────────────────────────────────

pub struct FontSizes {
    pub main_title:    f32,
    pub section_title: f32,
    pub button_text:   f32,
    pub legend_value:  f32,
}

impl FontSizes {
    pub fn from_base(base: f32) -> Self {
        FontSizes {
            main_title:    (base * 1.20).max(10.0),
            section_title: (base * 0.90).max(8.0),
            button_text:   (base * 0.85).max(8.0),
            legend_value:  (base * 0.70).max(7.0),
        }
    }
}

impl Default for FontSizes {
    fn default() -> Self {
        FontSizes { main_title: 24.0, section_title: 16.0, button_text: 16.0, legend_value: 14.0 }
    }
}
