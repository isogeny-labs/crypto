CXX ?= c++
NVCC ?= nvcc
ARCH ?= sm_80
CARGO ?= cargo
CXXFLAGS ?= -O3 -std=c++17 -Wall -Wextra
OUT := build
.PHONY: all cuda cuda-cached cuda-compact cuda-wide-index cuda-dense cuda-dense-cub host test test-cuda syntax clean
all: cuda $(OUT)/paper-driver
cuda: $(OUT)/explicit-relations-cuda
cuda-cached: $(OUT)/explicit-relations-cached
cuda-compact: $(OUT)/explicit-relations-compact
cuda-wide-index: $(OUT)/explicit-relations-wide-index
cuda-dense: $(OUT)/explicit-relations-dense
cuda-dense-cub: $(OUT)/explicit-relations-dense-cub
host: $(OUT)/explicit-relations-host $(OUT)/paper-driver
$(OUT):
	mkdir -p $@
$(OUT)/explicit-relations-host: service.cpp host.hpp curve.cuh ../rootfind/fp64.cuh | $(OUT)
	$(CXX) $(CXXFLAGS) -Wno-unknown-pragmas service.cpp -o $@
$(OUT)/explicit-relations-cuda: service.cpp device.cuh negative-filter.hpp host.hpp curve.cuh ../rootfind/fp64.cuh | $(OUT)
	$(NVCC) -O3 -std=c++17 -arch=$(ARCH) -DER_CUDA -x cu service.cpp -o $@
$(OUT)/explicit-relations-cached: service.cpp device.cuh negative-filter.hpp host.hpp curve.cuh ../rootfind/fp64.cuh | $(OUT)
	$(NVCC) -O3 -std=c++17 -arch=$(ARCH) -DER_CUDA -DER_QUERY_CACHED -x cu service.cpp -o $@
$(OUT)/explicit-relations-compact: service.cpp device.cuh negative-filter.hpp host.hpp curve.cuh ../rootfind/fp64.cuh | $(OUT)
	$(NVCC) -O3 -std=c++17 -arch=$(ARCH) -DER_CUDA -DER_COMPACT_ROWS -x cu service.cpp -o $@
$(OUT)/explicit-relations-wide-index: service.cpp device.cuh negative-filter.hpp host.hpp curve.cuh ../rootfind/fp64.cuh | $(OUT)
	$(NVCC) -O3 -std=c++17 -arch=$(ARCH) -DER_CUDA -DER_COMPACT_ROWS -DER_WIDE_INDEX -x cu service.cpp -o $@
$(OUT)/explicit-relations-dense: service.cpp device.cuh negative-filter.hpp host.hpp curve.cuh ../rootfind/fp64.cuh | $(OUT)
	$(NVCC) -O3 -std=c++17 -arch=$(ARCH) -DER_CUDA -DER_COMPACT_ROWS -DER_WIDE_INDEX -DER_DENSE_ROWS -x cu service.cpp -o $@
$(OUT)/explicit-relations-dense-cub: service.cpp device.cuh negative-filter.hpp host.hpp curve.cuh ../rootfind/fp64.cuh | $(OUT)
	$(NVCC) -O3 -std=c++17 -arch=$(ARCH) -DER_CUDA -DER_COMPACT_ROWS -DER_WIDE_INDEX -DER_DENSE_ROWS -DER_CUB_SORT -x cu service.cpp -o $@
$(OUT)/query-cub-check: query-check.cu device.cuh negative-filter.hpp host.hpp curve.cuh ../rootfind/fp64.cuh | $(OUT)
	$(NVCC) -O3 -std=c++17 -arch=$(ARCH) -DER_COMPACT_ROWS -DER_WIDE_INDEX -DER_DENSE_ROWS -DER_CUB_SORT query-check.cu -o $@
$(OUT)/query-dense-check: query-check.cu device.cuh negative-filter.hpp host.hpp curve.cuh ../rootfind/fp64.cuh | $(OUT)
	$(NVCC) -O3 -std=c++17 -arch=$(ARCH) -DER_COMPACT_ROWS -DER_WIDE_INDEX -DER_DENSE_ROWS query-check.cu -o $@
$(OUT)/query-wide-check: query-check.cu device.cuh negative-filter.hpp host.hpp curve.cuh ../rootfind/fp64.cuh | $(OUT)
	$(NVCC) -O3 -std=c++17 -arch=$(ARCH) -DER_COMPACT_ROWS -DER_WIDE_INDEX query-check.cu -o $@
$(OUT)/query-compact-check: query-check.cu device.cuh negative-filter.hpp host.hpp curve.cuh ../rootfind/fp64.cuh | $(OUT)
	$(NVCC) -O3 -std=c++17 -arch=$(ARCH) -DER_COMPACT_ROWS query-check.cu -o $@
$(OUT)/query-check: query-check.cu device.cuh negative-filter.hpp host.hpp curve.cuh ../rootfind/fp64.cuh | $(OUT)
	$(NVCC) -O3 -std=c++17 -arch=$(ARCH) query-check.cu -o $@
$(OUT)/query-cached-check: query-check.cu device.cuh negative-filter.hpp host.hpp curve.cuh ../rootfind/fp64.cuh | $(OUT)
	$(NVCC) -O3 -std=c++17 -arch=$(ARCH) -DER_QUERY_CACHED query-check.cu -o $@
$(OUT)/paper-driver: paper_driver.rs Cargo.toml Cargo.lock ../../src/cryptanalysis/hashed_ecdl.rs ../../crates/hashed-ecdl-tensor/Cargo.toml $(wildcard ../../crates/hashed-ecdl-tensor/src/*.rs ../../crates/hashed-ecdl-tensor/src/contraction/*.rs) | $(OUT)
	$(CARGO) build --locked --release --manifest-path Cargo.toml --target-dir $(OUT)/cargo
	cp $(OUT)/cargo/release/paper-driver $@
syntax:
	$(CXX) -std=c++17 -Wall -Wextra -Werror -Wno-unknown-pragmas -fsyntax-only service.cpp
test: host $(OUT)/field63-host-check $(OUT)/filter-host-check
	$(OUT)/field63-host-check
	$(OUT)/filter-host-check
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

.PHONY: cuda-persistent
cuda-persistent: $(OUT)/explicit-relations-persistent
$(OUT)/explicit-relations-persistent: service.cpp device.cuh negative-filter.hpp host.hpp curve.cuh ../rootfind/fp64.cuh | $(OUT)
	$(NVCC) -O3 -std=c++17 -arch=$(ARCH) -DER_CUDA -DER_COMPACT_ROWS -DER_WIDE_INDEX -DER_DENSE_ROWS -DER_CUB_SORT -DER_QUERY_PERSISTENT -x cu service.cpp -o $@
$(OUT)/query-persistent-check: query-check.cu device.cuh negative-filter.hpp host.hpp curve.cuh ../rootfind/fp64.cuh | $(OUT)
	$(NVCC) -O3 -std=c++17 -arch=$(ARCH) -DER_COMPACT_ROWS -DER_WIDE_INDEX -DER_DENSE_ROWS -DER_CUB_SORT -DER_QUERY_PERSISTENT -DER_PERSISTENT_BLOCK_LIMIT=2 query-check.cu -o $@

.PHONY: cuda-right-cache
cuda-right-cache: $(OUT)/explicit-relations-right-cache
$(OUT)/explicit-relations-right-cache: service.cpp device.cuh negative-filter.hpp host.hpp curve.cuh ../rootfind/fp64.cuh | $(OUT)
	$(NVCC) -O3 -std=c++17 -arch=$(ARCH) -DER_CUDA -DER_COMPACT_ROWS -DER_WIDE_INDEX -DER_DENSE_ROWS -DER_CUB_SORT -DER_QUERY_PERSISTENT -DER_RIGHT_FULL_CACHE -x cu service.cpp -o $@
$(OUT)/query-right-cache-check: query-check.cu device.cuh negative-filter.hpp host.hpp curve.cuh ../rootfind/fp64.cuh | $(OUT)
	$(NVCC) -O3 -std=c++17 -arch=$(ARCH) -DER_COMPACT_ROWS -DER_WIDE_INDEX -DER_DENSE_ROWS -DER_CUB_SORT -DER_QUERY_PERSISTENT -DER_RIGHT_FULL_CACHE -DER_PERSISTENT_BLOCK_LIMIT=2 query-check.cu -o $@

$(OUT)/field63-host-check: field63-host-check.cpp host.hpp curve.cuh ../rootfind/fp64.cuh | $(OUT)
	$(CXX) $(CXXFLAGS) -Wno-unknown-pragmas $< -o $@

$(OUT)/filter-host-check: filter-host-check.cpp negative-filter.hpp curve.cuh ../rootfind/fp64.cuh | $(OUT)
	$(CXX) $(CXXFLAGS) -Wno-unknown-pragmas $< -o $@

.PHONY: cuda-filter
cuda-filter: $(OUT)/explicit-relations-filter
$(OUT)/explicit-relations-filter: service.cpp device.cuh negative-filter.hpp host.hpp curve.cuh ../rootfind/fp64.cuh | $(OUT)
	$(NVCC) -O3 -std=c++17 -arch=$(ARCH) -DER_CUDA -DER_COMPACT_ROWS -DER_WIDE_INDEX -DER_DENSE_ROWS -DER_CUB_SORT -DER_QUERY_PERSISTENT -DER_RIGHT_FULL_CACHE -DER_NEGATIVE_FILTER -x cu service.cpp -o $@
$(OUT)/query-filter-check: query-check.cu device.cuh negative-filter.hpp host.hpp curve.cuh ../rootfind/fp64.cuh | $(OUT)
	$(NVCC) -O3 -std=c++17 -arch=$(ARCH) -DER_COMPACT_ROWS -DER_WIDE_INDEX -DER_DENSE_ROWS -DER_CUB_SORT -DER_QUERY_PERSISTENT -DER_RIGHT_FULL_CACHE -DER_NEGATIVE_FILTER -DER_PERSISTENT_BLOCK_LIMIT=2 query-check.cu -o $@

$(OUT)/affine-host-check: affine-host-check.cpp affine-fixtures.hpp host.hpp curve.cuh ../rootfind/fp64.cuh | $(OUT)
	$(CXX) $(CXXFLAGS) -Wno-unknown-pragmas $< -o $@
$(OUT)/affine-device-check: affine-device-check.cu affine-fixtures.hpp device.cuh host.hpp curve.cuh ../rootfind/fp64.cuh | $(OUT)
	$(NVCC) -O3 -std=c++17 -arch=$(ARCH) $< -o $@

.PHONY: cuda-affine
cuda-affine: $(OUT)/explicit-relations-affine
$(OUT)/explicit-relations-affine: service.cpp device.cuh negative-filter.hpp host.hpp curve.cuh ../rootfind/fp64.cuh | $(OUT)
	$(NVCC) -O3 -std=c++17 -arch=$(ARCH) -DER_CUDA -DER_COMPACT_ROWS -DER_WIDE_INDEX -DER_DENSE_ROWS -DER_CUB_SORT -DER_QUERY_PERSISTENT -DER_RIGHT_FULL_CACHE -DER_NEGATIVE_FILTER -DER_QUERY_CACHED -DER_AFFINE_QUERY -x cu service.cpp -o $@

# Opt-in uninstrumented specialization; admission failure keeps the generic path.
FULLCACHE_FLAGS = -DER_COMPACT_ROWS -DER_WIDE_INDEX -DER_DENSE_ROWS -DER_CUB_SORT -DER_QUERY_PERSISTENT -DER_RIGHT_FULL_CACHE -DER_NEGATIVE_FILTER -DER_QUERY_CACHED -DER_AFFINE_QUERY -DER_QUERY_FULL_CACHE
build/explicit-relations-fullcache: service.cpp device.cuh query-fullcache.cuh negative-filter.hpp curve.cuh host.hpp ../rootfind/fp64.cuh | build
	$(NVCC) -O3 -std=c++17 -arch=$(ARCH) -DER_CUDA $(FULLCACHE_FLAGS) -x cu service.cpp -o $@
build/query-fullcache-check: query-fullcache-check.cu device.cuh query-fullcache.cuh negative-filter.hpp curve.cuh host.hpp ../rootfind/fp64.cuh | build
	$(NVCC) -O3 -std=c++17 -arch=$(ARCH) $(FULLCACHE_FLAGS) query-fullcache-check.cu -o $@
