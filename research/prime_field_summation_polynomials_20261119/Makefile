CXX ?= g++
CXXFLAGS ?= -O3 -march=native -std=c++20 -Wall -Wextra -Wpedantic -Wconversion

.PHONY: all test clean

all: build/semaev_lab

build/semaev_lab: src/semaev_lab.cpp
	mkdir -p build
	$(CXX) $(CXXFLAGS) $< -o $@

test: build/semaev_lab
	./build/semaev_lab self-test

clean:
	rm -f build/semaev_lab
