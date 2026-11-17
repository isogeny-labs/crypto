# Exact mixed-base signed-4+4 support theorem (prime31)

**Status:** exact elementary combinatorial derivation; numerical application requires the actual first-x31 and Dickson-depth-six31 bases to share one affine point. The native Rust solver in [PR #4503](https://github.com/isogeny-labs/crypto/pull/4503) checks that overlap. Its CI results and numerical speed remain [pcc:unverified] pending execution.

## The new structural fact

A same-base 4+4 decomposition has **383,018,643,225 ordered formal half-pairs**, but only **5,654,193,417 distinct integer eight-leaf net vectors**. Most witnesses are cancellations or duplicate net signatures, not different algebraic relations. This explained the two prior Dickson31 targets with no eight-leaf support.

Now pair *different* 31-point bases with exactly one common affine point `s`. Write `A={s}∪A0` and `D={s}∪D0`, where `|A0|=|D0|=30` and `A0∩D0=∅`. Let

`D4(B)={u∈Z^B: ||u||₁≤4 and ||u||₁≡0 mod 2}`.

For fixed nonshared vectors of norms `α,β∈{0,...,4}`, admissible coefficients at `s` are exactly `−(4−α),−(4−α)+2,...,4−α` on the first side and similarly with `β` on the second. The sum at `s` therefore has exactly **9−α−β distinct possibilities**, not the product of these interval lengths.

For `k≥1`, the number of `m`-component integer vectors of L1 norm exactly `k` is

`N_m(k)=Σ_{j=1}^{min(k,m)} C(m,j)·2^j·C(k−1,j−1)`, with `N_m(0)=1`.

Consequently, as an exact equality in the *free integer lattice* on the 61 distinct basis points,

`|D4(A)+D4(D)| = Σ_{α=0}^4 Σ_{β=0}^4 N_30(α)N_30(β)(9−α−β)`.

With `N_30(0..4)=(1,60,1800,36020,541200)`, the result is

| Quantity | Exact value |
|---|---:|
| Mixed distinct formal signed net vectors | **381,434,284,809** |
| Ordered 4+4 half-signature pairs | 383,018,643,225 |
| Homogeneous distinct eight-net vectors | 5,654,193,417 |
| Mixed pairs per distinct formal net | 1.0041536864 |
| Mixed formal nets divided by prime31 group order 2,147,446,831 | 177.6222253 |

The mixed-base formal support is **about 67.46 times larger** than homogeneous support; nearly all the 4+4 formal pair count survives without trivial structural duplicates. This is a strong reason to test mixed-base *complete* IC rather than interpreting homogeneous 178 half-pair averages as genuine relation counts.

**Important limitation:** Mapping 381 billion formal vectors into the actual 2.147-billion-point elliptic group induces many additional collisions. The global mean 177.62 distinct lattice vectors *per group point* is exact, but neither full coverage nor a probability model follows from it. No faster ECDLP, non-generic polynomial extraction or asymptotic speedup has been proved. All group and timing claims remain separate.

## Reproducible proof checks

The companion `prime31_mixed_signature_count.rs` Rust example recomputes the sum with integer arithmetic, checks the 31+31−1 formula against the constants above, and exhaustively compares the same expression against small overlapping and disjoint signed-token bases. This is a math control, not a claimed ECDLP benchmark. Reuse the existing native Rust 1.100 project and pinned CI in #4503 when checking the original curve.

For next experiments, determine whether this near-lossless formal support translates into actual mixed-base coverage for the two known Dickson negatives, full 62-column rank and lower complete cold cost on fresh Q. Then investigate a *nonenumerative* target-conditioned RR9/S9 coefficient extractor: the generic 4+4 point lookup remains prior-art MITM.

Prior work: [#4391](https://github.com/isogeny-labs/crypto/pull/4391), [#4451](https://github.com/isogeny-labs/crypto/pull/4451), [#4466](https://github.com/isogeny-labs/crypto/pull/4466), [#4486](https://github.com/isogeny-labs/crypto/pull/4486), [#4503](https://github.com/isogeny-labs/crypto/pull/4503), and its preregistered mixed-base protocol. Classic Shanks baby-step/giant-step and APS 2018, DOI 10.1016/j.ffa.2018.01.009, rule out claiming the group-table method as a novel non-generic attack. Research KB and Conductor are unavailable in this connected runtime; GitHub primary files and published literature were inspected instead. [pcc:unverified]
