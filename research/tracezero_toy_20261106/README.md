# Trace-zero factor-base toy test (2026-11-06)

Question: does the factor base F_V = { Tr(Q) : x(Q) ∈ V ⊂ F_{p^k} } ⊂ E(F_p) behave
differently from random points of E(F_p)? Protocol and amendments A1–A4: `PROTOCOL.md`.
Instrument: `tracezero.py`; analysis: `analyze.py`. All results [pcc:unverified]
(no claim registered, no replay certificate, no validator report, no matched rho).

## Result

- Scale: p ∈ {11,…,31}, k ∈ {2,3}, d ∈ {1,2}, 5,064 curve rows × 20 V-pairs each.
  Rows (`rows.jsonl`, over the 64 KiB limit) are not in Git. SHA-256, registered run:
  `8c571702…da69781`; symmetric-null run: `df50c1dd…1197a`. Both are in the scratchpad
  only, so they cannot be re-derived from Git except by rerunning (deterministic seeds).
- **Registered run** (`summary.json`): verdict "no structure detected" for cov_2 and cov_3.
  Positive control and A/A band passed (A/A half-width ≈ 0.003–0.004).
  Only the two d = 1 cells moved: Δcov_2 ≈ −0.027, Δ|S| ≈ −0.16 (trace below random).
- **Post hoc A4** (`summary_symnull.json`): that shortfall came mostly from the null, which
  was not sign-symmetric like the lifts (Q, −Q). With a symmetric null: k=3, d=1: Δcov_2 =
  +0.0005 [−0.0025, +0.0039]; k=2, d=2 and k=3, d=2 contain 0. k=2, d=1 keeps a small
  negative Δcov_2 = −0.0086 [−0.0125, −0.0050] (about 0.9 points of coverage). One cell of
  four, unadjusted for multiple looks, unexplained, not replicated at k=3.
- d = 2 cells are saturated in the full base (random cov_2 = 1.00 at p ≥ 17); only the
  3-point small-base arm carries information there.

## Reading

At these sizes the trace map on lifts looks equidistributed in E(F_p) apart from one small
k=2, d=1 residual. This gives no sign of structure to exploit, consistent with the
original expectation. It says nothing about asymptotics, cost, or a comparison with rho.

## Not done

No figure or PDF (matplotlib is not installed here), no benchmark record, no Conductor task
(not installed), no research-kb query. The k=2, d=1 residual is untested beyond this run.
