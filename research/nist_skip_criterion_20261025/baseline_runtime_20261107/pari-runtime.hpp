#ifndef NIST_BASELINE_PARI_RUNTIME_HPP
#define NIST_BASELINE_PARI_RUNTIME_HPP
#include <pari/pari.h>
#include <cstddef>
#include <stdexcept>

namespace nist_runtime {
constexpr std::size_t maximum_stack_bytes = 1073741824;
inline void initialize(std::size_t initial_bytes, ulong prime_limit) {
    if (initial_bytes == 0 || initial_bytes > maximum_stack_bytes)
        throw std::runtime_error("PARI initial stack exceeds bounded runtime");
    ::pari_init(initial_bytes, prime_limit);
    // PARI library manual usersch5: call just after initialization; no GEN yet.
    // This enables demand growth up to 1 GiB, not unbounded host allocation.
    ::paristack_setsize(initial_bytes, maximum_stack_bytes);
}
} // namespace nist_runtime
#endif
