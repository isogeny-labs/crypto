# Operational non-split map checkpoint

Producer cryptanalysis9cd20ac5f80cdd140be2d16d49701c1b950ef13c, PR1478;
map source SHA-256
`d378a7d0cd835b88b494b9de0224b251b72ffc46467d6daf82fdb103e9e61870`;
balanced adapter SHA-256
`0ab6feecde9a47850e0a46ec70ee3cd72392342b62796fa0837be59ae03e7774`.
The unchanged native runtime stays crypto75a737b8. Full source-linked
[report](https://github.com/isogeny-labs/cryptanalysis/blob/9cd20ac5f80cdd140be2d16d49701c1b950ef13c/experiments/chained-transfer-reference/NONSPLIT_MAP.md)
and [inspected PDF](https://github.com/isogeny-labs/cryptanalysis/blob/9cd20ac5f80cdd140be2d16d49701c1b950ef13c/experiments/chained-transfer-reference/nonsplit-map-report.pdf).
All standing remains unsigned/unverified. [pcc:unverified]

The frozen determinant11 correspondence now operates on all285 rational
source/target classes with F17⁶ intermediates and coefficientwise descent.
Target fibers remain quadratic Mumford pairs; degree drops, multiplicities,
balanced infinities and raw target w=3Y are explicit. Both sign-corrected
dual norms[2],285-image bijection and paired additivity pass. Every19 elliptic
source scalar survives, with source norm[4]/inverse5 recovery. Maps use no
scalar labels. Native25/reference43 guard, formatting/strict Clippy/guards
pass;1 old benchmark is ignored. Producer claims383894/bd303e unsigned.
[pcc:unverified]

The independent field witness has128 cases, SHA-256
`46dc06ef4b2e8650a80aaa0f6107a90ca778c5b16ef79d2c5a2b71774e32617d`.
The pre-existing flat Fp⁶ engine and Rabin checker in
`src/bin/icprog/b5a.rs` replay products/inverses/Frobenius with a different
representation, γ=T+s and modulus[14,10,14,4,12,0,1]. Only a unit test/fixture
was added; the arithmetic engine is unchanged. Claim167595 binds the TSV and
passing replay. This checks the bounded extension layer, not independently
the whole Jacobian map. Both group engines use Cantor; producer-authored
harnesses/receipts do not supply signed independent scientific standing.
[pcc:unverified]

Four generic core Lean lemmas in RichelotAlgebra.lean pass7-declaration gate
and local PCCf7eb4f check: arbitrary-quadratic fundamental bracket identity,
cleared target-curve equation, balanced infinity coefficient and recovery
under19-torsion. Total44 written lemmas; gates22+8+7+12+6+5+7 declarations.
Finite fields, projective/Picard extension and actual global dual construction
remain written primary inputs. Failed/passing proof/format attempts remain
retained. [pcc:unverified]

Reproduce changed inputs only: consumer check.sh; root `cargo test --release
--bin icprog chained_nonsplit_compositum_witnesses -- --nocapture`;
proof-gate.sh on RichelotAlgebra.lean. The existing sparse-checkout script's
minimal profile automatically discovers the committed compile-time TSV;
existing ic-programme-harness on main compiles/runs the new reference test.
The producer source has no dependency on that sibling test source.

Next CT15 assesses frozen shift2 of E109, keeping the unshifted6-cycle
negative. CT16 separately assesses next nonbacktracking kernels and Galois
2-torsion of the accepted F17 codomain. Current curve nonisomorphism is not
promoted to unpolarized-Jacobian nonisomorphism. No solver, speed gain,
cross-prime descent or benchmark has been demonstrated. S3/live-budget,
KB/vault and crypto frozen-ledger/archive blockers remain. [pcc:unverified]
