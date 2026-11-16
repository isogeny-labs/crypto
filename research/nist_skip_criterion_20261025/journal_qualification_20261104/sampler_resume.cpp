// Versioned recovery wrapper; the historical preparation instrument is unchanged.
#define main frozen_sampler_main
#include "../random_curve_validation_20261029/sampler.cpp"
#undef main
#define main journal_checker_main
#include "journal_check.cpp"
#undef main
#include "resume.hpp"

int main(int argc, char** argv) {
    try {
        require(argc == 8, "sampler_resume --pilot|--generate COHORT BITS NONCE FRESH_DIR NEW|OLD_DIR ACK_FROZEN_CHECKPOINT");
        std::string mode = argv[1];
        require(mode == "--pilot" || mode == "--generate", "mode");
        require(std::string(argv[7]) == "ACK_FROZEN_CHECKPOINT", "explicit checkpoint acknowledgement");
        pari_init(256000000, 1000000);
        auto bits = audit::counter(argv[3]);
        require(bits <= 65535, "field width");
        fs::path original = argv[6];
        bool resume = original != "NEW";
        audit::Config source{audit::fixed_field(argv[2], static_cast<unsigned>(bits)),
                             audit::bytes<32>(argv[4]), original / "attempts.tsv", original / "accepted.tsv",
                             mode == "--pilot" ? uint64_t(10) : uint64_t(10000)};
        auto outcome = recovery::run(source, fs::absolute(argv[5]), resume);
        std::cout << "Completed prefix " << outcome.attempts << " with " << outcome.accepted
                  << " accepted; separate full-journal replay pending\n";
        pari_close();
        return 0;
    } catch (const std::exception& error) {
        std::cerr << "FAIL: " << error.what() << "\nOperational errors are not curve rejections.\n";
        return 1;
    }
}
