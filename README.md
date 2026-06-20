# Sugarscape — Wealth Distribution Simulator

Agent-based simulation of wealth dynamics on a 50×50 grid. Agents exchange wealth at random, with optional taxation and capital systems. Observe Gini coefficient evolution in real time.

Built with **Rust + macroquad**. Runs natively on desktop and in the browser via WebAssembly.

---

## Views

| View | Key |
|------|-----|
| Grid — agents colored by wealth | `G` |
| Chart — Gini curve over time | `C` |
| Config — simulation parameters | `S` |

---

## Simulation mechanics

### Wealth transfer
Each tick, a random agent transfers a percentage of their wealth (`transfer_pct`) to another random agent. Over time this produces Pareto-distributed wealth without any other mechanism.

### Tax (`tax_type`)
Applied every `tax_freq` ticks.

| Value | Mode |
|-------|------|
| `0` | Disabled |
| `1` | Flat — fixed % taken from all agents |
| `2` | Progressive — rate scales with wealth rank |

Collected tax is redistributed equally to all agents.

### Capital
When enabled, agents invest surplus wealth into capital (off-market patrimony immune to transaction tax).

- **Vases communicants** (every tick): if `compte_courant > seuil_a` → excess moves to capital; if `compte_courant < seuil_b` → liquidate capital to cover shortfall.
- **Rent** (every `capital_freq` ticks): agents without capital are levied proportionally; proceeds are distributed pro-rata to capital holders. Zero-sum — no new money is created.

### Gini coefficient
Two curves tracked independently:
- **G.CC** — compte courant (liquid wealth) only
- **G.Pat** — total patrimoine (CC + capital)

---

## Configuration

### Desktop
Edit `config/simulation.toml` and restart. Agents are regenerated on restart and written to `config/agents.toml`.

| Parameter | Default | Description |
|-----------|---------|-------------|
| `num_agents` | 10 | Number of agents |
| `init_wealth` | 100 | Starting wealth per agent |
| `deviance` | 0 | Random bonus range `[-d, +d]` per agent |
| `transfer_pct` | 10 | % of wealth transferred per tick |
| `tax_type` | 0 | Tax mode (0/1/2) |
| `tax_rate` | 2 | Tax rate % |
| `tax_freq` | 100 | Ticks between tax events |
| `capital_enabled` | 0 | Enable capital system (0/1) |
| `seuil_a_pct` | 150 | Invest threshold (% of init_wealth) |
| `seuil_b_pct` | 50 | Liquidation threshold (% of init_wealth) |
| `capital_rate` | 2 | Rent rate % of capital |
| `capital_freq` | 100 | Ticks between rent events |

### Web
Parameters are configured at runtime via the in-app config panel (no file persistence).

---

## Build

### Prerequisites
```bash
rustup target add wasm32-unknown-unknown
```

### Desktop
```bash
cargo run
```

### Web (WASM)
```bash
cargo build --release --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/release/sugarscape.wasm .
python3 -m http.server 8080
# open http://localhost:8080
```

---

## Project structure

```
src/
  main.rs          entry point, main loop
  simulation.rs    tick logic, Gini computation
  agent.rs         agent struct, color mapping
  config.rs        SimConfig load/save (desktop only)
  constants.rs     grid dimensions, cell size
  ui/              rendering (grid, chart, config panel, overlay)

host/              JavaScript host environment (WASM bridge)
  bindings.js      assembles WebAssembly importObject
  runtime/         state, utils, canvas, GL registry, plugins
  gl/              WebGL bindings (textures, buffers, shaders, pipeline)
  env/             system, events, filesystem, input

gl.js              WASM loader entry point
index.html         web entry point
config/            desktop config files (gitignored runtime output)
```

---

## Stack

- **Rust** + **macroquad 0.3** (rendering, input, audio)
- **miniquad 0.3.16** (WASM host via `host/`)
- `rand 0.8` with custom WASM entropy (seeded from `Date.now()`)
- `toml 0.5` for desktop config (excluded from WASM build)
