# X1(2,8) held-out prime field-multiplication price

Frozen before any held-out calibration launch. The earlier explicit-price
watcher covers the development primes only and remains a separate live series.
This series prices the six held-out primes in the frozen manifest SHA-256
`99140281d9a94c288b8993b503cf91e4226879052e25c63dfdad0fccd20c8bc8`:
`2081, 2113, 2273, 16417, 16481, 16673`, in that order. Do not substitute a
prime or combine development and held-out samples. The underlying PKM method
and its full-cost limits are described in the original [PKM paper](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf)
and `PROTOCOL_Z2XZ8_EQUAL_SIZE_COLD_20261128.md`; this protocol only measures
one arithmetic unit.

The cold CLI is source commit `62908d5299c04c0b9bda686eba7f19e271b09786`
and binary SHA-256
`de461514bcb5f6538781ee25785f11ac33940711a93c0b6cb6a5dfbe0a587002`.
The standalone `fp_mul_explicit.rs` source SHA-256 is
`1ef13cb97dc15782e9a8a2fca81659152535ee6aa806b15a6b4b89aa546dd7d6`.
The existing same-host build receipt SHA-256 is
`986197cbf10c3f903cd8ae4f8e0b41be024fc923b1cc8c7c0cf9295e3e5a2b23`,
pinning Rust 1.93.1, `rustc -O -C target-cpu=native`, arm64 host
`Mac.attlocal.net`, and binary SHA-256
`e70c562189a390b4d2a97a92d6d54484c3c99a340e49274efa5ce221878a3523`.
The standalone source uses the same `u128` modular multiplication as the
prime oracle. Each iteration executes eight dependent multiplications.

For each prime, run a fresh `fp_mul_explicit P 7 200000` process. Record seven
samples in nanoseconds per multiplication, the arithmetic median, all seven
deterministic result checks, exact command, source and executable hashes, child
CPU, wall, peak RSS, pre/post host observations and raw stdout/stderr hashes.
Run the six primes in forward order, then in reverse order in a second quiet
window. Do not start the held-out series while the development-price watcher
is still running. A pre-call host load above the logical CPU count or a named
competing compute process records a rejected check without a child launch.
The held-out store header pins the completed development store's SHA-256;
replay verifies all 12 qualifying development cases against that snapshot.
A post-call violation makes the attempt unqualified. Keep all attempts,
including failures and contamination, in an append-only store; refuse an
unresolved launch on resume. Cap each process at 30 seconds and 2 GiB, and at
most 20 attempts per prime per direction. Reverse starts only after all six
forward cases qualify.

Independent replay must verify the manifest's six prime/workload identities,
protocol, source, build and binary hashes, every command and raw content
hash, finite positive samples, `1,600,000` counted multiplications per sample,
result-check range, median selection and pre/post quiet gates. Freeze one
selected price per prime only when both directions qualify and their medians
agree within 10%; use the midpoint of those medians and retain both values
and spread. If a prime fails this check, leave it unpriced.

This measurement can price only explicitly counted field multiplications.
The v4 cold report still contains other positive unpriced work, including
coefficient/Jacobi arithmetic and Vélu evaluations. A selected multiplication
price therefore cannot by itself qualify a complete cold GAE total or a
matched-rho crossover. Keep the held-out cold store and its source pins
unchanged.
