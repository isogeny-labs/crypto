# CT238 — norm-fiber-compressed cubic-cover native speed test (2026-11-19)

**Status: preregistration for the *new native* test, not a native result. [pcc:unverified].** The Python exploratory toy screen was already run before this document and must be labeled *retrospective*; no retrospective count is an independently preregistered measurement.

## Why this distinct cover

CT236's F101 cubic cover has a 54-column signed factor base and 1,421 full-base relation candidates. It is decisively slower than direct BSGS. Its proof that the *full uncompressed* signed base grows as Θ(p) does not rule out exploiting the automorphism `u -> zeta*u` when `p=1 mod 3`.

At `p=103`, `zeta = 46` (also 56), `C:y²=u⁹+u³+24` maps by `(u,v)->(u³,v)` to `E:y²=x³+x+24`. Exploratory independent finite census found:
- `#E(F103)=109` prime. Public `G=(1,51)`, `[73]G=Q=(42,71)`.
- 66 signed rational `u` values on C, but only 22 distinct `u³` images in E; every such source image has three rational preimages. **22 norm-log variables**, not 66 independent Jacobian point-log variables.
- Experimental Python Cantor/elliptic source-norm pilot independently verified 27 fully split norm relations in 381 candidate walks, 22/22 source-norm log rank, and recovered scalar 73 after 27 target candidates. This is a *producer-local Python exploratory result*, not native Rust, PCC, independent mathematical asymptotic bound, or cost advantage. Different curve and field from the F101 baseline.

The construction does not assume direct isogeny transfer makes ECDLP cheap. It tests a distinct rational-fiber compression mechanism with exact retained source subgroup order 109.

## Frozen native measurement before implementation

1. Write a **separate** Rust integration test based on existing `ct226_native_genus4_transferred_ic.rs`, using exact genuine `FpPoly`/Mumford/Cantor implementation (no Python timing substitution), with constants `P=103,R=109,b=24,G=(1,51),Q=(42,71)`.
2. Enumerate the full C rational-point roots, form unique **source E norm x=u³** factor-base columns, and verify the 66-to-22 fiber census; preserve the same public 2-, 3-, 4-point offset schedule and degree-four fully split relation condition. Each C support contributes a signed entry to its E norm class (including repeated norm classes), so this is not the CT226 full-Jacobian 54-column matrix.
3. Do a *cold* one-target solve with source-norm log linear algebra and individual target log recovery. Assert 22 rank, 27 fully split relations, 381 candidates, and independently source-verified Q73. Print structured phase timings, Cantor and field-operation counters. Compare with exact same-host direct elliptic BSGS, interleaved 3 rounds; no two-target amortization.
4. Reject drift, incomplete rank, invalid support norms or wrong scalar. Keep all setup, relation search, linear algebra, target and verification in candidate cost. In this pilot memory is not separately attributed, so **not eligible** for a final complete-cost advantage claim even if a timing ratio appears favorable.
5. Ensure bounded GitHub-hosted native CI, source SHA and independent field-oracle checks. These are not EKS Spot paid measurements or signed independent PCC. Any native compile fail, unexpected counts or negative complete-cost result is recorded as such, not silently optimized away.
6. Same source field/order does not mean same *curve*: no performance ratio between CT236 F101 E and CT238 F103 E will be presented as a matched advantage. The decisive comparator is direct E(F103) BSGS and subsequently optimized rho/frontier IC and independent replay.

## Prior art and other branches

Double-large-prime techniques are established: Gaudry–Thomé–Thériault–Diem, https://eprint.iacr.org/2004/153. Automorphism fiber compression is an algebraic group-norm property; no novelty claimed. Do not duplicate original cryptanalysis #1533/#1692/#1729 norm-point reuse, crypto #4215, or other transfer catalog work.

**Standing gate:** no original authenticated 695/701 fixture replay, physical `C169,C13a,C13b,N2_actual,D` costs, memory-calibrated optimized rho/IC, multi-host confidence bounds or signed PCC. No paid compute or protected merge.
