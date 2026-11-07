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

## Replication of the k = 2, d = 1 residual (amendment A5)

`TZ_SALT=rep1 TZ_SYM=1`, d = 1, p ∈ {37,…,61}, 2,800 curve rows, independent seeds
(`summary_rep1.json`; rows SHA-256 `3c24c106…f88a298`, scratchpad only). Positive control
excluded 0 and A/A half-width was ≤ 0.003 in coverage.

- k = 2: pooled Δcov_2 = −0.0026 [−0.0039, −0.0012], Δcov_3 = −0.0083 [−0.0115, −0.0053].
  By the A5 rule this **replicates in sign**: the interval excludes 0 on the negative side.
- k = 3 (reference): Δcov_2 = −0.0002 [−0.0013, +0.0010]; contains 0, as in A4.
- Per prime the k = 2 effect shrinks as p grows: −0.0086 for p = 17–31 (A4), −0.0052 at
  p = 37, −0.0061 at 43, then −0.0005 to −0.0027 for p = 47–61 (most intervals contain 0).
  This is the pattern of a small-p effect that vanishes, not a stable bias. Only seven
  primes and one cell, so the trend is an observation and not a fit or a claim.
- No cost claim follows. A coverage deficit of 0.003 is far below anything usable.

## Explaining the k = 2, d = 1 residual (post hoc exploration)

Hypothesis, stated before the run: for k = 2 and d = 1, V = F_p·g is one of p + 1 lines of
F_{p²} over F_p, and two are degenerate. If g ∈ F_p ("rational"), Q and πQ have the same
x, so Tr(Q) is O or a doubling 2P. If g^p = −g ("pure"), x(πQ) = −x(Q). A uniform random g
lands on each with probability (p − 1)/(p² − 1) ≈ 1/(p + 1), which shrinks with p. For
k = 3 the chance that g ∈ F_p is about 1/p², matching no residual there.

`explore_line_classes.py` (output `explore_line_classes.out.txt`) enumerates all p + 1 lines
for 60 curves and 4 draws each, small-base cov_2 against the sign-symmetric null:

| p | generic Δcov_2 | pure | rational | weighted by 1/(p+1) exposure |
|---|---|---|---|---|
| 23 | +0.0015 | −0.061 | −0.138 | −0.0070 |
| 37 | −0.0026 | −0.039 | −0.051 | −0.0049 |
| 53 | −0.0014 | −0.009 | −0.029 | −0.0021 |

The generic lines (almost all of them) sit near 0. The two degenerate lines carry the
deficit, and the exposure-weighted mix (−0.0070, −0.0049, −0.0021) is the size and the
decay of the pooled residual (−0.0086 for p 17–31, −0.0052 at p = 37, −0.0005 to −0.0027
for p 47–61). So the residual is the fraction of random V that are degenerate, not a
property of the trace map on generic V. No confidence intervals were computed here (60
curves, one seed), and a small negative generic value at p = 37 and 53 is not excluded.
