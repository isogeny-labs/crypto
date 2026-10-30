#include "native_group.hpp"
using namespace cubic_native;
int main(int argc, char **argv) {
  try {
    require(argc == 5 || argc == 6,
            "usage: transfer_test PUBLIC_FIXTURE HISTORICAL_RESULTS "
            "NEW_OUTPUT ARM(ic|rho|rho-neg|map) [SEED]");
    uint64_t seed = 20261029;
    if (argc == 6) {
      size_t consumed = 0;
      seed = std::stoull(argv[5], &consumed);
      require(consumed == std::string(argv[5]).size(), "invalid solver seed");
    }
    std::string arm = argv[4];
    require(arm == "ic" || arm == "rho" || arm == "rho-neg" || arm == "map",
            "unsupported target arm");
    std::ifstream file(argv[1]);
    uint64_t p = 0;
    file >> p;
    require(bool(file), "fixture prime missing");
    field_start(p, 18, seed + p);
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
      auto rr = riemann_data(c);
      auto sections = symmetric_sections(c.quartic);
      auto cal = calibrate_sections(c, rr, sections);
      std::cout << "CALIBRATION_READY\n" << std::flush;
      std::array<Point, 3> source_points{s.P, s.Q, s.curve.add(s.P, s.Q)};
      std::array<Inversion, 3> inverses;
      std::array<std::array<uint64_t, 8>, 3> coordinates{};
      json retries = json::array();
      int multiplier = 1;
      for (; multiplier < 100; multiplier++) {
        try {
          for (int j = 0; j < 3; j++) {
            coordinates[j] = image_coordinates(
                c, rr, cal, s.curve.mul(multiplier, source_points[j]));
            std::cout << "INVERSION_START " << multiplier << " " << j << "\n"
                      << std::flush;
            inverses[j] = invert_coordinates(c, sections, coordinates[j]);
            std::cout << "INVERSION_DONE " << j
                      << " divisors=" << inverses[j].divisors.size() << "\n"
                      << std::flush;
          }
          break;
        } catch (const std::exception &e) {
          retries.push_back(
              {{"multiplier", multiplier}, {"rejection", e.what()}});
          std::cout << "CHART_RETRY " << multiplier << " " << e.what() << "\n"
                    << std::flush;
        }
      }
      require(multiplier < 100, "source inversion retry budget exhausted");
      std::vector<std::array<V3, 3>> all;
      for (auto &inverse : inverses) {
        all.insert(all.end(), inverse.divisors.begin(), inverse.divisors.end());
      }
      auto projection = group_projection(c, all);
      std::unique_ptr<NativeJacobian> chosen;
      uint64_t P = 0, Q = 0, order_witness = 0;
      json group_retries = json::array();
      int group_attempt = 1;
      for (; group_attempt <= 16; group_attempt++) {
        try {
          auto candidate_group = std::make_unique<NativeJacobian>(
              projection.quartic, "build/libcubic_jacobian.so",
              seed + p + 104729 * group_attempt);
          auto &group = *candidate_group;
          uint64_t witness = group.certificate(group.context, s.r);
          require(witness > 1,
                  "independent constructed-target order certificate failed");
          auto reference = c.contacts.at(c.odd_index);
          reference.push_back({{F(1), F(0), F(0)}, 1});
          uint64_t ref = insert_divisor(
              group, effective_rur(projection.quartic, projection.coordinates,
                                   reference));
          std::array<std::vector<uint64_t>, 3> images;
          // Exercise confluent interpolation independently of the generic
          // images.
          uint64_t triple_origin = insert_divisor(
              group, effective_rur(projection.quartic, projection.coordinates,
                                   {{{F(1), F(0), F(0)}, 3}}));
          require(triple_origin != 0, "triple-support Hermite control failed");
          for (int j = 0; j < 3; j++) {
            for (auto &D : inverses[j].divisors) {
              uint64_t effective =
                  insert_divisor(group, effective_rur(projection.quartic,
                                                      projection.coordinates,
                                                      effective_contacts(D)));
              uint64_t image = group.op(1, effective, ref);
              bool seen = false;
              for (auto &other : images[j]) {
                seen |= group.equal(image, other);
              }
              if (!seen) {
                images[j].push_back(image);
              }
            }
            require(!images[j].empty() && images[j].size() <= 2,
                    "inverse group image count");
            if (images[j].size() == 1) {
              images[j].push_back(group.op(2, images[j][0]));
            }
            require(group.equal(group.op(2, images[j][0]), images[j][1]),
                    "Kummer images are not opposites");
          }
          P = images[0][0];
          Q = 0;
          for (auto &candidate : images[1]) {
            uint64_t sum = group.op(0, P, candidate);
            for (auto &U : images[2]) {
              if (group.equal(sum, U)) {
                Q = candidate;
                break;
              }
            }
            if (Q) {
              break;
            }
          }
          require(Q != 0 && !group.equal(P, 1),
                  "three-image sign alignment failed");
          require(group.equal(group.op(3, P, s.r), 1) &&
                      group.equal(group.op(3, Q, s.r), 1),
                  "transported points failed subgroup order certificate");
          order_witness = witness;
          chosen = std::move(candidate_group);
          break;
        } catch (const std::exception &e) {
          std::string reason = e.what();
          if (reason.rfind("native Jacobian arithmetic retry budget exhausted",
                           0) != 0) {
            throw;
          }
          group_retries.push_back(
              {{"attempt", group_attempt}, {"rejection", reason}});
          std::cout << "GROUP_CHART_RETRY " << group_attempt << " " << reason
                    << "\n"
                    << std::flush;
        }
      }
      require(chosen != nullptr,
              "Jacobian base-point chart retry budget exhausted");
      auto &group = *chosen;
      std::cout << "ACTUAL_TRANSFER_READY\n" << std::flush;
      json record = {{"status", "map_passed"},
                     {"p", p},
                     {"seed", seed},
                     {"r", s.r},
                     {"isogeny_steps", c.chain.size()},
                     {"kernel_index", c.kernel_index},
                     {"odd_reference_index", c.odd_index},
                     {"quartic", c.quartic.encode()},
                     {"group_quartic", projection.quartic.encode()},
                     {"prime_order_target_certified", true},
                     {"exact_diagonal_rank", sections.rank},
                     {"calibration_holdouts", cal.holdouts},
                     {"source_randomization_multiplier", multiplier},
                     {"chart_rejections", retries},
                     {"projection_attempts", projection.attempts},
                     {"jacobian_base_point_attempts", group_attempt},
                     {"jacobian_chart_rejections", group_retries},
                     {"triple_support_hermite_control", true},
                     {"image_coordinates", coordinates},
                     {"three_image_sign_recovery", true},
                     {"source_log_read", false},
                     {"synthetic_target_used", false},
                     {"native_forward_inverse_transfer", true},
                     {"benchmark", false}};
      json fibers = json::array();
      for (auto &inv : inverses) {
        fibers.push_back({{"ordered_solutions", inv.ordered_solutions},
                          {"effective_divisors", inv.divisors.size()},
                          {"groebner_basis_length", inv.groebner_length},
                          {"lex_basis_length", inv.lex_length}});
      }
      record["inversions"] = fibers;
      record["actual_target_generator"] = divisor_certificate(group, P);
      record["actual_target_point"] = divisor_certificate(group, Q);
      record["order_witness"] = divisor_certificate(group, order_witness);
      if (arm != "map") {
        uint64_t answer = 0;
        std::array<uint8_t, 4096> report{};
        uint32_t code = arm == "ic" ? 0 : (arm == "rho" ? 1 : 2);
        int result = group.solve(group.context, P, Q, s.r, code, seed + p,
                                 &answer, report.data(), report.size());
        record["target_solver"] = json::parse(
            reinterpret_cast<const char *>(report.data()), nullptr, false);
        record["target_arm"] = arm;
        record["target_solver_status"] = result;
        record["recovered_scalar"] = answer;
        record["verified_on_original_source"] =
            result == 1 && s.curve.mul(answer, s.P) == s.Q;
        record["status"] = record["verified_on_original_source"] == true
                               ? "passed"
                               : "solver_failed";
        write_json(argv[3], record);
        require(record["verified_on_original_source"] == true,
                "native target solver did not verify on original source");
      } else {
        write_json(argv[3], record);
      }
      std::cout << "passed native actual transfer p=" << p << " arm=" << arm
                << "\n";
    }
    field_stop();
    return 0;
  } catch (const std::exception &e) {
    std::cerr << "ERROR " << e.what() << '\n';
    return 2;
  }
}
