# gpu/rootfind -- root-finding 2-decomposition oracle: CUDA + host emulation.
#
#   make test            CPU verification of the shared headers (no GPU, no nvcc)
#   make bench-host-inv  public-input host Fp64 inversion + batch benchmark
#   make bench-mul256   compare old versus native-128 host P-256 MAC
#   make bench-from256  compare generic vs direct REDC P-256 from_mont
#   make bench-sparse256  compare generic versus sparse-modulus P-256 REDC
#   make bench-pair-sweep  test pair-table and S3 sweep savings
#   make bench-carry256   compare host P-256 limb carry/borrow chains
#   make bench-subneg256  compare host P-256 modular subtraction and negation
#   make bench-sub64     compare Fp64 host modular subtraction
#   make bench-tail-core  compare padded vs short-tail Fp64 batch inversion
#   make bench-host-inv256  P-256 sparse-exponent host inverse + batch benchmark
#   make bench-chain256     previous radix16 vs 13-multiply P-256 inversion
#   make bench-thread-map   compare uint64 vs guarded uint32 quotient/remainder
#   make lib             librootfind_oracle.a, emulation backend only
#                        (what gpu/rootfind/rs/build.rs builds for the gpu-rootfind cargo crate)
#   make lib-cuda        librootfind_cuda.a, emulation + CUDA backend (needs nvcc)
#   make serve           rootfind_serve, host emulation (the IC oracle's worker; no GPU)
#   make serve-cuda      rootfind_serve_cuda, the same worker on the CUDA backend (needs nvcc)
#   make bench           CUDA driver: selftest vs emulation + throughput (needs nvcc)
#   make bench ARCH=sm_100      datacenter Blackwell; default sm_120 (RTX PRO 6000 Blackwell)
#   make ptxas-info      per-kernel registers / spills / smem for the chosen ARCH
#   make clean
#
# Conventions follow gpu/ecc: every arithmetic function is
# `__host__ __device__`, so `make test` compiles the *same* headers the
# kernels use.  Blackwell flags: -gencode arch=compute_120,code=sm_120 for
# native SASS plus -gencode arch=compute_120,code=compute_120 for a PTX
# fallback that JIT-compiles on future architectures.

CXX      ?= c++
NVCC     ?= nvcc
AR       ?= ar
CXXFLAGS ?= -O2 -std=c++17 -Wall -Wextra -Wno-unused-parameter -Wno-unknown-pragmas -pthread
ARCH     ?= sm_120
# compute_XX is derived from sm_XX
COMPUTE  := $(subst sm_,compute_,$(ARCH))
GENCODE  ?= -gencode arch=$(COMPUTE),code=$(ARCH) -gencode arch=$(COMPUTE),code=$(COMPUTE)
NVCCFLAGS ?= -O3 -std=c++17 -lineinfo $(GENCODE) -Xcompiler -pthread
# Launch-bounds knobs (see rootfind_cuda.cu header): block size and the
# minimum resident blocks per SM ptxas must honour (0 = unconstrained).
RF_BLOCK      ?= 128
RF_MIN_BLOCKS ?= 0
# 1: sampled clock64() phase profile in the kernels (`./bench profile`); 0 = off
RF_PROFILE ?= 0
# lookup variants (rootfind.cuh): Eytzinger sorted search, packed hash slots,
# paired eps lookups; on by default (A100: 1.12x at the best configuration of
# both workloads, research/ic_gpu_rootfind_lookup_20261114); 0 = old path
RF_SORTED_EYT ?= 1
RF_HASH_PACKED ?= 1
RF_PROBE_PAIR ?= 1
# Eytzinger prefetch distance in levels (0 = off) and L2-only prefetch
RF_EYT_PREFETCH ?= 0
RF_EYT_PREFETCH_L2 ?= 0
RF_DEFS := -DRF_BLOCK=$(RF_BLOCK) -DRF_MIN_BLOCKS=$(RF_MIN_BLOCKS) -DRF_PROFILE=$(RF_PROFILE) \
           -DRF_SORTED_EYT=$(RF_SORTED_EYT) -DRF_HASH_PACKED=$(RF_HASH_PACKED) -DRF_PROBE_PAIR=$(RF_PROBE_PAIR) \
           -DRF_EYT_PREFETCH=$(RF_EYT_PREFETCH) -DRF_EYT_PREFETCH_L2=$(RF_EYT_PREFETCH_L2)

HDRS := fp64.cuh fp256x64.cuh rootfind.cuh rootfind_host.hpp rootfind_api.h

.PHONY: all test lib lib-cuda serve serve-cuda bench bench-host-inv bench-host-inv256 bench-batch-core bench-sparse256 bench-carry256 bench-subneg256 bench-sub64 bench-tail-core bench-tail-threshold bench-tail27 bench-pair-sweep bench-chain256 bench-thread-map bench-from256 ptxas-info clean

all: test

test_cpu: test_cpu.cpp rootfind_api.cpp $(HDRS)
	$(CXX) $(CXXFLAGS) $(RF_DEFS) -DRF_HAVE_CUDA=0 -o $@ test_cpu.cpp rootfind_api.cpp

test: test_cpu
	./test_cpu

# --- CPU-only inversion microbenchmark, CUDA not needed ----------------
bench_host_inv: bench_host_inv.cpp fp64.cuh
	$(CXX) $(CXXFLAGS) -o $@ bench_host_inv.cpp

bench-host-inv: bench_host_inv
	./bench_host_inv

bench_host_inv256: bench_host_inv256.cpp fp256x64.cuh fp64.cuh
	$(CXX) $(CXXFLAGS) -o $@ bench_host_inv256.cpp

bench-host-inv256: bench_host_inv256
	./bench_host_inv256

# Compare frozen pre-optimization inversion cores against the current loops.
bench_batch_core: bench_batch_core.cpp fp64.cuh
	$(CXX) $(CXXFLAGS) -o $@ bench_batch_core.cpp

bench-batch-core: bench_batch_core
	./bench_batch_core

# Skip the generic multiply-by-one in host P-256 Montgomery conversion.
bench_from256: bench_from256.cpp fp256x64.cuh fp64.cuh
	$(CXX) $(CXXFLAGS) -o $@ bench_from256.cpp

bench-from256: bench_from256
	./bench_from256

# Compare original versus native-128 host P-256 Montgomery MAC.
bench_mul256: bench_mul256.cpp fp256x64.cuh fp64.cuh
	$(CXX) $(CXXFLAGS) -o $@ bench_mul256.cpp

bench-mul256: bench_mul256
	./bench_mul256

# Specialize the host P-256 modulus reduction without changing CUDA.
bench_sparse256: bench_sparse256.cpp fp256x64.cuh fp64.cuh
	$(CXX) $(CXXFLAGS) -o $@ bench_sparse256.cpp

bench-sparse256: bench_sparse256
	./bench_sparse256

# Compare original explicit carries against x86 carry-flag chains on host.
bench_carry256: bench_carry256.cpp fp256x64.cuh fp64.cuh
	$(CXX) $(CXXFLAGS) -o $@ bench_carry256.cpp

bench-carry256: bench_carry256
	./bench_carry256

# Frozen previous P-256 modular subtract/negate versus the current header.
bench_subneg256: bench_subneg256.cpp fp256x64.cuh fp64.cuh
	$(CXX) $(CXXFLAGS) -o $@ bench_subneg256.cpp

bench-subneg256: bench_subneg256
	./bench_subneg256

# Fp64 branchy ternary vs branchless host modular subtraction.
bench_sub64: bench_sub64.cpp fp64.cuh
	$(CXX) $(CXXFLAGS) -o $@ bench_sub64.cpp

bench-sub64: bench_sub64
	./bench_sub64

# Short-tail CPU inversion avoids multiplications by padded Montgomery one.
bench_tail_core: bench_tail_core.cpp fp64.cuh
	$(CXX) $(CXXFLAGS) -o $@ bench_tail_core.cpp

bench-tail-core: bench_tail_core
	./bench_tail_core

# Host tail threshold: prior <=16/32 vs proposed <=24/32.
bench_tail_threshold: bench_tail_threshold.cpp fp64.cuh
	$(CXX) $(CXXFLAGS) -o $@ bench_tail_threshold.cpp

bench-tail-threshold: bench_tail_threshold
	./bench_tail_threshold

# Host 32-element batch: previous 24 cutoff vs new 27 cutoff.
bench_tail27: bench_tail27.cpp fp64.cuh
	$(CXX) $(CXXFLAGS) -o $@ bench_tail27.cpp

bench-tail27: bench_tail27
	./bench_tail27

# Pair table and 2-sign rootfinder S3 sweep: frozen-before versus actual header.
bench_pair_sweep: bench_pair_sweep.cpp rootfind.cuh fp64.cuh fp256x64.cuh rootfind_api.h
	$(CXX) $(CXXFLAGS) -o $@ bench_pair_sweep.cpp

bench-pair-sweep: bench_pair_sweep
	./bench_pair_sweep

# Fixed P-256 chain: 35 vs 13 multiplications, actual header.
bench_chain256: bench_chain256.cpp fp256x64.cuh fp64.cuh
	$(CXX) $(CXXFLAGS) -o $@ bench_chain256.cpp

bench-chain256: bench_chain256
	./bench_chain256

# Fixed 64-bit mapping vs host 32-bit fast path for per-thread decomposition.
bench_thread_map: bench_thread_map.cpp rootfind.cuh fp64.cuh fp256x64.cuh rootfind_api.h
	$(CXX) $(CXXFLAGS) -o $@ bench_thread_map.cpp

bench-thread-map: bench_thread_map
	./bench_thread_map

# --- emulation-only static library (no CUDA toolchain required) -------
rootfind_api_emul.o: rootfind_api.cpp $(HDRS)
	$(CXX) $(CXXFLAGS) $(RF_DEFS) -DRF_HAVE_CUDA=0 -c -o $@ rootfind_api.cpp

librootfind_oracle.a: rootfind_api_emul.o
	$(AR) rcs $@ $^

lib: librootfind_oracle.a

# --- CUDA-enabled static library --------------------------------------
rootfind_api_cuda.o: rootfind_api.cpp $(HDRS)
	$(CXX) $(CXXFLAGS) $(RF_DEFS) -DRF_HAVE_CUDA=1 -c -o $@ rootfind_api.cpp

rootfind_cuda.o: rootfind_cuda.cu $(HDRS)
	$(NVCC) $(NVCCFLAGS) $(RF_DEFS) -DRF_HAVE_CUDA=1 -c -o $@ rootfind_cuda.cu

librootfind_cuda.a: rootfind_api_cuda.o rootfind_cuda.o
	$(AR) rcs $@ $^

lib-cuda: librootfind_cuda.a

# --- persistent worker for the IC oracle rootfind-subtract-gpu ---------
rootfind_serve: rootfind_serve.cpp rootfind_api.cpp $(HDRS)
	$(CXX) $(CXXFLAGS) $(RF_DEFS) -DRF_HAVE_CUDA=0 -o $@ rootfind_serve.cpp rootfind_api.cpp

serve: rootfind_serve

rootfind_serve_cuda: rootfind_serve.cpp rootfind_api_cuda.o rootfind_cuda.o $(HDRS)
	$(NVCC) $(NVCCFLAGS) $(RF_DEFS) -DRF_HAVE_CUDA=1 -o $@ rootfind_serve.cpp rootfind_api_cuda.o rootfind_cuda.o

serve-cuda: rootfind_serve_cuda

# --- CUDA driver -------------------------------------------------------
bench: bench.cu rootfind_api_cuda.o rootfind_cuda.o $(HDRS)
	$(NVCC) $(NVCCFLAGS) $(RF_DEFS) -DRF_HAVE_CUDA=1 -o $@ bench.cu rootfind_api_cuda.o rootfind_cuda.o

ptxas-info: rootfind_cuda.cu $(HDRS)
	$(NVCC) $(NVCCFLAGS) $(RF_DEFS) -DRF_HAVE_CUDA=1 -Xptxas -v -c -o /dev/null rootfind_cuda.cu

clean:
	rm -f test_cpu bench bench_tail_threshold bench_tail27 bench_host_inv bench_host_inv256 bench_batch_core bench_pair_sweep bench_chain256 bench_thread_map bench_from256 rootfind_serve rootfind_serve_cuda *.o *.a
