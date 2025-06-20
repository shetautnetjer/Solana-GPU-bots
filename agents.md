🤖 agents.md: Trading Bot Signal Agents & Future AI Integration
🧠 Core Philosophy

This bot is designed with CPU-first architecture, leveraging pure price-based analysis to generate actionable trading signals across multiple timeframes (1m, 5m, 15m).
All features, scores, and weights are computed from Pyth price feed only — without requiring order book snapshots, vault deltas, or transaction logs.

This structure ensures:

    Scalability: Can run anywhere — no GPU or order book needed at first.

    Modularity: All scoring functions are isolated and can later be offloaded to GPU.

    AI-ready data: We log every signal feature, decision, and outcome so future agents can learn, backtest, and optimize kernels or weights automatically.

🎯 Goal of Agents

These "agents" are modular scoring modules that process price tick history in isolated timeframes and emit weighted signals.
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

Each agent's output is tagged and scored. These tags are composed into strategy IDs (e.g., microcap_1m_hilbert_momentum) and logged with the following:

    raw_score

    weighted_score

    strategy_id

    timeframe

    market_cap_tier

    final_decision

These are stored in Arrow2 Parquet, and updated post-trade with PnL feedback using a strategy_weights.json or .parquet reinforcement file.
🚀 GPU Forward-Compatibility Plan

Because we're tracking structured, windowed price transforms:

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

This bot is not chasing AI hype — it's building a robust, interpretable system first.
The goal is to collect thousands of normalized, tagged, post-trade outcomes, so any intelligent agent in the future (human or machine) can:

    Learn what signals work

    Design new ones from existing inputs

    Migrate winning signal logic to GPU kernels

    Auto-optimize weights and confidence thresholds

This is how you future-proof trading intelligence.

# Codex Build Instructions (`agents.md`)

## 🧠 Purpose

This file describes how **Codex (or any AI-assisted dev environment)** should structure feature modules and setup scripts during initial development.

There are no runtime agents. This is not a training loop or self-updating system.

---

## ✅ Goal

When using Codex to build new feature modules (e.g. FFT scoring, Hilbert phase detection), the AI must:

1. Output the Rust code file with the logic
2. Output a matching setup script (PowerShell `.ps1`) if any install or system config is required
3. Log all signals in a structured way (Arrow2-compatible)

---

## 📂 Expected Outputs from Codex Tasks

### 1. Signal Feature Module
**Example Output File:**

Must include:
- A public `compute_feature()` function
- Return type matching the signal schema
- No runtime scoring logic — only math feature generation

### 2. Install Script
If the signal uses any crate not yet installed or system tool (like linking to CUDA or FFTW), also output:
follow an example of to auto find folders and add to path using powershell https://github.com/shetautnetjer/hello-gpu


Should:
- Run `cargo add rustfft`
- Export environment variables if needed
- Optionally add to `src/signals/mod.rs`

### 3. Log Schema
All feature modules must emit a `FeatureRecord`:

```rust
struct FeatureRecord {
  slot: u64,
  price: f64,
  zscore: f64,
  tag: &'static str,
  strategy_id: String,
  timeframe: String,
}
```

---

## 🪟 Windows GPU Development Environment Setup

### Critical Environment Paths for AI-Assisted Development

When Codex or AI assistants need to set up GPU-accelerated features, they must configure these critical paths:

#### OpenSSL Configuration
```powershell
# Environment Variables
$env:OPENSSL_DIR = "C:\Program Files\OpenSSL-Win64"
$env:OPENSSL_LIB_DIR = "C:\Program Files\OpenSSL-Win64\lib\VC\x64\MD"
$env:OPENSSL_INCLUDE_DIR = "C:\Program Files\OpenSSL-Win64\include"

# Key Files
OpenSSL Binary: C:\Program Files\OpenSSL-Win64\bin\openssl.exe
OpenSSL Libraries: C:\Program Files\OpenSSL-Win64\lib\VC\x64\MD\
  ├── libcrypto.lib
  ├── libssl.lib
  └── libcrypto_static.lib
OpenSSL Headers: C:\Program Files\OpenSSL-Win64\include\
  ├── openssl/
  └── crypto/
```

#### CUDA Configuration
```powershell
# Environment Variables
$env:CUDA_PATH = "C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.5"
$env:CUDA_HOME = "C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.5"
$env:CUDA_TOOLKIT_ROOT_DIR = "C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.5"
$env:CUDA_LIBRARY_PATH = "C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.5\lib\x64"

# Key Directories
CUDA Binaries: C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.5\bin\
  ├── nvcc.exe
  ├── cuda-memcheck.exe
  └── nvprof.exe
CUDA Libraries: C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.5\lib\x64\
  ├── cudart.lib
  ├── cuda.lib
  ├── cudadevrt.lib
  └── nvml.lib
CUDA Headers: C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.5\include\
  ├── cuda_runtime.h
  ├── cuda.h
  └── nvml.h
```

#### MSVC (Microsoft Visual C++) Configuration
```powershell
# Environment Variables
$env:CCBIN_PATH = "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\14.41.34120\bin\Hostx64\x64"

# Key Files
MSVC Compiler: C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\14.41.34120\bin\Hostx64\x64\cl.exe
MSVC Linker: C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\14.41.34120\bin\Hostx64\x64\link.exe
MSVC Libraries: C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\14.41.34120\lib\x64\
```

### Cargo Configuration (`.cargo/config.toml`)
```toml
[target.'cfg(windows)']
rustflags = [
    "-C", "link-arg=/LIBPATH:C:\\Program Files\\NVIDIA GPU Computing Toolkit\\CUDA\\v12.5\\lib\\x64",
    "-C", "link-arg=cudart.lib",
    "-C", "link-arg=cuda.lib",
    "-C", "link-arg=cudadevrt.lib",
    "-C", "link-arg=--allow-unsupported-compiler",
]

[env]
CUDA_PATH = "C:\\Program Files\\NVIDIA GPU Computing Toolkit\\CUDA\\v12.5"
CUDA_TOOLKIT_ROOT_DIR = "C:\\Program Files\\NVIDIA GPU Computing Toolkit\\CUDA\\v12.5"
CUDA_LIBRARY_PATH = "C:\\Program Files\\NVIDIA GPU Computing Toolkit\\CUDA\\v12.5\\lib\\x64"
CCBIN_PATH = "C:\\Program Files\\Microsoft Visual Studio\\2022\\Community\\VC\\Tools\\MSVC\\14.41.34120\\bin\\Hostx64\\x64"
OPENSSL_DIR = "C:\\Program Files\\OpenSSL-Win64"
OPENSSL_LIB_DIR = "C:\\Program Files\\OpenSSL-Win64\\lib\\VC\\x64\\MD"
OPENSSL_INCLUDE_DIR = "C:\\Program Files\\OpenSSL-Win64\\include"
```

### Automated Setup Scripts for AI Development
When AI assistants create new GPU features, they should include these setup scripts:

```powershell
# Initialize GPU environment
.\scripts\init_gpu_env.ps1

# Setup CUDA paths
.\scripts\setup_cuda_paths.ps1

# Setup OpenSSL
.\scripts\setup_openssl.ps1

# Test GPU functionality
.\scripts\gpu_smoke_test.ps1
```

### Critical Issues & Solutions for AI Development

#### 1. OpenSSL Library Path Issue
- **Problem**: `OPENSSL_LIB_DIR` pointing to `lib\VC` instead of `lib\VC\x64\MD`
- **Solution**: Correct path to `C:\Program Files\OpenSSL-Win64\lib\VC\x64\MD`
- **Why Important**: Rust's `openssl-sys` crate needs the exact library directory

#### 2. MSVC Version Compatibility
- **Problem**: MSVC 14.41 not compatible with CUDA 12.5 (max supported: 14.39)
- **Solution**: Added `--allow-unsupported-compiler` flag to nvcc
- **Why Important**: Prevents compilation failures due to version mismatch

#### 3. Environment Variable Persistence
- **Problem**: Environment variables not persisting across sessions
- **Solution**: Set in `.cargo/config.toml` for project-specific configuration
- **Why Important**: Ensures consistent builds regardless of shell environment

### Quick Reference Commands for AI Development
```powershell
# Check current environment
echo "CUDA_PATH: $env:CUDA_PATH"
echo "CCBIN_PATH: $env:CCBIN_PATH"
echo "OPENSSL_LIB_DIR: $env:OPENSSL_LIB_DIR"

# Verify installations
nvcc --version
cl.exe
openssl version

# Test GPU functionality
.\scripts\gpu_smoke_test.ps1
```

### AI Development Guidelines

When AI assistants create GPU-accelerated trading features:

1. **Always include environment setup** in PowerShell scripts
2. **Use the exact paths** specified above for Windows compatibility
3. **Include the `--allow-unsupported-compiler` flag** for nvcc compilation
4. **Test GPU functionality** before deploying features
5. **Document any new dependencies** or system requirements

This ensures that GPU-accelerated trading features can be reliably developed and deployed in the Windows environment.