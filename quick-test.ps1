# Quick Test Script for Solana GPU Trading Bot
# This script runs basic functionality tests to verify the installation

Write-Host "🧪 Solana GPU Trading Bot - Quick Test" -ForegroundColor Green
Write-Host "=====================================" -ForegroundColor Green

# Test 1: GPU Detection
Write-Host "`n🔍 Test 1: GPU Detection" -ForegroundColor Yellow
try {
    $output = cargo run -p hello-gpu 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ GPU detection successful" -ForegroundColor Green
        Write-Host $output -ForegroundColor Gray
    } else {
        Write-Host "❌ GPU detection failed" -ForegroundColor Red
        Write-Host $output -ForegroundColor Red
    }
} catch {
    Write-Host "❌ GPU detection failed with exception" -ForegroundColor Red
}

# Test 2: GPU Smoke Test
Write-Host "`n🔍 Test 2: GPU Smoke Test" -ForegroundColor Yellow
try {
    $output = cargo run -p pool_indexer -- gpu-test 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ GPU smoke test passed" -ForegroundColor Green
        Write-Host $output -ForegroundColor Gray
    } else {
        Write-Host "❌ GPU smoke test failed" -ForegroundColor Red
        Write-Host $output -ForegroundColor Red
    }
} catch {
    Write-Host "❌ GPU smoke test failed with exception" -ForegroundColor Red
}

# Test 3: Quote Test (without GPU)
Write-Host "`n🔍 Test 3: Quote Test" -ForegroundColor Yellow
try {
    $output = cargo run -p pool_indexer -- quote --pair SOL/USDC --amount 1000000000 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Quote test successful" -ForegroundColor Green
        Write-Host $output -ForegroundColor Gray
    } else {
        Write-Host "❌ Quote test failed" -ForegroundColor Red
        Write-Host $output -ForegroundColor Red
    }
} catch {
    Write-Host "❌ Quote test failed with exception" -ForegroundColor Red
}

# Test 4: Hello-GPU Vector Addition
Write-Host "`n🔍 Test 4: Hello-GPU Vector Addition" -ForegroundColor Yellow
try {
    $output = cargo test -p hello-gpu 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Hello-GPU tests passed" -ForegroundColor Green
    } else {
        Write-Host "❌ Hello-GPU tests failed" -ForegroundColor Red
        Write-Host $output -ForegroundColor Red
    }
} catch {
    Write-Host "❌ Hello-GPU tests failed with exception" -ForegroundColor Red
}

Write-Host "`n📊 Test Summary" -ForegroundColor Green
Write-Host "==============" -ForegroundColor Green
Write-Host "All basic functionality tests completed!" -ForegroundColor White

Write-Host "`n🎯 Next Steps:" -ForegroundColor Cyan
Write-Host "1. Run full build: .\build.ps1 -Release" -ForegroundColor White
Write-Host "2. Monitor pools: cargo run -p pool_indexer -- monitor --pair SOL/USDC --gpu" -ForegroundColor White
Write-Host "3. Check logs: Get-Content pool_updates.csv" -ForegroundColor White

Write-Host "`n✅ Quick test completed!" -ForegroundColor Green 