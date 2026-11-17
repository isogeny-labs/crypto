// SPDX-License-Identifier: LGPL-3.0-or-later
#pragma once
#include <cerrno>
#include <cstdint>
#include <iomanip>
#include <limits>
#include <optional>
#include <sstream>
#include <string>
#include <sys/resource.h>
#include <sys/wait.h>
#include <type_traits>
namespace jsr {
inline constexpr const char *cpu_semantics = "native-smallroot-cpu-v2";
struct CpuMicroseconds {
  std::optional<std::int64_t> value;
  std::string reason;
};
inline CpuMicroseconds checked_timeval_us(std::int64_t sec, std::int64_t usec) {
  if (sec < 0)
    return {std::nullopt, "negative_seconds"};
  if (usec < 0 || usec >= 1000000)
    return {std::nullopt, "invalid_microseconds"};
  constexpr auto maximum = std::numeric_limits<std::int64_t>::max();
  if (sec > (maximum - usec) / 1000000)
    return {std::nullopt, "conversion_overflow"};
  return {sec * 1000000 + usec, {}};
}
inline CpuMicroseconds timeval_us(const timeval &t) {
  static_assert(std::is_signed<decltype(t.tv_sec)>::value &&
                    sizeof(t.tv_sec) <= sizeof(std::int64_t),
                "unsupported timeval seconds representation");
  static_assert(std::is_signed<decltype(t.tv_usec)>::value &&
                    sizeof(t.tv_usec) <= sizeof(std::int64_t),
                "unsupported timeval microseconds representation");
  return checked_timeval_us(static_cast<std::int64_t>(t.tv_sec),
                            static_cast<std::int64_t>(t.tv_usec));
}
inline std::string seconds_six_digits(const CpuMicroseconds &v) {
  if (!v.value)
    return "unavailable";
  std::ostringstream out;
  out << *v.value / 1000000 << '.' << std::setfill('0') << std::setw(6)
      << *v.value % 1000000;
  return out.str();
}
struct CpuUsage {
  CpuMicroseconds user, system;
};
inline CpuUsage unavailable_usage(const std::string &reason) {
  return {{std::nullopt, reason}, {std::nullopt, reason}};
}
inline CpuUsage cpu_usage_result(int result, int error, const rusage &usage) {
  if (result != 0)
    return unavailable_usage("getrusage_errno_" + std::to_string(error));
  return {timeval_us(usage.ru_utime), timeval_us(usage.ru_stime)};
}
inline CpuUsage read_cpu_usage(decltype(RUSAGE_SELF) who) {
  rusage usage{};
  const int result = getrusage(who, &usage);
  const int error = result == 0 ? 0 : errno;
  return cpu_usage_result(result, error, usage);
}
inline CpuMicroseconds cpu_delta(const CpuMicroseconds &before,
                                 const CpuMicroseconds &after) {
  if (!before.value)
    return {std::nullopt, "start_" + before.reason};
  if (!after.value)
    return {std::nullopt, "stop_" + after.reason};
  if (*after.value < *before.value)
    return {std::nullopt, "counter_regression"};
  return {*after.value - *before.value, {}};
}
inline CpuUsage cpu_delta(const CpuUsage &before, const CpuUsage &after) {
  return {cpu_delta(before.user, after.user),
          cpu_delta(before.system, after.system)};
}
inline std::string cpu_value_fields(const std::string &key,
                                    const CpuMicroseconds &v,
                                    bool seconds = false) {
  std::string s =
      key + "_us=" + (v.value ? std::to_string(*v.value) : "unavailable") +
      "\n";
  s += key + "_status=" + (v.value ? "available" : "unavailable") + "\n";
  if (!v.value)
    s += key + "_reason=" + v.reason + "\n";
  if (seconds)
    s += key + "_seconds=" + seconds_six_digits(v) + "\n";
  return s;
}
inline std::string cpu_usage_fields(const std::string &prefix,
                                    const CpuUsage &v, bool seconds = false) {
  const auto p = prefix.empty() ? "" : prefix + "_";
  return cpu_value_fields(p + "user", v.user, seconds) +
         cpu_value_fields(p + "system", v.system, seconds);
}
struct PhaseCpuSnapshot {
  CpuUsage self, waited_children;
};
inline PhaseCpuSnapshot phase_cpu_snapshot() {
  return {read_cpu_usage(RUSAGE_SELF), read_cpu_usage(RUSAGE_CHILDREN)};
}
inline std::string phase_cpu_fields(const PhaseCpuSnapshot &before,
                                    const PhaseCpuSnapshot &after) {
  return std::string("cpu_semantics=") + cpu_semantics + "\n" +
         cpu_usage_fields("worker_self", cpu_delta(before.self, after.self)) +
         cpu_usage_fields("waited_children", cpu_delta(before.waited_children,
                                                       after.waited_children)) +
         "worker_self_scope=RUSAGE_SELF_all_threads\n"
         "waited_children_scope=RUSAGE_CHILDREN_terminated_and_waited_only_"
         "descendants_only_when_propagated\n"
         "waited_children_interval=usage_credited_on_reap_not_CPU_time_sliced_"
         "to_phase\n"
         "individual_child_receipts=diagnostics_not_additive\nphase_process_"
         "tree_cpu=unknown\n";
}
struct WaitUsage {
  pid_t result = -1;
  int status = 0;
  int error = 0;
  bool reaped = false;
  rusage raw{};
  CpuUsage cpu = unavailable_usage("child_not_reaped");
};
inline WaitUsage reaped_usage(pid_t child, int status, const rusage &raw) {
  return {child, status, 0,
          true,  raw,    {timeval_us(raw.ru_utime), timeval_us(raw.ru_stime)}};
}
inline WaitUsage wait_child_usage(pid_t child) {
  for (;;) {
    rusage usage{};
    int status = 0;
    const pid_t result = wait4(child, &status, 0, &usage);
    if (result == child)
      return reaped_usage(result, status, usage);
    const int error = errno;
    if (result < 0 && error == EINTR)
      continue;
    WaitUsage failed;
    failed.result = result;
    failed.error = error;
    failed.cpu = unavailable_usage("wait4_errno_" + std::to_string(error));
    return failed;
  }
}
inline std::string wait_usage_fields(const WaitUsage &v,
                                     const std::string &prefix = "child") {
  std::string s = std::string("cpu_semantics=") + cpu_semantics +
                  "\nwait_result=" + std::to_string(v.result) +
                  "\nwait_errno=" + std::to_string(v.error) +
                  "\nreaped=" + (v.reaped ? "true" : "false") + "\n";
  s += "wait_status=" + (v.reaped ? std::to_string(v.status) : "unavailable") +
       "\n";
  if (v.reaped) {
    s += "exit_code=" +
         (WIFEXITED(v.status) ? std::to_string(WEXITSTATUS(v.status))
                              : "unavailable") +
         "\n";
    s += "signal=" +
         (WIFSIGNALED(v.status) ? std::to_string(WTERMSIG(v.status))
                                : "unavailable") +
         "\n";
  }
  s += cpu_usage_fields(prefix, v.cpu, true);
  return s + "wait4_cpu_scope=reaped_child_including_waited_descendants_when_"
             "reported\n"
             "individual_child_receipt=diagnostic_not_an_extra_phase_"
             "addend\nfull_process_tree_cpu=unknown\n";
}
inline std::string supervisor_cpu_fields(const rusage &usage) {
  return std::string("cpu_semantics=") + cpu_semantics + "\n" +
         cpu_usage_fields(
             "", {timeval_us(usage.ru_utime), timeval_us(usage.ru_stime)},
             true) +
         "wait4_cpu_scope=reaped_worker_including_waited_descendants_when_"
         "reported\n"
         "full_process_tree_cpu=unknown\n";
}
} // namespace jsr
