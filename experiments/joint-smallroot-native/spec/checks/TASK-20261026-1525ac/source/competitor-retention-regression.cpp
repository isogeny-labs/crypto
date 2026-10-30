// SPDX-License-Identifier: LGPL-3.0-or-later
// Exactly two sequential synthetic children; no supervisor or resource fixture.
#include "usage.hpp"
#include <fcntl.h>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <stdexcept>
#include <unistd.h>
namespace fs = std::filesystem;
using namespace jsr;
void require(bool value, const char *why) {
  if (!value)
    throw std::runtime_error(why);
}
void retain(const fs::path &path, const std::string &text) {
  const int fd =
      open(path.c_str(), O_WRONLY | O_CREAT | O_EXCL | O_CLOEXEC, 0644);
  require(fd >= 0, "fresh receipt open failed");
  size_t offset = 0;
  while (offset < text.size()) {
    const auto n = write(fd, text.data() + offset, text.size() - offset);
    if (n < 0 && errno == EINTR)
      continue;
    require(n > 0, "receipt write failed");
    offset += static_cast<size_t>(n);
  }
  const int synced = fsync(fd);
  const int closed = close(fd);
  require(synced == 0 && closed == 0, "receipt persistence failed");
}
pid_t terminated_child(const fs::path &out, const std::string &name) {
  const pid_t child = fork();
  require(child >= 0, "fork failed; no retry");
  if (child == 0)
    _exit(23);
  siginfo_t info{};
  int result;
  do {
    result = waitid(P_PID, static_cast<id_t>(child), &info, WEXITED | WNOWAIT);
  } while (result < 0 && errno == EINTR);
  const int error = result < 0 ? errno : 0;
  retain(out / (name + ".waitid"),
         "expected_pid=" + std::to_string(child) + "\nreturn=" +
             std::to_string(result) + "\nerrno=" + std::to_string(error) +
             "\nsi_pid=" + std::to_string(info.si_pid) +
             "\nsi_code=" + std::to_string(info.si_code) +
             "\nsi_status=" + std::to_string(info.si_status) + "\n");
  require(result == 0 && info.si_pid == child && info.si_code == CLD_EXITED &&
              info.si_status == 23,
          "nonreaping termination observation mismatch");
  return child;
}
void generic_reap(const fs::path &out, const std::string &name,
                  pid_t expected) {
  int status = 0;
  pid_t result;
  do {
    result = waitpid(-1, &status, WNOHANG);
  } while (result < 0 && errno == EINTR);
  const int error = result < 0 ? errno : 0;
  retain(out / (name + ".generic-waitpid"),
         "return=" + std::to_string(result) +
             "\nerrno=" + std::to_string(error) +
             "\nraw_status=" + std::to_string(status) + "\n");
  if (expected > 0)
    require(result == expected && WIFEXITED(status) &&
                WEXITSTATUS(status) == 23,
            "generic old-order reap mismatch");
  else
    require(result == -1 && error == ECHILD, "correct-order cleanup mismatch");
}
int main(int argc, char **argv) {
  fs::path out;
  try {
    require(argc == 2, "requires fresh output path");
    out = argv[1];
    require(fs::create_directory(out), "fresh output required; no retry");
    const pid_t old_child = terminated_child(out, "old");
    generic_reap(out, "old", old_child);
    const auto old = wait_child_usage(old_child);
    retain(out / "old.dedicated-wait4", wait_usage_fields(old));
    require(old.result == -1 && old.error == ECHILD && !old.reaped &&
                !old.cpu.user.value && !old.cpu.system.value,
            "old-order unavailable receipt mismatch");
    const pid_t correct_child = terminated_child(out, "correct");
    const auto correct = wait_child_usage(correct_child);
    retain(out / "correct.dedicated-wait4", wait_usage_fields(correct));
    require(correct.result == correct_child && correct.reaped &&
                WIFEXITED(correct.status) &&
                WEXITSTATUS(correct.status) == 23 && correct.cpu.user.value &&
                correct.cpu.system.value,
            "correct-order retained receipt mismatch");
    generic_reap(out, "correct", -1);
    retain(out / "result", "passed\nchild_instances=2\nscience_calls=0\n");
    std::cout << "passed: old ECHILD retained; corrected exact-PID status and "
                 "actual usage retained before cleanup\n";
    return 0;
  } catch (const std::exception &e) {
    std::cerr << e.what() << '\n';
    if (!out.empty() && fs::exists(out)) {
      try {
        retain(out / "failure", e.what() + std::string("\n"));
      } catch (...) {
      }
    }
    return 1;
  }
}
