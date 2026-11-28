# CT118 — Valid two-prime global certificates can still conceal a projective-DLP table

**2026-11-14. Status: [pcc:unverified].** This is a bounded negative/control result for the fixed order-eleven 11a1 source and characteristic-two destination. It constructs genuine characteristic-zero elements with *both* residue certificates, then shows why that alone is not evidence of an efficient scalar-independent transfer.

## Sources, concurrent scope, and assumptions

Read root [#3877](https://github.com/isogeny-labs/crypto/pull/3877), CT102's degree-60 x-field, [CT115](../ct115/CT115_FROBENIUS_KUMMER_DESCENT.md), [CT116](../ct116/CT116_ORBIT_INDEX_EQUIVALENCE.md) and [CT117](../ct117/CT117_INTERPOLATION_ANTI_TABLE_CONTROL.md). CT117 already proves that a rational polynomial producing binary *bitmasks* can hide the projective DLP, but its outputs are not genuine global torsion x-coordinates. This experiment closes that **specific certificate gap** in the *negative control*. Checked concurrent [cryptanalysis Richelot #1511](https://github.com/isogeny-labs/cryptanalysis/pull/1511), [genus-three norm #1523](https://github.com/isogeny-labs/cryptanalysis/pull/1523) and crypto-autoresearcher Prym proposals; no overlapping branches changed.

Methods used: standard elliptic division-polynomial identity `x([m]R)=x(R)-psi_(m-1)(R)psi_(m+1)(R)/psi_m(R)^2`; exact rational Lagrange interpolation; Victor Shoup, *Lower Bounds for Discrete Logarithms and Related Problems*, [EUROCRYPT 1997](https://www.shoup.net/papers/dlbounds1.pdf), only for generic-group context (no new unconditional complexity bound here). **Sources checked (KB): KB unavailable** — the research-KB MCP server and the checkout-local wrapper are not exposed in this connector environment. The repository sources are the fallback, not a complete novelty audit. Conductor/full cargo tests and signed PCC verifier are likewise unavailable here.

## Exact global-field interpolation theorem

Use the fixed curve `E: y²+y=x³−x²−10x−20`, source `P=(19,22) in E(F59)[11]`, and CT102's chosen global eleven-torsion point `T`, with `alpha=x(T)` and `[Q(alpha):Q]=60`. Denote the two **pinned** residue maps by

- `rho59(alpha)=19 in F59`;
- `rho2(alpha)=t in F_(2^20)`, where `t` is the polynomial-basis element `0x2` modulo `0x130027`.

Let `beta_m=x([m]T)` for `1<=m<=5`. Standard division-polynomial rational functions `R_m(X)` give `beta_m=R_m(alpha)`. Their numerator degrees are `[1,4,9,16,25]` and denominator degrees `[0,3,8,15,24]`, all below 60. Thus their denominators cannot vanish at the degree-60 element `alpha`; both fixed-place denominator specializations were also computed explicitly and are nonzero:

| m | r_m = x([m]P) | rho2(beta_m) bitmask | denom mod 59 | denom at t (binary bitmask) |
|---|---:|---:|---:|---:|
| 1 | 19 | 2 | 1 | 1 |
| 2 | 20 | 17 | 19 | 1 |
| 3 | 43 | 4956 | 7 | 261 |
| 4 | 46 | 65536 | 26 | 1 |
| 5 | 22 | 4697 | 22 | 4941 |

The binary values above are **field elements**, not integer reductions of the displayed masks. Here the actual reduction happens by evaluating each rational function in the pinned residue field.

Define over the *characteristic-zero global field* `L=Q(alpha)` the degree-four polynomial

```text
ell_m(X) = product_(j!=m) (X-r_j)/(r_m-r_j)
A(X) = sum_(m=1)^5 ell_m(X) * beta_m   in L[X].
```

For every nonzero *public* source point `Q=[k]P`, choose the canonical integer representative of its x-coordinate, `r=x(Q) in {0,...,58}`. One has `r=r_m` for exactly one `m in {1,...,5}`, so the exact rational Kronecker identity gives

```text
A(r) = beta_m = x([m]T) in L
rho59(A(r)) = r = x(Q)
rho2(A(r)) = x([m]T_reduction) in F_(2^20).
```

There is **no post-hoc integer-to-bitmask reinterpretation** and no merely p-adic representative. Each output is literally the correct global algebraic x-coordinate, expressed as a rational function of `alpha`. The target sign is recovered by comparing `[m]P` and `Q`. The construction is projective/x-only, as appropriate: `Q` and `-Q` have the same x-coordinate.

**General form.** For an odd prime-order subgroup with `h=(ell-1)/2` distinct projective source abscissae, provided the `h` intended global conjugates and source residue maps exist, the identical interpolation produces a degree-`h-1` polynomial over the global field matching all classes. This is an existence construction, with `h` embedded targets; no compactness or scalable runtime follows.

## Why the valid certificates do not establish an attack

The five source abscissae `[19,20,43,46,22]` were populated **offline** using the explicit source scalar enumeration `[1]P,...,[5]P` (four new source additions). The algebraic coefficients `beta_m` are *assigned in that scalar order*. The constants of `A(X)` therefore encode a complete five-entry source projective DLP table even though evaluating `A(x(Q))` never performs a dictionary lookup, and both fixed-prime residue equalities hold exactly.

CT117 refuted a fake bitmask "global transfer". CT118 shows the stronger adversarial example: **genuine two-prime torsion-coordinate certificates can be manufactured using nonuniform discrete-log advice**. Verification of a proposed online mapping must therefore **separately** demand (i) a reproducible uniform construction that does not precompute source projective logs; (ii) a full cold-cost ledger including all advice, number-field setup, factorization, embeddings and destination solving; and (iii) growing-order tests. A certificate does not prove the construction was cheap.

No impossibility theorem for genuinely uniform coordinate-native cross-characteristic transfers is asserted. Shoup's generic-group bound does not cover arbitrary algebraic coordinate algorithms or nonuniform advice.

## Reproducible local checks and boundaries

```bash
D=research/cross_characteristic_global_torsion_20261112/ct118
python3 -B "$D/ct118_certified_advice.py" >/tmp/ct118-py.json
python3 -O -B "$D/ct118_certified_advice.py" >/tmp/ct118-pyo.json
node "$D/ct118_certified_advice_independent.js" >/tmp/ct118-js.json
node --no-opt "$D/ct118_certified_advice_independent.js" >/tmp/ct118-jso.json
cmp /tmp/ct118-py.json /tmp/ct118-pyo.json
cmp /tmp/ct118-py.json /tmp/ct118-js.json
cmp /tmp/ct118-py.json /tmp/ct118-jso.json
printf '%s  %s\n' 5da78c89bef8fbd902c56c97b400a971e1f28d6a5f40fa8c49f9de7aada54527 /tmp/ct118-py.json | sha256sum -c
```

Developer-local ordinary/optimized Python and independent BigInt JavaScript modes agreed at that SHA-256. The two programs use distinct interpolation algorithms (rational polynomial coefficients versus direct integer barycentric products), independent source group arithmetic, and different finite-field implementations. Ten nonzero encoded source points passed public-input/target/sign recovery; three invalid input controls rejected. The five source denominators and five characteristic-two denominators are verified nonzero. Independent PCC **scientific signing has not occurred**.

The single-target *source-only* BSGS control requires 4–6 source additions across these ten cases (50 summed across ten independent controls). Interpolation's **offline source advice** requires four source additions plus rational-map and finite-field construction. Neither implementation measures complete number-field setup, inversion, field representation, polynomial evaluation, x-only target DLP and amortization in matched units; no cold-cost ratio, cost advantage, or leaderboard change is claimed. The exact degree-60 field and residue maps rely on the CT102–115 prerequisites and are not independently reconstructed ab initio in this self-contained test.

## Next decisive, nonduplicate experiment

Attempt **uniform, no-table global root recognition**: factor the CT112–113 relative quintic over the fixed integral field presentation, enumerate no source scalar-labelled points, select a conjugate from the pinned source place using only the encoded `Q`, certify its field equality and binary residue, and charge the complete factoring/orbit-selection cost. Compare against the strongest matched source DLP and existing cold index-calculus baseline. If the selection computation extracts the cyclic automorphism index, CT116's projective-DLP equivalence applies. Keep genus-two Cantor/non-split/Richelot testing in the other open PR streams to avoid duplication.
