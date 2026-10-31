// SPDX-License-Identifier: LGPL-3.0-or-later
#pragma once
#include "supervisor.hpp"
namespace jsr {
struct PanelCellResult {
  std::string status;
  bool healthy;
};
inline std::string completed_status(const SupervisedExit &st) {
  return st.censored
             ? "censored"
             : (WIFEXITED(st.wait_status) && WEXITSTATUS(st.wait_status) == 0
                    ? "complete"
                    : "failed");
}
// Keep launch and terminal outcome outside both operations that can throw.
// In particular, a failed post-run health probe cannot erase completed work.
template <class Launch, class Health>
PanelCellResult run_panel_cell(Launch launch, Health health) {
  bool started = false;
  std::optional<SupervisedExit> result;
  try {
    result = launch(started);
    return {completed_status(*result) + "\tretained supervisor receipt",
            health()};
  } catch (const std::exception &e) {
    const std::string outcome = result ? completed_status(*result)
                                       : (started ? "failed" : "not_started");
    return {outcome + "\t" +
                (result
                     ? "post-run accounting incomplete: "
                     : (started ? "started; supervisor accounting incomplete: "
                                : "before launch: ")) +
                e.what(),
            false};
  }
}
} // namespace jsr
