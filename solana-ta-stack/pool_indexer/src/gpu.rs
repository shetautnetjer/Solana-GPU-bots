use anyhow::{Result, anyhow};
use cust::prelude::*;
use serde::{Serialize, Deserialize};
use log::{info, debug};

#[derive(Debug, Serialize, Deserialize)]
pub struct GpuReport {
    pub name: String,
    pub compute_capability: (u32, u32),
    pub total_mem_mb: u64,
    pub driver_version: String,
    pub cuda_version: String,
    pub kernel_ok: bool,
}

pub fn gpu_smoke_test() -> Result<GpuReport> {
    // Use hello-gpu's report generation
    let hello_report = hello_gpu::generate_report()
        .map_err(|e| anyhow!("Failed to generate GPU report: {}", e))?;
    
    // Convert hello-gpu's report to our format
    Ok(GpuReport {
        name: hello_report.name,
        compute_capability: (
            hello_report.sm_major_minor.0 as u32,
            hello_report.sm_major_minor.1 as u32
        ),
        total_mem_mb: hello_report.total_mem_mb,
        driver_version: hello_report.driver_version,
        cuda_version: format!("{}.{}", 
            hello_report.runtime_version.0,
            hello_report.runtime_version.1
        ),
        kernel_ok: hello_report.kernel_ok,
    })
}

pub struct GpuScorer {
    module: Module,
    stream: Stream,
    device: Device,
}

impl GpuScorer {
    pub fn new() -> Result<Self> {
        // Initialize CUDA using hello-gpu's approach
        cust::init(CudaFlags::empty())?;
        let device = Device::get_device(0)?;
        let _ctx = Context::new(device)?;
        
        // Try to load our compiled PTX
        let ptx = if let Ok(ptx_path) = std::env::var("KERNEL_SCORE_PTX") {
            // Use the PTX compiled by build.rs
            std::fs::read_to_string(ptx_path)?
        } else {
            // Fallback: try to find it in OUT_DIR
            let out_dir = std::env::var("OUT_DIR").unwrap_or_else(|_| "target/debug/build".to_string());
            let ptx_path = std::path::PathBuf::from(out_dir).join("score.ptx");
            
            if ptx_path.exists() {
                std::fs::read_to_string(ptx_path)?
            } else {
                // Ultimate fallback: include a pre-compiled PTX
                return Err(anyhow!("No PTX file found. Please ensure CUDA kernels are compiled."));
            }
        };
        
        let module = Module::from_ptx(&ptx, &[])?;
        let stream = Stream::new(StreamFlags::DEFAULT, None)?;
        
        Ok(Self { module, stream, device })
    }
    
    pub fn score_batch(
        &self,
        prices: &[f32],
        volumes: &[f32],
        weights: &[f32; 2],
    ) -> Result<Vec<f32>> {
        let n = prices.len();
        if n == 0 || volumes.len() != n {
            return Err(anyhow!("Price and volume vectors must be non-empty and of equal length."));
        }
        if weights.len() != 2 {
            return Err(anyhow!("Weights array must contain exactly 2 elements."));
        }
        debug!("Scoring batch of {} records on GPU.", n);

        // Copy data to device using the instance stream
        let d_prices = DeviceBuffer::from_slice_async(prices, &self.stream)?;
        let d_volumes = DeviceBuffer::from_slice_async(volumes, &self.stream)?;
        let d_weights = DeviceBuffer::from_slice_async(weights, &self.stream)?;
        
        // Allocate output buffer
        let mut d_scores = DeviceBuffer::<f32>::new(n)?;

        // Launch kernel
        let block_size = 256;
        let grid_size = (n as u32 + block_size - 1) / block_size;
        
        let kernel = self.module.get_function("score_kernel")?;
        
        unsafe {
            launch!(
                kernel<<<grid_size, block_size, 0, self.stream>>>(
                    d_prices.as_device_ptr(),
                    d_volumes.as_device_ptr(),
                    d_scores.as_device_ptr(),
                    n as i32,
                    d_weights.as_device_ptr()
                )
            )?;
        }

        self.stream.synchronize()?;

        // Copy results back
        let mut h_scores = vec![0.0f32; n];
        d_scores.copy_to(&mut h_scores)?;
        
        debug!("GPU scoring complete. First score: {}", h_scores.first().unwrap_or(&0.0));
        Ok(h_scores)
    }
    
    pub fn compute_sma(&self, values: &[f32], window: usize) -> Result<Vec<f32>> {
        let n = values.len();
        
        // Allocate device memory
        let d_values = DeviceBuffer::from_slice(values)?;
        let mut d_sma = unsafe { DeviceBuffer::<f32>::uninitialized(n)? };
        
        // Get the SMA kernel function
        let func = self.module.get_function("sma_kernel")?;
        
        // Launch kernel
        let block_size = 256;
        let grid_size = ((n + block_size - 1) / block_size) as u32;
        
        // Fixed launch! macro syntax
        let stream_inner = &self.stream;
        unsafe {
            launch!(
                func<<<grid_size, block_size as u32, 0, stream_inner>>>(
                    d_values.as_device_ptr(),
                    d_sma.as_device_ptr(),
                    n as u32,
                    window as u32
                )
            )?;
        }
        
        self.stream.synchronize()?;
        
        // Copy results back
        Ok(d_sma.as_host_vec()?)
    }
}

/// Utility function to test GPU with hello-gpu's vec_add
pub fn test_hello_gpu_vec_add() -> Result<()> {
    let a = vec![1.0f32; 1024];
    let b = vec![2.0f32; 1024];
    
    let c = hello_gpu::vec_add_gpu(&a, &b)
        .map_err(|e| anyhow!("GPU vec_add failed: {}", e))?;
    
    // Verify results
    for (i, &val) in c.iter().enumerate() {
        if (val - 3.0).abs() > 1e-6 {
            return Err(anyhow!("GPU computation error at index {}: expected 3.0, got {}", i, val));
        }
    }
    
    println!("✅ hello-gpu vec_add test passed!");
    Ok(())
}
