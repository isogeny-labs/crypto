CXX ?= g++
CXXFLAGS ?= -std=c++17 -O2 -Wall -Wextra -Wpedantic

.PHONY: all check run clean
all: build/simulate build/audit

build:
	mkdir -p $@

build/simulate: src/simulate.cpp | build
	$(CXX) $(CXXFLAGS) $< -o $@

build/audit: src/audit.cpp | build
	$(CXX) $(CXXFLAGS) $< -o $@

check: all
	bash tools/check.sh

run: all
	bash run.sh run-new

clean:
	rm -rf build
