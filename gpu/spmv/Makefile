CXX ?= c++
NVCC ?= nvcc
CXXFLAGS ?= -O3 -std=c++17 -Wall -Wextra -pedantic

.PHONY: all host-worker cuda-worker oracle-check clean

all: host-worker

host-worker: spmv_solve

cuda-worker: spmv_cuda

spmv_solve: spmv_solve.cpp spmv_io.hpp
	$(CXX) $(CXXFLAGS) -o $@ spmv_solve.cpp

spmv_cuda: spmv.cu spmv_io.hpp spmv_math.cuh
	$(NVCC) -O3 -std=c++17 -o $@ spmv.cu

# Host oracle for the CUDA worker's arithmetic (no GPU needed).
oracle-check: spmv-oracle-check
	./spmv-oracle-check

spmv-oracle-check: spmv-oracle-check.cpp spmv_math.cuh
	$(CXX) $(CXXFLAGS) -Wno-pedantic -o $@ spmv-oracle-check.cpp

clean:
	rm -f spmv_solve spmv_cuda spmv-oracle-check
