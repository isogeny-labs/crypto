CXX ?= c++
NVCC ?= nvcc
CXXFLAGS ?= -O3 -std=c++17 -Wall -Wextra -pedantic
# Without -arch, nvcc 12.x emits only sm_52 code and the driver JIT-compiles
# it on every newer GPU. all-major embeds native SASS for each major
# architecture (sm_89 runs the sm_80 SASS) plus PTX; ARCH=sm_80 etc. for one.
ARCH ?= all-major
NVCCFLAGS ?= -O3 -std=c++17 -arch=$(ARCH)

.PHONY: all host-worker cuda-worker oracle-check clean

all: host-worker

host-worker: spmv_solve

cuda-worker: spmv_cuda

spmv_solve: spmv_solve.cpp spmv_io.hpp
	$(CXX) $(CXXFLAGS) -o $@ spmv_solve.cpp

spmv_cuda: spmv.cu spmv_io.hpp spmv_math.cuh
	$(NVCC) $(NVCCFLAGS) -o $@ spmv.cu

# Host oracle for the CUDA worker's arithmetic (no GPU needed).
oracle-check: spmv-oracle-check
	./spmv-oracle-check

spmv-oracle-check: spmv-oracle-check.cpp spmv_math.cuh
	$(CXX) $(CXXFLAGS) -Wno-pedantic -o $@ spmv-oracle-check.cpp

clean:
	rm -f spmv_solve spmv_cuda spmv-oracle-check
