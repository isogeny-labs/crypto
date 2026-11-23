CXX ?= g++
FLINT_PREFIX ?= /workspace/native-deps/flint/usr
BLOCK ?= 32
CXXFLAGS ?= -O2 -std=c++17 -Wall -Wextra -Werror
PARENT = ../native
CPPFLAGS += -I$(FLINT_PREFIX)/include -I$(PARENT) -DROOT_PROJECTION_BLOCK=$(BLOCK)
LDFLAGS += -L$(FLINT_PREFIX)/lib/x86_64-linux-gnu -Wl,-rpath,$(FLINT_PREFIX)/lib/x86_64-linux-gnu
LDLIBS += -lflint -lmpfr -lgmp
.PHONY: all
all: build/rootfinder
build/rootfinder: blocks.cpp $(PARENT)/paper.cpp $(PARENT)/pullback.cpp $(PARENT)/isogeny.cpp $(PARENT)/root_cli.cpp
	mkdir -p build
	$(CXX) $(CPPFLAGS) $(CXXFLAGS) $^ $(LDFLAGS) $(LDLIBS) -o $@
