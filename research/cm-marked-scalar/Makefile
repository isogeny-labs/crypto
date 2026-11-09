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
$(BUILD_DIR)/analysis-tests: analysis_tests.cpp analysis.cpp analysis_load.hpp json.hpp
	mkdir -p $(BUILD_DIR)
	$(CXX) $(CXXFLAGS) analysis_tests.cpp -o $@ $(LDLIBS)
analysis-tests: $(BUILD_DIR)/analysis-tests
	$(BUILD_DIR)/analysis-tests
.PHONY: analysis-tests
$(BUILD_DIR)/construct: construct.cpp native.cpp json.hpp relation_verify.hpp orbit_verify.hpp
	mkdir -p $(BUILD_DIR)
	$(CXX) $(CXXFLAGS) construct.cpp -o $@ $(LDLIBS)
construct-binary: $(BUILD_DIR)/construct
.PHONY: construct-binary
$(BUILD_DIR)/qualification-tests: qualification_tests.cpp native.cpp relation_verify.hpp orbit_verify.hpp json.hpp
	mkdir -p $(BUILD_DIR)
	$(CXX) $(CXXFLAGS) qualification_tests.cpp -o $@ $(LDLIBS)
qualification-tests-binary: $(BUILD_DIR)/qualification-tests
.PHONY: qualification-tests-binary
$(BUILD_DIR)/recoding: recoding.cpp native.cpp relation_verify.hpp orbit_verify.hpp json.hpp
	mkdir -p $(BUILD_DIR)
	$(CXX) $(CXXFLAGS) recoding.cpp -o $@ $(LDLIBS)
recoding-binary: $(BUILD_DIR)/recoding
.PHONY: recoding-binary
$(BUILD_DIR)/recoding-replay: recoding_replay.cpp native.cpp relation_verify.hpp orbit_verify.hpp json.hpp
	mkdir -p $(BUILD_DIR)
	$(CXX) $(CXXFLAGS) recoding_replay.cpp -o $@ $(LDLIBS)
recoding-replay-binary: $(BUILD_DIR)/recoding-replay
.PHONY: recoding-replay-binary
$(BUILD_DIR)/lift-identity: lift_identity.cpp json.hpp
	mkdir -p $(BUILD_DIR)
	$(CXX) $(CXXFLAGS) lift_identity.cpp -o $@ -lgmpxx -lgmp
lift-identity-binary: $(BUILD_DIR)/lift-identity
.PHONY: lift-identity-binary
$(BUILD_DIR)/residual: residual.cpp native.cpp json.hpp relation_verify.hpp orbit_verify.hpp
	mkdir -p $(BUILD_DIR)
	$(CXX) $(CXXFLAGS) residual.cpp -o $@ $(LDLIBS)
residual-binary: $(BUILD_DIR)/residual
.PHONY: residual-binary
$(BUILD_DIR)/residual-replay: residual_replay.cpp json.hpp
	mkdir -p $(BUILD_DIR)
	$(CXX) $(CXXFLAGS) residual_replay.cpp -o $@ -lcrypto -lgmpxx -lgmp
residual-replay-binary: $(BUILD_DIR)/residual-replay
.PHONY: residual-replay-binary
$(BUILD_DIR)/cache-qual: cache_qual.cpp cache_core.hpp native.cpp json.hpp relation_verify.hpp orbit_verify.hpp
	mkdir -p $(BUILD_DIR)
	$(CXX) $(CXXFLAGS) cache_qual.cpp -o $@ -lcrypto -lgmpxx -lgmp
cache-qual-binary: $(BUILD_DIR)/cache-qual
.PHONY: cache-qual-binary
$(BUILD_DIR)/cache-eval: cache_eval.cpp cache_core.hpp native.cpp json.hpp relation_verify.hpp orbit_verify.hpp
	mkdir -p $(BUILD_DIR)
	$(CXX) $(CXXFLAGS) cache_eval.cpp -o $@ $(LDLIBS)
cache-eval-binary: $(BUILD_DIR)/cache-eval
.PHONY: cache-eval-binary
$(BUILD_DIR)/cache-analysis: cache_analyze.cpp json.hpp
	mkdir -p $(BUILD_DIR)
	$(CXX) $(CXXFLAGS) cache_analyze.cpp -o $@ -lcrypto
cache-analysis-binary: $(BUILD_DIR)/cache-analysis
.PHONY: cache-analysis-binary
$(BUILD_DIR)/cache-points: cache_points.cpp json.hpp
	mkdir -p $(BUILD_DIR)
	$(CXX) $(CXXFLAGS) cache_points.cpp -o $@ -lgmpxx -lgmp
cache-points-binary: $(BUILD_DIR)/cache-points
.PHONY: cache-points-binary
$(BUILD_DIR)/block-qual: block_qual.cpp block_core.hpp cache_core.hpp native.cpp json.hpp relation_verify.hpp orbit_verify.hpp
	mkdir -p $(BUILD_DIR)
	$(CXX) $(CXXFLAGS) block_qual.cpp -o $@ -lcrypto -lgmpxx -lgmp
block-qual-binary: $(BUILD_DIR)/block-qual
.PHONY: block-qual-binary
$(BUILD_DIR)/block-eval: block_eval.cpp block_core.hpp cache_eval.cpp cache_core.hpp native.cpp json.hpp relation_verify.hpp orbit_verify.hpp
	mkdir -p $(BUILD_DIR)
	$(CXX) $(CXXFLAGS) block_eval.cpp -o $@ -lcrypto
block-eval-binary: $(BUILD_DIR)/block-eval
$(BUILD_DIR)/block-analysis: block_analyze.cpp json.hpp
	mkdir -p $(BUILD_DIR)
	$(CXX) $(CXXFLAGS) block_analyze.cpp -o $@ -lcrypto
block-analysis-binary: $(BUILD_DIR)/block-analysis
.PHONY: block-eval-binary block-analysis-binary
$(BUILD_DIR)/block-tune: block_tune.cpp block_core.hpp cache_core.hpp native.cpp json.hpp relation_verify.hpp orbit_verify.hpp
	mkdir -p $(BUILD_DIR)
	$(CXX) $(CXXFLAGS) block_tune.cpp -o $@ -lcrypto
block-tune-binary: $(BUILD_DIR)/block-tune
.PHONY: block-tune-binary
$(BUILD_DIR)/block-tune-replay: block_tune_replay.cpp json.hpp
	mkdir -p $(BUILD_DIR)
	$(CXX) $(CXXFLAGS) block_tune_replay.cpp -o $@ -lcrypto -lgmpxx -lgmp
block-tune-replay-binary: $(BUILD_DIR)/block-tune-replay
.PHONY: block-tune-replay-binary
$(BUILD_DIR)/block-tune-mutate: block_tune_mutate.cpp json.hpp
	mkdir -p $(BUILD_DIR)
	$(CXX) $(CXXFLAGS) block_tune_mutate.cpp -o $@ -lcrypto
block-tune-mutate-binary: $(BUILD_DIR)/block-tune-mutate
.PHONY: block-tune-mutate-binary
$(BUILD_DIR)/block-compact-qual: block_compact_qual.cpp block_compact.hpp block_qual.cpp block_core.hpp cache_core.hpp native.cpp json.hpp relation_verify.hpp orbit_verify.hpp
	mkdir -p $(BUILD_DIR)
	$(CXX) $(CXXFLAGS) block_compact_qual.cpp -o $@ -lcrypto -lgmpxx -lgmp
block-compact-qual-binary: $(BUILD_DIR)/block-compact-qual
.PHONY: block-compact-qual-binary
$(BUILD_DIR)/block-compact-eval: block_compact_eval.cpp block_compact.hpp block_eval.cpp block_core.hpp cache_eval.cpp cache_core.hpp native.cpp json.hpp relation_verify.hpp orbit_verify.hpp
	mkdir -p $(BUILD_DIR)
	$(CXX) $(CXXFLAGS) block_compact_eval.cpp -o $@ -lcrypto
block-compact-eval-binary: $(BUILD_DIR)/block-compact-eval
.PHONY: block-compact-eval-binary
