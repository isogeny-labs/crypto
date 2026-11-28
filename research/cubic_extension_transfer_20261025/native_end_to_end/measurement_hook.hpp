#pragma once
// Optional resident observation hook; disabled unless the runner supplies both
// descriptor and source-bound library. No cryptographic input is added.
#include <cerrno>
#include <climits>
#include <cstdlib>
#include <dlfcn.h>
#include <stdexcept>

class CubicMeasurement {
  void *library_ = nullptr;
  void *state_ = nullptr;
  int (*finish_)(void *) = nullptr;
public:
  CubicMeasurement() {
    const char *library = std::getenv("CUBIC_MEASURE_LIBRARY");
    const char *descriptor = std::getenv("CUBIC_MEASURE_FD");
    if (!library && !descriptor) return;
    if (!library || !descriptor) throw std::runtime_error("incomplete measurement configuration");
    char *end = nullptr;
    errno = 0;
    long fd = std::strtol(descriptor, &end, 10);
    if (errno || !end || *end || fd < 3 || fd > INT_MAX)
      throw std::runtime_error("invalid measurement descriptor");
    library_ = dlopen(library, RTLD_NOW | RTLD_LOCAL);
    if (!library_) throw std::runtime_error("measurement library unavailable");
    auto begin = reinterpret_cast<void *(*)(int)>(dlsym(library_, "cubic_measure_begin"));
    finish_ = reinterpret_cast<int (*)(void *)>(dlsym(library_, "cubic_measure_finish"));
    if (!begin || !finish_ || !(state_ = begin(static_cast<int>(fd)))) {
      dlclose(library_);
      library_ = nullptr;
      throw std::runtime_error("measurement initialization failed");
    }
  }
  CubicMeasurement(const CubicMeasurement &) = delete;
  CubicMeasurement &operator=(const CubicMeasurement &) = delete;
  ~CubicMeasurement() {
    if (!library_) return;
    int status = finish_(state_);
    dlclose(library_);
    if (status != 0) std::_Exit(126);
  }
};
