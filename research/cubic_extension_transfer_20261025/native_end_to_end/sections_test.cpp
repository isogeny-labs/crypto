#include "native_sections.hpp"
using namespace cubic_native;
int main(int argc, char **argv) {
  try {
    require(
        argc == 4,
        "usage: sections_test PUBLIC_FIXTURE HISTORICAL_RESULTS NEW_OUTPUT");
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
      for (auto &m : models["models"]) {
        if (m["q_mod_3"] == 1 && m["trace_mod_3"] == 1 &&
            m["source_frobenius_type"] == "scalar") {
          kernels = m["transverse_kernel_bases"];
        }
      }
      Construction c = construct(s, kernels, 2, true);
      std::cout << "CONSTRUCTION_READY\n" << std::flush;
      RiemannData rr = riemann_data(c);
      std::cout << "RIEMANN_READY\n" << std::flush;
      auto sections = symmetric_sections(c.quartic);
      std::cout << "SECTIONS_READY\n" << std::flush;
      auto cal = calibrate_sections(c, rr, sections);
      std::cout << "CALIBRATION_READY\n" << std::flush;
      json images = json::array();
      int multiplier = 1;
      for (; multiplier < 100; multiplier++) {
        try {
          images = json::array();
          for (auto &P :
               std::array<Point, 3>{s.P, s.Q, s.curve.add(s.P, s.Q)}) {
            images.push_back(
                image_coordinates(c, rr, cal, s.curve.mul(multiplier, P)));
          }
          break;
        } catch (const std::exception &e) {
          std::cout << "CHART_RETRY " << multiplier << " " << e.what() << "\n";
        }
      }
      require(multiplier < 100, "source evaluation chart budget exhausted");
      write_json(argv[3],
                 {{"status", "passed"},
                  {"p", p},
                  {"r", s.r},
                  {"quartic", c.quartic.encode()},
                  {"diagonal_constraint_rows", sections.constraint_rows},
                  {"diagonal_constraint_rank", sections.rank},
                  {"section_dimension", sections.basis.size()},
                  {"exact_diagonal_identities_verified", 8},
                  {"interpolation_holdouts", cal.holdouts},
                  {"interpolation_attempts", cal.attempts},
                  {"source_randomization_multiplier", multiplier},
                  {"image_coordinates", images},
                  {"all_coordinates_in_base_field", true},
                  {"source_log_read", false},
                  {"elliptic_to_jacobian_map_complete", false},
                  {"benchmark", false}});
      std::cout << "passed native section calibration p=" << p << "\n";
    }
    field_stop();
    return 0;
  } catch (const std::exception &e) {
    std::cerr << "ERROR " << e.what() << '\n';
    return 2;
  }
}
