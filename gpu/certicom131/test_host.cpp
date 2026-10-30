#include <chrono>
#include <iostream>

#include "fixtures.hpp"

int main() {
    const certicom131::Fixtures fixtures = certicom131::load_fixtures();
    const auto started = std::chrono::steady_clock::now();
    const int code = certicom131::verify_all(fixtures);
    const auto elapsed = std::chrono::duration<double>(std::chrono::steady_clock::now() - started).count();
    if (code != 0) {
        std::cerr << "CERTICOM131 HOST VERIFY: FAIL code=" << code << " elapsed_s=" << elapsed << "\n";
        return code > 255 ? 1 : code;
    }
    std::cout << "CERTICOM131 HOST VERIFY: PASS"
              << " prime_field_vectors=256 binary_field_vectors=256"
              << " scalar_vectors_per_curve=16 subgroup_checks=4"
              << " elapsed_s=" << elapsed << "\n";
    return 0;
}
