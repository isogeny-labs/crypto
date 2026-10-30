#set page(margin: 1.8cm)
#set text(font: "DejaVu Sans", size: 8.5pt)
#show raw: set text(font: "DejaVu Sans")
= Lean bridge for S3Roots and Pratt certificates for the Oakley orders
2026-10-29. Origin: CFSG exposure audit (crypto-autoresearcher PR \#2690). Status: *proposed*; the Lean builds are verified only by the CI runs named in the PR, not locally (no Lean toolchain could be installed in this session).

#image("diagram.svg", width: 100%)

== Task 1: `S3Roots` and Mathlib's group law
Mathlib pin: `d13f23b7...`/`v4.34.1` (unchanged). Lemmas are in `papers/rootfinding-decomposition/lean/S3Roots.lean`.

#table(columns: (auto, 1fr, 1fr, 1fr), stroke: 0.4pt + gray, inset: 4pt,
[*Theorem*], [*Establishes*], [*Hypothesis-delta (binders the paper's sentence does not state)*], [*Does not establish*],
[`addX_eq_mathlib_addX`], [`addX xR yR xF yF = W.addX xR xF (W.slope xR xF yR yF)`], [`DecidableEq K`; `W.a₁=0`, `W.a₂=0` (paper: short Weierstrass). No curve equation, no nonsingularity.], [Anything about `a₄,a₆`; the equal-x branch.],
[`pointX_add_eq_xPlus`], [x-coordinate of Mathlib `some R + some F` is `xPlus`], [As above, plus `W.Nonsingular` at both points (Mathlib `Point` requires it); `xR ≠ xF`.], [`R = ±F` cases.],
[`pointX_add_neg_eq_xMinus`], [x-coordinate of `some R + -some F` is `xMinus`], [As above plus `W.a₃=0` (negY = -y).], [Same.],
[`S3_factor_mathlib`], [`S3_factor` for Mathlib points, with equations derived from `Nonsingular`; conjunction with the two x-coordinate facts], [`a₁=a₂=a₃=0`; `S3 W.a₄ W.a₆`.], [`S_m`, `m ≥ 4`, remains unformalized.]
)
Axiom audit: the four theorems are added to `AxiomAudit.lean` and the CI count in `rootfinding-s3roots-lean.yml` is raised from 3 to 7. The expected result is only `propext`, `Classical.choice`, `Quot.sound`; this is unverified until CI is green.

== Task 2: primality of the Oakley orders
`research/weak_curve_leaderboard_20261012/lean/OakleyPrimality.lean`.

#table(columns: (auto, 1fr, 1fr, 1fr), stroke: 0.4pt + gray, inset: 4pt,
[*Theorem*], [*Establishes*], [*Hypothesis-delta*], [*Does not establish*],
[`Pratt.pratt`], [`p<2^256`, factor list `L` of primes with product `p-1`, Fermat and witness checks by `powMod` imply `Nat.Prime p`], [`p < 2^256` (bounds the exponent for `powMod`), not in textbook Pratt.], [Anything beyond primality.],
[`Pratt.powModAux_cast`], [`powModAux` computes `acc·b^e` in `ZMod m` for `e < 2^fuel`], [none], [],
[`oakley3Prime_prime`, `oakley4Prime_prime`], [`Nat.Prime` of the two integers via 18 certificates (factors below `2^20` by `norm_num`)], [None beyond the numerals in `OakleyOrders.lean`.], [That they are RFC point orders; that the curves are as described.]
)
The same data is in `validation/oakley_pratt_certificates.json`; the Rust checker `src/bin/check_pratt_certificates.rs` (`num-bigint`) passed locally: 18 certificates, 49 primes. Certificates were produced with sympy factorisation of `p-1` (an untrusted producer: only the checks matter). `decide +kernel` uses kernel evaluation, not `native_decide`.

== Open
Lean builds, `leanchecker --fresh` and axiom output pending CI; if `decide +kernel` is too slow or a tactic fails, the certificate JSON and Python checker remain the fallback evidence.
