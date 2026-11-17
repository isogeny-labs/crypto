# Prime31 RR9 B8 signed-fibre decomposition: algebraic falsifier and Rust experiment
2026-11-17 UTC · [pcc:unverified] · PR #4545 stacked on #4541/#4539 · **NOT a complete non-generic ECDLP solver**

## Exact algebraic result

Let E/Fp be the actual prime31 curve y²=f(x)=x³+ax+b (p=2³¹−1, n=2,147,446,831), R=(r,s) a non-infinite public residual, and normalize the classical RR9 function as
`F=(x-r)A(x)+(y+s)V(x)`, `deg(A),deg(V)≤3`, `V_3=1`.
The monic residual eight-x polynomial from #4539 is
`M=D_R V²-(x-r)A²-2sAV`, where `D_R=x²+rx+r²+a`.

For a factor-base x_i with `x_i≠r`, choose either y_i or −y_i on E. Since
`(x_i-r)D_R(x_i)+s²=f(x_i)=y_i²`, a direct expansion proves the **exact split-quadric identity**
```
-(x_i-r) M(x_i) =
  [(x_i-r) A(x_i)+(s+y_i) V(x_i)]
  [(x_i-r) A(x_i)+(s-y_i) V(x_i)].
```
The two factors are *affine linear* in the seven normalized chart coefficients. Because the curve has odd prime order there is no rational affine 2-torsion and all chosen y_i≠0; the two signs correspond to distinct points.

For a base of exactly eight distinct rational x_i, the squarefree branch `M|H_B` with both polynomials monic degree eight is `M=H_B`. For x_i≠r, this is equivalent to selecting one sign for each of the eight split-quadric factors, then solving an 8×7 **linear** system in Fp. Thus the entire algebraic solution set is the union of at most **256 affine linear fibres**, one for each sign vector (and generically each admissible fibre is empty or a single point). This is an equality of zero sets, not an algorithmic lower bound, and does *not* prove that all possible Gröbner/elimination methods must enumerate 256 choices.

**Important exception:** at x_i=r, multiplying by x_i−r makes the split-quadric identity a tautology and no longer tests M(r)=0. An extractor must handle this chart separately. The Rust protocol excludes and counts such targets, never mislabels them as verified.

## Relation-collection consequence

On this prime-order curve, at most 256 distinct public group residuals have a squarefree eight-leaf signed B8 decomposition. Therefore uniformly sampled residuals land in the B8 stratum with probability at most `256/2147446831 ≈ 1.192×10⁻⁷`. Under independent uniform trials, the mean number of attempts before the first success is **at least 8,388,464** (and may be worse). Planted B8 positives are mathematically suitable as exact fibre *correctness controls*, but catastrophically unsuitable as an estimator of relation yield, B31 scalability or complete ECDLP cost.

For the B31 squarefree branch, the same observation holds for each eight-x subset: the parameter space is a union indexed by `2^8*binom(31,8)=2,019,513,600` signed formal supports. This is a structural statement; it neither says that this many F4 branches must be explored, nor measures any performance.

## Actual Rust validation

- Frozen before code: [RR9_B8_SIGNED_FIBRE_PROTOCOL_20261117.md](RR9_B8_SIGNED_FIBRE_PROTOCOL_20261117.md), commit `3d8fe4cbd02467cb0790d5e91ff3d853c41520fc`.
- Implementation: existing `research/prime31_ic_20261115/native/examples/prime31_rr9_evaluation.rs`, source commit `7caa5349af51984bfbb365bdbef196ce0d8d61b8`; no new Python implementation or alternative ECC.
- The Rust experiment uses first eight real Fp rational-x points and original-curve `Arithmetic` to construct every sign-mask sum as an independent test-only oracle. For each admissible unique R it solves all 256 *affine linear* chart systems and checks exact mask-set equivalence. It validates every recovered F(-R) and eight signed points, `M=H_B`, radical membership, actual group sum, arbitrary-A/V split-factor controls and four excluded-support negative residuals R+kG.
- New test `rr9_b8_full_signed_fibre_equivalence` is a bounded scientific **falsifier**. The entire procedure explicitly enumerates sign supports, so it cannot be presented as a novel non-generic extractor or a complete ECDLP algorithm.
- Exact-source CI: [GitHub Actions #44056622448](https://github.com/isogeny-labs/crypto/actions/runs/44056622448) **SUCCESS** on corrected code SHA `8b0364f005370c7d1eeaf7544759526e52c78cc0`, pinned Rust 1.100.0. **40/40 original Rust library tests and 7/7 RR9 example tests passed**, including the complete B8 signed-fibre equivalence test, all admissible B8 positive and four outside-support negative residuals, the 256×8 independently factored polynomial controls, and unchanged **128/128** original-curve determinant decisions. The full B8 JSON counters are preserved in [raw CI artifact #14315968389](https://github.com/isogeny-labs/crypto/actions/runs/44056622448/artifacts/14315968389), ZIP sha256 `699b7b08001b86c94f84392212d1e6e405014e8d3af08246612e3ab1470a4a36`. The action checks the original predicate panel while the Rust test itself asserts B8 exact set equivalence; don't confuse the 128 determinant decisions with additional unknown-target ECDLP solves.
- Failed precursor [CI #44055897564](https://github.com/isogeny-labs/crypto/actions/runs/44055897564) is retained: all 40 library tests passed, then E0308 from exactly two `Err(&str)` versus `Err(String)` constructors prevented the RR9 example from compiling; corrected only those two type conversions. There was no change to frozen protocol or mathematical checks. Both CI runs are producer-level [pcc:unverified], not independent signed certification.

## Earlier leads and priorities

| Lead and evidence | Interpretation / next decision |
| --- | --- |
| #4466 RR9 Hermite, #4539 signed chart, #4541 radical | Sound given-F relation predicates and smaller quadratic representation; do not infer a public-only factor extractor. This test is a direct continuation. |
| #4228/#4349 exact quotient/FGLM and root replay | Reuse as actual target-conditioned eliminator only after exact B8 fibre gate; account for construction and verification costs. |
| #4373 S3-chain m8 F4 | 8/8 capped/undetermined, not disproven generally. Do not repeat unchanged pairing permutations. |
| #4513 complete multi-target mixed-factor-base index calculus | Correct 1024/1024 and rho-proxy advantage, but 4+4 group MITM is generic-equivalent and not a new IC complexity advantage. |
| #4532 matching shared-precompute BSGS | **170.113 ms** for 1024 logs versus **2221.474 ms** mixed IC on same host/targets; BSGS is **13.06×** cheaper in that one unisolated producer run. New candidates must beat this, not just rho32. |
| crypto-autoresearcher #3301/#3305 | 41-lead heat-map and 28 Semaev-alternative audits; toy-scale nulls not universal obstructions. Preserve exact revisit triggers. |

Primary published background: Amadori–Pintore–Sala, *Finite Fields and Their Applications* 51 (2018), DOI 10.1016/j.ffa.2018.01.009, and McGuire–Müller (2017), ePrint 2017/1262. No novelty claim for Lagrange interpolation, split quadrics or the union-of-hyperplanes identity. Research-KB MCP/CLI unavailable in this runtime; GitHub primary files and paper catalogue were used instead.

## Next experiments and stop/go

1. **Exact independent fibre test**: preserve all counts, failed runs and signed/negative fixtures from the Rust source-specific CI; do not promote without a pass. Include positive *and* empty fibres, collisions and exceptional x(R)=x_i.
2. **B31 nonenumerative proof-of-mechanism**: implement a coefficient extractor taking only public R and H_B, with no table of eight-leaf supports, no signs and no hidden scalar. Begin a small B8 test but explicitly exclude enumeration from the candidate solver; add repeated-x strata and exceptional x(R) handling before claiming completeness.
3. **Full-cost threshold**: #4532's 170.113ms/1024Q shared BSGS implies an optimistic budget of only **156.8 μs per successful relation** if 61 basis relations and 1024 Q-relations suffice, every attempt succeeds, and all other IC costs are *free*. This is a **necessary, not sufficient** timing ceiling for that cohort, not a measured RR9 cost. Charge failed rootfinding, Gröbner/quotient work, 61-basis rank, online targets, verification, setup and memory. Compare same target list on same hardware against shared BSGS, strongest tuned rho and external IC.

Nothing in this B8 study yet shows a cheaper complete unknown-target prime-field ECDLP solver. Any future claimed speedup must recover unknown Q and survive the full-cost comparator gate.
