// SPDX-License-Identifier: LGPL-3.0-or-later
#pragma once
#include "backend.hpp"
#include "model.hpp"
namespace jsr {
// Native envelope is an additional local byte/guard gate, not a replacement for
// programme experiment_preapproval, Director lint, Coordinator approval or
// /run.
struct Admission {
  std::map<std::string, std::string> fields;
  explicit Admission(const fs::path &p) {
    std::istringstream in(read(p));
    std::string line;
    while (std::getline(in, line)) {
      if (line.empty())
        continue;
      auto k = line.find('=');
      require(k != std::string::npos, "admission key=value required");
      require(fields.emplace(line.substr(0, k), line.substr(k + 1)).second,
              "duplicate admission key");
    }
    require(get("schema") == "native-smallroot-admission-v1" &&
                get("status") == "approved",
            "review_required: approved admission absent");
    for (auto key :
         {"approved_by", "approval_decision", "owner_commit",
          "programme_commit", "runbook_lint_receipt", "preapproval_receipt",
          "independent_review_receipt", "control_amendment_approval"}) {
      auto value = get(key);
      require(!value.empty() && value.find("pending") == std::string::npos &&
                  value != "null",
              "unresolved admission field");
    }
    require(get("symbolic_bound_calls") == "24" &&
                get("control_constructions") == "4" &&
                get("automatic_retries") == "0",
            "frozen panel admission mismatch");
    for (auto key : {"source_manifest", "protocol", "runbook", "trialplan",
                     "dependency_lock", "worker_binary", "singular_binary",
                     "cdd_helper_binary"}) {
      auto path = get(key);
      auto digest = get(std::string(key) + "_sha256");
      require(digest.size() == 64 && sha256(read(path)) == digest,
              "admission digest mismatch: " + std::string(key));
    }
    require(get("director_card_status") == "CARD_OK" &&
                get("runbook_lint_status") == "LINT_OK" &&
                get("preapproval_status") == "approvable_true",
            "external gate receipt absent");
  }
  std::string get(const std::string &key) const {
    auto i = fields.find(key);
    require(i != fields.end(), "missing admission field: " + key);
    return i->second;
  }
};
inline void worker_guard(const Admission &a) {
  auto procs = read("/proc/self/cgroup");
  auto token = "joint-smallroot-" + std::to_string(getppid());
  require(procs.find(token) != std::string::npos,
          "worker requires supervisor cgroup");
  auto group = fs::path(a.get("delegated_cgroup")) / token;
  require(exact_memory_limit(read(group / "memory.max")) &&
              fs::exists(group / "cgroup.kill"),
          "worker memory/tree guard absent");
}
} // namespace jsr
