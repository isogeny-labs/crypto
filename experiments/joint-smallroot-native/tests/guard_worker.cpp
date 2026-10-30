// SPDX-License-Identifier: LGPL-3.0-or-later
// Fixed nonresearch workers. No algebra, scientific envelope or clock
// substitution.
#include "supervisor.hpp"
#include <iostream>
#include <sys/mman.h>
using namespace jsr;
using Clock = std::chrono::steady_clock;
long long stamp() {
  return std::chrono::duration_cast<std::chrono::nanoseconds>(
             Clock::now().time_since_epoch())
      .count();
}
std::string unified() {
  std::istringstream in(read("/proc/self/cgroup"));
  std::string line;
  while (std::getline(in, line))
    if (line.rfind("0::", 0) == 0)
      return line.substr(3);
  throw std::runtime_error("unified cgroup unavailable");
}
std::string identity() {
  std::string s = "monotonic_ns=" + std::to_string(stamp()) +
                  "\npid=" + std::to_string(getpid()) +
                  "\nppid=" + std::to_string(getppid()) +
                  "\npgid=" + std::to_string(getpgrp()) +
                  "\nsid=" + std::to_string(getsid(0)) +
                  "\nuid=" + std::to_string(getuid()) +
                  "\ngid=" + std::to_string(getgid()) +
                  "\ncgroup=" + unified() +
                  "\nproc_stat=" + read("/proc/self/stat");
  std::istringstream status(read("/proc/self/status"));
  std::string line;
  while (std::getline(status, line))
    if (line.rfind("Cap", 0) == 0 || line.rfind("Seccomp", 0) == 0 ||
        line.rfind("NoNewPrivs", 0) == 0 || line.rfind("NSpid", 0) == 0)
      s += line + "\n";
  return s;
}
void snapshot(const fs::path &out, const std::string &name) {
  write(out / (name + ".identity"), identity());
  auto cg = fs::path("/sys/fs/cgroup") / unified().substr(1);
  std::string s;
  for (auto key : {"memory.max", "memory.current", "memory.swap.max",
                   "memory.swap.current", "memory.events", "memory.peak",
                   "cpu.max", "cpu.stat", "pids.max", "pids.current",
                   "cgroup.procs", "cgroup.events"}) {
    try {
      s += std::string(key) + "\n" + read(cg / key);
    } catch (const std::exception &e) {
      s += std::string(key) + " unavailable=" + e.what() + "\n";
    }
  }
  write(out / (name + ".cgroup-files"), s);
  require(exact_memory_limit(read(cg / "memory.max")),
          "first instruction did not observe exact8GiB worker limit");
  write(out / (name + ".ready"), std::to_string(stamp()) + "\n");
}
void await(const fs::path &p) {
  auto start = Clock::now();
  while (!fs::exists(p)) {
    require(Clock::now() - start < std::chrono::seconds(5),
            "worker handshake deadline");
    std::this_thread::sleep_for(std::chrono::milliseconds(10));
  }
}
[[noreturn]] void sleep_forever() {
  for (;;)
    pause();
}
int main(int argc, char **argv) {
  try {
    require(argc == 3, "guard-worker mode output");
    std::string mode = argv[1];
    fs::path out = argv[2];
    snapshot(out, "primary");
    if (mode == "ordinary")
      return 0;
    if (mode == "tree-return" || mode == "tree-sleep") {
      pid_t child = fork();
      require(child >= 0, "worker child fork");
      if (child == 0) {
        require(setsid() >= 0, "child setsid");
        snapshot(out, "child");
        pid_t grand = fork();
        require(grand >= 0, "worker grandchild fork");
        if (grand == 0) {
          require(setsid() >= 0, "grandchild setsid");
          snapshot(out, "grandchild");
          sleep_forever();
        }
        await(out / "grandchild.ready");
        write(out / "child-tree.ready", std::to_string(stamp()) + "\n");
        sleep_forever();
      }
      await(out / "child-tree.ready");
      write(out / "tree.ready", std::to_string(stamp()) + "\n");
      if (mode == "tree-return")
        return 0;
      sleep_forever();
    }
    if (mode == "oom") {
      const std::size_t payload = 8657043456ULL;
      const auto cg = fs::path("/sys/fs/cgroup") / unified().substr(1);
      require(
          read("/sys/fs/cgroup/memory.swap.max").find_first_not_of("0\n\r ") ==
              std::string::npos,
          "enclosing effective zero swap absent");
      write(out / "pre-touch.events", read(cg / "memory.events"));
      write(out / "allocation",
            "payload_max_bytes=" + std::to_string(payload) +
                "\nmonotonic_start_ns=" + std::to_string(stamp()) + "\n");
      void *allocation = mmap(nullptr, payload, PROT_READ | PROT_WRITE,
                              MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
      require(allocation != MAP_FAILED,
              "mmap failed: not an OOM-enforcement pass");
      auto bytes = static_cast<volatile unsigned char *>(allocation);
      std::ofstream progress(out / "page-touch-progress");
      for (std::size_t i = 0; i < payload; i += 4096) {
        bytes[i] = 0xA5;
        if (i % (64ULL * 1024 * 1024) == 0) {
          progress << i << " " << stamp() << "\n";
          progress.flush();
        }
      }
      write(out / "allocation-completed",
            "all payload touched without OOM; qualification failed\n");
      munmap(allocation, payload);
      return 72;
    }
    throw std::runtime_error("unknown fixed guard worker mode");
  } catch (const std::exception &e) {
    std::cerr << e.what() << "\n";
    if (argc >= 3)
      try {
        write(fs::path(argv[2]) / "worker.exception",
              e.what() + std::string("\n"));
      } catch (...) {
      }
    return 73;
  }
}
