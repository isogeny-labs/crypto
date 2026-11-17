#pragma once
#include <algorithm>
#include <map>
#include <stdexcept>

#include "fpga/mailbox.h"
#include "software_backend.h"

namespace fpga::test {
enum class Fault {
  None,
  NeverComplete,
  WrongSequence,
  OversizedOutput,
  ShortOutput,
  IoError,
  BadAck
};
// A behavioral AFI model. Every crypto test traverses the real host mailbox
// codec, register writes, sequence checks, response reads and zeroization ACK.
class EmulatedRegisters final : public RegisterIo {
public:
  explicit EmulatedRegisters(std::unique_ptr<Backend> crypto, uint32_t version = reg::Version,
                             uint32_t capabilities = 0xff)
      : crypto_(std::move(crypto)), version_(version) {
    registers_[reg::Identity] = reg::Magic;
    registers_[reg::Abi] = version;
    registers_[reg::Capabilities] = capabilities;
    registers_[reg::Status] = reg::Idle;
  }
  ~EmulatedRegisters() override {
    wipe(input_);
    wipe(output_);
  }
  Fault fault{Fault::None};
  std::chrono::milliseconds latency{0};
  std::chrono::milliseconds erase_latency{0};
  uint64_t submissions{0}, acknowledgements{0};
  bool erased{true};
  Code last_code{Code::Ok};
  uint32_t read(uint64_t offset) override {
    if (fault == Fault::IoError)
      throw std::runtime_error("injected MMIO fault");
    if (offset == reg::Status && registers_[offset] == reg::Busy && fault != Fault::NeverComplete &&
        Clock::now() >= ready_at_) {
      if (erasing_)
        finishErase();
      else
        registers_[offset] = completion_status_;
    }
    const auto response = version_ == reg::PqVersion ? reg::PqResponseWindow : reg::ResponseWindow;
    if (offset >= response && offset < response + wireLimit()) {
      const size_t at = offset - response;
      uint32_t value = 0;
      for (size_t j = 0; j < 4 && at + j < output_.size(); ++j)
        value |= uint32_t(output_[at + j]) << (8 * j);
      return value;
    }
    return registers_[offset];
  }
  void write(uint64_t offset, uint32_t value) override {
    if (fault == Fault::IoError)
      throw std::runtime_error("injected MMIO fault");
    if (offset >= reg::RequestWindow && offset < reg::RequestWindow + wireLimit()) {
      const size_t at = offset - reg::RequestWindow;
      if (input_.size() < at + 4)
        input_.resize(at + 4);
      for (size_t j = 0; j < 4; ++j)
        input_[at + j] = static_cast<uint8_t>(value >> (8 * j));
      erased = false;
      return;
    }
    if (offset != reg::Doorbell) {
      registers_[offset] = value;
      return;
    }
    if (value == reg::Ack) {
      if (fault == Fault::BadAck)
        return;
      ++acknowledgements;
      if (erase_latency.count()) {
        erasing_ = true;
        registers_[reg::Status] = reg::Busy;
        ready_at_ = Clock::now() + erase_latency;
      } else
        finishErase();
      return;
    }
    if (value != reg::Submit || registers_[reg::Status] != reg::Idle)
      throw std::runtime_error("invalid mailbox command");
    ++submissions;
    input_.resize(registers_[reg::RequestLength]);
    auto request = decode(input_);
    Result result = request ? crypto_->execute(*request, Clock::now() + std::chrono::seconds(60))
                            : Result{Code::Invalid, {}};
    last_code = result.code;
    output_ = std::move(result.data);
    completion_status_ = result ? reg::Done : reg::Failed;
    registers_[reg::Error] = result.code == Code::AuthFailed                                  ? 1
                             : version_ == reg::PqVersion && result.code == Code::Invalid     ? 3
                             : version_ == reg::PqVersion && result.code == Code::Unavailable ? 4
                             : version_ == reg::PqVersion && result.code == Code::QueueFull   ? 5
                                                                                              : 2;
    registers_[reg::CompletedLo] =
        registers_[reg::SequenceLo] + (fault == Fault::WrongSequence ? 1 : 0);
    registers_[reg::CompletedHi] = registers_[reg::SequenceHi];
    registers_[reg::ResponseLength] = fault == Fault::OversizedOutput ? wireLimit() + 1
                                      : fault == Fault::ShortOutput
                                          ? 1
                                          : static_cast<uint32_t>(output_.size());
    registers_[reg::Status] = reg::Busy;
    ready_at_ = Clock::now() + latency;
  }
  void setRegister(uint64_t offset, uint32_t value) { registers_[offset] = value; }

private:
  void finishErase() {
    wipe(input_);
    wipe(output_);
    input_.clear();
    output_.clear();
    erased = true;
    erasing_ = false;
    registers_[reg::Status] = reg::Idle;
    registers_[reg::ResponseLength] = 0;
  }
  size_t wireLimit() const { return version_ == reg::PqVersion ? MaxPqWire : MaxWire; }
  std::unique_ptr<Backend> crypto_;
  uint32_t version_;
  bool erasing_{false};
  std::map<uint64_t, uint32_t> registers_;
  Bytes input_, output_;
  Clock::time_point ready_at_;
  uint32_t completion_status_{reg::Done};
};
struct Fixture {
  Pkey rsa = generateKey(true), ec = generateKey(false);
  EmulatedRegisters* registers;
  std::shared_ptr<Engine> engine;
  explicit Fixture(size_t capacity = 128) {
    auto software = std::make_unique<SoftwareBackend>();
    software->signing_keys[1] = rsa;
    software->signing_keys[2] = ec;
    software->aes_keys[10] = Bytes(16, 0);
    software->aes_keys[11] = Bytes(32, 0);
    auto io = std::make_unique<EmulatedRegisters>(std::move(software));
    registers = io.get();
    engine = std::make_shared<Engine>(std::make_unique<MailboxBackend>(std::move(io)), capacity);
  }
};
} // namespace fpga::test
