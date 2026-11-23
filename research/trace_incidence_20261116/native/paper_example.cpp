// Fixed-input development demonstration, not a performance experiment.
#include "pullback.hpp"
#include <iostream>
#include <stdexcept>
#include <string>

namespace {
void counts(const paper_ec::Stats& stats) {
    std::cout << '{'; bool first = true;
    for (const auto& entry : stats.counts) {
        if (!first) std::cout << ',';
        first = false; std::cout << '"' << entry.first << "\":" << entry.second;
    }
    std::cout << '}';
}
}
int main() {
    using namespace paper_ec;
    Stats setup;
    const PreparedDatum datum(paper_f83_datum(), &setup);
    const Coefficients f{26,12,7,48,4,58,78,41,32,28,1};
    const std::vector<Word> expected{11,19,25,34,41,64,65,68,71,72};
    std::cout << "{\"schema\":\"paper-example-structural-check/v1\",\"pcc_standing\":\"unverified\",\"timings_collected\":false,\"datum_construction\":\"given literal paper datum, not constructed here\",\"datum_validation_counts\":";
    counts(setup);
    std::cout << ",\"arms\":[";
    for (int arm = 0; arm < 3; ++arm) {
        Stats stats;
        const auto roots = arm == 2 ? root_find_pullback(datum, f, &stats)
            : root_find(datum, f, arm == 0 ? TranslationMethod::Dense : TranslationMethod::Trace, &stats);
        if (roots != expected) throw std::logic_error("paper example root mismatch");
        if (arm) std::cout << ',';
        std::cout << "{\"method\":\"" << (arm == 0 ? "literal-dense" : arm == 1 ? "literal-trace" : "source-pullback")
                  << "\",\"roots\":[";
        for (std::size_t i = 0; i < roots.size(); ++i) { if (i) std::cout << ','; std::cout << roots[i]; }
        std::cout << "],\"structural_counts\":"; counts(stats); std::cout << '}';
    }
    std::cout << "],\"scope\":\"One fixed correctness fixture; counters are structural diagnostics, not priced time or an end-to-end ECDLP ratio\"}\n";
}
