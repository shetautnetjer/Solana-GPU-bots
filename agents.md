🤖 agents.md: Trading Bot Signal Agents & Future AI Integration
🧠 Core Philosophy

This bot is designed with CPU-first architecture, leveraging pure price-based analysis to generate actionable trading signals across multiple timeframes (1m, 5m, 15m).
All features, scores, and weights are computed from Pyth price feed only — without requiring order book snapshots, vault deltas, or transaction logs.

This structure ensures:

    Scalability: Can run anywhere — no GPU or order book needed at first.

    Modularity: All scoring functions are isolated and can later be offloaded to GPU.

    AI-ready data: We log every signal feature, decision, and outcome so future agents can learn, backtest, and optimize kernels or weights automatically.

🎯 Goal of Agents

These “agents” are modular scoring modules that process price tick history in isolated timeframes and emit weighted signals.
They can be combined or evolved into RL/ML modules once GPU acceleration or reinforcement training is added.

Each agent must:

    Take in a tick stream: [slot, price]

    Compute feature(s): delta, cycle, deviation, etc.

    Output a score and tag

    Optionally: store z-score, confidence, and strategy metadata

📊 Tracked Metrics Per Agent

These are all numerically pure, transformable features (z-score/normalized), perfect for future GPU porting or AI modeling:
Feature	Description	Usage
momentum	ΔP over N ticks	Entry strength
hilbert_phase	Price phase angle from analytic Hilbert transform	Reversal zone detection
fft_band_power	Spectral energy in 0.04–0.10Hz band	Detect spoof cycles / cyclical pumps
permutation_entropy	Price entropy from ordinal pattern analysis	Trend exhaustion or chaotic motion
fama_gap	(Price - FAMA) / ATR	Trend fade or extension detection
fractal_atr	Hurst-adaptive ATR	Volatility-weighted position sizing
score_weight_matrix	Final score = Σ wᵢ · zᵢ	Blended signal output
🧬 Weighting System

Each agent’s output is tagged and scored. These tags are composed into strategy IDs (e.g., microcap_1m_hilbert_momentum) and logged with the following:

    raw_score

    weighted_score

    strategy_id

    timeframe

    market_cap_tier

    final_decision

These are stored in Arrow2 Parquet, and updated post-trade with PnL feedback using a strategy_weights.json or .parquet reinforcement file.
🚀 GPU Forward-Compatibility Plan

Because we’re tracking structured, windowed price transforms:

    All z-score transforms, FFTs, entropy calcs, and vector ops can be ported to GPU kernels.

    Features are stateless (i.e., they only depend on recent price arrays, not external data).

    Kernels can be evolved via AI or meta-learning tools to discover optimal transformations.

This ensures that any AI agent in the future can:

    Pick up this data

    See which strategies worked over thousands of trades

    Write or auto-generate GPU kernels for those signals

    Compress them into low-latency indicator modules (i.e. fast TA scoring agents)

🧾 Required Log Fields (per signal/trade)

Every agent must store:

    strategy_id

    timeframe

    score

    signal_tags

    feature_values: map of indicator → z-score

    final_weight

    PnL_after_trade

    priority_fee

    slippage_estimate

    volume_profile_flag (if applicable)

Logged to:

/data/logs/YYYY-MM-DD/signals.parquet
/data/logs/YYYY-MM-DD/trades.parquet

🧠 Summary

This bot is not chasing AI hype — it’s building a robust, interpretable system first.
The goal is to collect thousands of normalized, tagged, post-trade outcomes, so any intelligent agent in the future (human or machine) can:

    Learn what signals work

    Design new ones from existing inputs

    Migrate winning signal logic to GPU kernels

    Auto-optimize weights and confidence thresholds

This is how you future-proof trading intelligence.
