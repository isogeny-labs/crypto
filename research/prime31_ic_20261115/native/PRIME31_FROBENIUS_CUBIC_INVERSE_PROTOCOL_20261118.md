# Prospective Prime31 Frobenius-linear inverse Vélu gate — 2026-11-18

Status: preregistered component experiment, **[pcc:unverified]**. No measured Rust outcome is prespecified. This is NOT an index-calculus relation extractor and does not establish a complete ECDLP speedup.

## Exact original curve and kernel

No substituted or deliberately weak curve. Reuse `prime31_algebraic_ic::{PRIME31, Curve, Arithmetic, check_curve, Point}`:
- `p=2^31-1=2147483647`; `#E(Fp)=n=2147446831` (prime; cofactor 1).
- `E: y²=x³+847514846x+2087444724`; `G=(625037715,2100627251)`.
- Merged #4671 codomain `E': y²=x³+323891735x+1985248359`, `G'=(1767020261,634125456)`.
- Source kernel abscissa `u=533591203`, `tau=1555600864`, `v=259447350`; kernel ordinate square is a nonsquare, so the two nonzero geometric kernel points are exchanged by Frobenius.

For a nonidentity `Q=(X,Y) in E'(Fp)`, derive the inverse polynomial *without an ECDLP scalar*:

`F_X(T) = T(T-u)^2 + 2*tau*(T-u) + 4*v - X*(T-u)^2.`

Recover the rational source x via `gcd(F_X(T), T^p-T)=T-x` in Fp[T]. Recover source y **without a square root** from

`h=x-u; y=Y*h^3/(h^3-2*tau*h-8*v)`.

The inverse returns O for O, rejects off-curve input and non-linear/multiple Frobenius-gcd results, and verifies `phi(P)=Q` in the original native Rust arithmetic. No generic 4A hash table, scalar k, enumeration of source points or Semaev oracle is permitted inside the inversion.

## Mathematical falsifier / proof obligation

For every `Q != O`, the cubic must factor over Fp as one linear and one **irreducible quadratic**. Proof approach: the geometric source preimages of Q are `P,P+K,P-K` where `P in E(Fp)` and `K^p=-K`. Their x values are distinct (else 2K=O or 2P=O, both impossible for nonzero P), and Frobenius exchanges the last two. Any failure of the claimed factorization, of source/codomain curve membership or of a round-trip is a hard stop, not an accepted outlier. The group-order certificate for E' and original E is the prerequisite.

## Frozen test panel and comparison

- Public targets: `Q=[k]G'` for integers k=1..256, plus deterministic sign/identity and invalid-point controls. The caller may use k to construct and **afterward check** a Q; the cubic inverse API receives Q only.
- For all 256: degree(gcd)=1, quadratic nonsquare discriminant, original-curve P on curve, exact `phi(P)=Q`, `P=[k]G`; for -Q recovered -P; identity -> identity, bad ordinate -> explicit reject.
- Meter Fp multiplication and inversion counts including all exponentiations, polynomial remainder/gcd, root lifting and final verification. Record sequential wall time as L0 diagnostic (not isolated/cold ECDLP wall speed). Re-run A/B order before claiming any stage improvement.
- Baseline: normalized degree-3 dual map, its x/9 and y/27 scaling, then original Rust scalar multiplication by `3^(-1) mod n=1431631221`. Verify exactly the same P and charge dual work and native EC group arithmetic.
- Mandatory gate: Rust 1.100.0 compile/test in the pinned native crate, check exact GitHub SHA and attach raw JSON. Keep CI failures; do not replace original evidence. No AWS/EKS spend authorized by this gate.
- If this preprocessor passes, only then link it to the **existing** RR9/S4/S3 nonenumerative relation extractor, never create a second RR9 owner lane. A proposed 1,024-target complete-cost A/B must charge transfer, new relation collection (including negative queries), full rank, target recovery, memory and all fallback relative to same-host IC/BSGS/rho. Replaying archived witnesses is not that experiment.

## Prior art and coordination

- Existing extension-field **inverse-Vélu cubic already implemented** for a toy `Fp²` curve: `research/isogeny_torsion_fibers_20261012/portable_order_trend_20261015/inverse_velu/RESULTS.md` and `PROTOCOL_V2.md`. Its randomized splitting needed a correction and the complete toy IC route remained far slower than matched rho. The cubic method is therefore *not a novel ECDLP algorithm*. Our bounded question is whether a deterministic unique-linear-Fp-root specialization avoids splitting and lowers the actual Prime31 inverse stage.
- Merged `crypto#4671` establishes the scalar-blind isogeny, its dual, transported rank invariance and 1,024 archived witness replays. Do not repeat its forward map as new science.
- `crypto#4692` owns fresh RR9 shared-coefficient extraction; `crypto#4678` and `#4682` own Split4/rho-parity follow-up; `#4604` owns negative D9 algebraic controls; `#4623/#4635/#4647/#4652` own the generic 4A solver and B31 controls; `#4596` owns matched BSGS/IC accounting.
- External background: Banegas et al., *Fast and Frobenius: Rational Isogeny Evaluation over Finite Fields* (2023), https://arxiv.org/abs/2306.16072; Petit–Kosters–Messeng (2016), https://doi.org/10.1007/978-3-662-49387-8_1; Amadori–Pintore–Sala (2018), https://doi.org/10.1016/j.ffa.2018.01.009. These do not provide a demonstrated Prime31 whole-solver advantage.
- Shared research-KB MCP/CLI not exposed in this connector execution; fallback searched the three GitHub repositories for inverse Vélu, isogeny preimage/gcd, prior Semaev work and inspected relevant primary sources. Not an exhaustive novelty clearance.

No candidate is promoted to the leaderboard without native full-cost evidence, independent signed validation, and genuinely algebraic extraction instead of a generic meet-in-the-middle group table.
