#include "emulated_registers.h"
#include "fpga/boringssl.h"
#include "test.h"

#include "openssl/aead.h"
#include "openssl/err.h"
#include "openssl/x509.h"

#include <cstring>

using namespace fpga;
using namespace fpga::test;
using namespace std::chrono_literals;
namespace {
Fixture& fixture() { static Fixture f; return f; }
bssl::UniquePtr<X509> certificate(EVP_PKEY* key) {
  bssl::UniquePtr<X509> cert(X509_new());
  CHECK(cert && X509_set_version(cert.get(), 2));
  CHECK(ASN1_INTEGER_set(X509_get_serialNumber(cert.get()), 1));
  CHECK(X509_gmtime_adj(X509_get_notBefore(cert.get()), -60));
  CHECK(X509_gmtime_adj(X509_get_notAfter(cert.get()), 3600));
  CHECK(X509_set_pubkey(cert.get(), key));
  auto* name = X509_get_subject_name(cert.get());
  CHECK(X509_NAME_add_entry_by_txt(name, "CN", MBSTRING_ASC,
                                    reinterpret_cast<const uint8_t*>("localhost"), -1, -1, 0));
  CHECK(X509_set_issuer_name(cert.get(), name));
  CHECK(X509_sign(cert.get(), key, EVP_sha256()) > 0);
  return cert;
}
void retryAllowed(SSL* ssl, int result) {
  if (result > 0) return;
  const int error = SSL_get_error(ssl, result);
  if (error != SSL_ERROR_WANT_READ && error != SSL_ERROR_WANT_WRITE &&
      error != SSL_ERROR_WANT_PRIVATE_KEY_OPERATION) {
    ERR_print_errors_fp(stderr);
    throw std::runtime_error("unexpected TLS error " + std::to_string(error));
  }
}
void transfer(SSL* sender, SSL* receiver, const std::string& message) {
  bool sent = false;
  const auto end = Clock::now() + 3s;
  std::vector<char> buffer(message.size() + 128);
  while (Clock::now() < end) {
    if (!sent) {
      const int n = SSL_write(sender, message.data(), static_cast<int>(message.size()));
      retryAllowed(sender, n);
      if (n > 0) { CHECK(static_cast<size_t>(n) == message.size()); sent = true; }
    }
    const int n = SSL_read(receiver, buffer.data(), static_cast<int>(buffer.size()));
    if (n > 0) { CHECK(std::string(buffer.data(), n) == message); return; }
    retryAllowed(receiver, n);
    std::this_thread::sleep_for(100us);
  }
  throw std::runtime_error("TLS data transfer timed out");
}
void handshake(bool rsa, uint16_t version, uint16_t signature, bool aes256 = false, bool update = false) {
  auto& f = fixture();
  enableRecordOffload(f.engine, 500ms);
  const auto seals_before = recordCounters().seals.load(), opens_before = recordCounters().opens.load();
  auto cert = certificate((rsa ? f.rsa : f.ec).get());
  bssl::UniquePtr<SSL_CTX> client_context(SSL_CTX_new(TLS_method())), server_context(SSL_CTX_new(TLS_method()));
  CHECK(client_context && server_context);
  for (auto* ctx : {client_context.get(), server_context.get()}) {
    CHECK(SSL_CTX_set_min_proto_version(ctx, version));
    CHECK(SSL_CTX_set_max_proto_version(ctx, version));
    const char* cipher = rsa ? (aes256 ? "ECDHE-RSA-AES256-GCM-SHA384" : "ECDHE-RSA-AES128-GCM-SHA256")
                             : "ECDHE-ECDSA-AES128-GCM-SHA256";
    CHECK(SSL_CTX_set_strict_cipher_list(ctx, cipher));
  }
  CHECK(SSL_CTX_use_certificate(server_context.get(), cert.get()));
  // No private key is installed on the SSL_CTX. The emulator owns it in a key slot.
  SSL_CTX_set_private_key_method(server_context.get(), PrivateKeySession::method());
  CHECK(SSL_CTX_set_signing_algorithm_prefs(server_context.get(), &signature, 1));
  SSL_CTX_set_verify(client_context.get(), SSL_VERIFY_PEER, nullptr);
  CHECK(X509_STORE_add_cert(SSL_CTX_get_cert_store(client_context.get()), cert.get()));
  bssl::UniquePtr<SSL> client(SSL_new(client_context.get())), server(SSL_new(server_context.get()));
  CHECK(client && server);
  BIO *client_bio = nullptr, *server_bio = nullptr;
  CHECK(BIO_new_bio_pair(&client_bio, 0, &server_bio, 0));
  SSL_set_bio(client.get(), client_bio, client_bio);
  SSL_set_bio(server.get(), server_bio, server_bio);
  SSL_set_connect_state(client.get()); SSL_set_accept_state(server.get());
  CHECK(SSL_set_tlsext_host_name(client.get(), "localhost"));
  CHECK(X509_VERIFY_PARAM_set1_host(SSL_get0_param(client.get()), "localhost", 9));
  PrivateKeySession session(server.get(), f.engine, rsa ? 1 : 2, 500ms);
  bool client_done = false, server_done = false, saw_async = false;
  const auto end = Clock::now() + 5s;
  while ((!client_done || !server_done) && Clock::now() < end) {
    if (!client_done) {
      const int result = SSL_do_handshake(client.get());
      retryAllowed(client.get(), result); client_done = result == 1;
    }
    if (!server_done) {
      const int result = SSL_do_handshake(server.get());
      retryAllowed(server.get(), result); server_done = result == 1;
      saw_async = saw_async || (result < 0 && SSL_get_error(server.get(), result) == SSL_ERROR_WANT_PRIVATE_KEY_OPERATION);
    }
    std::this_thread::sleep_for(100us);
  }
  CHECK(client_done && server_done && saw_async);
  CHECK(SSL_version(client.get()) == version && SSL_get_verify_result(client.get()) == X509_V_OK);
  transfer(client.get(), server.get(), "request using FPGA AES-GCM");
  transfer(server.get(), client.get(), std::string(12000, 'x'));
  if (update) {
    CHECK(SSL_key_update(client.get(), SSL_KEY_UPDATE_REQUESTED));
    transfer(client.get(), server.get(), "new traffic key after KeyUpdate");
    transfer(server.get(), client.get(), "response after KeyUpdate");
  }
  CHECK(recordCounters().seals > seals_before && recordCounters().opens > opens_before);
}
}
TEST(tls12_rsa_pss_aes128_end_to_end) { handshake(true, TLS1_2_VERSION, SSL_SIGN_RSA_PSS_RSAE_SHA256); }
TEST(tls12_rsa_pkcs1_aes256_end_to_end) { handshake(true, TLS1_2_VERSION, SSL_SIGN_RSA_PKCS1_SHA256, true); }
TEST(tls12_ecdsa_aes128_end_to_end) { handshake(false, TLS1_2_VERSION, SSL_SIGN_ECDSA_SECP256R1_SHA256); }
TEST(tls13_rsa_pss_end_to_end) { handshake(true, TLS1_3_VERSION, SSL_SIGN_RSA_PSS_RSAE_SHA256); }
TEST(tls13_ecdsa_and_traffic_key_updates) { handshake(false, TLS1_3_VERSION, SSL_SIGN_ECDSA_SECP256R1_SHA256, false, true); }

TEST(tls_record_scatter_extra_plaintext_and_tampered_tag) {
  auto& f = fixture(); enableRecordOffload(f.engine, 500ms);
  for (bool big : {false, true}) {
    bssl::ScopedEVP_AEAD_CTX seal, open;
    Bytes key(big ? 32 : 16, 0x41), nonce(12, 0x18), aad{1, 2, 3};
    auto* aead = big ? EVP_aead_aes_256_gcm_tls13() : EVP_aead_aes_128_gcm_tls13();
    CHECK(EVP_AEAD_CTX_init_with_direction(seal.get(), aead, key.data(), key.size(), 16, evp_aead_seal));
    CHECK(EVP_AEAD_CTX_init_with_direction(open.get(), aead, key.data(), key.size(), 16, evp_aead_open));
    Bytes plaintext{0x61, 0x62, 0x63}, extra{0x17}, ciphertext(3), suffix(17);
    size_t suffix_length = 0;
    CHECK(EVP_AEAD_CTX_seal_scatter(seal.get(), ciphertext.data(), suffix.data(), &suffix_length,
                                    suffix.size(), nonce.data(), nonce.size(), plaintext.data(),
                                    plaintext.size(), extra.data(), extra.size(), aad.data(), aad.size()));
    CHECK(suffix_length == 17);
    ciphertext.insert(ciphertext.end(), suffix.begin(), suffix.end());
    Bytes output(plaintext.size() + extra.size(), 0xff);
    size_t length = 0;
    CHECK(EVP_AEAD_CTX_open(open.get(), output.data(), &length, output.size(), nonce.data(), nonce.size(),
                             ciphertext.data(), ciphertext.size(), aad.data(), aad.size()));
    CHECK(output == Bytes({0x61, 0x62, 0x63, 0x17}) && length == 4);
    ciphertext.back() ^= 1; output.assign(4, 0xff);
    CHECK(!EVP_AEAD_CTX_open(open.get(), output.data(), &length, output.size(), nonce.data(), nonce.size(),
                              ciphertext.data(), ciphertext.size(), aad.data(), aad.size()));
    CHECK(length == 0 && output == Bytes(4, 0));
    CHECK(!EVP_AEAD_CTX_seal_scatter(seal.get(), output.data(), suffix.data(), &suffix_length,
                                     suffix.size(), nonce.data(), nonce.size(), plaintext.data(),
                                     plaintext.size(), extra.data(), extra.size(), aad.data(), aad.size()));
    CHECK(suffix_length == 0 && suffix == Bytes(17, 0)); // Reused TLS nonce rejected.
  }
}
TEST(private_key_unsupported_algorithms_and_disconnect_cancellation) {
  auto& f = fixture();
  bssl::UniquePtr<SSL_CTX> context(SSL_CTX_new(TLS_method()));
  bssl::UniquePtr<SSL> ssl(SSL_new(context.get()));
  uint8_t out[512], input[3] = {1, 2, 3}; size_t length = 0;
  {
    PrivateKeySession session(ssl.get(), f.engine, 1, 500ms);
    CHECK(PrivateKeySession::method()->sign(ssl.get(), out, &length, sizeof(out),
                                            SSL_SIGN_RSA_PSS_RSAE_SHA384, input, sizeof(input)) == ssl_private_key_failure);
    CHECK(PrivateKeySession::method()->decrypt(ssl.get(), out, &length, sizeof(out), input, sizeof(input)) == ssl_private_key_failure);
    CHECK(PrivateKeySession::method()->sign(ssl.get(), out, &length, sizeof(out),
                                            SSL_SIGN_RSA_PSS_RSAE_SHA256, input, sizeof(input)) == ssl_private_key_retry);
    // Disconnect while signing is pending: no completion callback touches this SSL.
  }
  CHECK(PrivateKeySession::method()->complete(ssl.get(), out, &length, sizeof(out)) == ssl_private_key_failure);
  CHECK(f.engine->execute(Request{}, 500ms));
}
TEST(tls_record_hardware_timeout_has_no_software_fallback) {
  auto& f = fixture();
  bssl::ScopedEVP_AEAD_CTX seal;
  Bytes key(16), nonce(12), in(1, 0x61), out(1, 0xff), tag(16, 0xff);
  CHECK(EVP_AEAD_CTX_init_with_direction(seal.get(), EVP_aead_aes_128_gcm_tls12(), key.data(), key.size(), 16, evp_aead_seal));
  f.registers->fault = Fault::NeverComplete;
  size_t length = 0;
  CHECK(!EVP_AEAD_CTX_seal_scatter(seal.get(), out.data(), tag.data(), &length, tag.size(),
                                   nonce.data(), nonce.size(), in.data(), in.size(), nullptr, 0, nullptr, 0));
  CHECK(length == 0 && out == Bytes(1, 0) && tag == Bytes(16, 0));
  // The worker may publish its poisoned state just after the caller's deadline.
  const auto end = Clock::now() + 1s;
  while (f.engine->healthy() && Clock::now() < end) std::this_thread::yield();
  CHECK(!f.engine->healthy());
}
int main() { return tests::run(); }
