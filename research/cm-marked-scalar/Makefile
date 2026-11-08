CXX ?= c++
CXXFLAGS ?= -O3 -std=c++17 -Wall -Wextra -Wpedantic -Wno-deprecated-declarations
LDLIBS ?= -lcrypto
BUILD_DIR ?= /tmp/cm-marked-scalar-build
all: $(BUILD_DIR)/native
$(BUILD_DIR)/native: native.cpp json.hpp
	mkdir -p $(BUILD_DIR)
	$(CXX) $(CXXFLAGS) native.cpp -o $@ $(LDLIBS)
qualify: all
	$(BUILD_DIR)/native --qualify fixtures
.PHONY: all qualify
