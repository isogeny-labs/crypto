# CT216 — structural degree trap and bounded offset-coset escape screen

2026-11-18. Existing [crypto PR #4715](https://github.com/isogeny-labs/crypto/pull/4715), stacked on [#4697](https://github.com/isogeny-labs/crypto/pull/4697). **Finite exact arithmetic, not a complete ECDLP solver, benchmark, speedup or signed PCC**. [pcc:unverified]

## Motivation, established prior work and source compatibility

CT215's measured generic genus-2 transferred BSGS was 61.73× slower than direct elliptic BSGS over F31. The next unblocked question is whether the already implemented **genus-4 Jacobian index-calculus** instrumentation can be applied to an elliptic pullback without accidentally counting zero relations.

This is **not a new cubic cover**. The existing [cubic norm and closed-point report](../CUBIC_NORM_20261111.md) and [130-witness independent Rust replay](../../../..//examples/chained_cubic_norm_reference.rs) already use the exact F101 genus-4/genus-13 model; [P-256 genus covers](../../../p256_genus_dlp_20261115/REPORT.md) already give the analogous P-256 genus-4 construction. The [genus ladder implementation](../../../..//src/cryptanalysis/genus_ladder.rs) supplies full-base/double-large-prime IC and matched rho on random *ambient* near-prime Jacobians. Its `run_ic_ext` explicitly filters `cur.deg as usize != g` before attempting smoothness. These are different problem populations; a positive control on one does not imply success on the other. The shared research-KB MCP/wrapper was unavailable in this connector runtime; the cited primary repository sources were read. No novelty is claimed for these geometric facts.

Use

- `E/F101: Y²=X³+X+46`, full group order `109`, `G=(2,37)`;
- `C/F101: y²=x⁹+x³+46`, genus 4, cover `(x,y)↦(x³,y)`;
- the explicit pullback `Φ(X,Y)=(u=x³−X, v=Y)` and `Φ(O)=0` in reduced odd-degree Mumford coordinates.

**Structural obstruction to the existing relation sampler:** each of the 108 nonidentity source subgroup elements has reduced `deg(u)=3`; identity has degree 0. Thus any walk whose jumps and starting point lie entirely in `Φ(E[109])` stays at degrees 0 or 3, and **the present full-degree genus-4 IC collector accepts no candidates**. This follows from closure of the subgroup and the explicit reduced representation; the independent Python group-law implementation additionally checked all 109 cases. This is an instrument-compatibility statement, **not** a proof that no different index-calculus solver can exploit the embedded subgroup.

## Actual finite execution: 16 deterministic offset cosets

The new stdlib [finite screen](ct216_genus4_coset_screen.py) builds independent affine elliptic arithmetic, polynomial extended gcd, genus-4 Cantor composition/reduction, divisor validity checks and factorization by exhaustive F101 linear roots. It enumerates all 109 source multiples; verifies all distinct, `[109]G=0`, all 109 direct-to-Jacobian pullback identities, all additive inverses, and the explicit source order by point counting.

For each offset `D₀`, it independently computes and validates all `D₀ + [k]Φ(G)`, `0≤k<109`. The predeclared roster takes the **first eight rational cover points with distinct x** in ascending (x,y): `(4,19),(6,28),(7,31),(16,33),(17,4),(20,6),(21,25),(23,47)`. It forms all eight singleton offsets, four consecutive disjoint pairs, the first two consecutive triples and two consecutive disjoint quadruples. The roster is chosen without looking at smoothness.

| Offset type | Cosets | Degree-4 reduced divisors | Squarefree fully F101-split degree-4 divisors |
|---|---:|---:|---:|
| none: source subgroup only | 1 | 0 / 109 | 0 |
| 1 rational cover point | 8 | 856 / 872 | **0** |
| 2 points | 4 | 424 / 436 | **12** |
| 3 points | 2 | 216 / 218 | **10** |
| 4 points | 2 | 217 / 218 | **11** |
| **All nontrivial offsets** | **16** | **1713 / 1744** | **33** |

The eight singleton cosets each have `107` full-degree classes, **zero fully split**. The four pair offsets give `4,3,3,2` split classes; the two triple offsets `5,5`; the two quadruple offsets `5,6`. Overall for offsets of at least two points the observed fully split fraction of full-degree divisors is `33/857 ≈ 3.85%`; these finite, strongly dependent observations are *not* a scaling estimate, cold measurement or statistical confidence interval.

**Why singleton offsets fail, not just by chance.** Since `101 ≡ 2 (mod 3)`, for any nonzero `X∈F101` the polynomial `x³−X` has exactly one F101 root and an irreducible quadratic factor. A degree-1 offset supported at a rational cover point `(a,b)` generically composes without reduction to `u=(x−a)(x³−X)`, which has at most two linear rational roots; the exceptional overlapping-support cases reduce below degree 4. Consequently no squarefree fully split degree-4 relation is possible in these cosets. This does **not** rule out using degree-2 places as large primes in a different factor-base implementation.

**Positive but limited consequence:** pair/triple/quad offsets escape this particular full-split obstruction, giving 33 actual smooth class witnesses. They are *not* 33 independent relations, and no target log is recovered. The unchanged `run_ic_ext` cannot consume these events: a correct implementation must modify walk source, retain offset identities in relation rows, and verify the r-primary projection and matrix rank.

## Exact reproducibility and negative controls

Machine-readable [frozen observations](ct216_f101_coset_screen.json) use the first-eight roster, 16 per-offset degree histograms and fully split counts; the source regenerates identical JSON bytes. Local Python 3.13.5 commands executed successfully:

```sh
python3 research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct216_genus4_coset_screen.py --self-test
python3 research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct216_genus4_coset_screen.py > /tmp/ct216-screen.json
cmp /tmp/ct216-screen.json research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct216_f101_coset_screen.json
sha256sum /tmp/ct216-screen.json
# 5e2a5a2c3e70bba201b16d4526d2f0f5b5f8e003e0c2ed3a2a294a3aee9e2939
```

The bounded [CT205 workflow](../../../../.github/workflows/ct205-toy-transfer.yml) now independently repeats these commands on a GitHub-hosted `independent-field-oracle` job, alongside the existing F31 oracle. The **new exact-head remote CI result was not observed at document creation**. The Python implementation is an independent arithmetic check of the earlier Rust geometry, but it is *producer-written*, not a separately signed PCC or independent full-solve implementation.

## Next implementation gate: offset relation math before another benchmark

1. Build a prospective **two-point-or-higher offset** sampler from validated C/Fp divisor classes. Emit complete class identities, actual field/group operations, factor-base/large-prime symbols, and target coefficient, not just the number of smooth events. Start with F101 and use a distinct source subgroup target not involved in choosing offsets.
2. An offset relation has the form `D₀+[a]Φ(G)+[b]Φ(T)=Σ εᵢPᵢ`. The fixed `D₀` must be represented as an extra relation variable, or eliminated by legitimate within-coset differences; **dropping it is an invalid equation**. The current ladder uses `h=#J/r` to project onto order `r`; separately certify `#J`, `r`-adic multiplicity and that multiplication by `h` is injective on the source r-subgroup. If `r | h`, using that projection destroys the target information and requires a different projector/solver. Neither condition is verified here.
3. Reuse the existing genus-4 IC code and its LA/verification modules, rather than building another stand-alone generic BSGS. Require complete rank, target recovery checked on E, and cold cost with setup, offset selection, transfer, relation collection, LA, memory and failures. Compare strongest matched rho/BSGS/IC on the same host, and publish losses.
4. The owner's frozen 695/701-record `C169` vs `C13a+C13b`, norm-point reuse and `D` measurements remain separately blocked by the documented original-model S3 403. Do not bypass or re-create original custody. The independent full-cold adjudication belongs to [cryptanalysis #1766](https://github.com/isogeny-labs/cryptanalysis/pull/1766). [Crypto genus-4 IC #4693](https://github.com/isogeny-labs/crypto/pull/4693) is an adjacent performance control, *not* proof of a transfer win.

No paid EKS/Spot, merged protected branches, unsigned leaderboard promotion, or complete ECDLP advantage. **[pcc:unverified]**.
