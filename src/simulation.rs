use ::rand::Rng;
use macroquad::prelude::Color;

use crate::agent::{Agent, generate_and_save_agents, spawn_agents};
use crate::config::SimConfig;
use crate::constants::GRID_W;

pub const PERCENTILE_RANKS: &[f64] = &[10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0, 99.0];
pub const N_PCT: usize = 10;

pub struct SimState {
    pub agents:           Vec<Agent>,
    pub sorted_indices:   Vec<usize>, // ascending by total patrimoine (wealth + capital)
    pub rank_of:          Vec<usize>,
    pub pct_history:      Vec<[i64; N_PCT]>,
    pub gini_history:     Vec<f32>,   // compte_courant gini
    pub gini_pat_history: Vec<f32>,   // patrimoine gini
    pub tick_count:       u64,
    pub total_wealth:     i64,
    pub winner:           Option<(usize, Color)>,
}

#[inline]
fn total(a: &Agent) -> i64 { a.wealth + a.capital }

impl SimState {
    pub fn new(sim: &SimConfig) -> Self {
        let configs = generate_and_save_agents(sim);
        let mut agents = spawn_agents(configs, sim.init_wealth);
        let n = agents.len();

        let mut sorted_indices: Vec<usize> = (0..n).collect();
        sorted_indices.sort_unstable_by_key(|&i| total(&agents[i]));

        let mut rank_of = vec![0usize; n];
        for (rank, &idx) in sorted_indices.iter().enumerate() {
            rank_of[idx]    = rank;
            agents[idx].pos = pos_from_rank(rank);
        }

        let snapshot = compute_percentiles(&agents, &sorted_indices);
        let gini     = compute_gini_cc(&agents);
        let gini_pat = compute_gini_patrimoine(&agents);

        SimState {
            total_wealth:     sim.num_agents as i64 * sim.init_wealth as i64,
            sorted_indices,
            rank_of,
            pct_history:      vec![snapshot],
            gini_history:     vec![gini],
            gini_pat_history: vec![gini_pat],
            agents,
            tick_count:       0,
            winner:           None,
        }
    }

    pub fn step(
        &mut self,
        transfer_pct:    i32,
        tax_type:        i32,
        tax_rate:        i32,
        tax_freq:        i32,
        capital_enabled: bool,
        seuil_a:         i64,
        seuil_b:         i64,
        capital_rate:    i32,
        capital_freq:    i32,
    ) {
        // alive = CC > 0, ordered by total patrimoine (homophily by total wealth)
        let alive_sorted: Vec<usize> = self.sorted_indices.iter()
            .copied()
            .filter(|&i| self.agents[i].wealth > 0)
            .collect();

        if let Some((a, b)) = tick(&mut self.agents, transfer_pct, &alive_sorted) {
            update_rank(&mut self.sorted_indices, &mut self.rank_of, &mut self.agents, a);
            update_rank(&mut self.sorted_indices, &mut self.rank_of, &mut self.agents, b);
        }

        self.tick_count += 1;

        if tax_type > 0 && tax_rate > 0 && tax_freq > 0 && self.tick_count % tax_freq as u64 == 0 {
            apply_tax(&mut self.agents, &mut self.sorted_indices, &mut self.rank_of, tax_rate);
        }

        if capital_enabled {
            let pay_rent = capital_freq > 0 && self.tick_count % capital_freq as u64 == 0;
            apply_capital(
                &mut self.agents, &mut self.sorted_indices, &mut self.rank_of,
                seuil_a, seuil_b, capital_rate, pay_rent,
            );
        }

        self.total_wealth = self.agents.iter().map(|a| total(a)).sum();

        self.pct_history.push(compute_percentiles(&self.agents, &self.sorted_indices));
        self.gini_history.push(compute_gini_cc(&self.agents));
        self.gini_pat_history.push(compute_gini_patrimoine(&self.agents));

        if let Some(idx) = check_monopoly(&self.agents, self.total_wealth) {
            self.winner = Some((self.agents[idx].id, self.agents[idx].color));
        }
    }
}

// ── helpers ──────────────────────────────────────────────────────────────────

#[inline]
fn pos_from_rank(rank: usize) -> (usize, usize) {
    (rank % GRID_W, rank / GRID_W)
}

// O(k) insertion shift keyed on total patrimoine.
fn update_rank(
    sorted:  &mut Vec<usize>,
    rank_of: &mut Vec<usize>,
    agents:  &mut Vec<Agent>,
    idx:     usize,
) {
    let old_rank = rank_of[idx];
    let w        = total(&agents[idx]);
    let n        = sorted.len();

    let new_rank = if old_rank + 1 < n && total(&agents[sorted[old_rank + 1]]) < w {
        let mut r = old_rank + 1;
        while r + 1 < n && total(&agents[sorted[r + 1]]) < w { r += 1; }
        r
    } else if old_rank > 0 && total(&agents[sorted[old_rank - 1]]) > w {
        let mut r = old_rank;
        while r > 0 && total(&agents[sorted[r - 1]]) > w { r -= 1; }
        r
    } else {
        return;
    };

    if new_rank > old_rank {
        for r in old_rank..new_rank {
            sorted[r]          = sorted[r + 1];
            rank_of[sorted[r]] = r;
            agents[sorted[r]].pos = pos_from_rank(r);
        }
    } else {
        for r in (new_rank + 1..=old_rank).rev() {
            sorted[r]          = sorted[r - 1];
            rank_of[sorted[r]] = r;
            agents[sorted[r]].pos = pos_from_rank(r);
        }
    }
    sorted[new_rank]    = idx;
    rank_of[idx]        = new_rank;
    agents[idx].pos     = pos_from_rank(new_rank);
}

fn tick(agents: &mut Vec<Agent>, transfer_pct: i32, alive_sorted: &[usize]) -> Option<(usize, usize)> {
    let n = alive_sorted.len();
    if n < 2 { return None; }

    let mut rng  = ::rand::thread_rng();
    let ai       = rng.gen_range(0..n);
    let a        = alive_sorted[ai];
    let a_decile = (ai * 10) / n;

    let peers: Vec<usize> = (0..n)
        .filter(|&j| j != ai && (j * 10) / n == a_decile)
        .map(|j| alive_sorted[j])
        .collect();

    let b = if peers.is_empty() {
        let bi = loop { let x = rng.gen_range(0..n); if x != ai { break x; } };
        alive_sorted[bi]
    } else {
        peers[rng.gen_range(0..peers.len())]
    };

    let min_wealth = agents[a].wealth.min(agents[b].wealth);
    let transfer   = ((min_wealth as f64 * transfer_pct as f64 / 100.0).round() as i64).max(1);
    let p_a        = (0.5 + (agents[a].bonus - agents[b].bonus) as f64 / 200.0).clamp(0.01, 0.99);

    if rng.gen_bool(p_a) {
        let actual = transfer.min(agents[b].wealth);
        agents[a].wealth += actual;
        agents[b].wealth -= actual;
    } else {
        let actual = transfer.min(agents[a].wealth);
        agents[b].wealth += actual;
        agents[a].wealth -= actual;
    }

    Some((a, b))
}

// Percentiles now track total patrimoine (wealth + capital).
fn compute_percentiles(agents: &[Agent], sorted: &[usize]) -> [i64; N_PCT] {
    let alive: Vec<i64> = sorted.iter()
        .map(|&i| total(&agents[i]))
        .filter(|&t| t > 0)
        .collect();
    let n = alive.len();
    let mut out = [0i64; N_PCT];
    if n == 0 { return out; }
    for (pi, &rank) in PERCENTILE_RANKS.iter().enumerate() {
        let idx = ((rank / 100.0 * n as f64).ceil() as usize).saturating_sub(1).min(n - 1);
        out[pi] = alive[idx];
    }
    out
}

// CC gini — sorts independently (sorted_indices is now keyed on total, not CC).
fn compute_gini_cc(agents: &[Agent]) -> f32 {
    let mut cc: Vec<i64> = agents.iter()
        .map(|a| a.wealth)
        .filter(|&w| w > 0)
        .collect();
    cc.sort_unstable();
    let n = cc.len();
    if n < 2 { return 0.0; }
    let sum: i64 = cc.iter().sum();
    if sum == 0 { return 0.0; }
    let numer: i64 = cc.iter().enumerate()
        .map(|(i, &w)| (2 * (i as i64 + 1) - n as i64 - 1) * w)
        .sum();
    (numer as f32 / (n as f32 * sum as f32)).clamp(0.0, 1.0)
}

// Patrimoine gini — sorted_indices already sorted by total, but we sort independently for safety.
fn compute_gini_patrimoine(agents: &[Agent]) -> f32 {
    let mut totals: Vec<i64> = agents.iter()
        .map(|a| total(a))
        .filter(|&t| t > 0)
        .collect();
    totals.sort_unstable();
    let n = totals.len();
    if n < 2 { return 0.0; }
    let sum: i64 = totals.iter().sum();
    if sum == 0 { return 0.0; }
    let numer: i64 = totals.iter().enumerate()
        .map(|(i, &w)| (2 * (i as i64 + 1) - n as i64 - 1) * w)
        .sum();
    (numer as f32 / (n as f32 * sum as f32)).clamp(0.0, 1.0)
}

fn check_monopoly(agents: &[Agent], total_wealth: i64) -> Option<usize> {
    agents.iter().position(|a| total(a) >= total_wealth)
}

fn apply_tax(
    agents:   &mut Vec<Agent>,
    sorted:   &mut Vec<usize>,
    rank_of:  &mut Vec<usize>,
    tax_rate: i32,
) {
    let n = agents.len();
    if n == 0 { return; }

    let mut total_tax: i64 = 0;
    for agent in agents.iter_mut() {
        if agent.wealth > 0 {
            let tax = (agent.wealth as f64 * tax_rate as f64 / 100.0).floor() as i64;
            agent.wealth -= tax;
            total_tax    += tax;
        }
    }
    if total_tax == 0 { return; }

    let per = total_tax / n as i64;
    let rem = (total_tax % n as i64) as usize;
    for agent in agents.iter_mut() { agent.wealth += per; }
    // remainder to poorest by total patrimoine
    for &idx in sorted[..rem.min(n)].iter() { agents[idx].wealth += 1; }

    sorted.sort_unstable_by_key(|&i| total(&agents[i]));
    for (rank, &idx) in sorted.iter().enumerate() {
        rank_of[idx]    = rank;
        agents[idx].pos = pos_from_rank(rank);
    }
}

fn apply_capital(
    agents:       &mut Vec<Agent>,
    sorted:       &mut Vec<usize>,
    rank_of:      &mut Vec<usize>,
    seuil_a:      i64,
    seuil_b:      i64,
    capital_rate: i32,
    pay_rent:     bool,
) {
    // Rent cycle (zero-sum):
    //   1. Compute desired rent; levy it equally from no-capital agents (capped at their CC).
    //   2. Distribute exactly what was collected to capital holders, pro-rata to their capital.
    //   If nobody has no capital, nothing flows.
    if pay_rent && capital_rate > 0 {
        let rate = capital_rate as f64 / 100.0;
        let desired: i64 = agents.iter()
            .filter(|a| a.capital > 0)
            .map(|a| (a.capital as f64 * rate) as i64)
            .sum();

        if desired > 0 {
            // step 1: collect
            let payers: Vec<usize> = (0..agents.len())
                .filter(|&i| agents[i].capital == 0 && agents[i].wealth > 0)
                .collect();
            let mut collected = 0i64;
            if !payers.is_empty() {
                let levy = desired / payers.len() as i64;
                let rem  = (desired % payers.len() as i64) as usize;
                for (k, &idx) in payers.iter().enumerate() {
                    let ask   = levy + if k < rem { 1 } else { 0 };
                    let taken = ask.min(agents[idx].wealth);
                    agents[idx].wealth -= taken;
                    collected += taken;
                }
            }

            // step 2: distribute pro-rata to capital holders
            if collected > 0 {
                let total_cap: i64 = agents.iter().map(|a| a.capital).sum();
                if total_cap > 0 {
                    let mut paid = 0i64;
                    let n_holders = agents.iter().filter(|a| a.capital > 0).count();
                    let mut seen  = 0usize;
                    for agent in agents.iter_mut() {
                        if agent.capital > 0 {
                            seen += 1;
                            let share = if seen < n_holders {
                                collected * agent.capital / total_cap
                            } else {
                                collected - paid // last holder absorbs rounding
                            };
                            agent.wealth += share;
                            paid += share;
                        }
                    }
                }
            }
        }
    }

    for agent in agents.iter_mut() {
        if total(agent) <= 0 { continue; }
        if agent.wealth > seuil_a {
            let surplus    = agent.wealth - seuil_a;
            agent.capital += surplus;
            agent.wealth   = seuil_a;
        } else if agent.wealth < seuil_b && agent.capital > 0 {
            let manque    = seuil_b - agent.wealth;
            let vendu     = manque.min(agent.capital);
            agent.capital -= vendu;
            agent.wealth  += vendu;
        }
    }

    sorted.sort_unstable_by_key(|&i| total(&agents[i]));
    for (rank, &idx) in sorted.iter().enumerate() {
        rank_of[idx]    = rank;
        agents[idx].pos = pos_from_rank(rank);
    }
}
