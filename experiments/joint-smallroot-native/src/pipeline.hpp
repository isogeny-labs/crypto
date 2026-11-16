// SPDX-License-Identifier: LGPL-3.0-or-later
#pragma once
#include "backend.hpp"
#include "model.hpp"
#include "polytope.hpp"
#include <chrono>
#include <ctime>
namespace jsr {
struct Archive {
  fs::path dir;
  Order order;
  unsigned sequence = 0;
  void object(const std::string &name, const std::string &value) {
    write(dir / name, value);
  }
  void polys(const std::string &name, const std::vector<Poly> &v) {
    std::string s;
    Z max_num = 0, max_den = 1;
    for (size_t i = 0; i < v.size(); ++i)
      s += "POLY=" + std::to_string(i) + "\n" + v[i].codec(order) + "END\n";
    for (auto &f : v)
      for (auto &term : f.t) {
        Z numerator = abs(term.second.get_num());
        max_num = std::max(max_num, numerator);
        max_den = std::max(max_den, term.second.get_den());
      }
    object(name, s);
    object(name + ".coefficient-heights",
           "max_abs_numerator=" + max_num.get_str() +
               "\nmax_denominator=" + max_den.get_str() + "\n");
  }
  void exponents(const std::string &name, const std::vector<E> &v) {
    std::string s;
    for (auto &e : v)
      s += estr(e) + "\n";
    object(name, s);
  }
  void domain(const Domain &d) {
    std::string prefix = "polytope-" + std::to_string(sequence++);
    exponents(prefix + ".vertices", d.vertices);
    exponents(prefix + ".points", d.points);
    std::string s = "affine_dimension=" + std::to_string(d.dimension) + "\n";
    for (auto &f : d.facets) {
      s += f.equality ? "EQ:" : "GE:";
      for (auto &q : f.a)
        s += qstr(q) + ",";
      s += "\n";
    }
    object(prefix + ".facets", s);
  }
  void matrix(const std::string &name, const Matrix &B) {
    std::string s;
    for (size_t i = 0; i < B.size(); ++i)
      for (size_t j = 0; j < B[i].size(); ++j)
        s += "entry=" + std::to_string(i) + "," + std::to_string(j) + "\n" +
             B[i][j].codec(order) + "END\n";
    object(name, s);
  }
  void closure(const Closure &c) {
    exponents("M", c.M);
    exponents("M_bar", c.Mbar);
    polys("S_bar", c.Sbar);
    polys("S", c.S);
    matrix("B-before", c.before);
    matrix("B-after", c.after);
    std::string s;
    for (auto &x : c.trace)
      s += x + "\n";
    object("row-operations", s);
  }
  void augmented(int k, const Aug &a) {
    std::string p = "aug-" + std::to_string(k);
    exponents(p + ".points", a.points);
    std::string s;
    for (auto &x : a.values)
      s += estr(x.first) + ":" + std::to_string(x.second) + "\n";
    object(p + ".valuations", s);
  }
  void univariate(const std::string &name, const U &u) {
    std::string s;
    for (size_t i = 0; i < u.size(); ++i)
      s += std::to_string(i) + ":" + qstr(u[i]) + "\n";
    object(name, s);
  }
  template <typename F>
  auto phase(const std::string &name, F f) -> decltype(f()) {
    const auto start = std::chrono::steady_clock::now();
    const auto cpu = std::clock();
    const auto usage_start = phase_cpu_snapshot();
    std::ofstream log(dir / "phases", std::ios::app);
    log << name << " start monotonic_ns=" << start.time_since_epoch().count()
        << " cpu_ticks="
        << (cpu == std::clock_t(-1) ? "unavailable" : std::to_string(cpu))
        << " cpu_semantics=" << cpu_semantics << "\n";
    log.flush();
    auto finish = [&](const std::string &outcome) {
      const auto usage_stop = phase_cpu_snapshot();
      const auto cpu_stop = std::clock();
      const auto stop = std::chrono::steady_clock::now();
      const bool ticks_available = cpu != std::clock_t(-1) &&
                                   cpu_stop != std::clock_t(-1) &&
                                   cpu_stop >= cpu;
      log << name << " " << outcome << " wall_ns="
          << std::chrono::duration_cast<std::chrono::nanoseconds>(stop - start)
                 .count()
          << " cpu_ticks="
          << (ticks_available ? std::to_string(cpu_stop - cpu) : "unavailable")
          << " cpu_ticks_scope=legacy_worker_only clocks_per_sec="
          << CLOCKS_PER_SEC << " memory=supervisor\n"
          << phase_cpu_fields(usage_start, usage_stop);
      log.flush();
    };
    try {
      auto v = f();
      finish("stop");
      return v;
    } catch (const std::exception &e) {
      finish("failure=" + std::string(e.what()));
      object("failure-checkpoint", name + "\n" + e.what() + "\n");
      throw;
    } catch (...) {
      finish("failure=nonstandard_exception");
      object("failure-checkpoint", name + "\nnonstandard_exception\n");
      throw;
    }
  }
};
inline void pipeline(
    const Cell &c, const fs::path &out, const fs::path &singular,
    const std::function<void(Archive &)> &initialize = [](Archive &) {}) {
  // Prepared for the later approved native admission; never invoked in this
  // zero-run task.
  require(fs::create_directory(out), "fresh per-call archive required");
  Archive a{out, c.order};
  initialize(a);
  a.object("canonical-input", canonical_input(c));
  a.object("input.sha256", sha256(canonical_input(c)) + "\n");
  a.object("scope",
           "symbolic construction "
           "only\nreduction=not_requested\nextraction=not_requested\n");
  a.object("unavailable-metrics",
           "phase_process_tree_cpu=unavailable; call-level cgroup cpu.stat "
           "required\nphase_memory_peak=unavailable; call-level memory.peak "
           "required\n");
  Backend b{singular, out / "singular", c.order, c.names, 0, {}};
  std::vector<Poly> generators = c.F;
  generators.push_back(Poly::variable(c.names.size(), 0));
  auto powers = a.phase("ideal-generation",
                        [&]() { return ideal_power(generators, c.mu); });
  a.polys("ideal-generators", generators);
  a.polys("ideal-power-generators", powers);
  auto inf = a.phase("J_inf-groebner",
                     [&]() { return b.groebner({Poly(c.names.size())}); });
  a.polys("J_inf-basis", inf);
  require(inf.empty() || (inf.size() == 1 && inf[0].zero()),
          "nonempty J_inf unsupported");
  auto G = a.phase("J-groebner", [&]() { return b.groebner(powers); });
  a.polys("ordered-basis", G);
  auto d = a.phase("initial-convex-integral-domain", [&]() {
    return hull(initial_vertices(c), [&](const Domain &x) { a.domain(x); });
  });
  auto closure = a.phase("optimal-closure-and-echelon", [&]() {
    return optimal(
        d.points, G, c.order, [&](const Poly &f) { return b.normal_form(f); },
        [&](const Closure &x) { a.closure(x); });
  });
  a.closure(closure);
  auto one = a.phase("augmented-polytope", [&]() {
    return augment(closure.S, c.order, [&](const Domain &x) { a.domain(x); });
  });
  a.object("dimension", std::to_string(one.dim) + "\n");
  std::vector<Q> counts, C;
  std::vector<std::vector<Q>> xs(c.weights.size());
  Aug current = one;
  a.phase("discrete-sums-and-statistics", [&]() {
    for (int k = 1; k <= one.dim + 3; ++k) {
      if (k > 1)
        current = convolution(current, one);
      auto stat = statistics(current);
      a.augmented(k, current);
      counts.push_back(stat.count);
      C.push_back(stat.C);
      std::string row = "k=" + std::to_string(k) +
                        " count=" + qstr(stat.count) + " C=" + qstr(stat.C) +
                        " xs=";
      for (size_t i = 0; i < xs.size(); ++i) {
        xs[i].push_back(stat.xs[i]);
        row += qstr(stat.xs[i]) + ",";
      }
      a.object("sample-" + std::to_string(k), row + "\n");
    }
    return 0;
  });
  auto D = a.phase("interpolation", [&]() {
    auto dim = interpolate(counts), sc = interpolate(C);
    std::vector<U> sx;
    for (auto &v : xs)
      sx.push_back(interpolate(v));
    a.univariate("s_dim", dim);
    a.univariate("s_C", sc);
    for (size_t i = 0; i < sx.size(); ++i)
      a.univariate("s_x_" + std::to_string(i), sx[i]);
    require(dim.size() <= static_cast<size_t>(one.dim + 1) &&
                sc.size() <= static_cast<size_t>(one.dim + 2),
            "statistics degree failure");
    for (auto &v : sx)
      require(v.size() <= static_cast<size_t>(one.dim + 2),
              "coordinate degree failure");
    return std::make_tuple(dim, sx, sc);
  });
  auto result = a.phase("rational-limit", [&]() {
    return limit(std::get<0>(D), std::get<1>(D), std::get<2>(D), c.weights,
                 c.mu, [&](const Limit &value) {
                   a.univariate("raw-numerator", value.raw_num);
                   a.univariate("raw-denominator", value.raw_den);
                   a.univariate("cancellation-gcd", value.gcd);
                   a.univariate("numerator", value.num);
                   a.univariate("denominator", value.den);
                 });
  });
  a.univariate("raw-numerator", result.raw_num);
  a.univariate("raw-denominator", result.raw_den);
  a.univariate("cancellation-gcd", result.gcd);
  a.univariate("numerator", result.num);
  a.univariate("denominator", result.den);
  Q cap(1, c.r), endpoint = std::min(result.delta, cap);
  int active_weight_sum = 0;
  for (int weight : c.weights)
    active_weight_sum += weight;
  a.object(
      "result",
      "delta_star=" + qstr(result.delta) + "\ndomain=0<delta<" + qstr(cap) +
          "\nray_endpoint_supremum=" + qstr(endpoint) +
          "\nendpoint_attained=false\nbeta_supremum=" + qstr(endpoint) +
          "\ngamma_supremum=" + qstr(endpoint * c.r) +
          "\nactive_box_log_volume_supremum=" +
          qstr(endpoint * active_weight_sum) +
          "\nfull_tree_reference_V_supremum=" + qstr(endpoint * (4 + 2 * c.r)) +
          "\nfull_tree_reference_V_over_3_supremum=" +
          qstr(endpoint * Q(4 + 2 * c.r, 3)) +
          "\nreference_conversion_is_joint_feasibility_certificate=false" +
          "\ntau_cuso=" + estr(E(c.weights.size(), 0)) +
          "\nnonempty_positive_domain=" + (endpoint > 0 ? "true" : "false") +
          "\n");
  std::set<Z> denoms;
  std::string conditions;
  for (auto &f : closure.S)
    for (auto &t : f.terms(c.order)) {
      Z den = t.second.get_den();
      if (den != 1 && denoms.insert(den).second)
        conditions += den.get_str() + "\n";
    }
  a.object("coprime-conditions", conditions);
  a.object("specialization-obligations",
           "Clear rational denominators; require gcd(D,p)=1; preserve degrees, "
           "leading coefficients, pivots, ideal/span certificates; finite "
           "y-lifts, signs, subgroup and extraction not established. Fixed "
           "input heights do not cover growing centers/curve coefficients.\n");
  a.object("status", "complete-symbolic-only\n");
}
} // namespace jsr
