// SPDX-License-Identifier: LGPL-3.0-or-later
#include "admission.hpp"
#include "control.hpp"
#include "pipeline.hpp"
#include "panel_status.hpp"
#include "supervisor.hpp"
#include <iostream>
#include <sys/prctl.h>
using namespace jsr;
int main(int argc, char **argv) {
  try {
    if (argc == 2 && std::string(argv[1]) == "--status") {
      std::cout << "review_required; no approval envelope supplied; scientific "
                   "calls=0\n";
      return 0;
    }
    require(argc >= 3, "usage: native-smallroot --approved-panel admission | "
                       "--approved-worker admission case-id output | "
                       "--approved-control admission output");
    Admission admission(argv[2]);
    std::string mode = argv[1];
    if (mode == "--approved-worker" || mode == "--approved-control") {
      require(prctl(PR_SET_PDEATHSIG, SIGKILL) == 0,
              "parent death guard unavailable");
      worker_guard(admission);
      require(argc >= 4, "worker output absent");
      fs::path cdd_archive = fs::path(argv[argc - 1]).parent_path() /
                             (std::string(argv[argc - 2]) + "-cdd");
      if (mode == "--approved-control")
        cdd_archive = fs::path(argv[3]).parent_path() / "control-cdd";
      configure_cdd(admission.get("cdd_helper_binary"), cdd_archive);
      if (mode == "--approved-control") {
        require(argc == 4, "control args");
        control_panel(argv[3], admission.get("singular_binary"));
        return 0;
      }
      require(argc == 5, "worker args");
      for (auto &c : panel())
        if (c.id == argv[3]) {
          pipeline(c, argv[4], admission.get("singular_binary"),
                   [&](Archive &a) {
                     a.object("admission.snapshot", read(argv[2]));
                     for (auto key : {"dependency_lock", "source_manifest",
                                      "protocol", "runbook", "trialplan"})
                       a.object(std::string(key) + ".snapshot",
                                read(admission.get(key)));
                   });
          return 0;
        }
      throw std::runtime_error("unknown frozen case");
    }
    require(mode == "--approved-panel" && argc == 3,
            "unrecognized native mode");
    fs::path output = admission.get("output_directory");
    require(fs::create_directory(output), "fresh panel output required");
    write(output / "admission.snapshot", read(argv[2]));
    auto cells = panel();
    std::map<std::string, std::string> statuses;
    for (auto &c : cells)
      statuses[c.id] = "not_started\treview/control admission pending";
    auto save_statuses = [&]() {
      std::string all;
      for (auto &c : cells)
        all += c.id + "\t" + statuses.at(c.id) + "\n";
      write(output / "statuses", all);
    };
    save_statuses();
    auto delegated = fs::path(admission.get("delegated_cgroup"));
    fs::create_directory(output / "control-supervisor");
    auto control = supervised(
        delegated, output / "control-supervisor",
        admission.get("worker_binary"),
        {"--approved-control", argv[2], (output / "control").string()});
    require(!control.censored && WIFEXITED(control.wait_status) &&
                WEXITSTATUS(control.wait_status) == 0,
            "exact control failed/incomplete: all24 cells remain not_started");
    bool healthy = true;
    for (auto &c : cells) {
      if (!healthy) {
        statuses[c.id] = "not_started\tguard unhealthy";
        save_statuses();
        continue;
      }
      auto receipt = output / (c.id + "-supervisor");
      fs::create_directory(receipt);
      auto result = run_panel_cell(
          [&](bool &started) {
            return supervised(
                delegated, receipt, admission.get("worker_binary"),
                {"--approved-worker", argv[2], c.id, (output / c.id).string()},
                &started);
          },
          [&]() {
            return fs::exists(delegated / "cgroup.controllers") &&
                   available_memory() >= 12ULL * 1024 * 1024 * 1024;
          });
      statuses[c.id] = result.status;
      healthy = result.healthy;
      save_statuses();
    }
    return healthy ? 0 : 1;
  } catch (const std::exception &e) {
    std::cerr << e.what() << "\n";
    return 78;
  }
}
