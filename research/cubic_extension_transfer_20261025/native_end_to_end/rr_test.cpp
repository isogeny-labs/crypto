#include "native_rr.hpp"
using namespace cubic_native;
int main(int argc, char **argv) {
  try {
    require(argc == 4,
            "usage: rr_test PUBLIC_FIXTURE HISTORICAL_RESULTS NEW_OUTPUT");
    std::ifstream file(argv[1]);
    uint64_t p = 0;
    file >> p;
    require(bool(file), "fixture prime missing");
    field_start(p, 18, 20261029 + p);
    {
      Source s = source_fixture(argv[1]);
      json models =
               read_json(std::string(argv[2]) + "/kernel_classification.json"),
           kernels;
      for (auto &m : models["models"])
        if (m["q_mod_3"] == 1 && m["trace_mod_3"] == 1 &&
            m["source_frobenius_type"] == "scalar")
          kernels = m["transverse_kernel_bases"];
      Construction c = construct(s, kernels, 2, true);
      RiemannData rr = riemann_data(c);
      auto ps = source_signature(c, s.P), qs = source_signature(c, s.Q);
      require(ps.size() == 7 && qs.size() == 7, "source signature lengths");
      auto encode = [](const Univariate &v) {
        json a = json::array();
        for (auto &c : v)
          a.push_back(c.coeffs());
        return a;
      };
      json values = json::array();
      for (auto &fn : rr.functions)
        values.push_back({{"index", fn.index},
                          {"contact_a", fn.a},
                          {"contact_b", fn.b},
                          {"basis_dimension", 3},
                          {"at_zero", fn.at_zero.coeffs()}});
      write_json(argv[3], {{"status", "passed"},
                           {"p", p},
                           {"r", s.r},
                           {"identity_constraint_degree",
                            rr.identity_constraints.degree()},
                           {"identity_cubic_dimension", 1},
                           {"identity_u", encode(rr.identity.u)},
                           {"identity_v", encode(rr.identity.v)},
                           {"weil_functions", values},
                           {"source_P_signature", encode(ps)},
                           {"source_Q_signature", encode(qs)},
                           {"source_log_available", false},
                           {"elliptic_to_jacobian_map_complete", false},
                           {"benchmark", false}});
      std::cout << "passed native Riemann-Roch calibration p=" << p
                << " functions=" << rr.functions.size() << "\n";
    }
    field_stop();
    return 0;
  } catch (const std::exception &e) {
    std::cerr << "ERROR " << e.what() << '\n';
    return 2;
  }
}
