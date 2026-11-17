#set page(width: 8.5in, height: 11in, margin: 0.8in)
#set text(size: 10pt, font: "DejaVu Sans")
#set par(justify: true)

#align(center)[
  #text(15pt, weight: "bold")[A novel genus-3 cover of a NIST-shaped curve, its explicit same-field quartic, and the checked transfer map]
  #v(2pt)
  #text(9pt)[Study `nist_cover_quartic_map_20261116` · run `20261116T1637Z` · `[pcc:unverified]`]
]

#v(4pt)
*Milestone.* One explicit, same-field plane quartic `C` and a checked map from
the Jacobian of a novel genus-3 hyperelliptic cover `H` of a NIST-shaped
elliptic curve `E`, carrying a planted discrete logarithm the whole way:
`E(F_p) →^(φ*) J_H(F_p) →^(Ψ) Pic⁰(C)`. This connects the cover/transfer stage
to the non-hyperelliptic index-calculus stage. Distinct from Tian's degree-27
`(3,3,3)` route (a separate construction).

#figure(image("chain.svg", width: 100%))

*The novel cover.* For `E: y² = x³ + a x + b` and free `γ ≠ 0`, the degree-2 map
`φ(u,v) = ((u²+γ)/u, v/u²)` pulls `E` back to
`H_γ: v² = u·[(u²+γ)³ + a u²(u²+γ) + b u³]`, ascending coefficients
`[0, γ³, 0, 3γ²+aγ, b, 3γ+a, 0, 1]` (monic degree 7 ⇒ genus-3). At `γ=1, a=−3`
this is `v² = u⁷ + b u⁴ + u`, the known cover; `γ` free is the one-parameter
family searched here for a Smith-positive member. The pullback homomorphism
`φ*` is injective on the odd prime subgroup (`gcd(2,r)=1`, `φ_*φ* = [2]`).

*Results (run `20261116T1637Z`, grid `{1009,2003,4001,10007}`, `a=−3`).*
All four instances: `E` prime order, a Smith-positive novel cover, a smooth
same-field quartic `C` with `#C(F_p)=#H(F_p)`, `φ*` order-preserving and a
homomorphism, and the planted log recovered on `C` by BSGS.

#table(
  columns: 8,
  align: (right, right, right, right, right, center, center, center),
  table.header[`p`][`b`][`#E` (prime)][`γ`][`#C=#H`][`ord φ*P = r`][`φ* hom`][`DLP on C`],
  [1009], [4], [967], [16], [918], [✓], [✓], [`k=323` ✓],
  [2003], [1], [2039], [3], [2034], [✓], [✓], [`k=680` ✓],
  [4001], [9], [4079], [5], [4082], [✓], [✓], [`k=1360` ✓],
  [10007], [7], [10193], [9], [10218], [✓], [✓], [`k=3398` ✓],
)

#figure(image("counts.svg", width: 86%))

*Explicit artifact (`p=1009`).* `E: y²=x³−3x+4`, `#E=967` prime.
Novel cover `H_16: v² = u⁷ + 45u⁵ + 4u⁴ + 720u³ + 60u`.
Explicit same-field quartic `C` (monomials of degree 4 order):
`[1, 1007, 0, 492, 875, 624, 339, 134, 871, 0, 34, 942, 452, 639, 311]` over `F_1009`.
Generator `P=(0,2)`; target `Q=(601,861)=[323]P`; `Ψ(φ*Q)=[323]Ψ(φ*P)` and BSGS
on `C` returns `323`.

*Scope and distance to rho.* Exact-correctness construction; no timing or speed
claim. For a prime-field target, index calculus on a same-field genus-3 `C` is
`Õ(p)` group operations (Diem), *above* matched rho's `Õ(√r)` on `E`; the gap to
rho is the distance remaining, not a gate. The deliverable is the verified
explicit artifact and the checked map. Larger primes are not run:
`Transfer::usable_points` is `Õ(p)` (prior Smith studies stopped at `p=211`,
`8209`; `10007` already exceeds both); the full NIST primes need a bignum port
(`smith_*` is `p<2³¹`), recorded as a gap. Smith's isogeny theorem is assumed,
as in `smith_canonical_20261115`.

#text(8pt)[Evidence: `runs.jsonl` (`data.sha256`), `run.log`, `PROTOCOL.md`
(`frozen.sha256` `215befa2…`), binary/source (`binary.sha256`). Module
`src/cryptanalysis/nist_cover_quartic.rs`, binary
`src/bin/nist_cover_quartic_map.rs`; 4/4 library tests pass.]
