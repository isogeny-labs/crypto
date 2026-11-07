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

.PHONY: cpu-accounting
cpu-accounting: $(BUILD)/cpu-accounting $(BUILD)/fake-child panel-status
$(BUILD)/cpu-accounting: tests/cpu_accounting.cpp $(HEADERS) | $(BUILD)
	$(CXX) $(CPPFLAGS) $(CXXFLAGS) $< $(LDFLAGS) $(LDLIBS) -o $@
$(BUILD)/fake-child: tests/fake_child.cpp | $(BUILD)
	$(CXX) $(CPPFLAGS) $(CXXFLAGS) $< -o $@

.PHONY: guard-qualification
guard-qualification: $(BUILD)/guard-qualification $(BUILD)/guard-worker
$(BUILD)/guard-qualification: tests/guard_qualification.cpp $(HEADERS) | $(BUILD)
	$(CXX) $(CPPFLAGS) $(CXXFLAGS) $< $(LDFLAGS) $(LDLIBS) -o $@
$(BUILD)/guard-worker: tests/guard_worker.cpp $(HEADERS) | $(BUILD)
	$(CXX) $(CPPFLAGS) $(CXXFLAGS) $< $(LDFLAGS) $(LDLIBS) -o $@

.PHONY: panel-status
panel-status: $(BUILD)/panel-status
	$(BUILD)/panel-status
$(BUILD)/panel-status: tests/panel_status.cpp $(HEADERS) | $(BUILD)
	$(CXX) $(CPPFLAGS) $(CXXFLAGS) $< $(LDFLAGS) $(LDLIBS) -o $@
