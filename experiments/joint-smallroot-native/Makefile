PREFIX ?= /workspace/skeleton_sage-runtime
BUILD ?= /workspace/native-smallroot-build
CXX ?= g++
CPPFLAGS += -Isrc -I$(PREFIX)/include
CXXFLAGS += -std=c++17 -O2 -g -Wall -Wextra -Wpedantic -Werror
LDFLAGS += -L$(PREFIX)/lib -Wl,-rpath,$(PREFIX)/lib
LDLIBS += -lgmpxx -lgmp -lcrypto -pthread
HEADERS := $(wildcard src/*.hpp)
.PHONY: all structural-check
all: $(BUILD)/native-smallroot $(BUILD)/structural $(BUILD)/cdd-helper
$(BUILD):
	mkdir -p $@
$(BUILD)/native-smallroot: src/main.cpp $(HEADERS) | $(BUILD)
	$(CXX) $(CPPFLAGS) $(CXXFLAGS) $< $(LDFLAGS) $(LDLIBS) -o $@
$(BUILD)/structural: tests/structural.cpp $(HEADERS) | $(BUILD)
	$(CXX) $(CPPFLAGS) $(CXXFLAGS) $< $(LDFLAGS) $(LDLIBS) -o $@
structural-check: $(BUILD)/structural
	@echo 'Invoke structural with an explicit fresh output directory after reviewing sealed expected values.'

$(BUILD)/cdd-helper: src/cdd_helper.cpp | $(BUILD)
	$(CXX) $(CPPFLAGS) $(CXXFLAGS) $< $(LDFLAGS) -lcddgmp -lgmpxx -lgmp -o $@
