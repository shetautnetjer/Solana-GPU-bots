#include <cuda_runtime.h>
#include <stdio.h>

extern "C" __global__ void score_kernel(
    const float* prices,
    const float* volumes,
    float* scores,
    int n,
    const float* weights
) {
    int idx = blockIdx.x * blockDim.x + threadIdx.x;

    if (idx < n) {
        if (idx > 0 && prices[idx-1] > 1e-9 && volumes[idx-1] > 1e-9) {
            // Factor 1: Price change (momentum)
            float price_change = (prices[idx] - prices[idx-1]) / prices[idx-1];
            
            // Factor 2: Volume change
            float volume_change = (volumes[idx] - volumes[idx-1]) / volumes[idx-1];

            // Final weighted score
            scores[idx] = (weights[0] * price_change) + (weights[1] * volume_change);
        } else {
            scores[idx] = 0.0f;
        }
    }
}

// Additional kernels for technical indicators
extern "C" __global__ void sma_kernel(
    const float* values,
    float* sma,
    uint32_t n,
    uint32_t window
) {
    uint32_t idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (idx >= n || idx < window - 1) return;
    
    float sum = 0.0f;
    for (uint32_t i = 0; i < window; i++) {
        sum += values[idx - i];
    }
    sma[idx] = sum / window;
}
