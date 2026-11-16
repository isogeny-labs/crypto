#include "test.h"

#include "emulated_registers.h"
#include "fpga/boringssl.h"
#include "fpga/pq.h"
#include "openssl/ec.h"
#include "openssl/err.h"
#include "openssl/mlkem.h"
#include "openssl/x509v3.h"

using namespace fpga;
using namespace std::chrono_literals;
namespace {
std::string plugin;
std::shared_ptr<Engine> engine() {
  OqsOptions o;
  o.library = plugin;
  auto io = std::make_unique<fpga::test::EmulatedRegisters>(oqsBackend(o), reg::PqVersion, 0xff00);
  return std::make_shared<Engine>(std::make_unique<MailboxBackend>(std::move(io)));
}
Request req(Operation operation, const std::string& algorithm, uint32_t slot, Bytes data = {}) {
  Request r;
  r.operation = operation;
  r.algorithm = algorithm;
  r.key_slot = slot;
  r.data = std::move(data);
  return r;
}
bssl::UniquePtr<EVP_PKEY> caKey() {
  bssl::UniquePtr<EVP_PKEY_CTX> c(EVP_PKEY_CTX_new_id(EVP_PKEY_EC, nullptr));
  EVP_PKEY* p = nullptr;
  CHECK(c && EVP_PKEY_keygen_init(c.get()) &&
        EVP_PKEY_CTX_set_ec_paramgen_curve_nid(c.get(), NID_X9_62_prime256v1));
  CHECK(EVP_PKEY_keygen(c.get(), &p));
  return bssl::UniquePtr<EVP_PKEY>(p);
}
bssl::UniquePtr<X509> cert(EVP_PKEY* public_key, EVP_PKEY* signer, X509* issuer = nullptr) {
  bssl::UniquePtr<X509> c(X509_new());
  CHECK(c && X509_set_version(c.get(), 2));
  CHECK(ASN1_INTEGER_set(X509_get_serialNumber(c.get()), issuer ? 2 : 1));
  CHECK(X509_gmtime_adj(X509_get_notBefore(c.get()), -60));
  CHECK(X509_gmtime_adj(X509_get_notAfter(c.get()), 3600));
  CHECK(X509_set_pubkey(c.get(), public_key));
  auto* name = X509_get_subject_name(c.get());
  const char* cn = issuer ? "localhost" : "Test CA";
  CHECK(X509_NAME_add_entry_by_txt(name, "CN", MBSTRING_ASC, reinterpret_cast<const uint8_t*>(cn),
                                   -1, -1, 0));
  CHECK(X509_set_issuer_name(c.get(), issuer ? X509_get_subject_name(issuer) : name));
  bssl::UniquePtr<BASIC_CONSTRAINTS> constraints(BASIC_CONSTRAINTS_new());
  CHECK(constraints);
  constraints->ca = issuer ? 0 : 0xff;
  CHECK(X509_add1_ext_i2d(c.get(), NID_basic_constraints, constraints.get(), 1, 0));
  CHECK(X509_sign(c.get(), signer, EVP_sha256()) > 0);
  return c;
}
void retry(SSL* ssl, int n) {
  if (n > 0)
    return;
  const int e = SSL_get_error(ssl, n);
  if (e != SSL_ERROR_WANT_READ && e != SSL_ERROR_WANT_WRITE &&
      e != SSL_ERROR_WANT_PRIVATE_KEY_OPERATION) {
    ERR_print_errors_fp(stderr);
    throw std::runtime_error("TLS error " + std::to_string(e));
  }
}
void handshake(const std::string& name, int type, uint16_t signature) {
  auto e = engine();
  auto key = e->execute(req(Operation::PqKeygen, name, 1), 5s);
  CHECK(key);
  bssl::UniquePtr<EVP_PKEY> pk(
      EVP_PKEY_new_raw_public_key(type, nullptr, key.data.data(), key.data.size()));
  CHECK(pk);
  auto ca_key = caKey();
  auto root = cert(ca_key.get(), ca_key.get());
  auto leaf = cert(pk.get(), ca_key.get(), root.get());
  bssl::UniquePtr<SSL_CTX> cc(SSL_CTX_new(TLS_method())), sc(SSL_CTX_new(TLS_method()));
  CHECK(cc && sc);
  for (auto* ctx : {cc.get(), sc.get()}) {
    CHECK(SSL_CTX_set_min_proto_version(ctx, TLS1_3_VERSION));
    CHECK(SSL_CTX_set_max_proto_version(ctx, TLS1_3_VERSION));
    CHECK(SSL_CTX_set1_groups_list(ctx, "X25519MLKEM768"));
  }
  CHECK(SSL_CTX_use_certificate(sc.get(), leaf.get()));
  SSL_CTX_set_private_key_method(sc.get(),
                                 PrivateKeySession::method()); // No private key in SSL_CTX.
  CHECK(SSL_CTX_set_signing_algorithm_prefs(sc.get(), &signature, 1));
  uint16_t verify_algs[] = {signature, SSL_SIGN_ECDSA_SECP256R1_SHA256};
  CHECK(SSL_CTX_set_verify_algorithm_prefs(cc.get(), verify_algs, 2));
  SSL_CTX_set_verify(cc.get(), SSL_VERIFY_PEER, nullptr);
  CHECK(X509_STORE_add_cert(SSL_CTX_get_cert_store(cc.get()), root.get()));
  bssl::UniquePtr<SSL> client(SSL_new(cc.get())), server(SSL_new(sc.get()));
  CHECK(client && server);
  BIO *cb = nullptr, *sb = nullptr;
  CHECK(BIO_new_bio_pair(&cb, 0, &sb, 0));
  SSL_set_bio(client.get(), cb, cb);
  SSL_set_bio(server.get(), sb, sb);
  SSL_set_connect_state(client.get());
  SSL_set_accept_state(server.get());
  CHECK(X509_VERIFY_PARAM_set1_host(SSL_get0_param(client.get()), "localhost", 9));
  PrivateKeySession session(server.get(), e, 1, 5s);
  bool c_done = false, s_done = false, async = false;
  const auto end = Clock::now() + 10s;
  const auto before = e->counters().completed.load();
  while ((!c_done || !s_done) && Clock::now() < end) {
    if (!c_done) {
      int n = SSL_do_handshake(client.get());
      retry(client.get(), n);
      c_done = n == 1;
    }
    if (!s_done) {
      int n = SSL_do_handshake(server.get());
      retry(server.get(), n);
      s_done = n == 1;
      async = async ||
              (n < 0 && SSL_get_error(server.get(), n) == SSL_ERROR_WANT_PRIVATE_KEY_OPERATION);
    }
    std::this_thread::sleep_for(100us);
  }
  CHECK(c_done && s_done && async && e->counters().completed > before);
  CHECK(SSL_get_verify_result(client.get()) == X509_V_OK &&
        SSL_get_group_id(client.get()) == SSL_GROUP_X25519_MLKEM768);
  CHECK(SSL_write(client.get(), "hello", 5) == 5);
  char received[5];
  CHECK(SSL_read(server.get(), received, sizeof(received)) == 5);
  CHECK(std::string(received, 5) == "hello");
  // Independent BoringSSL verification of the plugin's FIPS 204 output.
  auto signature_bytes = e->execute(req(Operation::PqSign, name, 1, Bytes{1, 2, 3}), 5s);
  CHECK(signature_bytes);
  bssl::ScopedEVP_MD_CTX verify;
  CHECK(EVP_DigestVerifyInit(verify.get(), nullptr, nullptr, nullptr, pk.get()));
  const uint8_t message[] = {1, 2, 3};
  CHECK(EVP_DigestVerify(verify.get(), signature_bytes.data.data(), signature_bytes.data.size(),
                         message, sizeof(message)) == 1);
}
} // namespace
TEST(tls13_mldsa44_async_signing_and_hybrid_mlkem) {
  handshake("ML-DSA-44", EVP_PKEY_ML_DSA_44, SSL_SIGN_ML_DSA_44);
}
TEST(tls13_mldsa65_async_signing_and_hybrid_mlkem) {
  handshake("ML-DSA-65", EVP_PKEY_ML_DSA_65, SSL_SIGN_ML_DSA_65);
}
TEST(tls13_mldsa87_async_signing_and_hybrid_mlkem) {
  handshake("ML-DSA-87", EVP_PKEY_ML_DSA_87, SSL_SIGN_ML_DSA_87);
}
TEST(mlkem768_ciphertexts_interoperate_with_independent_boringssl) {
  auto e = engine();
  auto key = e->execute(req(Operation::PqKeygen, "ML-KEM-768", 1), 5s);
  CHECK(key);
  MLKEM768_public_key pk;
  CBS cbs;
  CBS_init(&cbs, key.data.data(), key.data.size());
  CHECK(MLKEM768_parse_public_key(&pk, &cbs) && CBS_len(&cbs) == 0);
  Bytes ciphertext(MLKEM768_CIPHERTEXT_BYTES), secret(MLKEM_SHARED_SECRET_BYTES);
  MLKEM768_encap(ciphertext.data(), secret.data(), &pk);
  auto result = e->execute(req(Operation::PqDecapsulate, "ML-KEM-768", 1, ciphertext), 5s);
  CHECK(result && result.data == secret);
  wipe(secret);
  MLKEM768_private_key private_key;
  Bytes public_key(MLKEM768_PUBLIC_KEY_BYTES);
  MLKEM768_generate_key(public_key.data(), nullptr, &private_key);
  auto encapsulated = e->execute(req(Operation::PqEncapsulate, "ML-KEM-768", 0, public_key), 5s);
  CHECK(encapsulated);
  auto parts = unpackParts(encapsulated.data, 2);
  CHECK(parts);
  secret.resize(MLKEM_SHARED_SECRET_BYTES);
  CHECK(MLKEM768_decap(secret.data(), (*parts)[0].data(), (*parts)[0].size(), &private_key));
  CHECK(secret == (*parts)[1]);
  wipe(secret);
  for (auto& b : *parts)
    wipe(b);
  OPENSSL_cleanse(&private_key, sizeof(private_key));
}
int main(int argc, char** argv) {
  if (argc != 2) {
    std::cerr << "usage: pq_tls_test /absolute/path/plugin.so\n";
    return 2;
  }
  plugin = argv[1];
  return tests::run();
}
