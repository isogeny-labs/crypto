CXX ?= g++
NVCC ?= nvcc
ARCH ?= sm_120
NIST_PTX ?= 1
MAXREG ?= 0
TABLE_SHARED ?= 0
BEST2 ?= 0
CXXFLAGS ?= -O3 -std=c++17 -Wall -Wextra -Wno-unknown-pragmas -DRHO_BEST2=$(BEST2)
NVCCFLAGS ?= -O3 -std=c++17 -arch=$(ARCH) -lineinfo -Xptxas=-v -DRHO_TABLE_SHARED=$(TABLE_SHARED) -DRHO_BEST2=$(BEST2)
ifneq ($(MAXREG),0)
NVCCFLAGS += -maxrregcount=$(MAXREG)
endif

.PHONY: all test test-best2 bench ptx clean
all: test
# Test binaries are named per BEST2 so changing it rebuilds them.
test: test_cpu_b$(BEST2) test_rho_b$(BEST2)
	./test_cpu_b$(BEST2)
	./test_rho_b$(BEST2)
test_cpu_b$(BEST2): test_cpu.cpp nist_fields.cuh nist_ptx.cuh nist_solinas.cuh
	$(CXX) $(CXXFLAGS) -o $@ test_cpu.cpp
# Rho engine: curve/walk/solver headers verified on the CPU (no GPU needed).
test_rho_b$(BEST2): test_rho.cpp nist_fields.cuh nist_curve.cuh nist_rho.cuh nist_rho_host.hpp
	$(CXX) $(CXXFLAGS) -o $@ test_rho.cpp
# Independent full-size curve oracle (OpenSSL development headers required).
test-best2: test_best2_b$(BEST2)
	./test_best2_b$(BEST2)
test_best2_b$(BEST2): test_best2.cpp nist_fields.cuh nist_curve.cuh nist_rho.cuh
	$(CXX) $(CXXFLAGS) -o $@ test_best2.cpp -lcrypto
best2_stats: best2_stats.cpp
	$(CXX) $(CXXFLAGS) -o $@ best2_stats.cpp
bench: bench.cu nist_fields.cuh nist_ptx.cuh nist_solinas.cuh nist_curve.cuh nist_rho.cuh
	$(NVCC) $(NVCCFLAGS) -DNIST_PTX=$(NIST_PTX) -o $@ bench.cu
ptx: bench.cu nist_fields.cuh nist_ptx.cuh nist_solinas.cuh nist_curve.cuh nist_rho.cuh
	$(NVCC) $(NVCCFLAGS) -DNIST_PTX=$(NIST_PTX) -ptx -o bench.ptx bench.cu
clean:
	rm -f test_cpu test_cpu_b* test_rho test_rho_b* test_best2_b* best2_stats bench bench-b* bench.ptx
