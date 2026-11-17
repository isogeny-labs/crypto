// SPDX-License-Identifier: LGPL-3.0-or-later
#include "pipeline.hpp"
#include "supervisor.hpp"
#include <iostream>
#include <limits>
using namespace jsr;
using Fields = std::map<std::string, std::string>;
Fields fields(const std::string &s) {
  Fields values;
  std::istringstream in(s);
  std::string line;
  while (std::getline(in, line)) {
    auto at = line.find('=');
    if (at != std::string::npos)
      values[line.substr(0, at)] = line.substr(at + 1);
  }
  return values;
}
std::int64_t number(const Fields &v, const std::string &key) {
  auto i = v.find(key);
  require(i != v.end(), "missing field " + key);
  size_t end = 0;
  auto n = std::stoll(i->second, &end);
  require(end == i->second.size() && n >= 0,
          "nonnegative integer field " + key);
  return n;
}
void assert_usage(const fs::path &p, int code, int signal) {
  auto v = fields(read(p));
  require(v.at("cpu_semantics") == cpu_semantics && v.at("reaped") == "true",
          "receipt semantics/reaped");
  (void)number(v, "child_user_us");
  (void)number(v, "child_system_us");
  require(v.at("child_user_status") == "available" &&
              v.at("child_system_status") == "available",
          "valid usage not retained");
  if (code >= 0)
    require(number(v, "exit_code") == code, "exit code");
  if (signal >= 0)
    require(number(v, "signal") == signal, "signal code");
  require(v.at("full_process_tree_cpu") == "unknown",
          "incorrect complete tree claim");
  require(v.at("individual_child_receipt") ==
              "diagnostic_not_an_extra_phase_addend",
          "additive child receipt");
}
CpuUsage values(std::int64_t user, std::int64_t system) {
  return {{user, {}}, {system, {}}};
}
int main(int argc, char **argv) {
  try {
    require(argc == 3,
            "usage cpu-accounting fake-child fresh-output-directory");
    const fs::path fake = fs::absolute(argv[1]), output = argv[2];
    require(fs::create_directory(output), "fresh fixture output required");
    std::ofstream results(output / "fixtures");
    unsigned completed = 0;
    auto fixture = [&](const std::string &id, auto run) {
      results << "START " << id << "\n";
      results.flush();
      try {
        run();
        results << "PASS " << id << "\n";
        results.flush();
        ++completed;
      } catch (const std::exception &e) {
        results << "FAIL " << id << " " << e.what() << "\n";
        results.flush();
        throw;
      }
    };
    for (const auto &item :
         std::vector<std::tuple<std::string, std::int64_t, std::int64_t,
                                std::int64_t, std::string>>{
             {"timeval-0-1", 0, 1, 1, "0.000001"},
             {"timeval-0-10", 0, 10, 10, "0.000010"},
             {"timeval-0-999999", 0, 999999, 999999, "0.999999"},
             {"timeval-1-0", 1, 0, 1000000, "1.000000"},
             {"timeval-1-1", 1, 1, 1000001, "1.000001"}}) {
      fixture(std::get<0>(item), [&]() {
        auto got = checked_timeval_us(std::get<1>(item), std::get<2>(item));
        require(got.value && *got.value == std::get<3>(item),
                "checked conversion");
        require(seconds_six_digits(got) == std::get<4>(item),
                "six-digit decimal");
        rusage raw{};
        raw.ru_utime.tv_sec = std::get<1>(item);
        raw.ru_utime.tv_usec = std::get<2>(item);
        auto text = supervisor_cpu_fields(raw);
        write(output / (std::get<0>(item) + ".supervisor-format"), text);
        auto v = fields(text);
        require(number(v, "user_us") == std::get<3>(item) &&
                    v.at("user_seconds") == std::get<4>(item) &&
                    v.at("cpu_semantics") == cpu_semantics,
                "supervisor authoritative/legacy fields");
      });
    }
    fixture("negative-timeval", [&]() {
      require(!checked_timeval_us(-1, 1).value &&
                  !checked_timeval_us(0, -1).value,
              "negative timeval wrapped");
    });
    fixture("invalid-usec-range", [&]() {
      require(!checked_timeval_us(0, 1000000).value &&
                  !checked_timeval_us(1, 1000001).value,
              "invalid usec accepted");
    });
    fixture("conversion-overflow", [&]() {
      constexpr auto maximum = std::numeric_limits<std::int64_t>::max();
      auto top = checked_timeval_us(maximum / 1000000, maximum % 1000000);
      require(top.value && *top.value == maximum, "largest valid conversion");
      auto bad = checked_timeval_us(maximum / 1000000, maximum % 1000000 + 1);
      require(!bad.value && bad.reason == "conversion_overflow" &&
                  !checked_timeval_us(maximum, 0).value,
              "conversion overflow wrapped");
    });
    std::map<std::string, fs::path> aliases;
    for (auto mode : {"exit0", "exit17", "sigterm", "malformed"}) {
      fs::path alias = fake.parent_path() / (std::string("fake-child-") + mode);
      if (fs::exists(alias))
        require(fs::equivalent(alias, fake), "fake alias differs");
      else
        fs::create_hard_link(fake, alias);
      aliases[mode] = alias;
    }
    auto backend = [&](const fs::path &executable, const fs::path &where) {
      Backend b{executable, where, Order{{1000}}, {"p"}, 0, {}};
      return b.call("", "fake-codec-only");
    };
    auto helper = [&](const fs::path &executable, const fs::path &where) {
      configure_cdd(executable, where);
      return cdd_halfspaces({E{0}});
    };
    auto child_fixture = [&](const std::string &id, const std::string &mode,
                             int code, int signal, bool parser_failure) {
      fixture(id, [&]() {
        auto where = output / id;
        fs::create_directory(where);
        auto executable = mode == "missing"
                              ? fake.parent_path() / "does-not-exist"
                              : aliases.at(mode);
        bool backend_failed = false;
        try {
          (void)backend(executable, where / "backend");
        } catch (const std::exception &e) {
          backend_failed = true;
          write(where / "backend.exception", e.what() + std::string("\n"));
        }
        require(backend_failed == (code != 0 || signal >= 0 || parser_failure),
                "backend failure expectation");
        assert_usage(where / "backend/0-fake-codec-only.cpu-receipt", code,
                     signal);
        bool helper_failed = false;
        try {
          const auto data = helper(executable, where / "helper");
          if (parser_failure) {
            require(data == "1 1\n0 0/1 1/1\n",
                    "fixture consumer rejects malformed fake codec");
          }
        } catch (const std::exception &e) {
          helper_failed = true;
          write(where / "helper.exception", e.what() + std::string("\n"));
        }
        require(helper_failed == (code != 0 || signal >= 0 || parser_failure),
                "helper failure expectation");
        assert_usage(where / "helper/0.cpu-receipt", code, signal);
      });
    };
    child_fixture("fake-child-exit0", "exit0", 0, -1, false);
    child_fixture("fake-child-exit17", "exit17", 17, -1, false);
    child_fixture("fake-child-sigterm", "sigterm", -1, SIGTERM, false);
    child_fixture("exec-missing", "missing", 127, -1, false);
    child_fixture("malformed-fake-codec", "malformed", 0, -1, true);
    fixture("phase-success", [&]() {
      auto where = output / "phase-success";
      fs::create_directory(where);
      Archive a{where, Order{{1000}}};
      auto value = a.phase("fake-child-phase", [&]() {
        return backend(aliases.at("exit0"), where / "backend").size();
      });
      require(value == 1, "fake parser result");
      auto text = read(where / "phases");
      auto v = fields(text);
      require(v.at("cpu_semantics") == cpu_semantics, "phase semantics");
      (void)number(v, "worker_self_user_us");
      (void)number(v, "worker_self_system_us");
      (void)number(v, "waited_children_user_us");
      (void)number(v, "waited_children_system_us");
      require(v.at("phase_process_tree_cpu") == "unknown" &&
                  v.at("individual_child_receipts") ==
                      "diagnostics_not_additive",
              "phase scopes");
    });
    fixture("phase-exception", [&]() {
      auto where = output / "phase-exception";
      fs::create_directory(where);
      Archive a{where, Order{{1000}}};
      bool threw = false;
      try {
        a.phase("fake-throw", [&]() -> int {
          (void)backend(aliases.at("exit17"), where / "backend");
          return 0;
        });
      } catch (const std::exception &) {
        threw = true;
      }
      require(threw && read(where / "failure-checkpoint").find("fake-throw") !=
                           std::string::npos,
              "exception checkpoint missing");
      auto v = fields(read(where / "phases"));
      (void)number(v, "worker_self_user_us");
      (void)number(v, "waited_children_system_us");
      require(v.at("phase_process_tree_cpu") == "unknown",
              "failure phase falsely complete");
      assert_usage(where / "backend/0-fake-codec-only.cpu-receipt", 17, -1);
    });
    fixture("counter-read-failure", [&]() {
      rusage stale{};
      stale.ru_utime.tv_sec = 88;
      auto missing = cpu_usage_result(-1, EIO, stale);
      require(!missing.user.value && !missing.system.value,
              "failed read used stale/zero data");
      PhaseCpuSnapshot before{values(1, 2), values(3, 4)},
          after{missing, missing};
      auto text = phase_cpu_fields(before, after);
      write(output / "counter-unavailable", text);
      auto v = fields(text);
      require(v.at("worker_self_user_us") == "unavailable" &&
                  v.at("waited_children_system_us") == "unavailable" &&
                  v.at("phase_process_tree_cpu") == "unknown",
              "unavailable phase mislabeled");
    });
    fixture("no-double-count", [&]() {
      PhaseCpuSnapshot before{values(10, 20), values(100, 50)},
          after{values(18, 24), values(140, 55)};
      auto text = phase_cpu_fields(before, after);
      auto v = fields(text);
      rusage raw{};
      raw.ru_utime.tv_usec = 40;
      raw.ru_stime.tv_usec = 5;
      auto receipt = wait_usage_fields(reaped_usage(42, 0, raw));
      write(output / "synthetic-child-diagnostic", receipt);
      write(output / "synthetic-phase-delta", text);
      require(number(v, "worker_self_user_us") == 8 &&
                  number(v, "waited_children_user_us") == 40 &&
                  number(v, "waited_children_system_us") == 5,
              "diagnostic added again");
      require(v.at("individual_child_receipts") == "diagnostics_not_additive" &&
                  v.at("phase_process_tree_cpu") == "unknown",
              "false full coverage");
    });
    require(completed == 17, "fixture count mismatch");
    results << "COMPLETE17\n";
    std::cout << "17 sealed native CPU software fixtures passed; no CAS, "
                 "mathematical helper, panel or supervisor launch\n";
    return 0;
  } catch (const std::exception &e) {
    std::cerr << e.what() << "\n";
    return 1;
  }
}
