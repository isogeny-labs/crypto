# Chained S3: exact x-root intersection oracle (Prime31 pilot)

2026-11-22. **Implementation committed to PR #4920; native Rust execution and performance validation pending.** Research status: component implementation, mathematically checked identity, not a complete ECDLP solver and not a demonstrated speedup. The separate symmetric-coordinate proposal PR #4921 and exact-FP64/static-elimination study PR #4922 are concurrent work, not merged into this prototype.

## Mathematical mechanism

For a nonsingular short-Weierstrass curve E/Fp, `f(x)=x^3+ax+b` and rational points P=(x,y), Q=(u,v), define `e1=x+u`, `e2=xu`, `M=e1(e2+a)+2b`. Then

```
S3(x,u,z) = (e1^2-4e2) z^2 - 2 M z + (e2-a)^2 - 4b e1
disc_z(S3) = 16 f(x) f(u)
x(P +/- Q) = (M +/- 2 y v)/(x-u)^2,  x != u.
```

The `+/-` ordering depends on which group-law sign is chosen, but the unordered roots coincide exactly with `{x(P+Q), x(P-Q)}`. This is an established Semaev/group-law identity, **not a novel algebraic breakthrough**. For `x=u`, the finite root is `x(2P)`; the other branch is the point at infinity `O` (if P has order 2, both are `O`). That second branch has no finite root of the affine polynomial and must be represented explicitly.

**Exact signed-three-sum lemma (finite-field version).** Let `F={P_i}` be a list of ordinary Fp-rational curve points with signs allowed, let `I` be a map from each finite S3 pair root and the infinity branch to its source index pair `(i,j)`. For any rational target R, `R=epsilon_1 P_i+epsilon_2 P_j+epsilon_3 P_k` for signs `epsilon_t in {+1,-1}` if and only if a key `x(R +/- P_k)`, with an infinity key when applicable, intersects `I` and one of the eight reconstructed signed witnesses sums exactly to R.

*Proof sketch.* In the forward direction move the third signed point to the right. Both sides lie in E(Fp), so equal points have the same x key (including O). S3's discriminant factorization enumerates both possible pair abscissae, including the exceptional infinity branch. In the reverse direction the candidate is only accepted after literal signed point addition to R. This proof assumes every listed factor-base point is a rational point on E, not just a root of a bare membership polynomial. The sign reconstruction and group check rule out spurious x-only hits. The implementation asserts the polynomial identity for every generic root before indexing it.

## Two paired Rust oracle arms

`examples/chained_s3_prime31_pilot.rs` reuses the existing `ic_boundary::{PrimeCurve,PrimePoint,CountedGroup,GroupOps}` and `semaev_prime_poly::Curve`. The reference arm builds a signed pair table keyed by complete points. The proposed S3 arm builds a quotient index keyed by the two affine x roots of each unordered factor-base pair. A target is tested by intersecting this index with the two S3 roots of `(R,P_k)`, reconstructing all necessary signs and accepting only a complete group-law witness. Equal-abscissa, order-two and identity intermediates are separate branches rather than unsafe inversions.

- Fixture: p=2147483647, a=847514846, b=2087444724, documented prime order n=2147446831, G=(625037715,2100627251), cofactor 1. This is the project's pre-existing ordinary Prime31 fixture; the code checks G is on the curve and `[n]G=O`, which **does not independently prove** the full point count or primality.
- Pilot B=32: first 32 rational x coordinates. Samples: 32 planted genuine three-term targets plus 32 public deterministic multiples of G. Both arms share every target and factor-base representative. This cohort is a correctness/control pilot, **not** an unbiased relation-yield benchmark; planted samples cannot estimate yield.
- `cargo test --release --example chained_s3_prime31_pilot`: exhaustive agreement across every point of a nonsingular curve over F101 with a four-element factor base, plus exact-root-vs-group-sum checks on F101 and the Prime31 fixture. **Not executed in this tool runtime:** Rust/cargo are not installed here; do not count tests as passing before original-source CI.
- `cargo run --release --example chained_s3_prime31_pilot`: prints pair table size, probes, root invocations, denominator inversions, exceptional cases, sign lift attempts, group additions/doublings and counted group-law field ops. **No measured output yet.** S3 root inversions are currently via `semaev_prime_poly::Fp::inv` (Fermat exponentiation), and not included in `GroupOps.field_ops`: the printed group-op totals are not a normalized wall-time comparison.

## Complexity and limits

For factor-base size B, both arms use O(B^2) pair preparation and storage; under bounded candidate multiplicity the S3 intersection uses O(B) root computations/hash probes per target, the same asymptotic query work as the signed pair table. Collisions and their signed witness checks have an additional data-dependent cost, up to O(B^3) in a completely degenerate bucket. The S3 index halves some signed-pair key duplication and avoids unconditionally computing four group sums per unordered pair, but replaces this with modular inversions. **It does not improve the asymptotic exponent over the existing balanced meet-in-the-middle family.**

For uniformly distributed subgroup targets, if there are at most O(B^3) signed three-sums, the success probability is at most approximately O(B^3/n), before overlaps and rank defects. Obtaining O(B) independent relations therefore needs roughly Omega(n/B^2) target draws in the unsaturated heuristic regime. At O(B) per trial plus O(B^2) setup and linear algebra of at least the output size, the natural cost envelope is O(B^2 + n/B), optimized near B=n^(1/3) to n^(2/3), *not* a rho-beating asymptotic bound. Any better claim must exhibit improved relation density, cheaper complete recovery, or a new non-generic structural mechanism, not merely a faster S3 root kernel.

## Protocol for the next validation rung

1. Obtain the original-source Rust test receipt; correct any compile or exceptional-branch disagreements without changing the registered fixture.
2. Benchmark the two arms at B=32,64,128,256 on identical held-out *uniform* target samples, separating setup, query, x-root inversions, candidate lifting, RSS and wall/CPU time; log failures and timeouts. Use batch inversion and Mersenne reduction only as a later separately registered arm, with full setup charged.
3. Integrate the faster *verified* oracle into the existing prime-field relation collector, with tagged signed factor-base columns, independent-rank and full cold single-target recovery; compare with the strongest project IC baseline and optimized negation rho under the same budget.
4. Investigate balanced m=4 S3 chains with **unrestricted rational auxiliary x coordinates**, explicit O branches, and structural elimination/caching. Compare against the project's existing m=4 MITM, F4 and Horner/PKM work: #4891, #4812, #4901, #4921, #4922. Do not treat expanded S5 vs unoptimized exhaustive enumeration as the incumbent.
5. For asymptotic claims, preregister increasing-prime families, fit complete-work exponents with intervals, and require a reduction below all matched frontier bounds; a constant-factor oracle win is not enough.

## Prior art, overlap, and provenance

- Semaev (2004), *Summation Polynomials and the Discrete Logarithm Problem on Elliptic Curves*, https://eprint.iacr.org/2004/031 — S3 construction and intended decomposition setting.
- Amadori–Pintore–Sala (2018), *On the discrete logarithm problem for prime-field elliptic curves*, https://ora.ox.ac.uk/objects/uuid:c99fdb39-f5b1-428d-999c-770b63363e25 — prime-field index calculus with reduced relation requirements, a mandatory non-naive comparison.
- Kousidis–Wiemers (2019), *On the first fall degree of summation polynomials*, https://doi.org/10.1515/jmc-2017-0022 — first-fall degree does not certify actual F4 solving complexity or an asymptotic speedup.
- Repository primary sources: `src/cryptanalysis/ic_boundary.rs`, `src/cryptanalysis/semaev_prime_poly.rs`, `RESEARCH_SEMAEV_ROOTFIND_VS_GROEBNER.md`, PRs #4812, #4891, #4901. The shared research-KB MCP was unavailable through this runtime; this is **not** a complete novelty audit.

**2026-11-22 progress gate:** exact S3 quotient design and original-repository Rust prototype checked in; mathematical identity checked symbolically on the recorded ordinary Prime31 example, but no Rust compiler/test receipt, measured time, linear-algebra integration, cold DLP recovery, or asymptotic gain from this pilot.
