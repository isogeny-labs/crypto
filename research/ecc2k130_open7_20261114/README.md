# ECC2K-130 open-items sweep (2026-11-14)

The owner listed seven items they believed were open for index calculus on binary Koblitz
curves of the ECC2K-130 type, then asked for all of them to run in parallel. Each item went to
one lane (L1–L7). Every lane first audited the prior work in `crypto` and `cryptanalysis`,
then either measured the question still left open or declared it closed by prior work
(CLOSED-BY-PRIOR).

- Shared rules: `COMMON.md`, including Amendment 1 (no Python; AGENTS.md).
- Toy scale only, on a local machine.
- Counted operations decide every verdict. Seven lanes shared 4 vCPU, so wall time was
  confounded and is not used.
- Every claim is `[pcc:unverified]`: there are no signed claims, no independent replay, and
  no EKS run.
- research-kb and the lead atlas were unavailable, so no lane makes a novelty claim.

## Verdicts

| Lane | Item | Verdict | Finding |
|---|---|---|---|
| L1 | Frobenius- and negation-closed non-linear factor base | CLOSED-BY-PRIOR | Already built (`compact-orbit-scan`, the Hamming-weight base `onb-weight`, goal22 `decomp.py`) and measured end to end. It saves a constant factor (2n fewer columns, the same yield per point) and stays 1.6–18× above signed-Frobenius rho. The gap grows with r. Under SAT or Gröbner oracles it costs more per trial than a subspace base. |
| L2 | UNSAT dominates the workload; cheap pre-solve filters | NO-CHANGE (measured) | Exact and exhaustive: no polynomial of degree ≤ 3 in the bits of x(R) separates decomposable from non-decomposable targets better than the random-subset null (24 cells, with positive controls). Linearisation refutes 0 of 694 m=3 UNSAT targets up to degree D=4. At m=2, D=4 refutes them but costs 3e3–7e7 word ops against a 1-probe oracle. The UNSAT fraction is 0.75–0.92 only for undersized bases, and 0.001–0.03 at d = ⌈n/m⌉+1. |
| L3 | Cross-instance amortisation (incremental SAT) | *pending* | |
| L4 | Missing baselines | (a) NEGATIVE (measured); (b), (c) CLOSED-BY-PRIOR | (a) The gcd of the summation polynomial with the subspace polynomial L_V: solution sets agree with all engines on 136 targets. It costs 7–14× (m=2) and 300–1500× (m=3) more than the pair-table oracle the baseline rows use. Against `inherited-f4` it is cheaper at m=3 and tied in exponent at m=2, with the order flipping inside the price band: a stage-level constant only. (b) FES was already compared head to head (`inherited_f4_next_20261112`, `fes_then_batched_20261112`). (c) Dinur and LPTWY were implemented with a counted model (`blindspots_20261018/03-polynomial-method`); the crossover against FES is at N ≥ 80. |
| L5 | Function-field Coppersmith on S3 | *pending* | |
| L6 | Gröbner-free splitting / dissection | CLOSED-BY-PRIOR | The m=4 2+2 pair-table MITM with the table shared across targets is the incumbent (`ic_m4_mitm_scaling_20261031`: planted exponent 0.41; on uniform targets B⁴/p rarity dominates). The table is already stored per ⟨σ,−1⟩ orbit. Dissection would cut memory only, not operations. Further table engineering is ruled out by Dinur–Keller–Marmor (`ic_leads_review_20261018` §2.5). |
| L7 | End-to-end units against rho | CLOSED-BY-PRIOR (method); coverage gaps | 147 complete rows in one table against matched rho. **Every prime-degree Koblitz IC arm moves away from rho as r grows**: +0.05 to +0.15 log2(IC/rho) per bit of r, at most 4% of draws crossing by log2 r = 129. Relation search is 50–92% of S, table build 5–45%, LA ≤ 1.2%. 2LP is never better across the band. |

## What the sweep changes in the owner's picture

- **Items 1, 4b, 4c, 6 and 7 were already in this repository**, built and measured. The
  owner's notes underestimate the lab's own prior work, so the open list needs pruning.
- **Item 2's premise holds only for undersized factor bases.** At balanced or larger bases
  most targets decompose. Exact filters on x(R) of degree ≤ 3 do not exist beyond the
  trace/2E filter, which ⟨G⟩ already satisfies.
- **The binding constraint is the scaling of relation search, not any phase constant.**
  L7's phase shares and slopes say a win must lower cost per trial or trials per relation by
  a factor that grows with r. Constant-factor wins (L1, L4a, dissection) cannot close a gap
  that widens by 0.05–0.15 bits per bit of r.

## Open threads collected from the lanes
- A validated Koblitz row in counted GAE with a real §5a band. The validated LB-A rows are
  timed.
- A complete m=83 counted IC row against matched rho (§8a gate).
- A GF(2^n) w_cal instrument (`ecbench wcal` covers prime fields only). It would also settle
  L2's m=2 word-op trade and L4's price band.
- Porting L4's gcd arm into `ic_framework`, so it shares one tally with `inherited-f4`.
- Low-degree refutation tests for the symmetrised S4 and Gaudry e-variable encodings (L2).
- A size-matched SAT comparison of the weight base against a random subspace at m ≥ 3 (L1).
  It is not expected to change the verdict.
