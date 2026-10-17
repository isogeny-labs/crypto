#include "software_backend.h"
#include "openssl/ec.h"
#include "openssl/rsa.h"

#include <memory>
#include <stdexcept>

namespace fpga::test {
namespace {
template<class T, void (*Free)(T*)> using Scoped = std::unique_ptr<T, decltype(Free)>;
bool padding(EVP_PKEY_CTX* context, Operation operation) {
  if (operation == Operation::EcdsaP256Sha256) return true;
  return EVP_PKEY_CTX_set_rsa_padding(context, operation == Operation::RsaPssSha256
                                                ? RSA_PKCS1_PSS_PADDING : RSA_PKCS1_PADDING) > 0 &&
         (operation != Operation::RsaPssSha256 ||
          (EVP_PKEY_CTX_set_rsa_pss_saltlen(context, 32) > 0 &&
           EVP_PKEY_CTX_set_rsa_mgf1_md(context, EVP_sha256()) > 0));
}
}
Pkey generateKey(bool rsa) {
  Scoped<EVP_PKEY_CTX, EVP_PKEY_CTX_free> context(EVP_PKEY_CTX_new_id(rsa ? EVP_PKEY_RSA : EVP_PKEY_EC, nullptr), EVP_PKEY_CTX_free);
  EVP_PKEY* key = nullptr;
  if (!context || EVP_PKEY_keygen_init(context.get()) <= 0 ||
      (rsa ? EVP_PKEY_CTX_set_rsa_keygen_bits(context.get(), 2048)
           : EVP_PKEY_CTX_set_ec_paramgen_curve_nid(context.get(), NID_X9_62_prime256v1)) <= 0 ||
      EVP_PKEY_keygen(context.get(), &key) <= 0) throw std::runtime_error("test key generation failed");
  return Pkey(key, EVP_PKEY_free);
}
Result SoftwareBackend::execute(const Request& r, Clock::time_point deadline) {
  if (Clock::now() >= deadline) return {Code::Timeout, {}};
  if (!valid(r)) return {Code::Invalid, {}};
  Result result;
  if (r.operation == Operation::Sha256) {
    result.data.resize(32);
    unsigned length = 0;
    if (!EVP_Digest(r.data.data(), r.data.size(), result.data.data(), &length, EVP_sha256(), nullptr) || length != 32)
      return {Code::DeviceError, {}};
    return result;
  }
  if (isSign(r.operation)) {
    auto it = signing_keys.find(r.key_slot);
    if (it == signing_keys.end()) return {Code::DeviceError, {}};
    const bool ec = r.operation == Operation::EcdsaP256Sha256;
    if (EVP_PKEY_id(it->second.get()) != (ec ? EVP_PKEY_EC : EVP_PKEY_RSA)) return {Code::Invalid, {}};
    Scoped<EVP_MD_CTX, EVP_MD_CTX_free> context(EVP_MD_CTX_new(), EVP_MD_CTX_free);
    EVP_PKEY_CTX* pkey = nullptr;
    size_t length = 0;
    if (!context || !EVP_DigestSignInit(context.get(), &pkey, EVP_sha256(), nullptr, it->second.get()) ||
        !padding(pkey, r.operation) || !EVP_DigestSignUpdate(context.get(), r.data.data(), r.data.size()) ||
        !EVP_DigestSignFinal(context.get(), nullptr, &length)) return {Code::DeviceError, {}};
    result.data.resize(length);
    if (!EVP_DigestSignFinal(context.get(), result.data.data(), &length)) return {Code::DeviceError, {}};
    result.data.resize(length);
    return result;
  }
  const Bytes* key = &r.inline_key;
  if (r.key_slot) {
    auto it = aes_keys.find(r.key_slot);
    if (it == aes_keys.end()) return {Code::DeviceError, {}};
    key = &it->second;
  }
  const size_t key_size = (r.operation == Operation::Aes128Seal || r.operation == Operation::Aes128Open) ? 16 : 32;
  if (key->size() != key_size) return {Code::Invalid, {}};
  const EVP_CIPHER* cipher = key_size == 16 ? EVP_aes_128_gcm() : EVP_aes_256_gcm();
  Scoped<EVP_CIPHER_CTX, EVP_CIPHER_CTX_free> context(EVP_CIPHER_CTX_new(), EVP_CIPHER_CTX_free);
  const bool seal = isSeal(r.operation);
  if (!context || !EVP_CipherInit_ex(context.get(), cipher, nullptr, nullptr, nullptr, seal ? 1 : 0) ||
      !EVP_CIPHER_CTX_ctrl(context.get(), EVP_CTRL_GCM_SET_IVLEN, 12, nullptr) ||
      !EVP_CipherInit_ex(context.get(), nullptr, nullptr, key->data(), r.nonce.data(), seal ? 1 : 0))
    return {Code::DeviceError, {}};
  int written = 0, final_bytes = 0;
  if (!r.aad.empty() && !EVP_CipherUpdate(context.get(), nullptr, &written, r.aad.data(), static_cast<int>(r.aad.size())))
    return {Code::DeviceError, {}};
  const size_t length = seal ? r.data.size() : r.data.size() - 16;
  result.data.resize(length + 16);
  if (!EVP_CipherUpdate(context.get(), result.data.data(), &written, r.data.data(), static_cast<int>(length)))
    return {Code::DeviceError, {}};
  if (!seal && !EVP_CIPHER_CTX_ctrl(context.get(), EVP_CTRL_GCM_SET_TAG, 16,
                                     const_cast<uint8_t*>(r.data.data() + length)))
    return {Code::DeviceError, {}};
  if (!EVP_CipherFinal_ex(context.get(), result.data.data() + written, &final_bytes)) {
    wipe(result.data);
    return {Code::AuthFailed, {}};
  }
  if (seal && !EVP_CIPHER_CTX_ctrl(context.get(), EVP_CTRL_GCM_GET_TAG, 16, result.data.data() + length))
    return {Code::DeviceError, {}};
  result.data.resize(length + (seal ? 16 : 0));
  return result;
}
bool verify(EVP_PKEY* key, const Request& r, const Bytes& signature) {
  Scoped<EVP_MD_CTX, EVP_MD_CTX_free> context(EVP_MD_CTX_new(), EVP_MD_CTX_free);
  EVP_PKEY_CTX* pkey = nullptr;
  return context && EVP_DigestVerifyInit(context.get(), &pkey, EVP_sha256(), nullptr, key) &&
         padding(pkey, r.operation) && EVP_DigestVerifyUpdate(context.get(), r.data.data(), r.data.size()) &&
         EVP_DigestVerifyFinal(context.get(), signature.data(), signature.size()) == 1;
}
} // namespace fpga::test
