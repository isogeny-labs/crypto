CXX ?= g++
FLINT_PREFIX ?= /usr
CPPFLAGS += -I$(FLINT_PREFIX)/include
CXXFLAGS ?= -O2 -std=c++17 -Wall -Wextra -Werror
LDFLAGS += -L$(FLINT_PREFIX)/lib/x86_64-linux-gnu -Wl,-rpath,$(FLINT_PREFIX)/lib/x86_64-linux-gnu
LDLIBS += -lflint -lmpfr -lgmp
CORE = translation.cpp paper.cpp pullback.cpp
HEADERS = api.hpp paper.hpp pullback.hpp isogeny.hpp incidence.hpp

.PHONY: all check clean
all: build/rootfinder build/paper_example

build:
	mkdir -p build

build/rootfinder: $(CORE) isogeny.cpp root_cli.cpp $(HEADERS) | build
	$(CXX) $(CPPFLAGS) $(CXXFLAGS) $(CORE) isogeny.cpp root_cli.cpp $(LDFLAGS) $(LDLIBS) -o $@

build/paper_example: $(CORE) paper_example.cpp $(HEADERS) | build
	$(CXX) $(CPPFLAGS) $(CXXFLAGS) $(CORE) paper_example.cpp $(LDFLAGS) $(LDLIBS) -o $@

check:
	FLINT_PREFIX=$(FLINT_PREFIX) CXX=$(CXX) bash check-paper.sh
	FLINT_PREFIX=$(FLINT_PREFIX) CXX=$(CXX) bash check-integration.sh

clean:
	rm -rf build
