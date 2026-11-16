#pragma once
#include <map>

#include "fpga/crypto.h"
#include "fpga/pq.h"

namespace fpga {
namespace reg {
constexpr uint32_t Magic = 0x46504331, Version = 1;
constexpr uint32_t PqVersion = 2;
constexpr uint64_t Identity = 0x00, Abi = 0x04, Capabilities = 0x08, Status = 0x0c;
constexpr uint64_t RequestLength = 0x10, ResponseLength = 0x14;
constexpr uint64_t SequenceLo = 0x18, SequenceHi = 0x1c;
constexpr uint64_t CompletedLo = 0x20, CompletedHi = 0x24;
constexpr uint64_t Doorbell = 0x28, Error = 0x2c;
constexpr uint64_t RequestWindow = 0x1000, ResponseWindow = 0x11000;
constexpr uint64_t PqResponseWindow = 0x801000;
constexpr uint32_t Idle = 0, Busy = 1, Done = 2, Failed = 3, Submit = 1, Ack = 2;
} // namespace reg
class RegisterIo {
public:
  virtual ~RegisterIo() = default;
  virtual uint32_t read(uint64_t offset) = 0;
  virtual void write(uint64_t offset, uint32_t value) = 0;
};
Bytes encode(const Request& request);
std::optional<Request> decode(const Bytes& bytes);
class MailboxBackend final : public Backend {
public:
  explicit MailboxBackend(std::unique_ptr<RegisterIo> io);
  Result execute(const Request&, Clock::time_point) override;
  bool healthy() const override { return healthy_.load(); }

private:
  Result poison(Code code);
  std::unique_ptr<RegisterIo> io_;
  std::atomic<bool> healthy_{true};
  uint32_t capabilities_{0};
  uint32_t version_{0};
  uint64_t sequence_{0};
  bool catalog_loaded_{false};
  std::map<std::string, PqAlgorithm> algorithms_;
};
struct AwsOptions {
  int slot{0};
  std::string library{"libfpga_mgmt.so"};
  std::string lock_directory{"/run/lock/envoy-fpga-crypto"};
  size_t queue_capacity{128};
  bool operator==(const AwsOptions& other) const;
};
// All providers/filters in this process share one engine per physical slot.
// An exclusive flock also prevents a second process from using the mailbox.
std::shared_ptr<Engine> awsEngine(const AwsOptions& options);
} // namespace fpga
