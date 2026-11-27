# Separate recorded-divisor audit

Frozen before implementation/call, 2026-11-24. This extends PR #5123 and the
user-authorized local P-192 pipeline. Original evidence stays byte-equal.

H-G: each recorded `source_dual` is exactly the reduced class
`[3] pi*(P)` on the stated odd-degree genus-3 cover. Test every fixture in
the existing six-fixture transfer result, including identity and -2G. Also
check each recorded forward cycle's full quartic equation and separating
coordinate, preserving length/multiplicity by working modulo its full support.
Accept only if all comparisons pass; retain any failure without relaxing them.

Use a fresh C++ implementation with OpenSSL BIGNUM prime-field arithmetic,
polynomial Euclidean division/Bezout, and Cantor composition/reduction. Import
no producer arithmetic or generated arithmetic header. Only the frozen typed
JSON parser may be copied; numerical tokens remain exact strings. Derive the
cover polynomial by substituting t=1+1/(1024 X) into the degree-eight model,
and derive the pullback from its quadratic fiber and the ramified anchor.
Recompute a triple by (D+D)+D and compare the canonical Mumford pair. Check
inverse, identity, associativity on the fixed classes and ramified 2-torsion.

Negative controls must reject a valid but wrong (negated) dual class, an
off-curve dual, changed cycle y, shortened cycle support, omitted exceptional
fixture, duplicated exceptional fixture, noncanonical coefficient and wrong
field pin. These are local deterministic correctness checks, not a benchmark.

Scope boundary: this checks the recorded source-side composition and target
cycle incidence. It does not reconstruct phi, its transpose, or their signed
reference cancellation independently. Passing does not change the original
`independent_geometric_verification=false`, supply an independent signer, prove
the general map identity or price original map synthesis. The original final
elliptic norm remains checked by the separate existing OpenSSL companion.

Inputs: exact primary local run's transfer/result.json, original cover input,
and exact kernel3 quartic model under baseline-e2e/relations/input. Fixed
fixture order is O,G,-G,2G,Q,-2G. The wrapper pins all bytes before calling.
Strict build <=300 seconds, worker <=120 seconds; each <=1GiB, one original
allowed CPU. Record build, checker and complete wrapper wall/user/system
clocks, exits, CPU, versions, source/input/binary digests and unchanged post
digests in a fresh evidence directory. No paid service or new point search.

Sources checked: exact parent RELATION_TRANSFER_PROTOCOL.md, cover_arithmetic,
quotient_arithmetic, complete_maps and POLARIZED_QUOTIENT.md; Steven Galbraith,
*Mathematics of Public Key Cryptography*, extended/corrected chapter 10,
Theorem 10.3.14, equation (10.11), Theorem 10.4.1 and Exercise 10.4.2:
https://www.math.auckland.ac.nz/~sgal018/crypto-book/ch10.pdf . Read their
ramified-model assumptions and composition/reduction formulas. The HAC
chapter-14 lookup lacked the needed hyperelliptic material and was not used.
KB unavailable (no loaded MCP and the prior wrapper lacks its executable),
Conductor unavailable. The Boost header probe failed; OpenSSL is already
installed and uses a different arithmetic library from the GMP producer.
No new algorithm or independent signed standing is claimed.
