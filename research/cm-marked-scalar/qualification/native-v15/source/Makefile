CXX ?= c++
CXXFLAGS ?= -O3 -std=c++17 -Wall -Wextra -Wpedantic -Wno-deprecated-declarations
LDLIBS ?= -lcrypto
BUILD_DIR ?= /tmp/cm-marked-scalar-build
all: $(BUILD_DIR)/native
$(BUILD_DIR)/native: native.cpp json.hpp relation_verify.hpp orbit_verify.hpp
	mkdir -p $(BUILD_DIR)
	$(CXX) $(CXXFLAGS) native.cpp -o $@ $(LDLIBS)
qualify: all
	$(BUILD_DIR)/native --qualify fixtures
.PHONY: all qualify
$(BUILD_DIR)/bench: bench.cpp native.cpp json.hpp relation_verify.hpp orbit_verify.hpp
	mkdir -p $(BUILD_DIR)
	$(CXX) $(CXXFLAGS) bench.cpp -o $@ $(LDLIBS)
bench-binary: $(BUILD_DIR)/bench
.PHONY: bench-binary
$(BUILD_DIR)/isolate: isolate.cpp
	mkdir -p $(BUILD_DIR)
	$(CXX) $(CXXFLAGS) isolate.cpp -o $@ -pthread $(LDLIBS)
isolate-binary: $(BUILD_DIR)/isolate
.PHONY: isolate-binary
$(BUILD_DIR)/analysis: analysis.cpp json.hpp analysis_load.hpp
	mkdir -p $(BUILD_DIR)
	$(CXX) $(CXXFLAGS) analysis.cpp -o $@ $(LDLIBS)
analysis-binary: $(BUILD_DIR)/analysis
native-all: all bench-binary isolate-binary analysis-binary
.PHONY: analysis-binary native-all
$(BUILD_DIR)/replay: replay.cpp json.hpp
	mkdir -p $(BUILD_DIR)
	$(CXX) $(CXXFLAGS) replay.cpp -o $@ $(LDLIBS)
replay-binary: $(BUILD_DIR)/replay
native-complete: native-all replay-binary
.PHONY: replay-binary native-complete
$(BUILD_DIR)/isolate-selftest: isolate.cpp json.hpp
	mkdir -p $(BUILD_DIR)
	$(CXX) $(CXXFLAGS) -DCM_ISOLATION_SELFTEST isolate.cpp -o $@ -pthread $(LDLIBS)
isolate-selftest-binary: $(BUILD_DIR)/isolate-selftest
.PHONY: isolate-selftest-binary
