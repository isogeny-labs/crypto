#pragma once

#include <atomic>
#include <chrono>
#include <condition_variable>
#include <cstdint>
#include <deque>
#include <memory>
#include <mutex>
#include <optional>
#include <string>
#include <thread>
#include <utility>
#include <vector>

namespace fpga {
using Bytes = std::vector<uint8_t>;
using Clock = std::chrono::steady_clock;
enum class Operation : uint32_t {
  Sha256 = 1,
  Aes128Seal = 2,
  Aes128Open = 3,
  Aes256Seal = 4,
  Aes256Open = 5,
  EcdsaP256Sha256 = 6,
  RsaPssSha256 = 7,
  RsaPkcs1Sha256 = 8,
  PqCatalog = 9,
  PqKeygen = 10,
  PqSign = 11,
  PqVerify = 12,
  PqEncapsulate = 13,
  PqDecapsulate = 14,
  PqPublicKey = 15,
  PqErase = 16,
};
enum class Code {
  Ok = 0,
  Invalid = 1,
  QueueFull = 2,
  Timeout = 3,
  Unavailable = 4,
  DeviceError = 5,
  AuthFailed = 6,
  Cancelled = 7,
};
constexpr size_t MaxData = 48 * 1024;
constexpr size_t MaxAad = 8 * 1024;
constexpr size_t MaxWire = 64 * 1024;
constexpr size_t MaxPqData = 3 * 1024 * 1024;
constexpr size_t MaxPqWire = 8 * 1024 * 1024;
void wipe(Bytes& bytes);
struct Request {
  Operation operation{Operation::Sha256};
  uint32_t key_slot{0};
  Bytes data, aad, nonce, inline_key;
  std::string algorithm; // Exact catalog name; no aliases between standards and drafts.
  Bytes auxiliary;       // PQ verification: framed public key and signature.
  Request() = default;
  Request(const Request&) = default;
  Request& operator=(const Request&);
  Request(Request&&) noexcept = default;
  Request& operator=(Request&&) noexcept;
  ~Request();
};
struct Result {
  Code code{Code::Ok};
  Bytes data;
  Result() = default;
  Result(Code status, Bytes bytes) : code(status), data(std::move(bytes)) {}
  Result(const Result&) = default;
  Result& operator=(const Result&);
  Result(Result&&) noexcept = default;
  Result& operator=(Result&&) noexcept;
  ~Result() { wipe(data); }
  explicit operator bool() const { return code == Code::Ok; }
};
bool isSeal(Operation op);
bool isOpen(Operation op);
bool isAes(Operation op);
bool isSign(Operation op);
bool isPq(Operation op);
bool valid(const Request& request);
size_t maxOutput(const Request& request);
const char* codeName(Code code);
std::string hex(const Bytes& data);
std::optional<Bytes> unhex(const std::string& text);

class Backend {
public:
  virtual ~Backend() = default;
  // Called only by the Engine's single worker. Deadline includes queue time.
  virtual Result execute(const Request&, Clock::time_point deadline) = 0;
  virtual bool healthy() const = 0;
};

class Ticket {
public:
  Ticket() = default;
  bool ready() const;
  std::optional<Result> take();
  Result waitUntil(Clock::time_point deadline);
  void cancel();

private:
  struct State {
    std::mutex mutex;
    std::condition_variable changed;
    std::optional<Result> result;
    std::atomic<bool> cancelled{false};
    ~State() {
      if (result)
        wipe(result->data);
    }
  };
  explicit Ticket(std::shared_ptr<State> state) : state_(std::move(state)) {}
  std::shared_ptr<State> state_;
  friend class Engine;
};

struct Counters {
  std::atomic<uint64_t> submitted{0}, completed{0}, failed{0}, rejected{0};
};
class Engine {
public:
  explicit Engine(std::unique_ptr<Backend> backend, size_t capacity = 128);
  ~Engine();
  Engine(const Engine&) = delete;
  Engine& operator=(const Engine&) = delete;
  Ticket submit(Request request, std::chrono::milliseconds timeout);
  Result execute(Request request, std::chrono::milliseconds timeout);
  bool healthy() const { return backend_->healthy(); }
  const Counters& counters() const { return counters_; }

private:
  struct Job {
    Request request;
    Clock::time_point deadline;
    std::shared_ptr<Ticket::State> state;
  };
  void run();
  std::unique_ptr<Backend> backend_;
  size_t capacity_, outstanding_{0};
  bool stopping_{false};
  std::mutex mutex_;
  std::condition_variable changed_;
  std::deque<Job> jobs_;
  Counters counters_;
  std::thread worker_;
};
} // namespace fpga
