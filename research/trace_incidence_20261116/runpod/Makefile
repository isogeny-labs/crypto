CXX ?= g++
FLINT_PREFIX ?= /usr
JSON_INCLUDE ?= /usr/include
CXXFLAGS ?= -O2 -std=c++17 -Wall -Wextra -Werror
CPPFLAGS += -I$(FLINT_PREFIX)/include -I$(JSON_INCLUDE)
LDFLAGS += -L$(FLINT_PREFIX)/lib/x86_64-linux-gnu -Wl,-rpath,$(FLINT_PREFIX)/lib/x86_64-linux-gnu
LDLIBS += -lflint -lmpfr -lgmp
CORE = ../native/translation.cpp ../native/paper.cpp ../native/pullback.cpp ../native/isogeny.cpp ../native/incidence.cpp
HEADERS = $(wildcard ../native/*.hpp)

.PHONY: all
all: build/benchmark
build:
	mkdir -p build
build/benchmark: benchmark.cpp incidence_lazy.cpp pair_control_callable.cpp $(CORE) $(HEADERS) | build
	$(CXX) $(CPPFLAGS) $(CXXFLAGS) $(CORE) incidence_lazy.cpp benchmark.cpp $(LDFLAGS) $(LDLIBS) -o $@
