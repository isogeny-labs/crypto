CXX ?= c++
NVCC ?= nvcc
ARCH ?= sm_80
CARGO ?= cargo
CXXFLAGS ?= -O3 -std=c++17 -Wall -Wextra
OUT := build
.PHONY: all cuda host test test-cuda syntax clean
all: cuda $(OUT)/paper-driver
cuda: $(OUT)/explicit-relations-cuda
host: $(OUT)/explicit-relations-host $(OUT)/paper-driver
$(OUT):
	mkdir -p $@
$(OUT)/explicit-relations-host: service.cpp host.hpp curve.cuh ../rootfind/fp64.cuh | $(OUT)
	$(CXX) $(CXXFLAGS) -Wno-unknown-pragmas service.cpp -o $@
$(OUT)/explicit-relations-cuda: service.cpp device.cuh host.hpp curve.cuh ../rootfind/fp64.cuh | $(OUT)
	$(NVCC) -O3 -std=c++17 -arch=$(ARCH) -DER_CUDA -x cu service.cpp -o $@
$(OUT)/paper-driver: paper_driver.rs Cargo.toml Cargo.lock ../../src/cryptanalysis/hashed_ecdl.rs ../../crates/hashed-ecdl-tensor/Cargo.toml $(wildcard ../../crates/hashed-ecdl-tensor/src/*.rs ../../crates/hashed-ecdl-tensor/src/contraction/*.rs) | $(OUT)
	$(CARGO) build --locked --release --manifest-path Cargo.toml --target-dir $(OUT)/cargo
	cp $(OUT)/cargo/release/paper-driver $@
syntax:
	$(CXX) -std=c++17 -Wall -Wextra -Werror -Wno-unknown-pragmas -fsyntax-only service.cpp
test: host
	$(CARGO) test --locked --release --manifest-path Cargo.toml --target-dir $(OUT)/cargo
	$(OUT)/explicit-relations-host --self-test
	$(OUT)/paper-driver $(OUT)/explicit-relations-host --self-test
test-cuda: all
	$(OUT)/explicit-relations-cuda --self-test
	$(OUT)/paper-driver $(OUT)/explicit-relations-cuda --self-test
clean:
	rm -rf $(OUT)
