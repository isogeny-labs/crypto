// SPDX-License-Identifier: LGPL-3.0-or-later
#include "panel_status.hpp"
#include <iostream>
using namespace jsr;
int main() {
  try {
    unsigned checked = 0;
    for (bool launched : {false, true}) {
      bool checked_health = false;
      auto result = run_panel_cell(
          [&](bool &started) -> SupervisedExit {
            started = launched;
            throw std::runtime_error("injected supervisor failure");
          },
          [&]() {
            checked_health = true;
            return true;
          });
      require(!result.healthy && !checked_health,
              "failure must stop the panel");
      require(result.status.rfind(launched ? "failed\tstarted;"
                                           : "not_started\tbefore launch:",
                                  0) == 0,
              "launch outcome lost after supervisor exception");
      ++checked;
    }
    for (auto st :
         {SupervisedExit{0, false, 1}, SupervisedExit{17 << 8, false, 1},
          SupervisedExit{SIGKILL, true, 1}}) {
      for (bool fail_health : {false, true}) {
        auto result = run_panel_cell(
            [&](bool &started) {
              started = true;
              return st;
            },
            [&]() {
              if (fail_health)
                throw std::runtime_error("injected MemAvailable failure");
              return true;
            });
        require(result.healthy == !fail_health, "health result lost");
        require(result.status.rfind(completed_status(st) + "\t", 0) == 0,
                "terminal worker outcome overwritten by health failure");
        if (fail_health)
          require(result.status.find("post-run accounting incomplete") !=
                      std::string::npos,
                  "post-run failure not disclosed");
        ++checked;
      }
    }
    std::cout << "PASS " << checked << " panel exception/outcome fixtures\n";
    return 0;
  } catch (const std::exception &e) {
    std::cerr << e.what() << '\n';
    return 1;
  }
}
