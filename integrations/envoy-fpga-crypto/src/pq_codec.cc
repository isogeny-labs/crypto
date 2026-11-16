#include <stdexcept>

#include "fpga/pq.h"

namespace fpga {
bool pqDefaultAllowed(const std::string& name) {
  return name == "ML-DSA-44" || name == "ML-DSA-65" || name == "ML-DSA-87" ||
         name == "ML-KEM-512" || name == "ML-KEM-768" || name == "ML-KEM-1024" ||
         name.compare(0, 13, "SLH_DSA_PURE_") == 0;
}
namespace {
void put(Bytes& b, uint32_t v) {
  for (unsigned i = 0; i < 4; ++i)
    b.push_back(static_cast<uint8_t>(v >> (8 * i)));
}
uint32_t get(const Bytes& b, size_t at) {
  return uint32_t(b[at]) | uint32_t(b[at + 1]) << 8 | uint32_t(b[at + 2]) << 16 |
         uint32_t(b[at + 3]) << 24;
}
std::string quote(const std::string& s) {
  std::string out = "\"";
  constexpr char hex[] = "0123456789abcdef";
  for (unsigned char c : s) {
    if (c == '"' || c == '\\') {
      out += '\\';
      out += static_cast<char>(c);
    } else if (c < 32 || c >= 127) {
      out += "\\u00";
      out += hex[c >> 4];
      out += hex[c & 15];
    } else
      out += static_cast<char>(c);
  }
  return out + '"';
}
} // namespace
Bytes packParts(const std::vector<Bytes>& parts) {
  if (parts.size() > 512)
    throw std::invalid_argument("too many PQ frame parts");
  size_t total = 8 + parts.size() * 4;
  for (const auto& p : parts) {
    if (p.size() > MaxPqWire || total > MaxPqWire - p.size())
      throw std::invalid_argument("oversized PQ frame");
    total += p.size();
  }
  Bytes out;
  out.reserve(total);
  put(out, 1);
  put(out, static_cast<uint32_t>(parts.size()));
  for (const auto& p : parts)
    put(out, static_cast<uint32_t>(p.size()));
  for (const auto& p : parts)
    out.insert(out.end(), p.begin(), p.end());
  return out;
}
std::optional<std::vector<Bytes>> unpackParts(const Bytes& b, size_t count) {
  if (count > 512 || b.size() < 8 + count * 4 || b.size() > MaxPqWire || get(b, 0) != 1 ||
      get(b, 4) != count)
    return std::nullopt;
  size_t at = 8 + count * 4, total = at;
  for (size_t i = 0; i < count; ++i) {
    const size_t n = get(b, 8 + i * 4);
    if (n > b.size() - total)
      return std::nullopt;
    total += n;
  }
  if (total != b.size())
    return std::nullopt;
  std::vector<Bytes> out;
  out.reserve(count);
  for (size_t i = 0; i < count; ++i) {
    const size_t n = get(b, 8 + i * 4);
    out.emplace_back(b.begin() + at, b.begin() + at + n);
    at += n;
  }
  return out;
}
Bytes encodeCatalog(const std::vector<PqAlgorithm>& algorithms) {
  std::vector<Bytes> entries;
  for (const auto& a : algorithms) {
    Bytes b;
    for (uint32_t v :
         {static_cast<uint32_t>(a.kind), uint32_t(a.standardized) | (uint32_t(a.context) << 1),
          a.level, a.public_key_bytes, a.secret_key_bytes, a.signature_bytes, a.ciphertext_bytes,
          a.shared_secret_bytes})
      put(b, v);
    entries.push_back(packParts(
        {b, Bytes(a.name.begin(), a.name.end()), Bytes(a.version.begin(), a.version.end())}));
  }
  return packParts(entries);
}
std::optional<std::vector<PqAlgorithm>> decodeCatalog(const Bytes& b) {
  if (b.size() < 8)
    return std::nullopt;
  auto entries = unpackParts(b, get(b, 4));
  if (!entries)
    return std::nullopt;
  std::vector<PqAlgorithm> out;
  for (const auto& entry : *entries) {
    auto parts = unpackParts(entry, 3);
    if (!parts || (*parts)[0].size() != 32 || (*parts)[1].empty() || (*parts)[1].size() > 96 ||
        (*parts)[2].size() > 4096)
      return std::nullopt;
    const auto& d = (*parts)[0];
    if ((get(d, 0) != 1 && get(d, 0) != 2) || get(d, 4) > 3 || get(d, 8) < 1 || get(d, 8) > 5)
      return std::nullopt;
    PqAlgorithm a;
    a.kind = static_cast<PqKind>(get(d, 0));
    a.standardized = (get(d, 4) & 1) != 0;
    a.context = (get(d, 4) & 2) != 0;
    a.level = get(d, 8);
    a.public_key_bytes = get(d, 12);
    a.secret_key_bytes = get(d, 16);
    a.signature_bytes = get(d, 20);
    a.ciphertext_bytes = get(d, 24);
    a.shared_secret_bytes = get(d, 28);
    a.name.assign((*parts)[1].begin(), (*parts)[1].end());
    a.version.assign((*parts)[2].begin(), (*parts)[2].end());
    Request name_check;
    name_check.operation = Operation::PqPublicKey;
    name_check.key_slot = 1;
    name_check.algorithm = a.name;
    if (!valid(name_check) || !a.public_key_bytes || !a.secret_key_bytes ||
        a.public_key_bytes > MaxPqData || a.secret_key_bytes > MaxPqData ||
        a.signature_bytes > MaxPqData || a.ciphertext_bytes > MaxPqData ||
        a.shared_secret_bytes > MaxPqData ||
        (a.kind == PqKind::Signature &&
         (!a.signature_bytes || a.ciphertext_bytes || a.shared_secret_bytes)) ||
        (a.kind == PqKind::Kem &&
         (a.signature_bytes || !a.ciphertext_bytes || !a.shared_secret_bytes || a.context)))
      return std::nullopt;
    for (const auto& previous : out)
      if (previous.name == a.name)
        return std::nullopt;
    out.push_back(std::move(a));
  }
  return out;
}
std::string catalogJson(const std::vector<PqAlgorithm>& algorithms, const std::string& backend) {
  std::string s = "{\"backend\":" + quote(backend) + ",\"algorithms\":[";
  bool first = true;
  for (const auto& a : algorithms) {
    if (!first)
      s += ',';
    first = false;
    s += "{\"name\":" + quote(a.name) + ",\"implementation\":" + quote(a.version) +
         ",\"kind\":" + quote(a.kind == PqKind::Signature ? "signature" : "kem") +
         ",\"standardized\":" + (a.standardized ? "true" : "false") + ",\"default_allowed\":" +
         ((a.standardized && pqDefaultAllowed(a.name)) ? "true" : "false") +
         ",\"context\":" + (a.context ? "true" : "false") +
         ",\"level\":" + std::to_string(a.level) +
         ",\"public_key_bytes\":" + std::to_string(a.public_key_bytes) +
         ",\"secret_key_bytes\":" + std::to_string(a.secret_key_bytes) +
         ",\"signature_bytes\":" + std::to_string(a.signature_bytes) +
         ",\"ciphertext_bytes\":" + std::to_string(a.ciphertext_bytes) +
         ",\"shared_secret_bytes\":" + std::to_string(a.shared_secret_bytes) + "}";
  }
  return s + "]}";
}
bool validPqRequest(const Request& r, const PqAlgorithm& a) {
  if (!valid(r) || a.name != r.algorithm || (!r.aad.empty() && !a.context))
    return false;
  if (r.operation == Operation::PqSign || r.operation == Operation::PqVerify) {
    if (a.kind != PqKind::Signature)
      return false;
    if (r.operation == Operation::PqVerify) {
      auto parts = unpackParts(r.auxiliary, 2);
      return parts && (*parts)[0].size() == a.public_key_bytes && !(*parts)[1].empty() &&
             (*parts)[1].size() <= a.signature_bytes;
    }
  }
  if (r.operation == Operation::PqEncapsulate)
    return a.kind == PqKind::Kem && r.data.size() == a.public_key_bytes;
  if (r.operation == Operation::PqDecapsulate)
    return a.kind == PqKind::Kem && r.data.size() == a.ciphertext_bytes;
  return true;
}
bool validPqOutput(const Request& r, const Bytes& b, const PqAlgorithm* a) {
  if (b.size() > MaxPqWire)
    return false;
  if (r.operation == Operation::PqCatalog)
    return decodeCatalog(b).has_value();
  if (r.operation == Operation::PqVerify || r.operation == Operation::PqErase)
    return b.empty();
  if (r.operation == Operation::PqEncapsulate) {
    // Validate without making another heap copy of the shared secret.
    if (b.size() < 16 || get(b, 0) != 1 || get(b, 4) != 2)
      return false;
    const size_t ct = get(b, 8), secret = get(b, 12);
    return ct > 0 && ct <= MaxPqData && secret > 0 && secret <= MaxPqData &&
           b.size() == 16 + ct + secret &&
           (!a || (ct == a->ciphertext_bytes && secret == a->shared_secret_bytes));
  }
  if (a) {
    if (r.operation == Operation::PqKeygen || r.operation == Operation::PqPublicKey)
      return b.size() == a->public_key_bytes;
    if (r.operation == Operation::PqDecapsulate)
      return b.size() == a->shared_secret_bytes;
    if (r.operation == Operation::PqSign)
      return !b.empty() && b.size() <= a->signature_bytes;
  }
  return !b.empty() && b.size() <= MaxPqData;
}
} // namespace fpga
