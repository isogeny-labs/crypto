#include <oqs/oqs.h>

#include <algorithm>
#include <cstring>
#include <map>
#include <mutex>
#include <stdexcept>

#include "fpga/mailbox.h"
#include "fpga/pq.h"
#include "fpga/pq_plugin.h"

#ifdef OQS_USE_OPENSSL
#error "Build the PQ plugin with OQS_USE_OPENSSL=OFF to isolate it from Envoy BoringSSL"
#endif

namespace {
using namespace fpga;
using Sig = std::unique_ptr<OQS_SIG, decltype(&OQS_SIG_free)>;
using Kem = std::unique_ptr<OQS_KEM, decltype(&OQS_KEM_free)>;
struct Entry {
  PqAlgorithm info;
  Sig sig{nullptr, OQS_SIG_free};
  Kem kem{nullptr, OQS_KEM_free};
};
struct Key {
  std::string algorithm;
  Bytes public_key, secret_key;
  ~Key() { wipe(secret_key); }
};
bool standardized(const std::string& name) {
  return name == "ML-DSA-44" || name == "ML-DSA-65" || name == "ML-DSA-87" ||
         name == "ML-KEM-512" || name == "ML-KEM-768" || name == "ML-KEM-1024" ||
         name.compare(0, 8, "SLH_DSA_") == 0;
}
class Instance {
public:
  Instance(bool experimental, uint32_t max_keys) : max_keys_(max_keys) {
    if (std::strcmp(OQS_version(), "0.16.0"))
      throw std::runtime_error("liboqs version mismatch");
    std::vector<PqAlgorithm> catalog;
    for (int i = 0; i < OQS_SIG_alg_count(); ++i) {
      const char* name = OQS_SIG_alg_identifier(i);
      if (!OQS_SIG_alg_is_enabled(name) || (!experimental && !pqDefaultAllowed(name)))
        continue;
      Entry entry;
      entry.sig.reset(OQS_SIG_new(name));
      if (!entry.sig)
        throw std::runtime_error("cannot initialize signature scheme");
      const auto& s = *entry.sig;
      entry.info = {name,
                    s.alg_version,
                    PqKind::Signature,
                    standardized(name),
                    s.sig_with_ctx_support,
                    s.claimed_nist_level,
                    static_cast<uint32_t>(s.length_public_key),
                    static_cast<uint32_t>(s.length_secret_key),
                    static_cast<uint32_t>(s.length_signature),
                    0,
                    0};
      catalog.push_back(entry.info);
      entries_.emplace(name, std::move(entry));
    }
    for (int i = 0; i < OQS_KEM_alg_count(); ++i) {
      const char* name = OQS_KEM_alg_identifier(i);
      if (!OQS_KEM_alg_is_enabled(name) || (!experimental && !pqDefaultAllowed(name)))
        continue;
      Entry entry;
      entry.kem.reset(OQS_KEM_new(name));
      if (!entry.kem)
        throw std::runtime_error("cannot initialize KEM scheme");
      const auto& k = *entry.kem;
      entry.info = {name,
                    k.alg_version,
                    PqKind::Kem,
                    standardized(name),
                    false,
                    k.claimed_nist_level,
                    static_cast<uint32_t>(k.length_public_key),
                    static_cast<uint32_t>(k.length_secret_key),
                    0,
                    static_cast<uint32_t>(k.length_ciphertext),
                    static_cast<uint32_t>(k.length_shared_secret)};
      catalog.push_back(entry.info);
      entries_.emplace(name, std::move(entry));
    }
    catalog_ = encodeCatalog(catalog);
    if (!decodeCatalog(catalog_))
      throw std::runtime_error("unsupported liboqs catalog sizes");
  }
  Result execute(const Request& r, Clock::time_point deadline) {
    if (!isPq(r.operation))
      return {Code::Unavailable, {}};
    if (r.operation == Operation::PqCatalog)
      return {Code::Ok, catalog_};
    const auto it = entries_.find(r.algorithm);
    if (it == entries_.end())
      return {Code::Unavailable, {}};
    const auto& e = it->second;
    const auto& a = e.info;
    const bool sign = r.operation == Operation::PqSign, verify = r.operation == Operation::PqVerify;
    const bool encap = r.operation == Operation::PqEncapsulate,
               decap = r.operation == Operation::PqDecapsulate;
    if (((sign || verify) && !e.sig) || ((encap || decap) && !e.kem) ||
        (!r.aad.empty() && !a.context))
      return {Code::Invalid, {}};
    auto key_it = keys_.find(r.key_slot);
    if (r.operation == Operation::PqKeygen) {
      if (key_it != keys_.end())
        return {Code::Invalid, {}}; // Never overwrite a key slot.
      if (keys_.size() >= max_keys_ ||
          bytes_ + a.public_key_bytes + a.secret_key_bytes > 64 * 1024 * 1024)
        return {Code::QueueFull, {}};
      auto key = std::make_unique<Key>();
      key->algorithm = r.algorithm;
      key->public_key.resize(a.public_key_bytes);
      key->secret_key.resize(a.secret_key_bytes);
      const auto status =
          e.sig ? OQS_SIG_keypair(e.sig.get(), key->public_key.data(), key->secret_key.data())
                : OQS_KEM_keypair(e.kem.get(), key->public_key.data(), key->secret_key.data());
      if (status != OQS_SUCCESS)
        return {Code::DeviceError, {}};
      // Expired key generation must not commit a hidden slot.
      if (Clock::now() >= deadline)
        return {Code::Timeout, {}};
      Result out{Code::Ok, key->public_key};
      bytes_ += key->public_key.size() + key->secret_key.size();
      keys_.emplace(r.key_slot, std::move(key));
      return out;
    }
    const Key* key = nullptr;
    if (r.key_slot) {
      if (key_it == keys_.end() || key_it->second->algorithm != r.algorithm)
        return {Code::Invalid, {}};
      key = key_it->second.get();
    }
    if (r.operation == Operation::PqPublicKey)
      return {Code::Ok, key->public_key};
    if (r.operation == Operation::PqErase) {
      bytes_ -= key->public_key.size() + key->secret_key.size();
      keys_.erase(key_it);
      return {Code::Ok, {}};
    }
    if (sign) {
      Result out{Code::Ok, Bytes(a.signature_bytes)};
      size_t length = out.data.size();
      // Call the context API even for an empty context when supported. This is
      // the standardized pure-message operation, not internal/prehashed ML-DSA.
      const auto status =
          a.context ? OQS_SIG_sign_with_ctx_str(e.sig.get(), out.data.data(), &length,
                                                r.data.data(), r.data.size(), r.aad.data(),
                                                r.aad.size(), key->secret_key.data())
                    : OQS_SIG_sign(e.sig.get(), out.data.data(), &length, r.data.data(),
                                   r.data.size(), key->secret_key.data());
      if (status != OQS_SUCCESS || length == 0 || length > out.data.size())
        return {Code::DeviceError, {}};
      std::fill(out.data.begin() + length, out.data.end(), 0);
      out.data.resize(length);
      return out;
    }
    if (verify) {
      auto parts = unpackParts(r.auxiliary, 2);
      if (!parts || (*parts)[0].size() != a.public_key_bytes || (*parts)[1].empty() ||
          (*parts)[1].size() > a.signature_bytes)
        return {Code::Invalid, {}};
      const auto& pk = (*parts)[0];
      const auto& sig = (*parts)[1];
      const auto status =
          a.context
              ? OQS_SIG_verify_with_ctx_str(e.sig.get(), r.data.data(), r.data.size(), sig.data(),
                                            sig.size(), r.aad.data(), r.aad.size(), pk.data())
              : OQS_SIG_verify(e.sig.get(), r.data.data(), r.data.size(), sig.data(), sig.size(),
                               pk.data());
      return {status == OQS_SUCCESS ? Code::Ok : Code::AuthFailed, {}};
    }
    if (encap) {
      if (r.data.size() != a.public_key_bytes)
        return {Code::Invalid, {}};
      std::vector<Bytes> parts{Bytes(a.ciphertext_bytes), Bytes(a.shared_secret_bytes)};
      struct Wiper {
        std::vector<Bytes>& parts;
        ~Wiper() {
          for (auto& b : parts)
            wipe(b);
        }
      } guard{parts};
      if (OQS_KEM_encaps(e.kem.get(), parts[0].data(), parts[1].data(), r.data.data()) !=
          OQS_SUCCESS)
        return {Code::DeviceError, {}};
      return {Code::Ok, packParts(parts)};
    }
    if (decap) {
      if (r.data.size() != a.ciphertext_bytes)
        return {Code::Invalid, {}};
      Result out{Code::Ok, Bytes(a.shared_secret_bytes)};
      if (OQS_KEM_decaps(e.kem.get(), out.data.data(), r.data.data(), key->secret_key.data()) !=
          OQS_SUCCESS)
        return {Code::AuthFailed, {}};
      // ML-KEM implicit rejection is SUCCESS with a pseudorandom shared secret.
      return out;
    }
    return {Code::Invalid, {}};
  }

private:
  uint32_t max_keys_;
  size_t bytes_{0};
  std::map<std::string, Entry> entries_;
  std::map<uint32_t, std::unique_ptr<Key>> keys_;
  Bytes catalog_;
};
void* create(uint32_t experimental, uint32_t max_keys) noexcept {
  try {
    if (experimental > 1 || !max_keys || max_keys > 1024)
      return nullptr;
    static std::once_flag init;
    std::call_once(init, [] { OQS_init(); });
    return new Instance(experimental != 0, max_keys);
  } catch (...) {
    return nullptr;
  }
}
void destroy(void* instance) noexcept { delete static_cast<Instance*>(instance); }
uint32_t execute(void* instance, const uint8_t* input, size_t size, uint8_t* output,
                 size_t* output_size, uint32_t budget_ms) noexcept {
  if (!output_size)
    return static_cast<uint32_t>(Code::Invalid);
  const size_t capacity = *output_size;
  *output_size = 0;
  try {
    if (!instance || !input || !output || size > MaxPqWire || capacity < MaxPqWire || !budget_ms ||
        budget_ms > 60000)
      return static_cast<uint32_t>(Code::Invalid);
    const auto deadline = Clock::now() + std::chrono::milliseconds(budget_ms);
    Bytes bytes(input, input + size);
    struct Wiper {
      Bytes& b;
      ~Wiper() { wipe(b); }
    } guard{bytes};
    auto request = decode(bytes);
    if (!request)
      return static_cast<uint32_t>(Code::Invalid);
    Result result = static_cast<Instance*>(instance)->execute(*request, deadline);
    if (Clock::now() >= deadline)
      return static_cast<uint32_t>(Code::Timeout);
    if (!result)
      return static_cast<uint32_t>(result.code);
    if (!validPqOutput(*request, result.data) || result.data.size() > capacity)
      return static_cast<uint32_t>(Code::DeviceError);
    if (!result.data.empty())
      std::memcpy(output, result.data.data(), result.data.size());
    *output_size = result.data.size();
    return 0;
  } catch (...) {
    return static_cast<uint32_t>(Code::DeviceError);
  }
}
const fpga_pq_api api{FPGA_PQ_PLUGIN_ABI, create, destroy, execute};
} // namespace
extern "C" __attribute__((visibility("default"))) const fpga_pq_api* fpga_pq_plugin_v1(void) {
  return &api;
}
