#include <sys/stat.h>
#include <unistd.h>

#include <random>

#include "test.h"

#include "emulated_registers.h"

using namespace fpga;
using namespace fpga::test;
using namespace std::chrono_literals;

TEST(sha256_known_answer_through_mailbox) {
  Fixture f;
  Request request;
  request.data = {'a', 'b', 'c'};
  auto result = f.engine->execute(request, 500ms);
  CHECK(result);
  CHECK(hex(result.data) == "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
  CHECK(f.registers->submissions == 1 && f.registers->acknowledgements == 1 && f.registers->erased);
}
TEST(aes128_and_aes256_nist_known_answers) {
  Fixture f;
  for (bool big : {false, true}) {
    Request request;
    request.operation = big ? Operation::Aes256Seal : Operation::Aes128Seal;
    request.key_slot = big ? 11 : 10;
    request.nonce.resize(12);
    request.data.resize(16);
    auto result = f.engine->execute(request, 500ms);
    CHECK(result);
    CHECK(hex(result.data) ==
          (big ? "cea7403d4d606b6e074ec5d3baf39d18d0d1c8a799996bf0265b98b5d48ab919"
               : "0388dace60b6a392f328c2b971b2fe78ab6e47d42cec13bdf53a67b21257bddf"));
    request.operation = big ? Operation::Aes256Open : Operation::Aes128Open;
    request.data = result.data;
    auto opened = f.engine->execute(request, 500ms);
    CHECK(opened && opened.data == Bytes(16, 0));
    request.data.back() ^= 1;
    auto rejected = f.engine->execute(request, 500ms);
    CHECK(rejected.code == Code::AuthFailed && rejected.data.empty() && f.engine->healthy());
  }
}
TEST(aad_empty_partial_and_maximum_payloads) {
  Fixture f;
  for (size_t length : {size_t(0), size_t(1), size_t(31), MaxData}) {
    Request seal;
    seal.operation = Operation::Aes256Seal;
    seal.inline_key.resize(32, 0x55);
    seal.nonce.resize(12, 0x13);
    seal.aad = {1, 2, 3, 4, 5};
    seal.data.resize(length, 0x41);
    auto result = f.engine->execute(seal, 500ms);
    CHECK(result && result.data.size() == length + 16);
    Request open = seal;
    open.operation = Operation::Aes256Open;
    open.data = result.data;
    auto opened = f.engine->execute(open, 500ms);
    CHECK(opened && opened.data == seal.data);
    open.aad[0] ^= 1;
    CHECK(f.engine->execute(open, 500ms).code == Code::AuthFailed);
  }
}
TEST(rsa_and_ecdsa_signatures_are_independently_verified) {
  Fixture f;
  for (auto op : {Operation::EcdsaP256Sha256, Operation::RsaPssSha256, Operation::RsaPkcs1Sha256}) {
    Request request;
    request.operation = op;
    request.key_slot = op == Operation::EcdsaP256Sha256 ? 2 : 1;
    request.data = {'s', 'i', 'g', 'n', ' ', 'm', 'e'};
    auto result = f.engine->execute(request, 500ms);
    CHECK(result);
    auto key = request.key_slot == 2 ? f.ec : f.rsa;
    CHECK(verify(key.get(), request, result.data));
    request.data[0] ^= 1;
    CHECK(!verify(key.get(), request, result.data));
  }
}
TEST(request_validation_rejects_bad_keys_nonces_and_lengths) {
  Fixture f;
  Request request;
  request.data.resize(MaxData + 1);
  CHECK(f.engine->execute(request, 500ms).code == Code::Invalid);
  request.data.clear();
  request.operation = Operation::Aes128Seal;
  request.nonce.resize(11);
  request.inline_key.resize(16);
  CHECK(!valid(request));
  request.nonce.resize(12);
  request.key_slot = 10;
  CHECK(!valid(request)); // A key slot and an inline key cannot coexist.
  request.inline_key.clear();
  CHECK(valid(request));
  request.operation = static_cast<Operation>(999);
  CHECK(!valid(request));
  CHECK(f.registers->submissions == 0);
}
TEST(codec_rejects_truncation_reserved_fields_and_arbitrary_bytes) {
  Request request;
  request.data = {0, 1, 2, 3, 4};
  auto bytes = encode(request);
  CHECK(decode(bytes)->data == request.data);
  for (size_t length = 0; length < bytes.size(); ++length) {
    CHECK(!decode(Bytes(bytes.begin(), bytes.begin() + length)));
  }
  bytes[28] = 1;
  CHECK(!decode(bytes));
  std::mt19937 random(42);
  for (size_t i = 0; i < 10000; ++i) {
    Bytes arbitrary(random() % 256);
    for (auto& b : arbitrary)
      b = static_cast<uint8_t>(random());
    CHECK(!decode(arbitrary));
  }
  CHECK(!unhex("0") && !unhex("zz") && unhex("00Af") == Bytes({0, 175}));
}
TEST(protocol_faults_poison_the_device) {
  for (auto fault : {Fault::WrongSequence, Fault::OversizedOutput, Fault::ShortOutput,
                     Fault::IoError, Fault::BadAck}) {
    Fixture f;
    f.registers->fault = fault;
    CHECK(f.engine->execute(Request{}, 500ms).code == Code::DeviceError);
    CHECK(!f.engine->healthy());
    CHECK(f.engine->execute(Request{}, 500ms).code == Code::Unavailable);
  }
}
TEST(hardware_timeout_is_fail_closed_and_late_results_cannot_be_reused) {
  Fixture f;
  f.registers->fault = Fault::NeverComplete;
  auto ticket = f.engine->submit(Request{}, 5ms);
  CHECK(ticket.waitUntil(Clock::now() + 1s).code == Code::Timeout);
  CHECK(!f.engine->healthy() && f.registers->acknowledgements == 0);
  CHECK(f.engine->execute(Request{}, 100ms).code == Code::Unavailable);
}
TEST(queue_capacity_counts_inflight_and_waiting_work) {
  Fixture f(1);
  f.registers->latency = 40ms;
  auto first = f.engine->submit(Request{}, 500ms);
  auto rejected = f.engine->submit(Request{}, 500ms);
  CHECK(rejected.waitUntil(Clock::now() + 1s).code == Code::QueueFull);
  CHECK(first.waitUntil(Clock::now() + 1s));
}
TEST(acknowledgement_waits_until_hardware_erasure_finishes) {
  Fixture f;
  f.registers->erase_latency = 3ms;
  CHECK(f.engine->execute(Request{}, 500ms));
  CHECK(f.registers->erased && f.registers->acknowledgements == 1 && f.engine->healthy());
}
TEST(erasure_timeout_poisonsthe_device_without_returning_output) {
  Fixture f;
  f.registers->erase_latency = 100ms;
  auto ticket = f.engine->submit(Request{}, 5ms);
  auto result = ticket.waitUntil(Clock::now() + 1s);
  CHECK(result.code == Code::Timeout && result.data.empty());
  CHECK(!f.engine->healthy() && !f.registers->erased && f.registers->acknowledgements == 1);
}
TEST(queue_deadlines_and_cancellation_do_not_submit_stale_jobs) {
  Fixture f(8);
  f.registers->latency = 40ms;
  auto first = f.engine->submit(Request{}, 500ms);
  auto expired = f.engine->submit(Request{}, 1ms);
  auto cancelled = f.engine->submit(Request{}, 500ms);
  cancelled.cancel();
  CHECK(first.waitUntil(Clock::now() + 1s));
  CHECK(expired.waitUntil(Clock::now() + 1s).code == Code::Timeout);
  CHECK(cancelled.waitUntil(Clock::now() + 1s).code == Code::Cancelled);
  CHECK(f.registers->submissions == 1);
}
TEST(image_identity_and_initial_busy_state_are_checked) {
  for (auto item :
       {std::make_pair(reg::Identity, uint32_t(0)), std::make_pair(reg::Abi, uint32_t(3)),
        std::make_pair(reg::Status, reg::Busy)}) {
    auto io = std::make_unique<EmulatedRegisters>(std::make_unique<SoftwareBackend>());
    io->setRegister(item.first, item.second);
    bool threw = false;
    try {
      MailboxBackend backend(std::move(io));
    } catch (const std::exception&) {
      threw = true;
    }
    CHECK(threw);
  }
}
TEST(unsupported_device_capability_is_rejected_without_submission) {
  auto io = std::make_unique<EmulatedRegisters>(std::make_unique<SoftwareBackend>());
  auto* raw = io.get();
  io->setRegister(reg::Capabilities, 0);
  Engine engine(std::make_unique<MailboxBackend>(std::move(io)));
  CHECK(engine.execute(Request{}, 500ms).code == Code::Unavailable);
  CHECK(raw->submissions == 0 && engine.healthy());
}
TEST(aws_sdk_load_failure_releases_the_slot_lock) {
  char path[] = "/tmp/fpga-sdk-test-XXXXXX";
  CHECK(mkdtemp(path));
  AwsOptions options;
  options.lock_directory = path;
  options.library = "/nonexistent/fpga-sdk.so";
  for (int i = 0; i < 2; ++i) {
    bool correct_failure = false;
    try {
      awsEngine(options);
    } catch (const std::exception& error) {
      correct_failure = std::string(error.what()).find("cannot load") != std::string::npos;
    }
    CHECK(correct_failure);
  }
  CHECK(unlink((std::string(path) + "/slot-0.lock").c_str()) == 0);
  CHECK(rmdir(path) == 0);
}
TEST(concurrent_clients_receive_their_own_results) {
  Fixture f;
  std::atomic<unsigned> errors{0};
  std::vector<std::thread> clients;
  for (unsigned i = 0; i < 8; ++i)
    clients.emplace_back([&, i] {
      for (unsigned j = 0; j < 20; ++j) {
        Request request;
        request.data = {static_cast<uint8_t>(i), static_cast<uint8_t>(j)};
        SoftwareBackend reference;
        const auto expected = reference.execute(request, Clock::now() + 1s);
        const auto actual = f.engine->execute(std::move(request), 500ms);
        if (!actual || actual.data != expected.data)
          ++errors;
      }
    });
  for (auto& client : clients)
    client.join();
  CHECK(errors == 0 && f.engine->counters().completed == 160);
}
int main() { return tests::run(); }
