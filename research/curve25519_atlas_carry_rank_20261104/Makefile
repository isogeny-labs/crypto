CXX ?= c++
CXXFLAGS ?= -std=c++20 -O2 -Wall -Wextra -Werror -pedantic
OUT ?= /tmp/curve25519-atlas-carry-rank
SOURCE_REVISION ?= $(shell git rev-parse HEAD)
SOURCE_DIRTY ?= $(shell test -z "$$(git status --porcelain)" && echo 0 || echo 1)

.PHONY: build test
build:
	$(CXX) $(CXXFLAGS) -DATLAS_SOURCE_REVISION='"$(SOURCE_REVISION)"' -DATLAS_SOURCE_DIRTY=$(SOURCE_DIRTY) study.cpp -lgmpxx -lgmp -lcrypto -o $(OUT)
	$(CXX) $(CXXFLAGS) openssl_replay.cpp -lcrypto -o $(OUT)-openssl
test: build
	$(OUT) --self-test
