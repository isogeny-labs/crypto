# Checked field-width extension (v9 device validation passed)

The completed v8 record uses p < 2^60. This separate extension admits certified
prime-order curves with p < 2^62 and r < 2^63. Exact Hasse/order-witness
validation is unchanged. Hash primes must be in [2p,4p] and below 2^63; an
empty supported interval returns a limit rather than panicking.

The existing Fp64 Montgomery path still requires modulus <2^62. Larger hash
moduli use canonical add/double multiplication: both operands and every
accumulator stay below s, so every sum is below 2s <2^64. This handles only
inverse-hash candidate construction, with no change to curve arithmetic,
point keys, witness order or relation search. CPU reference multiplication
uses u128. The extended Euclid inverse uses i128 coefficients; with s<2^63,
even the conservative product bound s^2<2^126 fits. Field expressions remain
below 4p<2^64, signed point encoding below 2p<2^63 and Hasse's radicand 4p
below 2^64. Scalar additions are below 2r<2^64; modular products use u128.

Self-tests compare boundary residues against CPU u128 and full signed
inverse-hash inventories against the host implementation on 61/62-bit fields,
including hash moduli near 2^63 and canonical multipliers/offsets. Existing
small-field exhaustive comparisons and sanitizer suites remain. These are
arithmetic fixtures; no group cardinality is asserted by those fixtures.
The separately frozen v9 public recovery ladder generates prime-order curves
with PARI and certifies them again using the order witness in the driver.

The explicit row cap extends to 2^32; this allocates nothing by itself.
GPU admission continues to charge three complete row buffers and preserve
25% of available memory. V9 uses work cap2^34 and tests actual maximum-cap
headers before data, retaining every refusal and timeout. It compares index
width at52bits, then probes both fixed61/62bit targets. All clocks remain L0.

Sources: the existing Fp64 contract in ../rootfind/fp64.cuh; HAC algorithms
2.107/2.142 as documented in the shared driver; PARI ellcard documentation
https://pari.math.u-bordeaux.fr/dochtml/html-stable/Elliptic_curves.html;
the supplied paper's inverse-hash construction. This is a bounded generic
MITM backend, not the paper's conditional tensor contraction.

V9 completed all19Rust tests, device differential and recorded sanitizer
checks; one61-bit public target verified and the other refused memory
admission. Both62-bit recovery cells were unattempted. See
../../research/explicit_relations_e2e_20261030/results-v9/export.json and its
CHECKER-NOTE.md for full outcomes and retained canonical coverage findings.
