CXX ?= c++
CXXFLAGS ?= -std=c++20 -O2 -Wall -Wextra -Werror -pedantic
OUT ?= /tmp/curve25519-atlas-carry-rank

.PHONY: build test
build:
	$(CXX) $(CXXFLAGS) study.cpp -lgmpxx -lgmp -o $(OUT)
test: build
	$(OUT) --self-test
