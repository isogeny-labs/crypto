CXX ?= c++
NVCC ?= nvcc
ARCH ?= sm_80
CARGO ?= cargo
CXXFLAGS ?= -O3 -std=c++17 -Wall -Wextra
OUT := build
.PHONY: all cuda cuda-cached host test test-cuda syntax clean
all: cuda $(OUT)/paper-driver
cuda: $(OUT)/explicit-relations-cuda
cuda-cached: $(OUT)/explicit-relations-cached
host: $(OUT)/explicit-relations-host $(OUT)/paper-driver
$(OUT):
	mkdir -p $@
$(OUT)/explicit-relations-host: service.cpp host.hpp curve.cuh ../rootfind/fp64.cuh | $(OUT)
	$(CXX) $(CXXFLAGS) -Wno-unknown-pragmas service.cpp -o $@
$(OUT)/explicit-relations-cuda: service.cpp device.cuh host.hpp curve.cuh ../rootfind/fp64.cuh | $(OUT)
	$(NVCC) -O3 -std=c++17 -arch=$(ARCH) -DER_CUDA -x cu service.cpp -o $@
$(OUT)/explicit-relations-cached: service.cpp device.cuh host.hpp curve.cuh ../rootfind/fp64.cuh | $(OUT)
	$(NVCC) -O3 -std=c++17 -arch=$(ARCH) -DER_CUDA -DER_QUERY_CACHED -x cu service.cpp -o $@
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
test-cuda: all cuda-cached $(OUT)/query-check $(OUT)/query-cached-check
	$(OUT)/explicit-relations-cuda --self-test
	$(OUT)/paper-driver $(OUT)/explicit-relations-cuda --self-test
	$(OUT)/query-check
	$(OUT)/explicit-relations-cached --self-test
	$(OUT)/paper-driver $(OUT)/explicit-relations-cached --self-test
	$(OUT)/query-cached-check
clean:
	rm -rf $(OUT)
