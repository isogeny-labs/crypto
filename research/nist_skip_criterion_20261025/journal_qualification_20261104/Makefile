# Run from this directory. Override PARI_ROOT for another installed PARI 2.x.
PARI_ROOT ?= /workspace/tools/sage_skeleton_env
BUILD_DIR ?= /tmp/nist-journal-build-20261104
CXX ?= c++
CXXFLAGS ?= -O2 -std=c++17 -Wall -Wextra -Werror -pedantic
CPPFLAGS += -I$(PARI_ROOT)/include
PARI_LDLIBS ?= -l:libpari-gmp-tls.so.9
LDLIBS += -L$(PARI_ROOT)/lib -Wl,-rpath,$(PARI_ROOT)/lib $(PARI_LDLIBS) -lgmpxx -lgmp -lcrypto
COHORT = ../strict_elkies_20261029/inputs.tsv

.PHONY: all test
all: $(BUILD_DIR)/journal_check $(BUILD_DIR)/sampler_resume $(BUILD_DIR)/journal_tests

$(BUILD_DIR):
	mkdir -p $@

$(BUILD_DIR)/journal_check: journal_check.cpp | $(BUILD_DIR)
	$(CXX) $(CPPFLAGS) $(CXXFLAGS) $< -o $@ $(LDLIBS)

$(BUILD_DIR)/sampler_resume: sampler_resume.cpp resume.hpp journal_check.cpp ../random_curve_validation_20261029/sampler.cpp | $(BUILD_DIR)
	$(CXX) $(CPPFLAGS) $(CXXFLAGS) $< -o $@ $(LDLIBS)

$(BUILD_DIR)/journal_tests: journal_tests.cpp resume.hpp journal_check.cpp ../random_curve_validation_20261029/sampler.cpp | $(BUILD_DIR)
	$(CXX) $(CPPFLAGS) $(CXXFLAGS) $< -o $@ $(LDLIBS)

test: all
	$(BUILD_DIR)/journal_tests $(COHORT) $(BUILD_DIR)/fixtures
	bash ./test-cli.sh $(BUILD_DIR)
