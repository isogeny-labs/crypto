// GNU/LLVM ELF --wrap bridge for BoringSSL 0.20260929.0 (Envoy v1.39.3).
// Keep TLS framing, key derivation, AAD and nonces in BoringSSL. Only AES-GCM
// seal/open is submitted to the accelerator. No CPU fallback after enrollment.
#include "fpga/boringssl.h"
#include "openssl/aead.h"
#include "openssl/err.h"

#include <cstring>
#include <limits>
#include <map>
#include <stdexcept>

namespace {
struct RecordContext {
  fpga::Bytes key;
  int version{0};
  uint64_t next{0}, mask{0};
  std::shared_ptr<fpga::Engine> engine;
  std::chrono::milliseconds timeout{10};
  std::mutex mutex;
  ~RecordContext() { fpga::wipe(key); }
};
struct Registry {
  std::mutex mutex;
  std::shared_ptr<fpga::Engine> engine;
  std::chrono::milliseconds timeout{10};
  std::map<const EVP_AEAD_CTX*, std::shared_ptr<RecordContext>> contexts;
  fpga::RecordCounters counters;
};
Registry& registry() { static Registry value; return value; }
std::shared_ptr<RecordContext> lookup(const EVP_AEAD_CTX* ctx) {
  auto& r = registry();
  std::lock_guard<std::mutex> lock(r.mutex);
  auto it = r.contexts.find(ctx);
  return it == r.contexts.end() ? nullptr : it->second;
}
bool enroll(EVP_AEAD_CTX* ctx, const EVP_AEAD* aead, const uint8_t* key, size_t key_len, size_t tag_len) {
  int version = 0;
  if (aead == EVP_aead_aes_128_gcm_tls12() || aead == EVP_aead_aes_256_gcm_tls12()) version = 12;
  if (aead == EVP_aead_aes_128_gcm_tls13() || aead == EVP_aead_aes_256_gcm_tls13()) version = 13;
  auto& r = registry();
  std::lock_guard<std::mutex> lock(r.mutex);
  r.contexts.erase(ctx);
  if (!version || !r.engine) return true;
  if ((key_len != 16 && key_len != 32) || (tag_len != 0 && tag_len != 16)) return false;
  auto state = std::make_shared<RecordContext>();
  state->key.assign(key, key + key_len);
  state->version = version; state->engine = r.engine; state->timeout = r.timeout;
  r.contexts[ctx] = std::move(state);
  return true;
}
bool nonceAllowed(RecordContext& ctx, const uint8_t* nonce, size_t len) {
  if (len != 12) return false;
  uint64_t sequence = 0;
  for (size_t i = 4; i < 12; ++i) sequence = (sequence << 8) | nonce[i];
  if (ctx.version == 13 && ctx.next == 0) {
    ctx.mask = sequence; ctx.next = 1; return true;
  }
  if (ctx.version == 13) sequence ^= ctx.mask;
  if (sequence == std::numeric_limits<uint64_t>::max() || sequence < ctx.next) return false;
  ctx.next = sequence + 1;
  return true;
}
fpga::Request requestFor(RecordContext& ctx, bool seal, const uint8_t* nonce, size_t nonce_len,
                         const uint8_t* in, size_t in_len, const uint8_t* aad, size_t aad_len) {
  fpga::Request request;
  request.operation = ctx.key.size() == 16 ? (seal ? fpga::Operation::Aes128Seal : fpga::Operation::Aes128Open)
                                         : (seal ? fpga::Operation::Aes256Seal : fpga::Operation::Aes256Open);
  request.inline_key = ctx.key;
  if (nonce_len) request.nonce.assign(nonce, nonce + nonce_len);
  if (in_len) request.data.assign(in, in + in_len);
  if (aad_len) request.aad.assign(aad, aad + aad_len);
  return request;
}
void recordError() {
  ++registry().counters.failures;
  OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
}
}
namespace fpga {
void enableRecordOffload(std::shared_ptr<Engine> engine, std::chrono::milliseconds timeout) {
  if (!engine || timeout.count() < 1 || timeout > std::chrono::seconds(60))
    throw std::invalid_argument("invalid record offload settings");
  auto& r = registry();
  std::lock_guard<std::mutex> lock(r.mutex);
  if (r.engine && (r.engine != engine || r.timeout != timeout))
    throw std::invalid_argument("TLS record offload is already configured process-wide");
  r.engine = std::move(engine); r.timeout = timeout;
}
const RecordCounters& recordCounters() { return registry().counters; }
}

extern "C" {
int __real_EVP_AEAD_CTX_init_with_direction(EVP_AEAD_CTX*, const EVP_AEAD*, const uint8_t*,
                                           size_t, size_t, evp_aead_direction_t);
void __real_EVP_AEAD_CTX_cleanup(EVP_AEAD_CTX*);
int __real_EVP_AEAD_CTX_open(const EVP_AEAD_CTX*, uint8_t*, size_t*, size_t, const uint8_t*,
                            size_t, const uint8_t*, size_t, const uint8_t*, size_t);
int __real_EVP_AEAD_CTX_seal_scatter(const EVP_AEAD_CTX*, uint8_t*, uint8_t*, size_t*, size_t,
                                    const uint8_t*, size_t, const uint8_t*, size_t,
                                    const uint8_t*, size_t, const uint8_t*, size_t);
int __wrap_EVP_AEAD_CTX_init_with_direction(EVP_AEAD_CTX* ctx, const EVP_AEAD* aead,
                                            const uint8_t* key, size_t key_len, size_t tag_len,
                                            evp_aead_direction_t direction) {
  if (!__real_EVP_AEAD_CTX_init_with_direction(ctx, aead, key, key_len, tag_len, direction)) return 0;
  try { if (enroll(ctx, aead, key, key_len, tag_len)) return 1; } catch (...) {}
  __real_EVP_AEAD_CTX_cleanup(ctx);
  recordError();
  return 0;
}
void __wrap_EVP_AEAD_CTX_cleanup(EVP_AEAD_CTX* ctx) {
  {
    auto& r = registry();
    std::lock_guard<std::mutex> lock(r.mutex);
    r.contexts.erase(ctx);
  }
  __real_EVP_AEAD_CTX_cleanup(ctx);
}
int __wrap_EVP_AEAD_CTX_seal_scatter(const EVP_AEAD_CTX* ctx, uint8_t* out, uint8_t* out_tag,
                                    size_t* out_tag_len, size_t max_tag, const uint8_t* nonce,
                                    size_t nonce_len, const uint8_t* in, size_t in_len,
                                    const uint8_t* extra, size_t extra_len,
                                    const uint8_t* aad, size_t aad_len) {
  auto state = lookup(ctx);
  if (!state) return __real_EVP_AEAD_CTX_seal_scatter(ctx, out, out_tag, out_tag_len, max_tag,
                                                    nonce, nonce_len, in, in_len, extra, extra_len, aad, aad_len);
  *out_tag_len = 0;
  try {
    std::lock_guard<std::mutex> lock(state->mutex);
    if (in_len <= fpga::MaxData && extra_len <= fpga::MaxData - in_len &&
        extra_len <= max_tag && max_tag - extra_len >= 16 && aad_len <= fpga::MaxAad &&
        nonceAllowed(*state, nonce, nonce_len)) {
      auto request = requestFor(*state, true, nonce, nonce_len, in, in_len, aad, aad_len);
      if (extra_len) request.data.insert(request.data.end(), extra, extra + extra_len);
      auto result = state->engine->execute(std::move(request), state->timeout);
      if (result && result.data.size() == in_len + extra_len + 16) {
        if (in_len) std::memcpy(out, result.data.data(), in_len);
        std::memcpy(out_tag, result.data.data() + in_len, extra_len + 16);
        *out_tag_len = extra_len + 16;
        fpga::wipe(result.data);
        ++registry().counters.seals;
        return 1;
      }
    }
  } catch (...) {}
  if (in_len) std::memset(out, 0, in_len);
  if (max_tag) std::memset(out_tag, 0, max_tag);
  recordError();
  return 0;
}
int __wrap_EVP_AEAD_CTX_open(const EVP_AEAD_CTX* ctx, uint8_t* out, size_t* out_len,
                            size_t max_out, const uint8_t* nonce, size_t nonce_len,
                            const uint8_t* in, size_t in_len, const uint8_t* aad, size_t aad_len) {
  auto state = lookup(ctx);
  if (!state) return __real_EVP_AEAD_CTX_open(ctx, out, out_len, max_out, nonce, nonce_len,
                                            in, in_len, aad, aad_len);
  *out_len = 0;
  try {
    std::lock_guard<std::mutex> lock(state->mutex);
    if (nonce_len == 12 && in_len >= 16 && in_len <= fpga::MaxData + 16 &&
        max_out >= in_len - 16 && aad_len <= fpga::MaxAad) {
      auto result = state->engine->execute(requestFor(*state, false, nonce, nonce_len,
                                                      in, in_len, aad, aad_len), state->timeout);
      if (result && result.data.size() == in_len - 16) {
        if (!result.data.empty()) std::memcpy(out, result.data.data(), result.data.size());
        *out_len = result.data.size();
        fpga::wipe(result.data);
        ++registry().counters.opens;
        return 1;
      }
      fpga::wipe(result.data);
    }
  } catch (...) {}
  if (max_out) std::memset(out, 0, max_out);
  recordError();
  return 0;
}
} // extern "C"
