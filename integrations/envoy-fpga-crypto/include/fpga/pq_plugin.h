#pragma once
// Plain C ABI: the Envoy/BoringSSL process does not link liboqs or OpenSSL.
#include <stddef.h>
#include <stdint.h>
#ifdef __cplusplus
extern "C" {
#endif
#define FPGA_PQ_PLUGIN_ABI 1u
typedef struct fpga_pq_api {
  uint32_t abi;
  // One instance is used only by one Engine worker. No exceptions cross this ABI.
  void* (*create)(uint32_t allow_experimental, uint32_t max_keys);
  void (*destroy)(void*);
  // Request = FPC1 ABI 2. Status = fpga::Code numeric value (frozen at ABI 1).
  // output_size is capacity on entry and actual length on success, zero on failure.
  uint32_t (*execute)(void*, const uint8_t*, size_t, uint8_t*, size_t*, uint32_t budget_ms);
} fpga_pq_api;
typedef const fpga_pq_api* (*fpga_pq_entry_fn)(void);
const fpga_pq_api* fpga_pq_plugin_v1(void);
#ifdef __cplusplus
}
#endif
