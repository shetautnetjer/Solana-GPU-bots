# Solana GPU Trading Bot

A high-performance Solana trading bot with GPU acceleration for real-time pool monitoring and automated trading strategies.

## 🚀 Features

- **GPU-Accelerated Pool Monitoring**: Real-time monitoring of Solana DEX pools with CUDA-accelerated scoring
- **Jupiter API Integration**: Advanced routing and quote fetching with the latest Jupiter API
- **Multi-DEX Support**: Monitor pools across Raydium, Orca, and other major DEXs
- **Technical Analysis**: GPU-accelerated technical indicators and momentum calculations
- **WebSocket Streaming**: Real-time data streaming with proper TLS support
- **Windows & Linux Support**: Cross-platform compatibility with automatic GPU detection

## 📋 Prerequisites

### System Requirements
- **GPU**: NVIDIA GPU with CUDA support (RTX 3000+ series recommended)
- **RAM**: 8GB+ system memory
- **Storage**: 10GB+ free space
- **OS**: Windows 10/11 or Linux

### Software Requirements
- **CUDA Toolkit**: 11.8+ (for GPU acceleration)
- **Rust**: 1.70+ with Cargo
- **Visual Studio Build Tools** (Windows only)
- **Git**: For cloning and dependency management

## 🛠️ Installation

### 1. Clone the Repository
```bash
git clone https://github.com/your-username/Solana-GPU-bots.git
cd Solana-GPU-bots
```

### 2. Install CUDA Toolkit
Download and install [CUDA Toolkit 11.8+](https://developer.nvidia.com/cuda-downloads)

**Windows:**
- Install Visual Studio Build Tools 2019/2022
- Add CUDA to PATH: `C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v11.8\bin`

**Linux:**
```bash
# Ubuntu/Debian
sudo apt update
sudo apt install nvidia-cuda-toolkit

# Verify installation
nvcc --version
```

### 3. Windows Users: Install OpenSSL (Required)

**Automated Installation (Recommended):**
We provide a PowerShell script that automates the entire OpenSSL setup process:

```powershell
# Run PowerShell as Administrator
.\setup-openssl.ps1
```

This script will:
- Download OpenSSL for Windows
- Install it to the correct location
- Set all required environment variables
- Verify the installation
- Configure your system for building the Solana SDK

**Manual Installation:**
If you prefer manual installation:
1. Download from [https://slproweb.com/products/Win32OpenSSL.html](https://slproweb.com/products/Win32OpenSSL.html)
2. Choose "Win64 OpenSSL v3.x.x EXE"
3. Install to default location: `C:\Program Files\OpenSSL-Win64`

### 4. Build the Project
```bash
# Build all components
cargo build --release

# Test GPU functionality
cargo run -p pool_indexer -- gpu-test
```

## Windows: CUDA & Visual Studio Auto-Setup

Before building on Windows, run the CUDA auto-setup script to ensure all environment variables and paths are correct:

```powershell
cd hello-gpu/scripts
./setup_cuda_paths.ps1
```

- This script will:
  - Find your latest CUDA installation
  - Find the correct Visual Studio C++ compiler (`cl.exe`)
  - Update `.cargo/config.toml` with the right paths
  - Set the `LIB` environment variable (optionally persist with `-Persist`)
- **Run this script after installing or updating CUDA or Visual Studio.**
- If you see errors about missing `cl.exe` or CUDA libraries, re-run this script.

After running, restart your terminal or build in the same session.

## 🎯 Usage

### GPU Diagnostics
Test your GPU setup and CUDA installation:
```bash
cargo run -p pool_indexer -- gpu-test
```

### Pool Monitoring
Monitor pools for a specific token pair with GPU acceleration:
```bash
# Monitor SOL/USDC with GPU acceleration
cargo run -p pool_indexer -- monitor --pair SOL/USDC --gpu --logfile sol_usdc_pools.csv

# Monitor without GPU (CPU-only)
cargo run -p pool_indexer -- monitor --pair SOL/USDC --logfile sol_usdc_pools.csv
```

### Quote Analysis
Get quotes and analyze with GPU-accelerated scoring:
```bash
# Get quote for 1 SOL with 0.5% slippage
cargo run -p pool_indexer -- quote --pair SOL/USDC --amount 1000000000 --slippage-bps 50
```

### Hello-GPU Testing
Test the integrated GPU functionality:
```bash
# Run hello-gpu tests
cargo run -p hello-gpu

# Test vector addition on GPU
cargo test -p hello-gpu
```

## 📊 Project Structure

```
Solana-GPU-bots/
├── Cargo.toml                 # Main workspace configuration
├── hello-gpu/                 # GPU utilities and auto-detection
│   ├── src/
│   ├── kernels/               # CUDA kernels
│   └── scripts/               # Setup and test scripts
├── solana-ta-stack/
│   └── pool_indexer/          # Main trading bot
│       ├── src/
│       ├── kernels/           # Trading-specific CUDA kernels
│       └── build.rs           # CUDA compilation
├── solana_cpu_listener/       # Legacy CPU-based listener
└── whitelist/                 # Token whitelist configuration
```

## 🔧 Configuration

### Environment Variables
```bash
# Skip CUDA compilation (for development)
export SKIP_CUDA=1

# Specify CUDA architecture
export CUDA_ARCH=compute_80

# Enable debug logging
export RUST_LOG=debug
```

### Token Whitelist
Create `whitelist/mint_whitelist.txt` with allowed token mint addresses:
```
EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v  # USDC
So11111111111111111111111111111111111111112   # SOL
```

## 🚀 Performance Optimization

### GPU Memory Management
- The bot automatically manages GPU memory allocation
- Large datasets are processed in batches
- Memory is freed after each operation

### CUDA Kernel Optimization
- Kernels are compiled for your specific GPU architecture
- Block and grid sizes are optimized for your hardware
- Shared memory usage for improved performance

### Network Optimization
- WebSocket connections with connection pooling
- Rate limiting to avoid API throttling
- Automatic reconnection on connection loss

## 🐛 Troubleshooting

### Common Issues

**1. CUDA Compilation Errors**
```bash
# Check CUDA installation
nvcc --version

# Verify Visual Studio Build Tools (Windows)
where cl.exe

# Rebuild with verbose output
cargo clean && cargo build -vv
```

**2. GPU Detection Issues**
```bash
# Test GPU detection
cargo run -p hello-gpu

# Check NVIDIA drivers
nvidia-smi
```

**3. WebSocket Connection Issues**
```bash
# Test network connectivity
curl -I https://lite-api.jup.ag/swap/v1/quote

# Check TLS configuration
cargo run -p pool_indexer -- quote --pair SOL/USDC
```

**4. OpenSSL Build Errors (Windows)**
```powershell
# Run the automated setup script as Administrator
.\setup-openssl.ps1

# Or manually verify OpenSSL installation
openssl version
echo $env:OPENSSL_DIR
echo $env:OPENSSL_LIB_DIR
```

### Debug Mode
```bash
# Enable detailed logging
export RUST_LOG=debug
cargo run -p pool_indexer -- monitor --pair SOL/USDC --gpu
```

## 📈 Performance Benchmarks

| Operation | CPU (ms) | GPU (ms) | Speedup |
|-----------|----------|----------|---------|
| Pool Scoring (1000 pools) | 45 | 2.3 | 19.5x |
| SMA Calculation (10k points) | 12 | 0.8 | 15x |
| Quote Analysis (100 quotes) | 8.5 | 1.2 | 7.1x |

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## ⚠️ Disclaimer

This software is for educational and research purposes. Trading cryptocurrencies involves significant risk. Use at your own risk and never invest more than you can afford to lose.

## 🆘 Support

- **Issues**: [GitHub Issues](https://github.com/your-username/Solana-GPU-bots/issues)
- **Discussions**: [GitHub Discussions](https://github.com/your-username/Solana-GPU-bots/discussions)
- **Documentation**: [Wiki](https://github.com/your-username/Solana-GPU-bots/wiki)
