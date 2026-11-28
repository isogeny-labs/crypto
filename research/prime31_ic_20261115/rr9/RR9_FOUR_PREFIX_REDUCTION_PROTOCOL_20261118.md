# Frozen protocol: Prime31 RR9 four-prefix elimination (2026-11-18)

**Owner / lane:** existing draft [PR #4728](https://github.com/isogeny-labs/crypto/pull/4728); original curve and native Rust code, no new repo or curve. **Standing:** prospective, `[pcc:unverified]`; commit this protocol *before* the source-specific test.

## Mechanism and hypothesis

For target (R=(r,s)), normalize the nine-point function as

[
F(X,Y)=(X-r)A(X)+(Y+s)V(X),quad
deg Ale3,quad V=X^3+v_2X^2+v_1X+v_0.
]

Choose four **signed** leaf points (Q_i=(x_i,y_i)) from four distinct weighted slot sets. If the four (x_i) are distinct and none equals (r), then

[
A(x_i)=-{y_i+s\over x_i-r}V(x_i).
]

Degree-three Lagrange interpolation makes (A) **affine-linear in just three unknowns** ((v_0,v_1,v_2)). The other four weighted resultant constraints are consequently four products of at most (B) quadratics in three variables, not eight products in seven variables. This is a conditional polynomial dimension reduction, not a complete solver, a nonenumerative prefix selector or an F4 speedup claim.

**Original curve (unchanged):** (p=2^{31}-1=2147483647), (a=847514846), (b=2087444724), order (n=2147446831), cofactor one, canonical first-31 liftable rational-x basis, weights ((1,3,5,7,11,13,17,19)), (pm[w_i]P_j). Use original Rust arithmetic pinned at `1df714645aa683cf975bc76fcacb9842cb4780fd` and the existing `prime31_rr9_weighted_resultants.rs` example.

## Preregistered native test and negative controls

1. For the same frozen 16 planted signed eight-point fixtures `t=0..15` in `prime31_rr9_weighted_resultants.rs`, compute public target, interpolated seven-coefficient RR9 chart and the four-prefix affine representation independently within the original example.
2. Reconstruct **all four** chart coefficients of (A) from the three chart coefficients of (V). Every field residue must be equal exactly; any mismatch rejects the experiment.
3. Check (M(x)=0) for the four remaining selected leaves and evaluate the (4\times3) Jacobian of their four norm-root equations in ((v_0,v_1,v_2)) at the planted chart. Record the exact finite-field rank, require rank three on the current frozen fixtures; a rank deficit is reported, not hidden.
4. Preserve the original `r=x_i` exceptional branch explicitly: reject this interpolation pivot (do **not** divide by zero or claim global completeness), and retest a different pivot/constraint scheme before any future complete extractor.
5. Perturb (v_0\mapsto v_0+1), reinterpolate (A) so the four prefix equations still hold, and require at least one of the four *remaining* slot resultants to fail for each of the 16 fixtures. If not, report the surviving mutation.
6. Run the example's original 40 library tests and focused exact-source Rust test on GitHub Actions. Independently compare a separately authored Python BigInt check if available. Record compilation failures and corrections, including excluded costs and unverified evidence tier.

## Complexity falsifier and follow-on gate

Even a successful 4-to-3 reduction will require ((2B)^4) **signed prefix choices** unless a separate nonenumerative prefix solver is found: (8^4=4096) at B4, (16^4=65536) at B8 and (62^4=14776336) at B31. The reduction alone is not sufficient to beat the ~144ms historical 1024-target shared BSGS result: at B31, a full all-prefix search on **every one of 1,024 public targets** would have a budget of only about **0.00952 ns per signed branch** (144.049 ms / (1024 × 62^4)) before F4 or verification. Even if the 144.049 ms were spent on *only one* target, the budget would be about 9.75 ns per signed branch. That is a budget implication, not a measured minimum.

If the 16 fixture tests succeed, next implement **public-only** unknown-prefix selection plus a factored/saturated three-variable solve on the frozen B8 46-hit/18-miss natural corpus. Compare directly with the seven-variable RR9 solver; count all failed branches, F4 ops and memory, then B31 rank31 / 1024 blinded verified ECDLP targets and cold/cached optimized mixed-IC/BSGS/rho baselines. **No unknown public logarithm is supplied to an extractor and no generic 4+4 sum table is allowed in the candidate.**

## Previous work, reuse and novelty boundary

- [RR9 weighted slot encoding and known-chart certificate, same PR](RR9_WEIGHTED_SLOT_RESULTANT_RESULT_20261118.md); [W8 zeta/Walsh sign evaluator](W8_ZETA_WALSH_RESULT_20261118.md) computes **given-x** sign decisions, not x indices.
- [#4692](https://github.com/isogeny-labs/crypto/pull/4692) is already tasked with the full seven-variable RR9/F4 unknown-chart lane; this proposed reduction is a scoped preprocessing experiment for that lane, not a duplicate solver fork.
- [#4714](https://github.com/isogeny-labs/crypto/pull/4714) complete generic M6 vs shared BSGS negative control; [#4717](https://github.com/isogeny-labs/crypto/pull/4717) shared-root multiplicity strata; [#4720](https://github.com/isogeny-labs/crypto/pull/4720) signed pair projector; [#4722](https://github.com/isogeny-labs/crypto/pull/4722) exact M6 yield bound.
- Petit–Kosters–Messeng (PKC 2016, DOI 10.1007/978-3-662-49387-8_1) and Amadori–Pintore–Sala (FFA 2018, DOI 10.1016/j.ffa.2018.01.009) are relevant prime-field algebraic index-calculus prior art. The conditional linear interpolation step is classical; **do not claim novelty of interpolation**.
- Shared research-KB MCP and EKS runner are not available in this execution; current GitHub PRs and primary paper records were consulted. No expensive compute authorized.

**Prospective decision:** accept only a correct algebraic preprocessing component; advancement to B8/B31 requires natural public unknown-index extraction and full costs. Signed PCC independent verification remains a separate gate.

**2026-11-18 arithmetic correction, before native evaluation:** the initial protocol divided the 1,024-target cached BSGS time by 62^4 only. The corrected per-branch budget divides by 1,024 × 62^4; this is a transparent protocol amendment, not a post-outcome selection.
