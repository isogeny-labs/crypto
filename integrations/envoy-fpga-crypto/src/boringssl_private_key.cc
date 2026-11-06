#include <cstring>
#include <stdexcept>

#include "fpga/boringssl.h"

namespace fpga {
int PrivateKeySession::index() {
  static const int value = SSL_get_ex_new_index(0, nullptr, nullptr, nullptr, nullptr);
  if (value < 0)
    throw std::runtime_error("SSL ex-data allocation failed");
  return value;
}
PrivateKeySession::PrivateKeySession(SSL* ssl, std::shared_ptr<Engine> engine, uint32_t key_slot,
                                     std::chrono::milliseconds timeout,
                                     std::function<void()> on_pending)
    : ssl_(ssl), engine_(std::move(engine)), key_slot_(key_slot), timeout_(timeout),
      on_pending_(std::move(on_pending)) {
  if (!ssl || !engine_ || key_slot == 0 || SSL_get_ex_data(ssl, index()) != nullptr ||
      !SSL_set_ex_data(ssl, index(), this))
    throw std::invalid_argument("invalid SSL registration");
}
PrivateKeySession::~PrivateKeySession() {
  ticket_.cancel();
  SSL_set_ex_data(ssl_, index(), nullptr);
}
const SSL_PRIVATE_KEY_METHOD* PrivateKeySession::method() {
  static const SSL_PRIVATE_KEY_METHOD value = {sign, decrypt, complete};
  return &value;
}
ssl_private_key_result_t PrivateKeySession::sign(SSL* ssl, uint8_t*, size_t*, size_t,
                                                 uint16_t algorithm, const uint8_t* in,
                                                 size_t len) {
  try {
    auto* self = static_cast<PrivateKeySession*>(SSL_get_ex_data(ssl, index()));
    if (!self || self->pending_ || len > MaxData)
      return ssl_private_key_failure;
    Request request;
    switch (algorithm) {
    case SSL_SIGN_ECDSA_SECP256R1_SHA256:
      request.operation = Operation::EcdsaP256Sha256;
      break;
    case SSL_SIGN_RSA_PSS_RSAE_SHA256:
      request.operation = Operation::RsaPssSha256;
      break;
    case SSL_SIGN_RSA_PKCS1_SHA256:
      request.operation = Operation::RsaPkcs1Sha256;
      break;
    case SSL_SIGN_ML_DSA_44:
      request.operation = Operation::PqSign;
      request.algorithm = "ML-DSA-44";
      break;
    case SSL_SIGN_ML_DSA_65:
      request.operation = Operation::PqSign;
      request.algorithm = "ML-DSA-65";
      break;
    case SSL_SIGN_ML_DSA_87:
      request.operation = Operation::PqSign;
      request.algorithm = "ML-DSA-87";
      break;
    default:
      return ssl_private_key_failure;
    }
    request.key_slot = self->key_slot_;
    request.data.assign(in, in + len);
    self->ticket_ = self->engine_->submit(std::move(request), self->timeout_);
    self->pending_ = true;
    if (self->on_pending_)
      self->on_pending_();
    return ssl_private_key_retry;
  } catch (...) {
    return ssl_private_key_failure;
  }
}
ssl_private_key_result_t PrivateKeySession::decrypt(SSL*, uint8_t*, size_t*, size_t, const uint8_t*,
                                                    size_t) {
  return ssl_private_key_failure; // No legacy TLS RSA key exchange.
}
ssl_private_key_result_t PrivateKeySession::complete(SSL* ssl, uint8_t* out, size_t* out_len,
                                                     size_t max_out) {
  *out_len = 0;
  auto* self = static_cast<PrivateKeySession*>(SSL_get_ex_data(ssl, index()));
  if (!self || !self->pending_)
    return ssl_private_key_failure;
  auto result = self->ticket_.take();
  if (!result)
    return ssl_private_key_retry;
  self->pending_ = false;
  if (!*result || result->data.size() > max_out) {
    wipe(result->data);
    return ssl_private_key_failure;
  }
  std::memcpy(out, result->data.data(), result->data.size());
  *out_len = result->data.size();
  wipe(result->data);
  return ssl_private_key_success;
}
} // namespace fpga
