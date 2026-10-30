CXX ?= c++
NVCC ?= nvcc
ARCH ?= sm_80
CARGO ?= cargo
CXXFLAGS ?= -O3 -std=c++17 -Wall -Wextra
OUT := build
.PHONY: all cuda cuda-cached cuda-compact cuda-wide-index cuda-dense host test test-cuda syntax clean
all: cuda $(OUT)/paper-driver
cuda: $(OUT)/explicit-relations-cuda
cuda-cached: $(OUT)/explicit-relations-cached
cuda-compact: $(OUT)/explicit-relations-compact
cuda-wide-index: $(OUT)/explicit-relations-wide-index
cuda-dense: $(OUT)/explicit-relations-dense
host: $(OUT)/explicit-relations-host $(OUT)/paper-driver
$(OUT):
	mkdir -p $@
$(OUT)/explicit-relations-host: service.cpp host.hpp curve.cuh ../rootfind/fp64.cuh | $(OUT)
	$(CXX) $(CXXFLAGS) -Wno-unknown-pragmas service.cpp -o $@
$(OUT)/explicit-relations-cuda: service.cpp device.cuh host.hpp curve.cuh ../rootfind/fp64.cuh | $(OUT)
	$(NVCC) -O3 -std=c++17 -arch=$(ARCH) -DER_CUDA -x cu service.cpp -o $@
$(OUT)/explicit-relations-cached: service.cpp device.cuh host.hpp curve.cuh ../rootfind/fp64.cuh | $(OUT)
	$(NVCC) -O3 -std=c++17 -arch=$(ARCH) -DER_CUDA -DER_QUERY_CACHED -x cu service.cpp -o $@
$(OUT)/explicit-relations-compact: service.cpp device.cuh host.hpp curve.cuh ../rootfind/fp64.cuh | $(OUT)
	$(NVCC) -O3 -std=c++17 -arch=$(ARCH) -DER_CUDA -DER_COMPACT_ROWS -x cu service.cpp -o $@
$(OUT)/explicit-relations-wide-index: service.cpp device.cuh host.hpp curve.cuh ../rootfind/fp64.cuh | $(OUT)
	$(NVCC) -O3 -std=c++17 -arch=$(ARCH) -DER_CUDA -DER_COMPACT_ROWS -DER_WIDE_INDEX -x cu service.cpp -o $@
$(OUT)/explicit-relations-dense: service.cpp device.cuh host.hpp curve.cuh ../rootfind/fp64.cuh | $(OUT)
	$(NVCC) -O3 -std=c++17 -arch=$(ARCH) -DER_CUDA -DER_COMPACT_ROWS -DER_WIDE_INDEX -DER_DENSE_ROWS -x cu service.cpp -o $@
$(OUT)/query-dense-check: query-check.cu device.cuh host.hpp curve.cuh ../rootfind/fp64.cuh | $(OUT)
	$(NVCC) -O3 -std=c++17 -arch=$(ARCH) -DER_COMPACT_ROWS -DER_WIDE_INDEX -DER_DENSE_ROWS query-check.cu -o $@
$(OUT)/query-wide-check: query-check.cu device.cuh host.hpp curve.cuh ../rootfind/fp64.cuh | $(OUT)
	$(NVCC) -O3 -std=c++17 -arch=$(ARCH) -DER_COMPACT_ROWS -DER_WIDE_INDEX query-check.cu -o $@
$(OUT)/query-compact-check: query-check.cu device.cuh host.hpp curve.cuh ../rootfind/fp64.cuh | $(OUT)
	$(NVCC) -O3 -std=c++17 -arch=$(ARCH) -DER_COMPACT_ROWS query-check.cu -o $@
$(OUT)/query-check: query-check.cu device.cuh host.hpp curve.cuh ../rootfind/fp64.cuh | $(OUT)
	$(NVCC) -O3 -std=c++17 -arch=$(ARCH) query-check.cu -o $@
$(OUT)/query-cached-check: query-check.cu device.cuh host.hpp curve.cuh ../rootfind/fp64.cuh | $(OUT)
	$(NVCC) -O3 -std=c++17 -arch=$(ARCH) -DER_QUERY_CACHED query-check.cu -o $@
$(OUT)/paper-driver: paper_driver.rs Cargo.toml Cargo.lock ../../src/cryptanalysis/hashed_ecdl.rs ../../crates/hashed-ecdl-tensor/Cargo.toml $(wildcard ../../crates/hashed-ecdl-tensor/src/*.rs ../../crates/hashed-ecdl-tensor/src/contraction/*.rs) | $(OUT)
	$(CARGO) build --locked --release --manifest-path Cargo.toml --target-dir $(OUT)/cargo
	cp $(OUT)/cargo/release/paper-driver $@
syntax:
	$(CXX) -std=c++17 -Wall -Wextra -Werror -Wno-unknown-pragmas -fsyntax-only service.cpp
test: host
	$(CARGO) test --locked --release --manifest-path Cargo.toml --target-dir $(OUT)/cargo
	$(OUT)/explicit-relations-host --self-test
	$(OUT)/paper-driver $(OUT)/explicit-relations-host --self-test
test-cuda: all cuda-cached cuda-compact $(OUT)/query-check $(OUT)/query-cached-check $(OUT)/query-compact-check
	$(OUT)/explicit-relations-cuda --self-test
	$(OUT)/paper-driver $(OUT)/explicit-relations-cuda --self-test
	$(OUT)/query-check
	$(OUT)/explicit-relations-cached --self-test
	$(OUT)/paper-driver $(OUT)/explicit-relations-cached --self-test
	$(OUT)/query-cached-check
	$(OUT)/explicit-relations-compact --self-test
	$(OUT)/paper-driver $(OUT)/explicit-relations-compact --self-test
	$(OUT)/query-compact-check
clean:
	rm -rf $(OUT)
