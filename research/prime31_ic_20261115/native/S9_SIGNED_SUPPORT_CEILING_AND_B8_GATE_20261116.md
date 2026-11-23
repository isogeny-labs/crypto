# Prime31 eight-leaf S9: exact signed-support ceiling and actual B4/B8 reach

Date: 2026-11-16. Status: **exact mathematical counting identity and independent finite-field arithmetic readback**; dedicated Rust CI has been committed but is **not yet observed passing**. [pcc:unverified]. This is an experiment-design correction, **not** a newly completed non-generic S9 extractor, a new ECDLP solve, or an optimized-rho speedup.

## Curve and reused implementation

Use the unchanged ordinary cofactor-one prime-field curve already in `research/prime31_ic_20261115/native/lib.rs::PRIME31`:

- `p=2147483647=2^31-1`; `E: y^2=x^3+847514846 x+2087444724`; subgroup/group order `n=2147446831` prime.
- Existing native Rust `Arithmetic`, `check_curve`, `PRIME31` and source-curve arithmetic are reused by [`examples/prime31_s9_support_ceiling.rs`](examples/prime31_s9_support_ceiling.rs).
- Fixed independent support oracle: take the first B rational abscissae (ascending), choose canonical smaller y, allow either sign on every leaf; repetitions and all opposite-sign cancellations allowed. The first eight affine representatives are
  `(0,309380359), (1,749117756), (2,51658786), (3,166645833), (5,457995111), (6,865603774), (8,849737830), (9,548858265)`.
- This base convention is independently reproducible; it should **not be assumed** to coincide with a different B8 validator's base without checking their manifests.

## Exact counting lemma (proof independent of ECDLP algorithms)

For B chosen abscissae `x_i`, fix rational representatives `P_i`. Any eight-leaf signed sum `Q=Σ_{j=1}^8 ε_j P_{i_j}` with `ε_j∈{−1,+1}` has a **net signature** `d=(d_1,...,d_B)∈Z^B`, where `d_i` is the number of positive occurrences minus negative occurrences. Cancelling opposite-sign pairs at each x gives

```text
    ||d||_1 <= 8,          ||d||_1 ≡ 8 (mod 2).
```

Conversely, every such d can be padded to exactly eight signed leaves using opposite pairs, for B>=1. This establishes an *exact characterization of the integer signatures*, including all repeated-root and cancellation cases. It does not assume any distribution of elliptic-curve point sums.

The generating function for vectors of absolute weight k is

```text
    Σ_{k>=0} c(B,k) z^k = ((1+z)/(1-z))^B,
    c(B,0)=1,
    c(B,k)=Σ_{j=1}^{min(B,k)} C(B,j) 2^j C(k-1,j-1)  (k>=1).
```

Therefore the number of possible signed net signatures is exactly

```text
    S(B,8) = c(B,0)+c(B,2)+c(B,4)+c(B,6)+c(B,8).
```

The number of *distinct group targets* is **at most** S(B,8), because two different d may map to the same group point. For uniform non-identity target `Q∈E(F_p)\{O}`, the success probability of any exact eight-leaf decomposition over this fixed factor base is bounded by `(S(B,8)-1)/(n-1)`. This ceiling applies to every factor base of B rational ±-pairs on the cofactor-one curve, not just to the one used here. It is independent of the S9 presentation and the root solver.

## Exact integer ceilings and first-B rational-x curve image

| B | Unordered monic f choices C(B+7,8) | S(B,8), integer signatures | Target coverage ceiling |
|--:|--:|--:|--:|
| 4 | 165 | 2,241 | 0.00010431% |
| 8 | 6,435 | 187,137 | 0.00871435% |
| 12 | 75,582 | 3,579,585 | 0.166690% |
| 16 | 490,314 | 31,910,913 | 1.485993% |
| 20 | 2,220,075 | 180,013,761 | 8.382688% |
| 24 | 7,888,725 | 750,695,169 | 34.957567% |
| 28 | 23,535,820 | 2,528,808,129 | *trivial: exceeds group order* |
| 31 | 48,903,492 | 5,654,193,417 | *trivial: exceeds group order* |

**Independent exact finite-field arithmetic check** (separately implemented, not a native Rust CI result): enumerated every integer signature, evaluated its elliptic-curve sum on the actual PRIME31 curve, and deduplicated the resulting group points. For B4: **2,241 distinct points including O, 2,240 nonidentity, zero collisions**. For B8: **187,137 distinct points including O, 187,136 nonidentity, zero collisions**. These are independent *curve-specific* checks of the Rust test's expected values; they do not establish PCC. The GitHub Rust example tests the identical point list and exact image counts, and emits a JSON producer receipt. Its Actions run must be checked before claiming Rust execution or a runtime result.

Thus, for these *specific* first-B factor bases, the uniform-nonidentity natural-target probabilities are exactly

```text
    B4: 2,240 / 2,147,446,830 = 0.0000010430991672...
    B8: 187,136 / 2,147,446,830 = 0.000087143484712...
```

These are **complete coverage probabilities**, not the probability an algebraic root extractor terminates, and do not establish logarithm recovery. For only 24 natural B8 Q, the union bound makes even one eight-leaf positive at most `24 × 187136/(n−1) ≈ 0.00209144` (0.2091%). To expect 12 natural positives, even with a perfect oracle, requires **at least 137,704** independently sampled nonzero targets on B8; B4 requires at least **11,504,180**. In contrast, planted positives can be selected cheaply by constructing Q from eight *public* factor-base points, withholding the chosen witness from the candidate extractor.

The zero-collision result is **uniqueness of net signatures**, *not* uniqueness of raw eight-leaf words or of their monic f: inserting ±P cancellation pairs changes the raw leaf multiset without changing its net signature.

## Decision: correct the algebraic pass/fail gate

Earlier [prime31 post-rho lead triage](APS_LEAD_TRIAGE_AFTER_RHO_48Q_20261116.md) requested both natural positives and negatives at B4/B8. A small uniformly sampled B8 natural-positive panel is mathematically almost guaranteed empty **even if the S9 solver is perfect**, and is not a scientifically useful falsification. Reframe:

1. **B4/B8 conditioned positives:** a held-out set of signed eight-sums generated outside the solver. Give the extractor *only* B, E, Q, not its leaf witness or polynomial f. Verify returned rational leaves and signs in original E(F_p), including the original ideal / selected-root / saturation certificate. Include repeated x, cancellation pairs, Q=O and zero/negative branches. Separate discovery time from independent enumeration used solely by the verifier.
2. **B4/B8 negatives:** exact finite-curve support census provides a complete negative oracle. Demand no spurious solutions outside the census. Test a blinded mix and report precision/recall and all work.
3. **B31 real target search:** target-conditioned `f(T)` extraction, with no scan over `C(38,8)=48,903,492` f. S(B31,8)>n is not a proof that every Q is representable; measure empirical reach and all failures. Complete unknown-`log_G(Q)` recovery with a checked signed relation and no target scalar leakage before a claimed ECDLP solve.
4. **Performance frontier:** compare fully cold one-logarithm APS/PKM/F4 against same-instance strongest tuned Wiener–Zuccherato rho and native unfolded rho with valid complete solves; include independent PCC and independent host reproduction before a fastest claim.

## Previous leads and avoidable duplication

- [crypto PR #4391](https://github.com/isogeny-labs/crypto/pull/4391): published APS S3 explicit pair index, generic MITM-equivalent; 48-Q fully cold APS 109.734280 ms vs unfolded rho64 99.473873 ms on the fixed previous sample, both 48/48, bootstrap crosses parity. New jump-width sweep preregistered and source committed, but source-exact Actions output **not retrieved**.
- [crypto #4228](https://github.com/isogeny-labs/crypto/pull/4228) selected-root quotient and [#4349](https://github.com/isogeny-labs/crypto/pull/4349) Rust exact root/certificate/F4 work: useful components, no reported natural B31 target-conditioned eight-leaf f extraction. The B8 *given-f* 6,435-case validator does not supply f-from-Q.
- Previous anchored m6 F4 has very low natural support and censored F4 even on planted instances; do not repeat an unchanged m6 cap or claim a negative from empty natural B8.
- [crypto-autoresearcher Semaev alternatives summary](https://github.com/isogeny-labs/crypto-autoresearcher/blob/main/experiments/semaev-alternatives-20261105/SUMMARY.md) and ERRATA: Vieta sieve SA-02, tower F4 SA-14b, genus-two SA-24c lead restrictions; preserve KILL vs INCONCLUSIVE.
- [cryptanalysis #1536](https://github.com/isogeny-labs/cryptanalysis/pull/1536): auxiliary-base/alternative IC needs a complete matched prime31 gate; do not substitute partial relation yield.
- APS [2018](https://doi.org/10.1016/j.ffa.2018.01.009), Wiener–Zuccherato [1998](https://doi.org/10.1007/3-540-48892-8_15), and Bos/Kleinjung/Lenstra [2010](https://doi.org/10.1007/978-3-642-14518-6_9) are established prior work; no novelty claim attaches to cancellation counting or net signed sums.

Sources checked (KB): **KB unavailable** (no `kb_status` / `search_knowledge` MCP or local wrapper in this runtime). Bounded fallback: actual GitHub files, prior PRs/logs, existing lead atlas, primary publication records. An inaccessible KB is not evidence a calculation is novel.

## Check-in and validation

New: [Rust native exact-support oracle](examples/prime31_s9_support_ceiling.rs), pinned toolchain [CI workflow](../../../.github/workflows/prime31-s9-support-ceiling.yml), this report, shared session-log entry. The Rust example reuses existing crate arithmetic, independently enumerates all net signatures on the actual curve, and has exact B4/B8 golden-result tests plus brute signed-word equivalence for smaller B. It does **not** replace, implement, or accelerate the current full APS ECDLP solver.

**Still missing:** observed green Rust Actions run, independent signed PCC, fresh optimized WZ rho comparison, and genuine nonenumerative S9 rational-root extractor with complete B31 unknown-target ECDLP timings. No leaderboard or fastest claim is warranted.


## 2026-11-16 exact-source native Rust completion (supersedes pending-CI statements above)

The initial Rust source compile failed with `E0499` in a nested mutable arithmetic borrow; source commit [15733881](https://github.com/isogeny-labs/crypto/commit/15733881eef897a1a144008c0653f689eb239aa5) corrects this by computing `rhs` before invoking `sqrt`. [GitHub Actions run 43950402621](https://github.com/isogeny-labs/crypto/actions/runs/43950402621), at this **exact source** and pinned Rust 1.100.0, is **SUCCESS**: 40/40 reused native library tests and 3/3 dedicated signed-S9-support tests passed. Actual executable source JSON reports B4 **2,241** and B8 **187,137** distinct signed eight-leaf PRIME31 group points including identity; both reproduce the independently calculated integers without collisions. Chargeable group-addition census counters are B4 **3,648** and B8 **265,728**, with unisolated elapsed diagnostic times **1.110199 ms** and **79.340145 ms**; these are not complete ECDLP solves or timings against rho. The immutable producer artifact is [S9 native census output](https://github.com/isogeny-labs/crypto/actions/runs/43950402621/artifacts/14266384771). Standing: **GitHub source-specific producer CI passed [pcc:unverified]**; independent signed PCC verification, a nonenumerative S9 extractor and complete solver benchmark are still distinct obligations.
