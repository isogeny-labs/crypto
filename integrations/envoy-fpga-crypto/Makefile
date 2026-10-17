CXX ?= g++
CPPFLAGS += -Iinclude -Itests
CXXFLAGS ?= -O2 -g
CXXFLAGS += -std=c++17 -Wall -Wextra -Wpedantic -Werror -pthread
LDLIBS = -lcrypto -ldl -pthread
CORE = src/crypto.cc src/mailbox.cc src/aws_pci.cc src/pq_codec.cc src/pq_loader.cc
EMULATOR = tests/software_backend.cc
HEADERS = $(wildcard include/fpga/*.h tests/*.h)
TLS = src/boringssl_private_key.cc src/boringssl_records.cc tests/tls_test.cc
BORINGSSL_LIBS = $(if $(BORINGSSL_BUILD),$(BORINGSSL_BUILD)/libssl.a $(BORINGSSL_BUILD)/libcrypto.a)
WRAP = -Wl,--wrap=EVP_AEAD_CTX_init_with_direction -Wl,--wrap=EVP_AEAD_CTX_cleanup -Wl,--wrap=EVP_AEAD_CTX_seal_scatter -Wl,--wrap=EVP_AEAD_CTX_open
ASAN_OPTIONS ?= detect_leaks=1

.PHONY: all test test-boringssl sanitize sanitize-boringssl clean
all: build/core_test
build:
	mkdir -p $@
build/core_test: $(CORE) $(EMULATOR) tests/core_test.cc $(HEADERS) | build
	$(CXX) $(CPPFLAGS) $(CXXFLAGS) $(CORE) $(EMULATOR) tests/core_test.cc -o $@ $(LDLIBS)
test: build/core_test
	./build/core_test

# BoringSSL version must match Envoy's pin. tools/test_boringssl.py automates this.
build/tls_test: $(CORE) $(EMULATOR) $(TLS) $(HEADERS) $(BORINGSSL_LIBS) | build
	test -n "$(BORINGSSL_SRC)" && test -n "$(BORINGSSL_BUILD)"
	$(CXX) $(CPPFLAGS) -isystem $(BORINGSSL_SRC)/include $(CXXFLAGS) $(CORE) $(EMULATOR) $(TLS) -o $@ $(BORINGSSL_LIBS) $(WRAP) -ldl -pthread
test-boringssl: build/tls_test
	./build/tls_test
build/core_test_sanitize: $(CORE) $(EMULATOR) tests/core_test.cc $(HEADERS) | build
	$(CXX) $(CPPFLAGS) $(CXXFLAGS) -O1 -fsanitize=address,undefined -fno-omit-frame-pointer $(CORE) $(EMULATOR) tests/core_test.cc -o $@ $(LDLIBS)
sanitize: build/core_test_sanitize
	ASAN_OPTIONS=$(ASAN_OPTIONS) ./build/core_test_sanitize
build/tls_test_sanitize: $(CORE) $(EMULATOR) $(TLS) $(HEADERS) $(BORINGSSL_LIBS) | build
	test -n "$(BORINGSSL_SRC)" && test -n "$(BORINGSSL_BUILD)"
	$(CXX) $(CPPFLAGS) -isystem $(BORINGSSL_SRC)/include $(CXXFLAGS) -O1 -fsanitize=address,undefined -fno-omit-frame-pointer $(CORE) $(EMULATOR) $(TLS) -o $@ $(BORINGSSL_LIBS) $(WRAP) -ldl -pthread
sanitize-boringssl: build/tls_test_sanitize
	ASAN_OPTIONS=$(ASAN_OPTIONS) ./build/tls_test_sanitize
clean:
	rm -rf build

# Built separately from BoringSSL/Envoy, with no OpenSSL dependency or exported
# C++/OQS symbols. OQS_BUILD is produced by tools/build_pq.py.
OQS_BUILD ?= .cache/liboqs-build
PQ_PLUGIN = $(abspath build/libfpga_pq_oqs.so)
PQ_SOURCES = pq/oqs_plugin.cc src/crypto.cc src/mailbox.cc src/pq_codec.cc
PQ_LINK = -Wl,--exclude-libs,ALL -Wl,-Bsymbolic -Wl,-z,defs -Wl,--version-script=pq/plugin.map
build/libfpga_pq_oqs.so: pq/oqs_plugin.cc pq/plugin.map src/crypto.cc src/mailbox.cc src/pq_codec.cc $(HEADERS) $(OQS_BUILD)/lib/liboqs.a | build
	$(CXX) $(CPPFLAGS) -isystem $(OQS_BUILD)/include $(CXXFLAGS) -fPIC -fvisibility=hidden -shared $(PQ_SOURCES) $(PQ_LINK) -o $@ $(OQS_BUILD)/lib/liboqs.a -pthread -lm
build/pq_test: $(CORE) tests/pq_test.cc tests/emulated_registers.h $(HEADERS) | build
	$(CXX) $(CPPFLAGS) $(CXXFLAGS) $(CORE) tests/pq_test.cc -o $@ -ldl -pthread
.PHONY: pq-plugin test-pq
pq-plugin: build/libfpga_pq_oqs.so
test-pq: build/pq_test pq-plugin
	./build/pq_test $(PQ_PLUGIN)
build/pq_tls_test: $(CORE) src/boringssl_private_key.cc tests/pq_tls_test.cc $(HEADERS) $(BORINGSSL_LIBS) | build
	test -n "$(BORINGSSL_SRC)" && test -n "$(BORINGSSL_BUILD)"
	$(CXX) $(CPPFLAGS) -isystem $(BORINGSSL_SRC)/include $(CXXFLAGS) $(CORE) src/boringssl_private_key.cc tests/pq_tls_test.cc -o $@ $(BORINGSSL_LIBS) -ldl -pthread
.PHONY: test-pq-boringssl
test-pq-boringssl: build/pq_tls_test pq-plugin
	./build/pq_tls_test $(PQ_PLUGIN)
build/libfpga_pq_oqs_sanitize.so: $(PQ_SOURCES) pq/plugin.map $(HEADERS) $(OQS_BUILD)/lib/liboqs.a | build
	$(CXX) $(CPPFLAGS) -isystem $(OQS_BUILD)/include $(CXXFLAGS) -O1 -fsanitize=address,undefined -fno-omit-frame-pointer -fPIC -fvisibility=hidden -shared $(PQ_SOURCES) $(PQ_LINK) -o $@ $(OQS_BUILD)/lib/liboqs.a -pthread -lm
build/pq_test_sanitize: $(CORE) tests/pq_test.cc $(HEADERS) | build
	$(CXX) $(CPPFLAGS) $(CXXFLAGS) -O1 -fsanitize=address,undefined -fno-omit-frame-pointer $(CORE) tests/pq_test.cc -o $@ -ldl -pthread
.PHONY: sanitize-pq
sanitize-pq: build/pq_test_sanitize build/libfpga_pq_oqs_sanitize.so
	ASAN_OPTIONS=$(ASAN_OPTIONS) ./build/pq_test_sanitize $(abspath build/libfpga_pq_oqs_sanitize.so)
