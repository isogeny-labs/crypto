#include "fpga/mailbox.h"

#include <algorithm>
#include <limits>
#include <stdexcept>

#include "fpga/pq.h"

namespace fpga {
namespace {
uint32_t load(const Bytes& bytes, size_t at) {
  return uint32_t(bytes[at]) | uint32_t(bytes[at + 1]) << 8 | uint32_t(bytes[at + 2]) << 16 |
         uint32_t(bytes[at + 3]) << 24;
}
void store(Bytes& bytes, uint32_t value) {
  for (size_t i = 0; i < 4; ++i)
    bytes.push_back(static_cast<uint8_t>(value >> (8 * i)));
}
} // namespace
Bytes encode(const Request& r) {
  if (!valid(r))
    throw std::invalid_argument("invalid FPGA request");
  Bytes bytes;
  if (isPq(r.operation)) {
    bytes.reserve(36 + r.algorithm.size() + r.aad.size() + r.auxiliary.size() + r.data.size());
    for (uint32_t v : {reg::PqVersion, static_cast<uint32_t>(r.operation), r.key_slot,
                       static_cast<uint32_t>(r.data.size()), static_cast<uint32_t>(r.aad.size()),
                       uint32_t(0), uint32_t(0), static_cast<uint32_t>(r.algorithm.size()),
                       static_cast<uint32_t>(r.auxiliary.size())})
      store(bytes, v);
    bytes.insert(bytes.end(), r.algorithm.begin(), r.algorithm.end());
    for (const auto* part : {&r.aad, &r.auxiliary, &r.data})
      bytes.insert(bytes.end(), part->begin(), part->end());
    return bytes;
  }
  bytes.reserve(32 + r.nonce.size() + r.inline_key.size() + r.aad.size() + r.data.size());
  for (uint32_t v : {reg::Version, static_cast<uint32_t>(r.operation), r.key_slot,
                     static_cast<uint32_t>(r.data.size()), static_cast<uint32_t>(r.aad.size()),
                     static_cast<uint32_t>(r.nonce.size()),
                     static_cast<uint32_t>(r.inline_key.size()), uint32_t(0)})
    store(bytes, v);
  for (const auto* part : {&r.nonce, &r.inline_key, &r.aad, &r.data})
    bytes.insert(bytes.end(), part->begin(), part->end());
  return bytes;
}
std::optional<Request> decode(const Bytes& b) {
  if (b.size() >= 36 && b.size() <= MaxPqWire && load(b, 0) == reg::PqVersion) {
    const size_t data = load(b, 12), context = load(b, 16), name = load(b, 28), aux = load(b, 32);
    if (load(b, 20) || load(b, 24) || data > MaxPqData || context > 255 || name > 96 ||
        aux > MaxPqData || b.size() != 36 + data + context + name + aux)
      return std::nullopt;
    Request r;
    r.operation = static_cast<Operation>(load(b, 4));
    r.key_slot = load(b, 8);
    if (!isPq(r.operation))
      return std::nullopt;
    r.algorithm.assign(b.begin() + 36, b.begin() + 36 + name);
    size_t at = 36 + name;
    const std::pair<Bytes*, size_t> parts[] = {
        {&r.aad, context}, {&r.auxiliary, aux}, {&r.data, data}};
    for (const auto& part : parts) {
      part.first->assign(b.begin() + at, b.begin() + at + part.second);
      at += part.second;
    }
    if (!valid(r))
      return std::nullopt;
    return r;
  }
  if (b.size() < 32 || b.size() > MaxWire || load(b, 0) != reg::Version || load(b, 28) != 0)
    return std::nullopt;
  const size_t data = load(b, 12), aad = load(b, 16), nonce = load(b, 20), key = load(b, 24);
  if (data > MaxData + 16 || aad > MaxAad || nonce > 12 || key > 32 ||
      b.size() != 32 + data + aad + nonce + key)
    return std::nullopt;
  Request r;
  r.operation = static_cast<Operation>(load(b, 4));
  r.key_slot = load(b, 8);
  if (isPq(r.operation))
    return std::nullopt;
  size_t pos = 32;
  const std::pair<Bytes*, size_t> parts[] = {
      {&r.nonce, nonce}, {&r.inline_key, key}, {&r.aad, aad}, {&r.data, data}};
  for (const auto& part : parts) {
    part.first->assign(b.begin() + pos, b.begin() + pos + part.second);
    pos += part.second;
  }
  if (!valid(r))
    return std::nullopt;
  return r;
}
MailboxBackend::MailboxBackend(std::unique_ptr<RegisterIo> io) : io_(std::move(io)) {
  if (!io_ || io_->read(reg::Identity) != reg::Magic)
    throw std::runtime_error("FPGA image does not implement FPC1");
  version_ = io_->read(reg::Abi);
  if (version_ != reg::Version && version_ != reg::PqVersion)
    throw std::runtime_error("FPGA image does not implement FPC1 ABI version 1 or 2");
  capabilities_ = io_->read(reg::Capabilities);
  if (io_->read(reg::Status) != reg::Idle)
    throw std::runtime_error("FPGA mailbox is not idle; operator recovery required");
}
Result MailboxBackend::poison(Code code) {
  healthy_ = false;
  return {code, {}};
}
Result MailboxBackend::execute(const Request& r, Clock::time_point deadline) {
  if (!healthy_)
    return {Code::Unavailable, {}};
  if (!valid(r))
    return {Code::Invalid, {}};
  const auto op = static_cast<uint32_t>(r.operation);
  if (isPq(r.operation) && version_ != reg::PqVersion)
    return {Code::Unavailable, {}};
  if ((capabilities_ & (uint32_t(1) << (op - 1))) == 0)
    return {Code::Unavailable, {}};
  if (Clock::now() >= deadline)
    return {Code::Timeout, {}};
  const PqAlgorithm* algorithm = nullptr;
  if (isPq(r.operation) && r.operation != Operation::PqCatalog) {
    if (!catalog_loaded_) {
      Request query;
      query.operation = Operation::PqCatalog;
      const auto discovered = execute(query, deadline);
      if (!discovered)
        return {discovered.code, {}};
    }
    const auto found = algorithms_.find(r.algorithm);
    if (found == algorithms_.end())
      return {Code::Unavailable, {}};
    algorithm = &found->second;
    if (!validPqRequest(r, *algorithm))
      return {Code::Invalid, {}};
  }
  Bytes bytes = encode(r);
  struct Wiper {
    Bytes& bytes;
    ~Wiper() { wipe(bytes); }
  } wiper{bytes};
  try {
    if (io_->read(reg::Status) != reg::Idle || sequence_ == std::numeric_limits<uint64_t>::max())
      return poison(Code::DeviceError);
    ++sequence_;
    for (size_t i = 0; i < bytes.size(); i += 4) {
      if (Clock::now() >= deadline)
        return poison(Code::Timeout);
      uint32_t word = 0;
      for (size_t j = 0; j < 4 && i + j < bytes.size(); ++j)
        word |= uint32_t(bytes[i + j]) << (j * 8);
      io_->write(reg::RequestWindow + i, word);
    }
    io_->write(reg::RequestLength, static_cast<uint32_t>(bytes.size()));
    io_->write(reg::SequenceLo, static_cast<uint32_t>(sequence_));
    io_->write(reg::SequenceHi, static_cast<uint32_t>(sequence_ >> 32));
    // Readback drains posted MMIO writes before ringing the doorbell.
    if (io_->read(reg::RequestLength) != bytes.size())
      return poison(Code::DeviceError);
    io_->write(reg::Doorbell, reg::Submit);
    for (;;) {
      if (Clock::now() >= deadline)
        return poison(Code::Timeout);
      const uint32_t status = io_->read(reg::Status);
      if (status == reg::Busy) {
        std::this_thread::sleep_for(std::chrono::microseconds(25));
        continue;
      }
      if (status != reg::Done && status != reg::Failed)
        return poison(Code::DeviceError);
      const uint64_t completed =
          uint64_t(io_->read(reg::CompletedLo)) | uint64_t(io_->read(reg::CompletedHi)) << 32;
      if (completed != sequence_)
        return poison(Code::DeviceError);
      Result result;
      if (status == reg::Failed) {
        if (io_->read(reg::ResponseLength) != 0)
          return poison(Code::DeviceError);
        const auto error = io_->read(reg::Error);
        result.code = error == 1                                 ? Code::AuthFailed
                      : version_ == reg::PqVersion && error == 3 ? Code::Invalid
                      : version_ == reg::PqVersion && error == 4 ? Code::Unavailable
                      : version_ == reg::PqVersion && error == 5 ? Code::QueueFull
                                                                 : Code::DeviceError;
      } else {
        const uint32_t length = io_->read(reg::ResponseLength);
        const size_t expected = maxOutput(r);
        const bool variable = isSign(r.operation);
        const size_t minimum = r.operation == Operation::EcdsaP256Sha256 ? 8 : 256;
        if (length > expected || (!isPq(r.operation) && ((!variable && length != expected) ||
                                                         (variable && length < minimum))))
          return poison(Code::DeviceError);
        result.data.resize(length);
        for (size_t i = 0; i < length; i += 4) {
          if (Clock::now() >= deadline) {
            wipe(result.data);
            return poison(Code::Timeout);
          }
          const uint32_t word = io_->read(
              (version_ == reg::PqVersion ? reg::PqResponseWindow : reg::ResponseWindow) + i);
          for (size_t j = 0; j < 4 && i + j < length; ++j)
            result.data[i + j] = static_cast<uint8_t>(word >> (8 * j));
        }
        if (isPq(r.operation) && !validPqOutput(r, result.data, algorithm))
          return poison(Code::DeviceError);
        if (r.operation == Operation::PqCatalog) {
          const auto catalog = decodeCatalog(result.data);
          algorithms_.clear();
          for (const auto& a : *catalog)
            algorithms_.emplace(a.name, a);
          catalog_loaded_ = true;
        }
      }
      io_->write(reg::Doorbell, reg::Ack); // Erasure may take multiple FPGA cycles.
      for (;;) {
        if (Clock::now() >= deadline) {
          wipe(result.data);
          return poison(Code::Timeout);
        }
        const uint32_t state = io_->read(reg::Status);
        if (state == reg::Idle)
          return result;
        if (state != reg::Busy) {
          wipe(result.data);
          return poison(Code::DeviceError);
        }
        std::this_thread::sleep_for(std::chrono::microseconds(25));
      }
    }
  } catch (...) {
    return poison(Code::DeviceError);
  }
}
} // namespace fpga
