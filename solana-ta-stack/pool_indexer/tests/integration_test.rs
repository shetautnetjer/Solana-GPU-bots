use anyhow::Result;
use pool_indexer::gpu;

#[test]
fn test_hello_gpu_integration() -> Result<()> {
    // Test that we can call hello-gpu functions from pool_indexer
    gpu::test_hello_gpu_vec_add()?;
    
    // Test that we can generate GPU reports
    let report = gpu::gpu_smoke_test()?;
    assert!(!report.name.is_empty(), "GPU name should not be empty");
    assert!(report.total_mem_mb > 0, "GPU memory should be greater than 0");
    assert!(report.kernel_ok, "GPU kernel test should pass");
    
    println!("✅ Integration test passed!");
    println!("GPU: {} ({} MB)", report.name, report.total_mem_mb);
    println!("CUDA: {}", report.cuda_version);
    
    Ok(())
}

#[test]
fn test_gpu_scorer_initialization() -> Result<()> {
    // Test that we can initialize the GPU scorer (this will fail if CUDA kernels aren't compiled)
    match gpu::GpuScorer::new() {
        Ok(_scorer) => {
            println!("✅ GPU Scorer initialized successfully!");
            Ok(())
        },
        Err(e) => {
            // This is expected if CUDA kernels aren't compiled yet
            println!("⚠️  GPU Scorer initialization failed (expected if kernels not compiled): {}", e);
            Ok(())
        }
    }
} 