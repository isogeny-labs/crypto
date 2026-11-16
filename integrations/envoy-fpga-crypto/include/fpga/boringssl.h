#pragma once
#include "fpga/crypto.h"
#include "openssl/ssl.h"

#include <functional>

namespace fpga {
// Own on the SSL connection's thread; destroy before SSL_free.
class PrivateKeySession {
public:
  PrivateKeySession(SSL* ssl, std::shared_ptr<Engine> engine, uint32_t key_slot,
                    std::chrono::milliseconds timeout, std::function<void()> on_pending = {});
  ~PrivateKeySession();
  PrivateKeySession(const PrivateKeySession&) = delete;
  PrivateKeySession& operator=(const PrivateKeySession&) = delete;
  bool pending() const { return pending_; }
  bool ready() const { return pending_ && ticket_.ready(); }
  static const SSL_PRIVATE_KEY_METHOD* method();
private:
  static int index();
  static ssl_private_key_result_t sign(SSL*, uint8_t*, size_t*, size_t, uint16_t,
                                        const uint8_t*, size_t);
  static ssl_private_key_result_t decrypt(SSL*, uint8_t*, size_t*, size_t, const uint8_t*, size_t);
  static ssl_private_key_result_t complete(SSL*, uint8_t*, size_t*, size_t);
  SSL* ssl_;
  std::shared_ptr<Engine> engine_;
  uint32_t key_slot_;
  std::chrono::milliseconds timeout_;
  std::function<void()> on_pending_;
  Ticket ticket_;
  bool pending_{false};
};

// Process-wide, opt-in Linux/ELF link-time integration. Install at startup,
// before creating TLS connections. All new TLS AES-GCM AEAD contexts use it.
// Reconfiguration to a different live Engine is rejected.
void enableRecordOffload(std::shared_ptr<Engine> engine, std::chrono::milliseconds timeout);
struct RecordCounters {
  std::atomic<uint64_t> seals{0}, opens{0}, failures{0};
};
const RecordCounters& recordCounters();
} // namespace fpga
