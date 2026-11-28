// Wake-up probe for the fes serve worker (--serve-bin).  Replays captured requests (REQ=file of back-to-back
// binary requests) against a worker, one at a time, with a controlled idle gap between calls and a choice of how
// this client waits for the reply, and prints the round trip per call.
//   wake_probe WORKER GAP_US GAP_MODE(spin|sleep) REPLY_MODE(block|spin) [N]
// Output (one line): n, mean/p50/p90 round trip in us, and a checksum of every reply byte (identical across
// modes when the worker answers identically).  The worker is started as `WORKER --serve-bin`; its environment
// (FES_SERVE_SPIN_US, FES_SERVE_TIMING) is inherited.  Timing probe only.
#include <algorithm>
#include <chrono>
#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <vector>
#include <errno.h>
#include <fcntl.h>
#include <sys/prctl.h>
#include <sys/wait.h>
#include <time.h>
#include <unistd.h>
using clk = std::chrono::steady_clock;
static bool rd(int fd, void *b, size_t n, bool spin) {
  char *p = (char *)b;
  while (n) {
    ssize_t r = read(fd, p, n);
    if (r > 0) { p += r; n -= (size_t)r; continue; }
    if (r == 0) return false;
    if (errno == EAGAIN && spin) continue;
    if (errno == EINTR) continue;
    return false;
  }
  return true;
}
int main(int argc, char **argv) {
  if (argc < 5 || !getenv("REQ")) { fprintf(stderr, "usage: REQ=file %s WORKER GAP_US spin|sleep block|spin [N]\n", argv[0]); return 1; }
  const char *worker = argv[1];
  const int gap = atoi(argv[2]);
  const bool gap_spin = !strcmp(argv[3], "spin");
  const bool reply_spin = !strcmp(argv[4], "spin");
  const int N = argc > 5 ? atoi(argv[5]) : 1500;
  prctl(PR_SET_TIMERSLACK, 1);
  FILE *f = fopen(getenv("REQ"), "rb");
  if (!f) { perror("REQ"); return 1; }
  std::vector<std::vector<char>> reqs;
  for (;;) {
    uint32_t h[4];
    if (fread(h, 16, 1, f) != 1) break;
    size_t n = h[1], t = n * (n + 1) / 2;
    std::vector<char> r(16 + 8 + 8 * n + 8 * t);
    memcpy(r.data(), h, 16);
    if (fread(r.data() + 16, r.size() - 16, 1, f) != 1) break;
    reqs.push_back(std::move(r));
    if ((int)reqs.size() >= N) break;
  }
  if (reqs.empty()) { fprintf(stderr, "no requests in %s\n", getenv("REQ")); return 1; }
  int in[2], out[2];
  if (pipe(in) || pipe(out)) return 1;
  pid_t pid = fork();
  if (!pid) {
    dup2(in[0], 0); dup2(out[1], 1); close(in[1]); close(out[0]);
    execl(worker, worker, "--serve-bin", (char *)0);
    _exit(127);
  }
  close(in[0]); close(out[1]);
  if (reply_spin) fcntl(out[0], F_SETFL, fcntl(out[0], F_GETFL) | O_NONBLOCK);
  std::vector<double> rt;
  rt.reserve(reqs.size());
  uint64_t sum = 1469598103934665603ULL;  // FNV-1a over all reply bytes
  auto mix = [&](const void *b, size_t n) { const unsigned char *p = (const unsigned char *)b; for (size_t i = 0; i < n; ++i) { sum ^= p[i]; sum *= 1099511628211ULL; } };
  for (size_t i = 0; i < reqs.size(); ++i) {
    if (i && gap > 0) {
      if (gap_spin) { auto t0 = clk::now(); while (std::chrono::duration_cast<std::chrono::nanoseconds>(clk::now() - t0).count() < gap * 1000LL) {} }
      else { timespec ts{gap / 1000000, (gap % 1000000) * 1000L}; nanosleep(&ts, 0); }
    }
    auto t0 = clk::now();
    if (write(in[1], reqs[i].data(), reqs[i].size()) != (ssize_t)reqs[i].size()) return 2;
    uint32_t h[2]; uint64_t c[2];
    if (!rd(out[0], h, 8, reply_spin) || !rd(out[0], c, 16, reply_spin)) return 3;
    std::vector<uint64_t> sols(c[0]);
    if (c[0] && !rd(out[0], sols.data(), 8 * c[0], reply_spin)) return 4;
    auto t1 = clk::now();
    rt.push_back(std::chrono::duration_cast<std::chrono::nanoseconds>(t1 - t0).count() / 1e3);
    mix(h, 8); mix(c, 16); mix(sols.data(), 8 * sols.size());
  }
  close(in[1]);
  waitpid(pid, 0, 0);  // worker prints its own timing at EOF
  std::vector<double> s = rt;
  std::sort(s.begin(), s.end());
  double m = 0;
  for (double x : rt) m += x;
  m /= rt.size();
  printf("n=%zu mean=%.2f p50=%.2f p90=%.2f sum=%016llx\n", rt.size(), m, s[s.size() / 2], s[s.size() * 9 / 10], (unsigned long long)sum);
}
