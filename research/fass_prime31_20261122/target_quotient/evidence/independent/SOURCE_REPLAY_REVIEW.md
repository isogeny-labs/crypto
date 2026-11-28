# Independent native source and replay review

Date: 2026-11-23 UTC. Reviewer: separate `/root/map_cost_audit` agent on the same host and session team. Tracking PR #4952. The review and replay are unsigned; they are not signed independent execution, a Lean proof, or a separate-host reproduction. [pcc:unverified]

## Outcome

The frozen qualifier replay exited **0** under the same 60-second timeout and 2 GiB virtual-address-space cap. Its complete 168,796-byte stdout was byte-identical to the producer's attempt 1 (`cmp` exit 0); stderr was empty. All seven checked input/source/binary hashes were unchanged across the call. No rebuild, source mutation, corrected retry, benchmark, or full-library test was performed by this reviewer.

The source inspection found no correctness defect in the fixed-fixture construction, rational-root extraction controls, remainder reconstruction, matrix ranks, selected cusp minors, or selector-target zero-row assertions. The checks remain at their specified finite polynomial scope. The generic theorem is separately audited in `PRIOR_SCOPE_REVIEW.md`; the selector-target mechanism is in `SELECTOR_TARGET_LEMMA.md`.

## 1. Executable and source custody

The reviewed main source and `oracle.rs` were read in full. Before execution, live main/oracle/Cargo files, the published arithmetic module, and the protocol were compared byte-for-byte with the producer's immutable source snapshot. All comparisons passed. The producer build log identifies the crate inside that snapshot, rather than an unrelated live path. The imported arithmetic file also matches the file at base commit `f0ca664eca0081a30adfcc79dcf62a32de40d2af`.

| Subject | SHA-256 |
|---|---|
| `native/main.rs` | `a524f3acb1263d26626f08f93dcc95f316a4f349533a5c5fe7bd6d3d4bbce23c` |
| `native/oracle.rs` | `77d935654429df6990bde72a7492ec24f4c0fbff1ae384b698203bb6d5d626d8` |
| `native/Cargo.toml` | `60e572ba5e60cc8ad72a6ce2efabab97c8d8fbe6e91f4fe5433b5ed2a8359f38` |
| `native/Cargo.lock` | `ef756f37fdadba9caaa055986891e5301806cb840d8ee605e3206ef084597a7c` |
| Published `src/cryptanalysis/semaev_prime_poly.rs` | `c8249ba825c946dbdb37fa5d6762f8f32d15320344693443e8af0d52f122784a` |
| Frozen protocol | `3eafeab5c2dc73c0add918bb23c9e8cdac1cf82d567d18a49792fd84aee0b20f` |
| `native/target/immutable/qualifier-attempt1` | `6f540765eafa6fc45754a4f242324073b0d366ae1c293d97ebb85b5f75ca5b3b` |
| Producer and reviewer qualifier stdout | `a1c1fffe57f3f3b65408171b9244cceff4ba57a52276d041a6ba8d52f02339d9` |

The replay ran from the `target_quotient/native` directory using exactly:

```sh
timeout 60s sh -c 'ulimit -v 2097152; exec target/immutable/qualifier-attempt1'
```

Pre-call receipt time was `2026-11-23T04:21:05Z`; post-call receipt time was `2026-11-23T04:21:07Z`. These are custody timestamps, not a performance measurement. The pre-call git HEAD was `ab905abd2a7b234c1c94946e8bae85493d120457`; source identity is bound by the individual hashes above. The host compiler reported rustc/cargo 1.100.0. The reviewer used the producer's frozen executable and did not independently rebuild it.

All raw records and copied producer manifests are retained under `qualifier-replay1/`. The extracted pre/post input-hash lists are explicitly post-call extractions from the retained original manifests; they do not replace or reconstruct a missing pre-call record.

## 2. Source-level checks

The cyclic-left norm helper expands the correct symmetric S3 coefficients, reduces only the x exponent modulo `d`, and retains the full t degree. Its field arithmetic is the unchanged published module. The reused matrix elimination performs ordinary Gaussian elimination over the field; the signed pivot product gives the square minors used in cusp controls.

The right-factor builder reconstructs its degree-at-most-two polynomial from scalar S3 values at `t=0,1,-1`, independently of the norm helper's coefficient table. Characteristic is greater than two. Actual trimmed degrees are used for quadratic, linear, and constant factors, while an identically zero factor is rejected. The current nonsingular finite targets and the cusp's `X=0` outside the selector do not require that missing all-field branch.

The primary rational part is computed as `gcd(R,t^p-t)` using published modular powering and gcd. A second construction independently unions the roots of every quadratic/linear factor, deduplicates them, and multiplies their linear polynomials. The two monic results must agree. F97 additionally checks every field element. The discriminant identity `16 f(z) f(X)` and squarefreeness are asserted. This is a distinct construction using shared trusted field/polynomial primitives, not an independently implemented arithmetic library.

The canonical remainder reduces each x coefficient polynomial modulo `G`. Its control instead fixes each selector x, multiplies scalar S3 quadratics modulo `G`, and interpolates in x. Every reconstructed coefficient must agree. Every full split-grid entry is also checked against the raw scalar product, and coefficient/evaluation ranks must match. Boolean support is counted and ranked as a separate matrix. Empty quotient dimensions are handled without inventing a zero root.

Ordinary rank saturation is not asserted. The only rank equalities asserted from formulas are the cusp witness and the previously published rank-one exception. For selector targets, the implementation asserts the expected zero row and the full/liftable upper bounds; a genuine lower rank would be retained. Full-right unchanged assertions are made only when `X^d != 1`.

## 3. Reproduced coverage

| Check recorded by the frozen executable | Count |
|---|---:|
| Finite target records | 91 |
| Full ordinary norm records | 20 |
| Rational-root-set comparisons | 91 |
| F97 exhaustive field-membership checks | 4,171 |
| Independent remainder coefficients checked | 35,391 |
| Split quotient-grid entries checked | 35,391 |
| Coefficient/evaluation rank comparisons | 91 |
| Noncollision full-right unchanged checks | 49 |
| Cusp nonreduced minor controls | 52 |
| Empty-subset controls | 20 |
| Rank-one exceptional targets | 3 |
| Norm scalar comparisons | 2,700 |
| Right-message scalar comparisons | 1,584 |
| Infinity-chart exclusions explicitly recorded | 20 |

The 91 target records include 20 retained duplicates across input categories and 42 selector-target records. There were 36 target records with empty algebraic zero support. No finite-target record had `G=1`; that dimensional edge is exercised by the 20 empty-subset controls. These are metadata counts of the retained records, not distributional estimates.

Prime31 uses exactly `d in {1,2,3,6,7,9,14,18,21,31,42,63}` and the deterministic public scan coordinates `0,1,2`, plus a selector-collision category. In this fixture the latter repeats `X=1`. The panel does not test all integers through 63, does not test `d=1302`, and does not sample uniform random group targets. F97 uses exactly the eight degrees in the protocol, adds its 2-torsion control, and retains the separate rank-one-curve controls.

## 4. Decisive observations and exception classification

At the fixed Prime31 curve and `d=63`, the records show:

| Target X | Right degree | Rational degree e | Norm rank | Rational remainder rank | Zero cells | Boolean support rank |
|---:|---:|---:|---:|---:|---:|---:|
| 0 | 126 | 68 | 63 | 63 | 0 | 0 |
| 1 | 125 | 67 | 63 | 62 | 133 | 34 |
| 2 | 126 | 68 | 63 | 63 | 0 | 0 |

The `X=1` row is repeated under the selector-collision category. There are 34 liftable left rows in these cells. Their value ranks are 34, 33, and 34 respectively. The selector-target loss is explained exactly by `H(X,t)=R_V(t)` and the resulting zero row. The all-zero Boolean support on `X=0,2` coexists with full coefficient rank; it does not give an algorithm for discovering emptiness without the qualifier's enumeration.

Across the regular-curve records, the only observed failures of the raw bound `rank=min(d,e)` are selector targets. This is an observation about this panel. The separate rank-one curve already has norm rank one: its ordinary target retains rank one, while its selector 2-torsion target has rational remainder rank zero. Neither is concealed by a saturation assertion.

Every F97 `X=30` 2-torsion control has rational degree `d` and repeated-factor degree loss `d`, consistent with the exact square/Mobius formula in `SELECTOR_TARGET_LEMMA.md`. The exceptional rank-one curve at selector 2-torsion `X=1,d=2` has full right degree 2 and rational degree 1, also as predicted. The cusp controls separately show a nonreduced rank `min(d,2k)` and radical rank one; these are not elliptic-curve targets.

## 5. Remaining scope boundary

This qualifier uses dense matrices, direct products, root enumeration, and interpolation as correctness oracles. It supplies no cheap procedure for constructing the Boolean support or finding a relation on a large target. Algebraic zero counts have not been promoted to signed relation counts; target-infinity branches are explicitly outside the finite affine calls. Group order, subgroup membership, signed recovery, whole-repository integration, formal verification, and performance are not established by this replay.

The fixed-degree identity certificate, if issued, is reviewed in a separate receipt with a fresh checker seed. It is not part of the 91-target result and cannot discharge the generic quotient theorem.
