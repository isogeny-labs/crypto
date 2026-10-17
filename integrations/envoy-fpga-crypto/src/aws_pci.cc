#include "fpga/mailbox.h"

#include <dlfcn.h>
#include <fcntl.h>
#include <sys/file.h>
#include <unistd.h>

#include <map>
#include <stdexcept>
#include <utility>

namespace fpga {
namespace {
class AwsPci final : public RegisterIo {
public:
  explicit AwsPci(const AwsOptions& options) {
    if (options.slot < 0 || options.slot > 7) throw std::invalid_argument("invalid FPGA slot");
    const std::string path = options.lock_directory + "/slot-" + std::to_string(options.slot) + ".lock";
    lock_fd_ = open(path.c_str(), O_CREAT | O_RDWR | O_CLOEXEC | O_NOFOLLOW, 0600);
    if (lock_fd_ < 0) throw std::runtime_error("cannot open FPGA slot lock; create lock directory first");
    if (flock(lock_fd_, LOCK_EX | LOCK_NB) != 0) { cleanup(); throw std::runtime_error("FPGA slot is in use"); }
    try {
      library_ = dlopen(options.library.c_str(), RTLD_NOW | RTLD_LOCAL);
      if (!library_) throw std::runtime_error("cannot load AWS FPGA userspace SDK library");
      const auto init = symbol<int (*)()>("fpga_pci_init");
      const auto attach = symbol<int (*)(int, int, int, uint32_t, int*)>("fpga_pci_attach");
      detach_ = symbol<int (*)(int)>("fpga_pci_detach");
      peek_ = symbol<int (*)(int, uint64_t, uint32_t*)>("fpga_pci_peek");
      poke_ = symbol<int (*)(int, uint64_t, uint32_t)>("fpga_pci_poke");
      // AWS F2 SDK: FPGA_APP_PF = 0, APP_PF_BAR0 = 0, pci_bar_handle_t = int.
      if (init() != 0 || attach(options.slot, 0, 0, 0, &handle_) != 0)
        throw std::runtime_error("cannot attach AWS FPGA application BAR0");
    } catch (...) { cleanup(); throw; }
  }
  ~AwsPci() override { cleanup(); }
  uint32_t read(uint64_t offset) override {
    uint32_t value = 0;
    if (peek_(handle_, offset, &value) != 0) throw std::runtime_error("FPGA MMIO read failed");
    return value;
  }
  void write(uint64_t offset, uint32_t value) override {
    if (poke_(handle_, offset, value) != 0) throw std::runtime_error("FPGA MMIO write failed");
  }
private:
  template<class T> T symbol(const char* name) {
    auto result = reinterpret_cast<T>(dlsym(library_, name));
    if (!result) throw std::runtime_error("AWS FPGA SDK is missing PCI API symbols");
    return result;
  }
  void cleanup() {
    if (handle_ != -1 && detach_) { detach_(handle_); handle_ = -1; }
    if (library_) { dlclose(library_); library_ = nullptr; }
    if (lock_fd_ != -1) { close(lock_fd_); lock_fd_ = -1; }
  }
  int lock_fd_{-1}, handle_{-1};
  void* library_{nullptr};
  int (*detach_)(int){nullptr};
  int (*peek_)(int, uint64_t, uint32_t*){nullptr};
  int (*poke_)(int, uint64_t, uint32_t){nullptr};
};
}
bool AwsOptions::operator==(const AwsOptions& o) const {
  return slot == o.slot && library == o.library && lock_directory == o.lock_directory &&
         queue_capacity == o.queue_capacity;
}
std::shared_ptr<Engine> awsEngine(const AwsOptions& options) {
  static std::mutex mutex;
  static std::map<int, std::pair<AwsOptions, std::weak_ptr<Engine>>> engines;
  std::lock_guard<std::mutex> lock(mutex);
  auto it = engines.find(options.slot);
  if (it != engines.end()) {
    if (auto engine = it->second.second.lock()) {
      if (!(it->second.first == options)) throw std::invalid_argument("conflicting settings for FPGA slot");
      return engine;
    }
  }
  auto engine = std::make_shared<Engine>(
      std::make_unique<MailboxBackend>(std::make_unique<AwsPci>(options)), options.queue_capacity);
  engines[options.slot] = {options, engine};
  return engine;
}
} // namespace fpga
