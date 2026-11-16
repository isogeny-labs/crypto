#include "spmv_io.hpp"
#include "spmv_math.cuh"

#include <cuda_runtime.h>
#include <iostream>

static void check_cuda(cudaError_t status, const char *operation) {
    if (status != cudaSuccess)
        throw std::runtime_error(
            std::string(operation) + ": " + cudaGetErrorString(status));
}

__global__ static void spmv_kernel(
    const std::uint64_t *__restrict__ row_ptr,
    const std::uint64_t *__restrict__ column_index,
    const std::uint64_t *__restrict__ coefficient,
    const std::uint64_t *__restrict__ x,
    std::uint64_t *__restrict__ y,
    std::uint64_t rows,
    std::uint64_t lanes,
    SpmvModulus mod) {
    const std::uint64_t task = (std::uint64_t)blockIdx.x * blockDim.x + threadIdx.x;
    if (task >= rows * lanes) return;
    const std::uint64_t row = task / lanes;
    const std::uint64_t lane = task % lanes;
    y[task] = spmv_task(row_ptr, column_index, coefficient, x, row, lane, lanes, mod);
}

template <class T>
static T *copy_to_device(const std::vector<T> &host) {
    T *device = nullptr;
    check_cuda(cudaMalloc(&device, host.size() * sizeof(T)), "cudaMalloc");
    check_cuda(
        cudaMemcpy(device, host.data(), host.size() * sizeof(T), cudaMemcpyHostToDevice),
        "cudaMemcpy host-to-device");
    return device;
}

int main(int argc, char **argv) {
    try {
        auto p = read_spmv1(input_path(argc, argv));
        // Reduce once on the host instead of once per lane per nonzero on
        // the device; a*b mod m depends only on a mod m and b mod m.
        for (auto &v : p.coefficient) v %= p.modulus;
        for (auto &v : p.x) v %= p.modulus;
        const SpmvModulus mod = spmv_modulus(p.modulus);
        auto *row_ptr = copy_to_device(p.row_ptr);
        auto *column_index = copy_to_device(p.column_index);
        auto *coefficient = copy_to_device(p.coefficient);
        auto *x = copy_to_device(p.x);
        std::uint64_t *y = nullptr;
        check_cuda(
            cudaMalloc(&y, p.rows * p.lanes * sizeof(std::uint64_t)),
            "cudaMalloc output");
        const std::uint64_t tasks = p.rows * p.lanes;
        spmv_kernel<<<(tasks + 255) / 256, 256>>>(
            row_ptr, column_index, coefficient, x, y, p.rows, p.lanes, mod);
        check_cuda(cudaGetLastError(), "launch SPMV1 kernel");
        check_cuda(cudaDeviceSynchronize(), "run SPMV1 kernel");
        std::vector<std::uint64_t> output(tasks);
        check_cuda(
            cudaMemcpy(output.data(), y, tasks * sizeof(std::uint64_t), cudaMemcpyDeviceToHost),
            "cudaMemcpy device-to-host");
        cudaFree(row_ptr);
        cudaFree(column_index);
        cudaFree(coefficient);
        cudaFree(x);
        cudaFree(y);
        print_spmv1(p, output);
        return 0;
    } catch (const std::exception &error) {
        std::cerr << error.what() << '\n';
        return 2;
    }
}
