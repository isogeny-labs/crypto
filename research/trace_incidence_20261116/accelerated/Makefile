CXX ?= g++
CARGO ?= cargo
FLINT_PREFIX ?= /usr
JSON_INCLUDE ?= /usr/include
CARGO_TARGET_DIR ?= $(CURDIR)/build/cargo
CXXFLAGS ?= -O2 -std=c++17 -Wall -Wextra -Werror
CPPFLAGS += -I$(FLINT_PREFIX)/include -I$(JSON_INCLUDE)
LDFLAGS += -L$(FLINT_PREFIX)/lib/x86_64-linux-gnu -Wl,-rpath,$(FLINT_PREFIX)/lib/x86_64-linux-gnu -Wl,--gc-sections
LDLIBS += -lflint -lmpfr -lgmp -lssl -lcrypto -ldl -lpthread -lm -lutil -lrt
CORE = ../native/translation.cpp ../native/paper.cpp ../native/pullback.cpp ../native/incidence.cpp
HEADERS = $(wildcard ../native/*.hpp)
STATIC = $(CARGO_TARGET_DIR)/release/libtrace_incidence_accelerated.a
.PHONY: all rust check
all: build/audit build/benchmark build/analyze
build:
	mkdir -p build
rust:
	CARGO_TARGET_DIR=$(CARGO_TARGET_DIR) $(CARGO) build --manifest-path rust/Cargo.toml --release --locked
$(STATIC): rust
build/audit: audit.cpp accelerated.cpp accelerated.hpp $(HEADERS) Makefile $(CORE) $(STATIC) | build
	$(CXX) $(CPPFLAGS) $(CXXFLAGS) accelerated.cpp audit.cpp $(CORE) $(STATIC) $(LDFLAGS) $(LDLIBS) -o $@
build/benchmark: benchmark.cpp accelerated.cpp accelerated.hpp $(HEADERS) Makefile ../runpod/pair_control_callable.cpp $(CORE) $(STATIC) | build
	$(CXX) $(CPPFLAGS) $(CXXFLAGS) accelerated.cpp benchmark.cpp $(CORE) $(STATIC) $(LDFLAGS) $(LDLIBS) -o $@
build/analyze: analyze.cpp ../runpod/analyze.cpp Makefile | build
	$(CXX) -I$(JSON_INCLUDE) $(CXXFLAGS) analyze.cpp -o $@
check: all
	CARGO_TARGET_DIR=$(CARGO_TARGET_DIR) $(CARGO) test --manifest-path rust/Cargo.toml --release --locked
	bash ./check.sh
	bash ./check-analyzer.sh
