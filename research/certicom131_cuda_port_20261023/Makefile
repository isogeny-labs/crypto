# CUDA port of the ECCp-131 and ECC2-131 rho walks.
#   make host-test     CPU run of the exact per-thread code with the correctness gates (no GPU needed)
#   make cuda-compile  clang CUDA front end -> PTX -> ptxas for several architectures (no GPU needed)
#   make cuda-run      on a machine with a CUDA toolkit and a GPU: build with nvcc, run the device parity
#                      gate and a timed launch (NOT run in the container that wrote this directory)
CXX      ?= g++
CLANG    ?= clang++
CUDAPATH ?= /usr/local/cuda
PTXAS    ?= $(CUDAPATH)/bin/ptxas
NVCC     ?= $(CUDAPATH)/bin/nvcc
SMARCH   ?= sm_120
ARCHS    ?= sm_80 sm_86 sm_89 sm_90
FLAGS    := -std=c++17 -Wno-unknown-cuda-version

host_emul: host_emul.cpp host_common.hpp gpu_walk.cuh
	$(CXX) -O2 -std=c++17 -march=native -DECC_NO_CUDA -Wall -Wno-unknown-pragmas -Wno-unused-function -o $@ host_emul.cpp

host-test: host_emul
	./host_emul --psteps 12 --bsteps 6

build/cudashim.h:
	mkdir -p build
	printf '#include <cuda_runtime.h>\nextern "C" cudaError_t cudaConfigureCall(dim3, dim3, size_t, cudaStream_t);\n' > $@

cuda-compile: build/cudashim.h
	$(CLANG) -x cuda --cuda-host-only --cuda-path=$(CUDAPATH) -nocudalib $(FLAGS) -O2 -include build/cudashim.h -c walk_cuda.cu -o build/walk_host.o
	for a in $(ARCHS); do \
	  $(CLANG) -x cuda --cuda-device-only --cuda-gpu-arch=$$a --cuda-path=$(CUDAPATH) $(FLAGS) -O3 -include build/cudashim.h -S walk_cuda.cu -o build/walk_$$a.ptx && \
	  $(PTXAS) -arch=$$a -O3 -v build/walk_$$a.ptx -o build/walk_$$a.cubin 2> build/ptxas_$$a.txt; \
	done

cuda-run:
	$(NVCC) -O3 -std=c++17 -arch=$(SMARCH) -o build/walk_cuda walk_cuda.cu
	./build/walk_cuda --threads 65536 --steps 8

clean:
	rm -rf build host_emul
.PHONY: host-test cuda-compile cuda-run clean
