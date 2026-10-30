// SPDX-License-Identifier: LGPL-3.0-or-later
#pragma once
#include "backend.hpp"
#include <chrono>
#include <csignal>
#include <sys/file.h>
#include <sys/resource.h>
#include <thread>
namespace jsr {
struct SupervisedExit {
  int wait_status;
  bool censored;
  long long wall_ns;
};
inline unsigned long long available_memory() {
  std::ifstream in("/proc/meminfo");
  std::string line;
  while (std::getline(in, line))
    if (line.rfind("MemAvailable:", 0) == 0) {
      std::istringstream s(line.substr(13));
      unsigned long long kb = 0;
      s >> kb;
      require(bool(s), "MemAvailable parse");
      return kb * 1024;
    }
  throw std::runtime_error("MemAvailable unavailable");
}
// Prepared process guard; production approval/task-card admission is still
// missing.
inline SupervisedExit supervised(const fs::path &delegated,
                                 const fs::path &archive,
                                 const fs::path &binary,
                                 const std::vector<std::string> &args,
                                 bool *started = nullptr) {
  if (started)
    *started = false;
  require(available_memory() >= 12ULL * 1024 * 1024 * 1024,
          "infrastructure-not-started: less than12GiB available");
  for (auto name : {"meminfo", "cpuinfo", "loadavg"})
    write(archive / (std::string("host-") + name),
          read(fs::path("/proc") / name));
  require(fs::exists(delegated / "cgroup.controllers"),
          "infrastructure-not-started: no cgroup-v2 delegation");
  int lockfd = open((archive.parent_path() / ".joint-smallroot.lock").c_str(),
                    O_CREAT | O_RDWR, 0600);
  require(lockfd >= 0 && flock(lockfd, LOCK_EX | LOCK_NB) == 0,
          "sequential supervisor lock unavailable");
  struct Lock {
    int f;
    ~Lock() {
      flock(f, LOCK_UN);
      close(f);
    }
  } lock{lockfd};
  fs::path group = delegated / ("joint-smallroot-" + std::to_string(getpid()));
  require(fs::create_directory(group), "fresh delegated cgroup required");
  bool reaped = false;
  pid_t child = -1;
  std::optional<WaitUsage> worker_usage;
  auto kill_tree = [&]() {
    try {
      write(group / "cgroup.kill", "1\n");
    } catch (...) {
    }
    if (child > 0)
      kill(-child, SIGKILL);
  };
  try {
    write(group / "memory.max", "8589934592\n");
    require(exact_memory_limit(read(group / "memory.max")),
            "memory limit readback");
    require(fs::exists(group / "cgroup.kill") &&
                fs::exists(group / "memory.peak") &&
                fs::exists(group / "memory.events"),
            "required cgroup controls unavailable");
    write(archive / "guard-start",
          "memory.max=8589934592\nwatchdog_seconds=1800\nMemAvailable=" +
              std::to_string(available_memory()) + "\n");
    auto start = std::chrono::steady_clock::now();
    child = fork();
    require(child >= 0, "worker fork failed");
    if (child > 0 && started)
      *started = true;
    if (child == 0) {
      if (setpgid(0, 0) != 0)
        _exit(126);
      raise(SIGSTOP);
      int out = open((archive / "worker.stdout").c_str(),
                     O_CREAT | O_EXCL | O_WRONLY, 0600),
          err = open((archive / "worker.stderr").c_str(),
                     O_CREAT | O_EXCL | O_WRONLY, 0600);
      if (out < 0 || err < 0)
        _exit(126);
      dup2(out, 1);
      dup2(err, 2);
      close(out);
      close(err);
      std::vector<char *> argv{const_cast<char *>(binary.c_str())};
      for (auto &s : args)
        argv.push_back(const_cast<char *>(s.c_str()));
      argv.push_back(nullptr);
      execv(binary.c_str(), argv.data());
      _exit(127);
    }
    int status = 0;
    require(waitpid(child, &status, WUNTRACED) == child && WIFSTOPPED(status),
            "child startup stop failed");
    write(group / "cgroup.procs", std::to_string(child) + "\n");
    require(read(group / "cgroup.procs").find(std::to_string(child)) !=
                std::string::npos,
            "worker cgroup readback failed");
    kill(child, SIGCONT);
    bool censored = false;
    rusage usage{};
    for (;;) {
      pid_t result = wait4(child, &status, WNOHANG, &usage);
      if (result == child) {
        reaped = true;
        worker_usage = reaped_usage(child, status, usage);
        break;
      }
      require(result == 0 || errno == EINTR, "wait4 worker");
      if (std::chrono::steady_clock::now() - start >=
          std::chrono::seconds(1800)) {
        censored = true;
        kill_tree();
        while (wait4(child, &status, 0, &usage) < 0)
          require(errno == EINTR, "wait4 after kill");
        reaped = true;
        worker_usage = reaped_usage(child, status, usage);
        break;
      }
      std::this_thread::sleep_for(std::chrono::milliseconds(100));
    }
    write(archive / "supervisor-cpu-receipt",
          wait_usage_fields(*worker_usage, "worker"));
    kill_tree();
    std::string fields = "wait_status=" + std::to_string(status) +
                         "\ncensored=" + (censored ? "true" : "false") +
                         "\nru_maxrss_kib=" + std::to_string(usage.ru_maxrss) +
                         "\n" + supervisor_cpu_fields(usage);
    for (auto name :
         {"memory.peak", "memory.events", "cpu.stat", "cgroup.events"})
      write(archive / name, read(group / name));
    auto wall = std::chrono::duration_cast<std::chrono::nanoseconds>(
                    std::chrono::steady_clock::now() - start)
                    .count();
    write(archive / "supervisor-receipt",
          fields + "monotonic_wall_ns=" + std::to_string(wall) + "\n");
    std::error_code ec;
    fs::remove(group, ec);
    if (ec)
      write(archive / "cgroup-cleanup", ec.message() + "\n");
    return {status, censored, wall};
  } catch (const std::exception &e) {
    kill_tree();
    if (child > 0 && !reaped) {
      worker_usage = wait_child_usage(child);
      reaped = worker_usage->reaped;
    }
    if (worker_usage)
      write(archive / "supervisor-cpu-receipt",
            wait_usage_fields(*worker_usage, "worker"));
    write(archive / "supervisor-failure", e.what() + std::string("\n"));
    for (auto name :
         {"memory.peak", "memory.events", "cpu.stat", "cgroup.events"})
      try {
        write(archive / name, read(group / name));
      } catch (...) {
        write(archive / (std::string(name) + ".unavailable"),
              "unavailable after supervisor failure\n");
      }
    std::error_code ec;
    fs::remove(group, ec);
    throw;
  }
}
} // namespace jsr
