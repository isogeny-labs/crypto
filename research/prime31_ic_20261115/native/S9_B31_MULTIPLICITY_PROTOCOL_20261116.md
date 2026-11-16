# Prime31 signed S9 B31: net-root multiplicity versus generic half-sum witness multiplicity

Frozen 2026-11-16 **before** implementing or executing the accompanying Rust example. This is a diagnostic/verification oracle, not a solver performance claim. [pcc:unverified]

## Why measure this

The exact signed-eight support census establishes S(31,8)=5,654,193,417 *integer net signatures*. The complete 4+4 generic membership oracle at the actual prime31 cofactor-one curve, in a source-pinned successful Rust run [43951521215](https://github.com/isogeny-labs/crypto/actions/runs/43951521215), establishes that the first-31-x half-sum image contains 618,885 distinct group points, one per integer four-signature, and returns an eight-leaf witness for all 24 fixed natural targets. The 24 hits do *not* establish total target coverage, nor do many 4+4 hits equal many rational eight-leaf roots.

Let D4={u in Z^31: ||u||_1<=4, ||u||_1 even} and D8 defined with bound eight. By cancellation/padding, every d in D8 splits as u+v with u,v in D4. Direct combinatorics gives

- |D4| = 618,885; |D8| = 5,654,193,417.
- Exactly |D4|^2 = 383,018,643,225 **ordered integer half-signature pairs** (u,v) contribute to the full 8-signature image; their average multiplicity per integer eight-signature is exactly 67.74063336.
- Since #E(F_p)=n=2,147,446,831 is prime, uniformly averaging over **all** group elements Q gives exactly 178.3600123 ordered integer-half decompositions per Q, but only 2.632984126 distinct integer eight-signatures per Q. These are exact *global means*, not statements about individual Q or coverage.
- If images behaved like independent Poisson occupancy with mean 2.633 (not a theorem), coverage would be roughly 92.8%, not 100%. Thus observed 24/24 generic hits are compatible with a nontrivial missing fraction. A separate finite-field check is required.

The distinction matters for a target-conditioned S9 algebraic extractor: do not score 4+4 ordered witness counts as distinct rational f(T) roots or assume every Q has a polynomial solution.

## Before-running protocol

Reuse only the existing Rust native PRIME31 Arithmetic, Base::build depth six, point operations and generator. Compare **both** (a) first 31 ascending rational x, canonical smaller-y points and (b) the true 31-point depth-six Dickson fibre from the m8 quotient bridge. Never mix these bases. On each base construct all 677,040 unordered four-token signed words (62 signed tokens), retain one canonical **net vector** for each of the 618,885 distinct integer four-signatures, build an exact point-to-vector index, detect any extra group collisions and report them. If four-signature/group injectivity fails, the counting logic must instead retain all net signatures at a collided point, or report a censored multiplicity count—not silently drop them.

For each base, generate exactly 32 distinct nonzero caller-known verifier-only Q=[k]G using SplitMix64 seed 0x5339423331524f4f, and 12 blinded constructed-positive Q using token seed 0x533942333150524f. Include Q=O as a cancellation test; each base uses the **same target scalar seeds**. The candidate routine may see only public Q. For **every** Q (not only the first hit), exhaustively visit every group point U in 4A and query Q-U, count all *ordered* half-signature pairs and build the HashSet of summed net integer vectors d; assert ||d||_1<=8, even parity and that every distinct d replays to the actual Q by independently re-adding its signed factor-base leaves. Report each d (or an independently verifiable canonical digest plus exact per-target replay) and the ratio of ordered half pairs to distinct d. Record 0-count natural negatives without claiming the algebraic solver was invoked. Query time, group additions, full cold construction cost and memory estimates must be emitted separately; this is not cost-compatible with APS/rho yet.

Pure mathematical controls (no curve arithmetic): |D4|=618885, |D8|=5654193417, 62 choose-multiset 4=677040, and a smaller B4/B8 hand-enumerated exhaustive partition check that all net-eight signatures admit a 4+4 split and every split d=u+v is valid. The latter ensures dedup is by **net signature**, not arbitrary eight signed words or f(T) roots.

## Pass/fail decisions

PASS as a *generic oracle diagnostic* only if exact-source pinned Rust 1.100.0 CI passes, both bases finish exhaustively, no unhandled group collisions occur, and every unique d passes original-curve replay. Record the realized natural support/histogram, including negatives and multi-root examples. A cap, timeout or failed compile is **CENSORED**, not absence. Do not infer non-genericity, improved ECDLP complexity or optimized-rho advantage.

NEXT ALGEBRAIC GATE: feed the exact natural-positive Q and d-signature records into a separate blinded **target-conditioned** S9/K–S5 coefficient/root extractor, using the original-ideal and saturation certificates from crypto PRs #4228/#4349. An extractor must recover f(T) and orientation without consulting the 4A oracle. Only then run full rank-32 unknown-target ECDLP and matched cold optimized Wiener–Zuccherato rho plus external PKM/APS/F4.

Sources/prior-lead audit: existing [B31 4+4 protocol](S9_B31_FOUR_PLUS_FOUR_ORACLE_PROTOCOL_20261116.md), [B4/B8 exact support](S9_SIGNED_SUPPORT_CEILING_AND_B8_GATE_20261116.md), [APS vs unfolded rho 48Q](APS_RHO_FOLD_SENSITIVITY_48Q_RESULT_20261116.md), #4228, #4349, #4391, crypto-autoresearcher Semaev alternatives SA-02/SA-14b/SA-24c, cryptanalysis #1536. KB MCP and repository wrapper unavailable in this connector runtime; absence of a search hit cannot establish novelty. No paid EKS, merge, leaderboard edit or independently signed PCC claim authorized by this protocol.


## Separately labeled retrospective cross-base regression controls (amendment)

Added **after** observing the earlier oracle's distinct source-specific [successful two-base run 43952234526](https://github.com/isogeny-labs/crypto/actions/runs/43952234526), not to the prospective 32-natural Q selection rule. That GitHub Actions producer log reports first-x 24/24 and Dickson-depth-six **22/24** eight-sum coverage on identical fixed natural public targets, both 618,885 half-sum points with no distinct-four-signature collisions. These outcomes are [pcc:unverified].

The two *known* Dickson misses, with caller-only scalars independently replayed via separate modular curve arithmetic (no signed PCC), are:

| Prior case | Public Q=(x,y) | Caller-verifier k (never passed to oracle) |
| --- | --- | --- |
| 4 | (854640086, 1374782344) | 1610011808 |
| 21 | (1146958692, 202234633) | 1264623843 |

On the *earlier sample*, both cases were positive in the first-x31 base and absent after complete scan of the special Dickson31 four-sum image. Add them as labeled **retrospective cross-base regression controls** in the new Rust source: Dickson target has zero net signatures, first-x target has >=1. Do **not** count them in the new 32-case prospective coverage estimate or claim they are new holdouts. This is a valuable falsifier for the false assumption that any B31 base has 100% eight-leaf target coverage. It does not prove a general coverage fraction or a polynomial extraction method.
