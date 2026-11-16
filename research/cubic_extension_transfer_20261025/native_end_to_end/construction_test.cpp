#include "native_construction.hpp"
using namespace cubic_native;
int main(int argc, char **argv) {
  try {
    require(argc == 4,
            "usage: construction_test FIXTURE HISTORICAL_RESULTS OUTPUT");
    std::ifstream f(argv[1]);
    uint64_t p = 0;
    f >> p;
    require(bool(f), "fixture prime unreadable");
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
      // A prime-field nonsquare becomes square in this even extension. The
      // obstruction predicate must still reject it using the prime-field test.
      long nonsquare = 2;
      while (uint64_t(nonsquare) < FP && positive_obstruction(nonsquare))
        nonsquare++;
      require(uint64_t(nonsquare) < FP && F(nonsquare).square() &&
                  !positive_obstruction(nonsquare) && positive_obstruction(1),
              "prime-field obstruction negative control failed");
      // Independent positive and negative controls for projective smoothness.
      auto X = Polynomial::variable(0), Y = Polynomial::variable(1),
           Z = Polynomial::variable(2);
      require(smooth(X.pow(4) + Y.pow(4) + Z.pow(4)),
              "smooth Fermat control rejected");
      require(!smooth((X.pow(2) + Y.pow(2) + Z.pow(2)).pow(2)),
              "singular double-conic control accepted");
      Construction c = construct(s, kernels, 2, true);
      if (p == 157)
        require(c.chain.size() == 2,
                "certified p157 source must require two three-isogenies");
      json contacts = json::object();
      for (auto &[index, d] : c.contacts) {
        json rows = json::array();
        for (auto &C : d) {
          json P = json::array();
          for (auto &v : C.point)
            P.push_back(v.coeffs());
          rows.push_back({{"point", P}, {"multiplicity", C.multiplicity}});
        }
        contacts[std::to_string(index)] = rows;
      }
      json modulus = json::array();
      for (long i = 0; i <= fq_nmod_ctx_degree(FC); i++)
        modulus.push_back(nmod_poly_get_coeff_ui(FC->modulus, i));
      json record = {
          {"status", "passed"},
          {"implementation",
           "C++17/FLINT and native Singular; no Sage/Python execution"},
          {"p", p},
          {"r", s.r},
          {"ambient_modulus", modulus},
          {"source_log_read", false},
          {"full_source_order_certified", true},
          {"isogeny_steps", c.chain.size()},
          {"elliptic_preprocessing_subgroup_verified", true},
          {"prepared_a", c.prepared.a.coeffs()},
          {"prepared_b", c.prepared.b.coeffs()},
          {"prepared_P", point_json(c.P)},
          {"prepared_Q", point_json(c.Q)},
          {"kernel_index", c.kernel_index},
          {"odd_reference_index", c.odd_index},
          {"quartic", c.quartic.encode()},
          {"bitangents_verified", c.contacts.size()},
          {"contact_divisors", contacts},
          {"walk", c.walk},
          {"prime_field_square_test_validated", true},
          {"ambient_square_but_base_nonsquare_control", nonsquare},
          {"smoothness_positive_control", true},
          {"smoothness_negative_control", true},
          {"rational_actions_match_interpolation", true},
          {"elliptic_to_jacobian_map_complete", false},
          {"target_group_order_independently_certified", false},
          {"benchmark", false}};
      write_json(argv[3], record);
      std::cout << "passed native construction p=" << p
                << " steps=" << c.chain.size()
                << " bitangents=" << c.contacts.size() << "\n";
    }
    field_stop();
    return 0;
  } catch (const std::exception &e) {
    std::cerr << "ERROR " << e.what() << '\n';
    return 2;
  }
}
