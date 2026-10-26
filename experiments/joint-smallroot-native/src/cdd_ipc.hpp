// SPDX-License-Identifier: LGPL-3.0-or-later
#pragma once
#include "backend.hpp"
namespace jsr {
struct CddIPC {
  fs::path binary, archive;
  unsigned sequence = 0;
};
inline CddIPC cdd_ipc;
inline void configure_cdd(const fs::path &binary, const fs::path &archive) {
  cdd_ipc = {binary, archive, 0};
  fs::create_directories(archive);
}
inline std::string cdd_halfspaces(const std::vector<E> &vertices) {
  require(!cdd_ipc.binary.empty(), "native cdd helper not configured");
  std::string name = std::to_string(cdd_ipc.sequence++);
  auto input = cdd_ipc.archive / (name + ".vertices"),
       out = cdd_ipc.archive / (name + ".stdout"),
       err = cdd_ipc.archive / (name + ".stderr");
  std::string s = std::to_string(vertices.size()) + " " +
                  std::to_string(vertices.front().size()) + "\n";
  for (auto &v : vertices) {
    for (int x : v) {
      require(x >= 0, "nonnegative vertex required");
      s += std::to_string(x) + " ";
    }
    s += "\n";
  }
  write(input, s);
  pid_t p = fork();
  require(p >= 0, "cdd helper fork");
  if (p == 0) {
    int a = open(out.c_str(), O_CREAT | O_EXCL | O_WRONLY, 0600),
        b = open(err.c_str(), O_CREAT | O_EXCL | O_WRONLY, 0600);
    if (a < 0 || b < 0)
      _exit(126);
    dup2(a, 1);
    dup2(b, 2);
    close(a);
    close(b);
    execl(cdd_ipc.binary.c_str(), cdd_ipc.binary.c_str(), input.c_str(),
          static_cast<char *>(nullptr));
    _exit(127);
  }
  int status = 0;
  while (waitpid(p, &status, 0) < 0)
    require(errno == EINTR, "wait cdd helper");
  write(cdd_ipc.archive / (name + ".wait-status"),
        std::to_string(status) + "\n");
  require(WIFEXITED(status) && WEXITSTATUS(status) == 0, "cdd helper failure");
  return read(out);
}
} // namespace jsr
