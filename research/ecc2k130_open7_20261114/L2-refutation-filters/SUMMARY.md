# L2 — refutation (UNSAT) workload and pre-solve filters: NO-CHANGE

The subagent could not write this file, so the main session saved its report here. Details
are in `RESULTS.md`, `tables.md`, `results.json` and `figure.svg`.

The main session checked:
- the 43 entries of `raw/SHA256SUMS`: all verify;
- `raw/aa/AA_CHECK.txt`: all A/A reruns byte-identical;
- the frozen protocol hash `67e5d4da…`: it equals the PROTOCOL.md committed in `73990ef46e`
  before any data existed.

All claims `[pcc:unverified]`. The measurements are exact counts from a std-only Rust harness
on y²+xy=x³+ax²+1 at n = 11, 13 and 19 (n prime, 2 primitive mod n). No wall time was used,
and there is no end-to-end number, so no gap to rho.

## Prior work that already covered part of this
- Trace and half-trace conditions are closed: `trace_homomorphism_tests_20261109`.
  χ(P)=Tr(x+a) has kernel 2E and is 0 on ⟨G⟩.
- In-solver early abort: `lit2026_w4_early_abort_20261011` (no-change / negative).
- Degree-2 Macaulay rows are already fed to production SAT:
  `notes/ecc2k130/RESEARCH_KOBLITZ_SCALING_TARGET.md` (H2′).
- m=2 refutation degree is 2 or 3: `semaev_t2_trace_20261102`.
- SAT refutes about one sorted triple per conflict, and a perfect UNSAT predictor would gain
  5.2–9.0×. Features of x_R give AUC ≈ 0.5: `pdp_ml_sat_ceiling_20261027`.

## (a) Workload
- Exhaustive UNSAT fraction over all targets in ⟨G⟩:

  | base dimension d | UNSAT fraction |
  |---|---|
  | ⌈n/m⌉ − 1 | 0.75–0.92 |
  | ⌈n/m⌉ | 0.31–0.52 |
  | ⌈n/m⌉ + 1 | 0.001–0.03 |

- exp(−C(|F|+m−1, m)/#E) predicts every m ≥ 2 cell within 0.05.
- So **"almost every instance is UNSAT" holds only for undersized bases.**
- UNSAT cost share: it equals the UNSAT fraction for rootfind oracles, and is 0.85–0.90 for
  the earlier SAT enumerator.

## (b, c) Filter arms
- **T1: every polynomial of degree ≤ 3 in the bits of x(R).** This covers every trace
  condition. The filter space is computed exactly. Its dimension equals the random-subset null
  in all 24 m ≥ 2 cells, and is 0 at n=19 (65,436 targets, 1,160 monomials). Both positive
  controls detect their filters: PC1 is the m=1 subspace condition; PC2 is the Kosters–Yeo
  trace filter on all of E. **NO-CHANGE.**
- **T2: Macaulay linearisation at D=2, 3, 4.**
  - m=3 split S3 chain: 0 of 694 UNSAT targets refuted (Wilson upper bound 0.020–0.041
    per cell). **NO-CHANGE.**
  - m=2: D=4 refutes 97–100% of UNSAT targets, except 0% at n=19, d=10. But each target costs
    3e3–7e7 word ops, against 1 probe for the rootfind oracle. The filter loses at 0.1 and
    1 GAE per word op. Break-even is 2.6e-8 to 1.3e-3 GAE per word op.
  - Formally INCONCLUSIVE (a trade). The lane's unmeasured assumption is that one word op is
    at least about 1e-3 GAE at n=131, which would put the filter on the losing side.
- **NC2 (contrarian control).** Against an XL/F4 oracle, the filter is the oracle's own
  degree-D elimination, so it saves 0. **NO-CHANGE**, by identity.

## What is new here
1. An exact, exhaustive test with zero false negatives of every filter of degree ≤ 3 on x(R),
   with positive and null controls. It turns the earlier statistical AUC≈0.5 result into an
   exact negative at toy n.
2. A per-cell UNSAT table and a predictor that checks out against it.
3. m=3 split-chain systems are not refuted by linearisation up to D=4.

## Changes to how the lane ran
- The Python prototype was deleted without being run; all measurements come from the Rust
  harness.
- The lane built into a private target directory.
- PROTOCOL Amendment 1 (appended): k=3 also run at n=19; n=13 with a=1 dropped (its subgroup
  has order 79).
- The binary was rebuilt once mid-run to add `analyze`; the measured paths did not change.
- No PDF: there is no LaTeX or typst engine here.

## Open threads
- Low-degree refutation not tested for the symmetrised S4 and Gaudry e-variable encodings.
- Degree-4 filters on targets not tested.
- The word-op to GAE price at n=131 is not measured; it decides the m=2 trade.
