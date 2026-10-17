#include <dlfcn.h>

#include <stdexcept>

#include "fpga/mailbox.h"
#include "fpga/pq.h"
#include "fpga/pq_plugin.h"

namespace fpga {
namespace {
class OqsBackend final : public Backend {
public:
  explicit OqsBackend(const OqsOptions& o) {
    if (o.library.empty() || o.library.front() != '/' || !o.max_keys || o.max_keys > 1024)
      throw std::invalid_argument(
          "software-liboqs requires an absolute plugin path and max_keys 1..1024");
    library_ = dlopen(o.library.c_str(), RTLD_NOW | RTLD_LOCAL);
    if (!library_)
      throw std::runtime_error("cannot load PQ software plugin: " + std::string(dlerror()));
    auto entry = reinterpret_cast<fpga_pq_entry_fn>(dlsym(library_, "fpga_pq_plugin_v1"));
    api_ = entry ? entry() : nullptr;
    if (api_ && api_->abi == FPGA_PQ_PLUGIN_ABI && api_->create && api_->destroy && api_->execute)
      instance_ = api_->create(o.allow_experimental, o.max_keys);
    if (!instance_) {
      dlclose(library_);
      library_ = nullptr;
      throw std::runtime_error("incompatible or uninitializable PQ software plugin");
    }
  }
  ~OqsBackend() override {
    api_->destroy(instance_);
    dlclose(library_);
  }
  bool healthy() const override { return healthy_.load(); }
  Result execute(const Request& r, Clock::time_point deadline) override {
    if (!healthy_)
      return {Code::Unavailable, {}};
    if (!valid(r))
      return {Code::Invalid, {}};
    if (!isPq(r.operation))
      return {Code::Unavailable, {}};
    if (Clock::now() >= deadline)
      return {Code::Timeout, {}};
    Bytes input = encode(r);
    struct Wiper {
      Bytes& b;
      ~Wiper() { wipe(b); }
    } guard{input};
    Result out;
    out.data.resize(MaxPqWire);
    size_t length = out.data.size();
    const auto remaining =
        std::chrono::duration_cast<std::chrono::milliseconds>(deadline - Clock::now()).count();
    if (remaining < 1)
      return {Code::Timeout, {}};
    const uint32_t status = api_->execute(instance_, input.data(), input.size(), out.data.data(),
                                          &length, static_cast<uint32_t>(remaining));
    if (status > static_cast<uint32_t>(Code::Cancelled) || length > out.data.size() ||
        (status != 0 && length != 0)) {
      healthy_ = false;
      return {Code::DeviceError, {}};
    }
    if (status != 0)
      return {static_cast<Code>(status), {}};
    // Wipe unused capacity before shrinking; outputs can contain KEM shared secrets.
    for (size_t i = length; i < out.data.size(); ++i)
      out.data[i] = 0;
    out.data.resize(length);
    if (!validPqOutput(r, out.data)) {
      healthy_ = false;
      return {Code::DeviceError, {}};
    }
    return out;
  }

private:
  void* library_{nullptr};
  void* instance_{nullptr};
  const fpga_pq_api* api_{nullptr};
  std::atomic<bool> healthy_{true};
};
} // namespace
std::unique_ptr<Backend> oqsBackend(const OqsOptions& o) { return std::make_unique<OqsBackend>(o); }
std::shared_ptr<Engine> oqsEngine(const OqsOptions& o) {
  return std::make_shared<Engine>(oqsBackend(o), o.queue_capacity);
}
} // namespace fpga
