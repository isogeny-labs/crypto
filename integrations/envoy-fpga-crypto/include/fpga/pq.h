#pragma once
#include "fpga/crypto.h"

namespace fpga {
enum class PqKind : uint32_t { Signature = 1, Kem = 2 };
struct PqAlgorithm {
  std::string name, version;
  PqKind kind{PqKind::Signature};
  bool standardized{false}, context{false};
  uint32_t level{0}, public_key_bytes{0}, secret_key_bytes{0};
  uint32_t signature_bytes{0}, ciphertext_bytes{0}, shared_secret_bytes{0};
};
// Conservative default: pure standardized parameter sets. Prehash combinations
// and experimental families require an explicit opt-in, even if library-enabled.
bool pqDefaultAllowed(const std::string& name);
// Versioned, little-endian framing used by verify, encapsulate and catalog.
Bytes packParts(const std::vector<Bytes>& parts);
std::optional<std::vector<Bytes>> unpackParts(const Bytes& bytes, size_t expected_parts);
Bytes encodeCatalog(const std::vector<PqAlgorithm>& algorithms);
std::optional<std::vector<PqAlgorithm>> decodeCatalog(const Bytes& bytes);
std::string catalogJson(const std::vector<PqAlgorithm>& algorithms, const std::string& backend);
bool validPqRequest(const Request& request, const PqAlgorithm& algorithm);
bool validPqOutput(const Request& request, const Bytes& bytes,
                   const PqAlgorithm* algorithm = nullptr);

struct OqsOptions {
  std::string library; // Absolute path to the explicitly selected CPU plugin.
  bool allow_experimental{false};
  uint32_t max_keys{32};
  size_t queue_capacity{128};
};
// No fallback: this creates a CPU backend, never a hardware backend.
std::unique_ptr<Backend> oqsBackend(const OqsOptions& options);
std::shared_ptr<Engine> oqsEngine(const OqsOptions& options);
} // namespace fpga
