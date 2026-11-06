#include <cstdlib>
#include <random>

#include "test.h"

#include "emulated_registers.h"
#include "fpga/pq.h"

using namespace fpga;
using namespace fpga::test;
using namespace std::chrono_literals;
namespace {
std::string plugin;
OqsOptions options(bool experimental = false, uint32_t max_keys = 32) {
  OqsOptions o;
  o.library = plugin;
  o.allow_experimental = experimental;
  o.max_keys = max_keys;
  return o;
}
Request request(Operation op, const std::string& algorithm = "", uint32_t slot = 0,
                Bytes data = {}) {
  Request r;
  r.operation = op;
  r.algorithm = algorithm;
  r.key_slot = slot;
  r.data = std::move(data);
  return r;
}
struct PqFixture {
  EmulatedRegisters* registers;
  std::shared_ptr<Engine> engine;
  explicit PqFixture(bool experimental = false, uint32_t max_keys = 32) {
    auto io = std::make_unique<EmulatedRegisters>(oqsBackend(options(experimental, max_keys)),
                                                  reg::PqVersion, 0xff00);
    registers = io.get();
    engine = std::make_shared<Engine>(std::make_unique<MailboxBackend>(std::move(io)));
  }
  Result run(Request r) { return engine->execute(std::move(r), 60s); }
  std::vector<PqAlgorithm> catalog() {
    auto result = run(request(Operation::PqCatalog));
    CHECK(result);
    auto algorithms = decodeCatalog(result.data);
    CHECK(algorithms);
    return *algorithms;
  }
};
void roundTrip(PqFixture& f, const PqAlgorithm& a) {
  auto key = f.run(request(Operation::PqKeygen, a.name, 1));
  CHECK(key);
  CHECK(key.data.size() == a.public_key_bytes);
  auto pub = f.run(request(Operation::PqPublicKey, a.name, 1));
  CHECK(pub && pub.data == key.data);
  CHECK(f.run(request(Operation::PqKeygen, a.name, 1)).code == Code::Invalid);
  if (a.kind == PqKind::Signature) {
    Bytes message{'t', 'e', 's', 't', 0, 'p', 'q'};
    auto sign = request(Operation::PqSign, a.name, 1, message);
    if (a.context)
      sign.aad = Bytes{'c', 't', 'x'};
    auto signature = f.run(sign);
    if (!signature)
      throw std::runtime_error(a.name + " signing returned " + codeName(signature.code) +
                               " (emulated backend: " + codeName(f.registers->last_code) + ")");
    CHECK(!signature.data.empty() && signature.data.size() <= a.signature_bytes);
    auto verify = request(Operation::PqVerify, a.name, 0, message);
    verify.aad = sign.aad;
    verify.auxiliary = packParts({key.data, signature.data});
    CHECK(f.run(verify));
    verify.data[0] ^= 1;
    CHECK(f.run(verify).code == Code::AuthFailed);
    verify.data = message;
    signature.data.back() ^= 1;
    verify.auxiliary = packParts({key.data, signature.data});
    CHECK(f.run(verify).code == Code::AuthFailed);
    verify.auxiliary = packParts({Bytes(1), signature.data});
    CHECK(f.run(verify).code == Code::Invalid);
  } else {
    auto encapsulated = f.run(request(Operation::PqEncapsulate, a.name, 0, key.data));
    CHECK(encapsulated);
    auto parts = unpackParts(encapsulated.data, 2);
    CHECK(parts);
    CHECK((*parts)[0].size() == a.ciphertext_bytes && (*parts)[1].size() == a.shared_secret_bytes);
    auto decapsulated = f.run(request(Operation::PqDecapsulate, a.name, 1, (*parts)[0]));
    CHECK(decapsulated && decapsulated.data == (*parts)[1]);
    if (a.name.compare(0, 7, "ML-KEM-") == 0) {
      (*parts)[0][0] ^= 1;
      auto rejected = f.run(request(Operation::PqDecapsulate, a.name, 1, (*parts)[0]));
      CHECK(rejected && rejected.data != (*parts)[1]); // FIPS 203 implicit rejection.
    }
    CHECK(f.run(request(Operation::PqDecapsulate, a.name, 1, Bytes(1))).code == Code::Invalid);
    for (auto& b : *parts)
      wipe(b);
  }
  CHECK(f.run(request(Operation::PqErase, a.name, 1)));
  CHECK(f.run(request(Operation::PqPublicKey, a.name, 1)).code == Code::Invalid);
  CHECK(f.engine->healthy() && f.registers->erased);
}
} // namespace
TEST(catalog_discovers_only_enabled_and_policy_allowed_algorithms) {
  PqFixture standard, experimental(true);
  auto s = standard.catalog(), x = experimental.catalog();
  CHECK(s.size() == 18 && x.size() > s.size());
  for (const auto& a : s)
    CHECK(a.standardized);
  bool falcon = false, mlkem = false, mldsa = false;
  for (const auto& a : x) {
    if (a.name == "Falcon-512") {
      falcon = true;
      CHECK(!a.standardized);
    }
    mlkem = mlkem || a.name == "ML-KEM-1024";
    mldsa = mldsa || a.name == "ML-DSA-87";
  }
  CHECK(falcon && mlkem && mldsa);
  CHECK(standard.run(request(Operation::PqKeygen, "Falcon-512", 1)).code == Code::Unavailable);
  CHECK(standard.run(request(Operation::PqKeygen, "SLH_DSA_SHA2_224_PREHASH_SHAKE_256S", 1)).code ==
        Code::Unavailable);
  CHECK(experimental.run(request(Operation::PqKeygen, "FN-DSA-512", 1)).code == Code::Unavailable);
  CHECK(experimental.run(request(Operation::PqKeygen, "Dilithium3", 1)).code == Code::Unavailable);
  std::cout << "  catalog: " << s.size() << " default-allowed modes, " << x.size()
            << " total enabled modes\n";
}
TEST(standardized_parameter_sets_and_falcon_round_trip_through_mailbox) {
  PqFixture f(true);
  size_t tested = 0;
  for (const auto& a : f.catalog()) {
    if (a.name.compare(0, 7, "ML-KEM-") == 0 || a.name.compare(0, 7, "ML-DSA-") == 0 ||
        a.name.compare(0, 13, "SLH_DSA_PURE_") == 0 || a.name.compare(0, 6, "Falcon") == 0) {
      std::cout << "  " << a.name << std::endl;
      roundTrip(f, a);
      ++tested;
    }
  }
  CHECK(tested == 22); // 3 ML-KEM + 3 ML-DSA + 12 pure SLH-DSA + 4 Falcon.
}
TEST(key_slots_are_bound_to_algorithms_bounded_and_not_exportable) {
  PqFixture f(false, 1);
  CHECK(f.run(request(Operation::PqKeygen, "ML-DSA-44", 8)));
  CHECK(f.run(request(Operation::PqSign, "ML-DSA-65", 8)).code == Code::Invalid);
  CHECK(f.run(request(Operation::PqSign, "ML-KEM-512", 8)).code == Code::Invalid);
  CHECK(f.run(request(Operation::PqPublicKey, "ML-DSA-44", 9)).code == Code::Invalid);
  CHECK(f.run(request(Operation::PqKeygen, "ML-KEM-512", 9)).code == Code::QueueFull);
  CHECK(f.run(request(Operation::PqErase, "ML-DSA-44", 8)));
  CHECK(f.run(request(Operation::PqKeygen, "ML-KEM-512", 9)));
  CHECK(f.run(request(Operation::PqErase, "ML-KEM-512", 9)));
}
TEST(context_is_bound_into_signature_and_unsupported_contexts_are_rejected) {
  PqFixture f(true);
  auto key = f.run(request(Operation::PqKeygen, "ML-DSA-65", 1));
  CHECK(key);
  auto s = request(Operation::PqSign, "ML-DSA-65", 1, Bytes{1, 2, 3});
  s.aad = Bytes{4, 5};
  auto signature = f.run(s);
  CHECK(signature);
  auto v = request(Operation::PqVerify, s.algorithm, 0, s.data);
  v.auxiliary = packParts({key.data, signature.data});
  CHECK(f.run(v).code == Code::AuthFailed);
  v.aad = s.aad;
  CHECK(f.run(v));
  s.data.clear();
  s.aad.assign(255, 0x42);
  signature = f.run(s);
  CHECK(signature);
  v.data.clear();
  v.aad = s.aad;
  v.auxiliary = packParts({key.data, signature.data});
  CHECK(f.run(v));
  CHECK(f.run(request(Operation::PqKeygen, "Falcon-512", 2)));
  s.algorithm = "Falcon-512";
  s.key_slot = 2;
  CHECK(f.run(s).code == Code::Invalid);
}
TEST(large_public_keys_and_messages_cross_new_mailbox_windows) {
  PqFixture f(true);
  for (const auto& a : f.catalog())
    if (a.name == "Classic-McEliece-8192128f" || a.name == "OV-V")
      roundTrip(f, a);
  auto key = f.run(request(Operation::PqKeygen, "ML-DSA-44", 1));
  CHECK(key);
  auto s = request(Operation::PqSign, "ML-DSA-44", 1, Bytes(256 * 1024, 0x55));
  auto sig = f.run(s);
  CHECK(sig);
  auto v = request(Operation::PqVerify, s.algorithm, 0, s.data);
  v.auxiliary = packParts({key.data, sig.data});
  CHECK(f.run(v));
}
TEST(pq_wire_framing_rejects_truncation_overflow_unknown_op_and_trailing_bytes) {
  auto r = request(Operation::PqVerify, "ML-DSA-44", 0, Bytes{1});
  r.auxiliary = packParts({Bytes{2}, Bytes{3}});
  auto wire = encode(r);
  auto decoded = decode(wire);
  CHECK(decoded && decoded->algorithm == r.algorithm && decoded->auxiliary == r.auxiliary);
  for (size_t i = 0; i < wire.size(); ++i) {
    auto truncated = wire;
    truncated.resize(i);
    CHECK(!decode(truncated));
  }
  auto changed = wire;
  changed.push_back(0);
  CHECK(!decode(changed));
  for (size_t offset :
       {size_t(4), size_t(12), size_t(16), size_t(20), size_t(24), size_t(28), size_t(32)}) {
    changed = wire;
    for (size_t j = 0; j < 4; ++j)
      changed[offset + j] = 255;
    CHECK(!decode(changed));
  }
  auto parts = packParts({Bytes(10, 1), Bytes(20, 2)});
  CHECK(unpackParts(parts, 2));
  parts.push_back(0);
  CHECK(!unpackParts(parts, 2));
  CHECK(!unpackParts(parts, 513));
  r.key_slot = 4;
  CHECK(!valid(r));
  r.key_slot = 0;
  r.aad.resize(256);
  CHECK(!valid(r));
  std::mt19937 rng(741);
  for (size_t i = 0; i < 10000; ++i) {
    Bytes fuzz(rng() % 128);
    for (auto& b : fuzz)
      b = static_cast<uint8_t>(rng());
    (void)decode(fuzz);
    (void)decodeCatalog(fuzz);
    (void)unpackParts(fuzz, rng() % 8);
  }
}
TEST(pq_capabilities_and_bad_responses_fail_without_software_fallback) {
  for (const auto fault :
       {Fault::WrongSequence, Fault::OversizedOutput, Fault::ShortOutput, Fault::BadAck}) {
    PqFixture f;
    f.registers->fault = fault;
    auto result = f.engine->execute(request(Operation::PqCatalog), 50ms);
    CHECK(!result && !f.engine->healthy());
    CHECK(f.run(request(Operation::PqCatalog)).code == Code::Unavailable);
  }
  auto io = std::make_unique<EmulatedRegisters>(oqsBackend(options()), reg::Version, 0xffff);
  auto* registers = io.get();
  MailboxBackend abi1(std::move(io));
  CHECK(abi1.execute(request(Operation::PqCatalog), Clock::now() + 1s).code == Code::Unavailable);
  CHECK(registers->submissions == 0 && abi1.healthy());
  auto software = oqsBackend(options());
  Request classical;
  CHECK(software->execute(classical, Clock::now() + 1s).code == Code::Unavailable);
}
TEST(plugin_missing_or_wrong_library_fails_at_startup) {
  for (const auto& path : {std::string("relative.so"), std::string("/missing/libfpga_pq.so"),
                           std::string("/lib/x86_64-linux-gnu/libc.so.6")}) {
    auto o = options();
    o.library = path;
    bool threw = false;
    try {
      (void)oqsBackend(o);
    } catch (const std::exception&) {
      threw = true;
    }
    CHECK(threw);
  }
}
int main(int argc, char** argv) {
  if (argc != 2) {
    std::cerr << "usage: pq_test /absolute/path/plugin.so\n";
    return 2;
  }
  plugin = argv[1];
  const int result = tests::run();
  if (result)
    return result;
  if (std::getenv("PQ_TEST_ALL")) {
    PqFixture discovery(true);
    size_t count = 0, failures = 0;
    for (const auto& a : discovery.catalog()) {
      const char* selected = std::getenv("PQ_TEST_MATCH");
      if (selected && a.name.find(selected) == std::string::npos)
        continue;
      std::cout << "ALL " << ++count << " " << a.name << std::endl;
      try {
        PqFixture f(true);
        roundTrip(f, a);
      } catch (const std::exception& e) {
        ++failures;
        std::cerr << e.what() << std::endl;
      }
    }
    std::cout << "ALL " << count - failures << '/' << count << " enabled algorithm modes passed\n";
    return failures ? 1 : 0;
  }
}
