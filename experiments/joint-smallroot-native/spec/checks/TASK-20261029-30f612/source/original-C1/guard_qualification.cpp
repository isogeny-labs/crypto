// SPDX-License-Identifier: LGPL-3.0-or-later
// Direct calls to unchanged production supervised(); fixed nonresearch classes
// only.
#include "supervisor.hpp"
#include <iostream>
#include <openssl/evp.h>
#include <sys/stat.h>
#include <array>
#include <iomanip>
using namespace jsr;
using Clock = std::chrono::steady_clock;
long long stamp() {
  return std::chrono::duration_cast<std::chrono::nanoseconds>(
             Clock::now().time_since_epoch())
      .count();
}
std::string trim(std::string s) {
  auto a = s.find_first_not_of(" \r\n\t");
  if (a == std::string::npos)
    return {};
  return s.substr(a, s.find_last_not_of(" \r\n\t") - a + 1);
}
std::optional<unsigned long long> limit(const std::string &s) {
  auto v = trim(s);
  if (v == "max")
    return std::nullopt;
  size_t end = 0;
  auto n = std::stoull(v, &end);
  require(end == v.size(), "invalid counter");
  return n;
}
std::map<std::string, std::string> fields(const std::string &s) {
  std::istringstream in(s);
  std::string line;
  std::map<std::string, std::string> m;
  while (std::getline(in, line)) {
    auto at = line.find('=');
    if (at != std::string::npos)
      m[line.substr(0, at)] = line.substr(at + 1);
  }
  return m;
}
std::map<std::string, unsigned long long> counters(const std::string &s) {
  std::istringstream in(s);
  std::string k;
  unsigned long long v;
  std::map<std::string, unsigned long long> m;
  while (in >> k >> v)
    m[k] = v;
  return m;
}
void copy_record(const fs::path &p, const fs::path &out) {
  try {
    write(out, read(p));
  } catch (const std::exception &e) {
    write(out, e.what() + std::string("\n"));
  }
}
void group_record(const fs::path &cg, const fs::path &out) {
  fs::create_directories(out);
  write(out / "mapping", cg.string() + "\n");
  for (auto key :
       {"memory.max", "memory.current", "memory.swap.max",
        "memory.swap.current", "memory.events", "memory.peak", "memory.stat",
        "cpu.max", "cpu.stat", "pids.max", "pids.current", "cgroup.procs",
        "cgroup.events", "cgroup.controllers", "cgroup.subtree_control"})
    copy_record(cg / key, out / key);
}
std::string identity() {
  std::string s = "pid=" + std::to_string(getpid()) +
                  "\nuid=" + std::to_string(getuid()) +
                  "\ngid=" + std::to_string(getgid()) +
                  "\nmonotonic_ns=" + std::to_string(stamp()) +
                  "\nproc_stat=" + read("/proc/self/stat") +
                  "cgroup=" + read("/proc/self/cgroup");
  std::istringstream in(read("/proc/self/status"));
  std::string line;
  while (std::getline(in, line))
    if (line.rfind("Cap", 0) == 0 || line.rfind("Seccomp", 0) == 0 ||
        line.rfind("NoNewPrivs", 0) == 0)
      s += line + "\n";
  return s;
}
void bootstrap(const fs::path &out) {
  const fs::path root = "/sys/fs/cgroup";
  group_record(root, out / "container-before");
  require(fs::create_directory(root / "qualifier"),
          "fresh qualifier leaf required");
  write(root / "qualifier/cgroup.procs", std::to_string(getpid()) + "\n");
  write(root / "cgroup.subtree_control", "+memory +cpu +pids\n");
  require(fs::create_directory(root / "delegated"),
          "fresh delegated subtree required");
  write(root / "delegated/cgroup.subtree_control", "+memory +cpu +pids\n");
  group_record(root, out / "container-after-bootstrap");
  write(out / "driver.identity", identity());
  require(trim(read(root / "memory.max")) == "17179869184",
          "parent is not16GiB");
  require(trim(read(root / "memory.swap.max")) == "0",
          "parent effective swap0 absent");
}
unsigned long long headroom(const fs::path &out) {
  unsigned long long H = available_memory();
  std::string detail =
      "host_MemAvailable=" + std::to_string(H) +
      "\nworker8GiB_excluded=true\nhidden_ancestors=unobserved\n";
  unsigned index = 0;
  auto include = [&](const fs::path &p, const std::string &name) {
    auto mx = limit(read(p / "memory.max"));
    auto current = limit(read(p / "memory.current"));
    require(bool(current), "unbounded current invalid");
    detail += name + " memory.max=" + (mx ? std::to_string(*mx) : "max") +
              " memory.current=" + std::to_string(*current) + "\n";
    if (mx)
      H = std::min(H, *current >= *mx ? 0ULL : *mx - *current);
  };
  include("/sys/fs/cgroup", "container_root");
  include("/sys/fs/cgroup/delegated", "delegated_enclosing");
  const fs::path outer = "/records/outer";
  require(fs::exists(outer / "0/memory.max"), "visible outer record absent");
  while (fs::exists(outer / std::to_string(index) / "memory.max")) {
    include(outer / std::to_string(index),
            "outer_visible_" + std::to_string(index));
    ++index;
  }
  detail += "H_bytes=" + std::to_string(H) + "\nrequired_H_bytes=12884901888\n";
  write(out / "headroom-admission", detail);
  return H;
}
std::string state(const std::string &stat) {
  auto end = stat.rfind(')');
  require(end != std::string::npos, "proc stat codec");
  return stat.substr(end + 2, 1);
}
std::string starttime(const std::string &stat) {
  auto end = stat.rfind(')');
  require(end != std::string::npos, "proc stat codec");
  std::istringstream in(stat.substr(end + 2));
  std::string v;
  for (int field = 3; field <= 22; ++field)
    require(bool(in >> v), "proc stat fields");
  return v;
}
bool cleanup(const fs::path &out, const fs::path &group) {
  auto begin = Clock::now();
  bool alive = false;
  bool populated = false;
  std::string records;
  do {
    alive = false;
    for (auto name : {"primary", "child", "grandchild"}) {
      auto f = out / "worker" / (std::string(name) + ".identity");
      if (!fs::exists(f))
        continue;
      auto id = fields(read(f));
      auto pid = std::stol(id.at("pid"));
      auto proc = fs::path("/proc") / std::to_string(pid) / "stat";
      if (!fs::exists(proc)) {
        records += std::string(name) + " absent\n";
        continue;
      }
      auto now = read(proc), expected = id.at("proc_stat");
      bool same = starttime(now) == starttime(expected);
      auto st = state(now);
      records += std::string(name) + " pid=" + std::to_string(pid) +
                 " same_starttime=" + (same ? "true" : "false") +
                 " state=" + st + "\n";
      if (same && st != "Z" && st != "X")
        alive = true;
    }
    populated = false;
    if (fs::exists(group / "cgroup.events")) {
      auto e = counters(read(group / "cgroup.events"));
      populated = e["populated"] != 0;
      records += "populated=" + std::to_string(e["populated"]) + "\n";
    } else
      records += "group_removed=true\n";
    if (!alive && !populated)
      break;
    std::this_thread::sleep_for(std::chrono::milliseconds(100));
  } while (Clock::now() - begin < std::chrono::seconds(10));
  write(out / "cleanup-observations", records);
  int status;
  while (waitpid(-1, &status, WNOHANG) > 0) {
  }
  if (fs::exists(group)) {
    std::error_code ec;
    fs::remove(group, ec);
    write(out / "owned-group-remove", ec ? ec.message() : "removed");
  }
  return !alive && !populated;
}
void await(const fs::path &p) {
  auto begin = Clock::now();
  while (!fs::exists(p)) {
    require(Clock::now() - begin < std::chrono::seconds(15),
            "qualification handshake deadline");
    std::this_thread::sleep_for(std::chrono::milliseconds(10));
  }
}
std::string fd_sha256(int fd) {
  EVP_MD_CTX *ctx = EVP_MD_CTX_new();
  require(ctx != nullptr, "digest context");
  require(EVP_DigestInit_ex(ctx, EVP_sha256(), nullptr) == 1, "digest init");
  std::array<unsigned char, 1048576> buffer{};
  ssize_t got;
  while ((got = ::read(fd, buffer.data(), buffer.size())) > 0)
    require(EVP_DigestUpdate(ctx, buffer.data(), static_cast<size_t>(got)) == 1,
            "digest update");
  require(got == 0, "digest read");
  unsigned char digest[EVP_MAX_MD_SIZE];
  unsigned count = 0;
  require(EVP_DigestFinal_ex(ctx, digest, &count) == 1, "digest final");
  EVP_MD_CTX_free(ctx);
  std::ostringstream out;
  out << std::hex << std::setfill('0');
  for (unsigned i = 0; i < count; ++i)
    out << std::setw(2) << static_cast<unsigned>(digest[i]);
  return out.str();
}
std::string stat_identity(const struct stat &s) {
  return std::to_string(s.st_uid) + "\t" + std::to_string(s.st_dev) + "\t" +
         std::to_string(s.st_ino) + "\t" + std::to_string(s.st_size) + "\t" +
         std::to_string(s.st_mtim.tv_sec) + "." +
         std::to_string(s.st_mtim.tv_nsec);
}
int cache_advice(const fs::path &list, const fs::path &out) {
  require(fs::create_directory(out), "fresh advice output; no second pass");
  write(out / "started", std::to_string(stamp()) + "\n");
  std::istringstream in(read(list));
  std::string line;
  std::ofstream records(out / "advice.tsv");
  unsigned failures = 0, selected = 0;
  while (std::getline(in, line)) {
    if (line.empty())
      continue;
    auto tab = line.find('\t');
    require(tab != std::string::npos, "allowlist line");
    auto pathname = line.substr(0, tab);
    std::istringstream fields(line.substr(tab + 1));
    unsigned long long uid, device, inode, size;
    long long sec, nsec;
    std::string expected;
    require(bool(fields >> uid >> device >> inode >> size >> sec >> nsec >>
                 expected),
            "allowlist metadata");
    ++selected;
    int fd =
        open(pathname.c_str(), O_RDONLY | O_CLOEXEC | O_NOFOLLOW | O_NOATIME);
    if (fd < 0) {
      records << pathname << "\topen_errno=" << errno << "\n";
      ++failures;
      continue;
    }
    try {
      struct stat before{}, after{};
      require(fstat(fd, &before) == 0 && S_ISREG(before.st_mode),
              "stable regular file required");
      require(before.st_uid == uid && before.st_dev == device &&
                  before.st_ino == inode &&
                  static_cast<unsigned long long>(before.st_size) == size &&
                  before.st_mtim.tv_sec == sec &&
                  before.st_mtim.tv_nsec == nsec,
              "allowlist fstat mismatch");
      auto hash = fd_sha256(fd);
      require(hash == expected, "allowlist SHA mismatch");
      require(fstat(fd, &after) == 0 &&
                  stat_identity(before) == stat_identity(after),
              "metadata changed while hashing");
      int rc = posix_fadvise(fd, 0, 0, POSIX_FADV_DONTNEED);
      struct stat final_stat{};
      bool stable = fstat(fd, &final_stat) == 0 &&
                    stat_identity(before) == stat_identity(final_stat);
      records << pathname << "\tsha256=" << hash
              << "\tbefore=" << stat_identity(before) << "\tfadvise_rc=" << rc
              << "\tstable_after=" << (stable ? "true" : "false") << "\n";
      if (rc || !stable)
        ++failures;
    } catch (const std::exception &e) {
      records << pathname << "\trejected=" << e.what() << "\n";
      ++failures;
    }
    close(fd);
    records.flush();
  }
  write(out / "result", "selected=" + std::to_string(selected) +
                            "\nfailures=" + std::to_string(failures) +
                            "\nadvice_is_not_reclamation_proof=true\n");
  return failures ? 1 : 0;
}
void readable_records() noexcept {
  try {
    for (auto &entry : fs::recursive_directory_iterator("/records")) {
      if (entry.is_symlink())
        continue;
      std::error_code ec;
      fs::permissions(entry.path(),
                      entry.is_directory()
                          ? fs::perms::owner_all | fs::perms::group_read |
                                fs::perms::group_exec | fs::perms::others_read |
                                fs::perms::others_exec
                          : fs::perms::owner_read | fs::perms::owner_write |
                                fs::perms::group_read | fs::perms::others_read,
                      fs::perm_options::replace, ec);
      (void)chown(entry.path().c_str(), 1000, 1000);
    }
  } catch (...) {
  }
}
struct RecordAccess {
  ~RecordAccess() { readable_records(); }
};
int main(int argc, char **argv) {
  RecordAccess record_access;
  fs::path out;
  try {
    require(
        argc == 4,
        "guard-qualification Q1|Q2|Q3|Q4|Q5-missing|Q5-readonly worker output");
    const std::string which = argv[1];
    if (which == "--cache-advice")
      return cache_advice(argv[2], argv[3]);
    const fs::path worker = argv[2];
    out = argv[3];
    require(fs::create_directory(out), "fresh qualification output required");
    write(out / "class-start", "class=" + which +
                                   "\nmonotonic_ns=" + std::to_string(stamp()) +
                                   "\nscience_calls=0\n");
    write(out / "driver-before.identity", identity());
    const bool readonly = which == "Q5-readonly";
    if (!readonly)
      bootstrap(out);
    else {
      group_record("/sys/fs/cgroup", out / "readonly-root");
      write(out / "access-W_OK",
            std::to_string(access("/sys/fs/cgroup", W_OK)) + "\n");
    }
    if (which == "Q4" && headroom(out) < 12884901888ULL) {
      write(out / "result", "infrastructure_not_started\nmeasured enclosing "
                            "headroom below12GiB; no worker/OOM attempted\n");
      return 0;
    }
    fs::create_directory(out / "primary");
    fs::create_directory(out / "worker");
    auto delegated = readonly ? fs::path("/sys/fs/cgroup")
                              : fs::path("/sys/fs/cgroup/delegated");
    auto group = delegated / ("joint-smallroot-" + std::to_string(getpid()));
    pid_t competitor = -1;
    if (which == "Q3") {
      fs::create_directory(out / "competing");
      competitor = fork();
      require(competitor >= 0, "competitor fork");
      if (competitor == 0) {
        try {
          await(out / "worker/tree.ready");
          write(out / "competitor.identity", identity());
          fs::create_directory(out / "competing-worker");
          auto r =
              supervised(delegated, out / "competing", worker,
                         {"ordinary", (out / "competing-worker").string()});
          write(out / "serial-result",
                "failed: competing supervisor launched worker; status=" +
                    std::to_string(r.wait_status) + "\n");
          _exit(1);
        } catch (const std::exception &e) {
          write(out / "serial.exception", e.what() + std::string("\n"));
          bool expected = std::string(e.what()).find(
                              "sequential supervisor lock unavailable") !=
                          std::string::npos;
          write(out / "serial-result",
                expected ? "expected_rejection\n" : "failed_other_rejection\n");
          _exit(expected ? 0 : 1);
        }
      }
    }
    const auto begin = Clock::now();
    write(out / "supervisor-call-start", std::to_string(stamp()) + "\n");
    std::string mode = which == "Q2"   ? "tree-return"
                       : which == "Q3" ? "tree-sleep"
                       : which == "Q4" ? "oom"
                                       : "ordinary";
    auto executable = which == "Q5-missing"
                          ? fs::path("/nonexistent/task6374dd-worker")
                          : worker;
    try {
      auto r = supervised(delegated, out / "primary", executable,
                          {mode, (out / "worker").string()});
      auto elapsed = std::chrono::duration_cast<std::chrono::nanoseconds>(
                         Clock::now() - begin)
                         .count();
      write(out / "independent-elapsed-ns", std::to_string(elapsed) + "\n");
      write(out / "supervisor-return",
            "wait_status=" + std::to_string(r.wait_status) +
                "\ncensored=" + (r.censored ? "true" : "false") +
                "\nproduction_wall_ns=" + std::to_string(r.wall_ns) + "\n");
      bool clean = cleanup(out, group);
      if (which == "Q2" || which == "Q3") {
        auto primary_id = fields(read(out / "worker/primary.identity"));
        for (auto role : {"child", "grandchild"}) {
          auto id =
              fields(read(out / "worker" / (std::string(role) + ".identity")));
          require(id.at("sid") == id.at("pid") &&
                      id.at("sid") != primary_id.at("sid") &&
                      id.at("cgroup") == primary_id.at("cgroup"),
                  "session escape or cgroup identity mismatch");
        }
      }
      bool pass = false;
      if (which == "Q1" || which == "Q2")
        pass = WIFEXITED(r.wait_status) && WEXITSTATUS(r.wait_status) == 0 &&
               !r.censored && clean && fs::exists(out / "worker/primary.ready");
      else if (which == "Q3") {
        auto waited = wait_child_usage(competitor);
        write(out / "competitor.wait4", wait_usage_fields(waited));
        pass = r.censored && elapsed >= 1800000000000LL && clean &&
               waited.reaped && WIFEXITED(waited.status) &&
               WEXITSTATUS(waited.status) == 0 &&
               !fs::exists(out / "competing-worker/primary.ready");
        int lockfd = open((out / ".joint-smallroot.lock").c_str(), O_RDWR);
        bool released = lockfd >= 0 && flock(lockfd, LOCK_EX | LOCK_NB) == 0;
        write(out / "lock-after", released ? "released\n" : "not_released\n");
        if (lockfd >= 0) {
          flock(lockfd, LOCK_UN);
          close(lockfd);
        }
        pass = pass && released;
      } else if (which == "Q4") {
        auto before = counters(read(out / "worker/pre-touch.events")),
             after = counters(read(out / "primary/memory.events"));
        require(after["oom_kill"] >= before["oom_kill"],
                "OOM counter regression");
        auto delta = after["oom_kill"] - before["oom_kill"];
        write(out / "oom-kill-delta", std::to_string(delta) + "\n");
        pass = delta > 0 && WIFSIGNALED(r.wait_status) &&
               WTERMSIG(r.wait_status) == SIGKILL && !r.censored && clean;
      } else if (which == "Q5-missing")
        pass = WIFEXITED(r.wait_status) && WEXITSTATUS(r.wait_status) == 127 &&
               clean && fs::exists(out / "primary/supervisor-cpu-receipt");
      write(out / "driver-after.identity", identity());
      write(out / "result", pass ? "passed\n" : "failed\n");
      return pass ? 0 : 1;
    } catch (const std::exception &e) {
      write(out / "supervisor.exception", e.what() + std::string("\n"));
      if (readonly) {
        std::error_code ec;
        fs::create_directory(group, ec);
        write(out / "actual-mkdir-probe",
              "errno=" + std::to_string(ec.value()) +
                  "\nmessage=" + ec.message() + "\n");
        bool expected = ec == std::errc::read_only_file_system &&
                        std::string(e.what()).find("Read-only file system") !=
                            std::string::npos;
        write(out / "result", expected ? "passed\n" : "failed\n");
        return expected ? 0 : 1;
      }
      throw;
    }
  } catch (const std::exception &e) {
    std::cerr << e.what() << "\n";
    if (!out.empty())
      try {
        write(out / "driver.exception", e.what() + std::string("\n"));
        write(out / "result", "failed\n");
      } catch (...) {
      }
    return 1;
  }
}
