#include <cuda_runtime.h>

#include <iostream>

#include "fixtures.hpp"

namespace {

void checked(cudaError_t status, const char *what) {
    if (status != cudaSuccess) {
        std::cerr << what << ": " << cudaGetErrorString(status) << "\n";
        std::exit(2);
    }
}

__global__ void verify_kernel(const certicom131::Fixtures *fixtures, int *code) {
    if (blockIdx.x == 0 && threadIdx.x == 0) *code = certicom131::verify_all(*fixtures);
}

} // namespace

int main() {
    const certicom131::Fixtures host = certicom131::load_fixtures();
    certicom131::Fixtures *device = nullptr;
    int *device_code = nullptr;
    int code = -1;
    checked(cudaMalloc(&device, sizeof(host)), "cudaMalloc fixtures");
    checked(cudaMalloc(&device_code, sizeof(code)), "cudaMalloc code");
    checked(cudaMemcpy(device, &host, sizeof(host), cudaMemcpyHostToDevice), "cudaMemcpy fixtures");
    verify_kernel<<<1, 1>>>(device, device_code);
    checked(cudaGetLastError(), "verify_kernel launch");
    checked(cudaMemcpy(&code, device_code, sizeof(code), cudaMemcpyDeviceToHost), "cudaMemcpy code");
    checked(cudaFree(device_code), "cudaFree code");
    checked(cudaFree(device), "cudaFree fixtures");
    if (code != 0) {
        std::cerr << "CERTICOM131 CUDA VERIFY: FAIL code=" << code << "\n";
        return 1;
    }
    std::cout << "CERTICOM131 CUDA VERIFY: PASS"
              << " prime_field_vectors=256 binary_field_vectors=256"
              << " scalar_vectors_per_curve=16 subgroup_checks=4\n";
    return 0;
}
