#include "fpga/crypto.h"

#include <stdexcept>
#include <utility>

namespace fpga {
void wipe(Bytes& bytes) {
  volatile uint8_t* p = bytes.data();
  for (size_t i = 0; i < bytes.size(); ++i)
    p[i] = 0;
}
Request::~Request() {
  wipe(inline_key);
  wipe(data);
}
Request& Request::operator=(const Request& other) {
  if (this != &other) {
    Request temporary(other);
    *this = std::move(temporary);
  }
  return *this;
}
Request& Request::operator=(Request&& other) noexcept {
  if (this != &other) {
    wipe(inline_key);
    wipe(data);
    operation = other.operation;
    key_slot = other.key_slot;
    data = std::move(other.data);
    aad = std::move(other.aad);
    nonce = std::move(other.nonce);
    inline_key = std::move(other.inline_key);
    algorithm = std::move(other.algorithm);
    auxiliary = std::move(other.auxiliary);
  }
  return *this;
}
Result& Result::operator=(const Result& other) {
  if (this != &other) {
    Result temporary(other);
    *this = std::move(temporary);
  }
  return *this;
}
Result& Result::operator=(Result&& other) noexcept {
  if (this != &other) {
    wipe(data);
    code = other.code;
    data = std::move(other.data);
  }
  return *this;
}
bool isSeal(Operation op) { return op == Operation::Aes128Seal || op == Operation::Aes256Seal; }
bool isOpen(Operation op) { return op == Operation::Aes128Open || op == Operation::Aes256Open; }
bool isAes(Operation op) { return isSeal(op) || isOpen(op); }
bool isSign(Operation op) {
  return op == Operation::EcdsaP256Sha256 || op == Operation::RsaPssSha256 ||
         op == Operation::RsaPkcs1Sha256;
}
bool isPq(Operation op) { return op >= Operation::PqCatalog && op <= Operation::PqErase; }
bool valid(const Request& r) {
  if (isPq(r.operation)) {
    if (!r.nonce.empty() || !r.inline_key.empty() || r.aad.size() > 255 ||
        r.data.size() > MaxPqData || r.auxiliary.size() > MaxPqData ||
        36 + r.algorithm.size() + r.aad.size() + r.data.size() + r.auxiliary.size() > MaxPqWire)
      return false;
    if (r.operation == Operation::PqCatalog)
      return r.algorithm.empty() && !r.key_slot && r.data.empty() && r.aad.empty() &&
             r.auxiliary.empty();
    if (r.algorithm.empty() || r.algorithm.size() > 96)
      return false;
    for (unsigned char c : r.algorithm)
      if (!((c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z') || (c >= '0' && c <= '9') ||
            c == '-' || c == '_'))
        return false;
    const bool verify = r.operation == Operation::PqVerify;
    if ((!verify && !r.auxiliary.empty()) || (verify && r.auxiliary.empty()))
      return false;
    if (r.operation != Operation::PqSign && !verify && !r.aad.empty())
      return false;
    if (verify || r.operation == Operation::PqEncapsulate)
      return r.key_slot == 0;
    if (!r.key_slot)
      return false;
    return (r.operation == Operation::PqSign || r.operation == Operation::PqDecapsulate) ||
           r.data.empty();
  }
  if (!r.algorithm.empty() || !r.auxiliary.empty())
    return false;
  if (r.data.size() > MaxData + (isOpen(r.operation) ? 16 : 0) || r.aad.size() > MaxAad)
    return false;
  if (isAes(r.operation)) {
    const size_t key_size =
        (r.operation == Operation::Aes128Seal || r.operation == Operation::Aes128Open) ? 16 : 32;
    return r.nonce.size() == 12 && (!isOpen(r.operation) || r.data.size() >= 16) &&
           ((r.key_slot == 0 && r.inline_key.size() == key_size) ||
            (r.key_slot != 0 && r.inline_key.empty()));
  }
  if (!r.nonce.empty() || !r.aad.empty() || !r.inline_key.empty())
    return false;
  return (r.operation == Operation::Sha256 && r.key_slot == 0) ||
         (isSign(r.operation) && r.key_slot != 0);
}
size_t maxOutput(const Request& r) {
  if (isPq(r.operation)) {
    if (r.operation == Operation::PqVerify || r.operation == Operation::PqErase)
      return 0;
    return MaxPqWire;
  }
  if (isSeal(r.operation))
    return r.data.size() + 16;
  if (isOpen(r.operation))
    return r.data.size() - 16;
  if (r.operation == Operation::Sha256)
    return 32;
  if (r.operation == Operation::EcdsaP256Sha256)
    return 72;
  return 512; // RSA-2048 through RSA-4096.
}
const char* codeName(Code code) {
  switch (code) {
  case Code::Ok:
    return "ok";
  case Code::Invalid:
    return "invalid_request";
  case Code::QueueFull:
    return "queue_full";
  case Code::Timeout:
    return "deadline_exceeded";
  case Code::Unavailable:
    return "device_unavailable";
  case Code::DeviceError:
    return "device_error";
  case Code::AuthFailed:
    return "authentication_failed";
  case Code::Cancelled:
    return "cancelled";
  }
  return "unknown";
}
std::string hex(const Bytes& data) {
  constexpr char digits[] = "0123456789abcdef";
  std::string result;
  result.reserve(data.size() * 2);
  for (uint8_t c : data) {
    result += digits[c >> 4];
    result += digits[c & 15];
  }
  return result;
}
std::optional<Bytes> unhex(const std::string& text) {
  if (text.size() % 2 != 0 || text.size() > MaxWire * 2)
    return std::nullopt;
  auto digit = [](char c) -> int {
    if (c >= '0' && c <= '9')
      return c - '0';
    if (c >= 'a' && c <= 'f')
      return c - 'a' + 10;
    if (c >= 'A' && c <= 'F')
      return c - 'A' + 10;
    return -1;
  };
  Bytes result;
  result.reserve(text.size() / 2);
  for (size_t i = 0; i < text.size(); i += 2) {
    const int hi = digit(text[i]), lo = digit(text[i + 1]);
    if (hi < 0 || lo < 0)
      return std::nullopt;
    result.push_back(static_cast<uint8_t>((hi << 4) | lo));
  }
  return result;
}
bool Ticket::ready() const {
  if (!state_)
    return false;
  std::lock_guard<std::mutex> lock(state_->mutex);
  return state_->result.has_value();
}
std::optional<Result> Ticket::take() {
  if (!state_)
    return std::nullopt;
  std::lock_guard<std::mutex> lock(state_->mutex);
  auto result = std::move(state_->result);
  state_->result.reset();
  return result;
}
Result Ticket::waitUntil(Clock::time_point deadline) {
  if (!state_)
    return {Code::Invalid, {}};
  std::unique_lock<std::mutex> lock(state_->mutex);
  if (!state_->changed.wait_until(lock, deadline, [&] { return state_->result.has_value(); })) {
    state_->cancelled = true;
    return {Code::Timeout, {}};
  }
  Result result = std::move(*state_->result);
  state_->result.reset();
  return result;
}
void Ticket::cancel() {
  if (state_)
    state_->cancelled = true;
}
Engine::Engine(std::unique_ptr<Backend> backend, size_t capacity)
    : backend_(std::move(backend)), capacity_(capacity) {
  if (!backend_ || capacity_ == 0 || capacity_ > 65536)
    throw std::invalid_argument("invalid engine");
  worker_ = std::thread([this] { run(); });
}
Engine::~Engine() {
  {
    std::lock_guard<std::mutex> lock(mutex_);
    stopping_ = true;
    for (auto& job : jobs_)
      job.state->cancelled = true;
  }
  changed_.notify_one();
  worker_.join(); // Backend must honor its deadline, including during shutdown.
}
Ticket Engine::submit(Request request, std::chrono::milliseconds timeout) {
  auto state = std::make_shared<Ticket::State>();
  Ticket ticket(state);
  auto reject = [&](Code code) {
    state->result = Result{code, {}};
    ++counters_.rejected;
    return ticket;
  };
  if (!valid(request) || timeout.count() < 1 || timeout > std::chrono::seconds(60))
    return reject(Code::Invalid);
  std::lock_guard<std::mutex> lock(mutex_);
  if (stopping_ || !backend_->healthy())
    return reject(Code::Unavailable);
  if (outstanding_ >= capacity_)
    return reject(Code::QueueFull);
  jobs_.push_back({std::move(request), Clock::now() + timeout, state});
  ++outstanding_;
  ++counters_.submitted;
  changed_.notify_one();
  return ticket;
}
Result Engine::execute(Request request, std::chrono::milliseconds timeout) {
  const auto deadline = Clock::now() + timeout;
  return submit(std::move(request), timeout).waitUntil(deadline);
}
void Engine::run() {
  for (;;) {
    std::optional<Job> job;
    {
      std::unique_lock<std::mutex> lock(mutex_);
      changed_.wait(lock, [&] { return stopping_ || !jobs_.empty(); });
      if (jobs_.empty() && stopping_)
        return;
      job.emplace(std::move(jobs_.front()));
      jobs_.pop_front();
    }
    Result result;
    if (job->state->cancelled)
      result.code = Code::Cancelled;
    else if (Clock::now() >= job->deadline)
      result.code = Code::Timeout;
    else {
      try {
        result = backend_->execute(job->request, job->deadline);
      } catch (...) {
        result = {Code::DeviceError, {}};
      }
      if (Clock::now() > job->deadline) {
        wipe(result.data);
        result = {Code::Timeout, {}};
      }
    }
    if (job->state->cancelled) {
      wipe(result.data);
      result = {Code::Cancelled, {}};
    }
    if (result)
      ++counters_.completed;
    else
      ++counters_.failed;
    {
      std::lock_guard<std::mutex> lock(job->state->mutex);
      job->state->result = std::move(result);
    }
    {
      std::lock_guard<std::mutex> lock(mutex_);
      --outstanding_;
    }
    job->state->changed.notify_all();
  }
}
} // namespace fpga
