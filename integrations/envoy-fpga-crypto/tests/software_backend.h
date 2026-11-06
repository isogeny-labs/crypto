#pragma once
// Test/emulation only. This file is excluded from the production Envoy target.
#include "fpga/crypto.h"
#include "openssl/evp.h"

#include <map>

namespace fpga::test {
using Pkey = std::shared_ptr<EVP_PKEY>;
class SoftwareBackend final : public Backend {
public:
  std::map<uint32_t, Bytes> aes_keys;
  std::map<uint32_t, Pkey> signing_keys;
  ~SoftwareBackend() override { for (auto& item : aes_keys) wipe(item.second); }
  Result execute(const Request&, Clock::time_point) override;
  bool healthy() const override { return true; }
};
Pkey generateKey(bool rsa);
bool verify(EVP_PKEY* key, const Request& request, const Bytes& signature);
} // namespace fpga::test
