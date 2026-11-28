# Protocol: two round-2 leads on Table B (L2-STACK, L2-FEWCOL), 2026-11-18

This protocol was frozen before any measured row; the 4-target P20 smoke run checked only that the arms run. Every statement here is `[pcc:unverified]`. The leads are in `docs/leads/2026-11-18/ic-leads-round2/PROPOSALS.md`. Neither needs a code change: both arms are existing plugin parameters, run with the `ecbench` binary built at the commit that freezes this file.

## Arms and workloads
Specs: `specs/P20.json`, `specs/P22.json` and `specs/P24.json`. They use the three Table B curves, with the same 48 public targets per curve and the same target seeds as `research/ic_ksum_sibling_20261118`.

| arm | role | factor base | oracle |
|---|---|---|---|
| `rho-neg` | reference | — | `rho.negation` |
| `ksum5-sib` | baseline (Table B incumbent after #4649) | `prime-abscissa:size=9/10/11` | `ksum:m=5,sib=1` |
| `ksum5-ps` | candidate, L2-STACK | same | `ksum:m=5,prune=1,sib=1` |
| `tw2l` | candidate, L2-FEWCOL | `prime-projected-abscissa:size=2,scan_cap=32` | `wwalk:x=2.0,jumps=16,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run` |
| `tw2l-aa` | A/A control | same as `tw2l` | same as `tw2l` |

`tw2l` is the Curve25519-autolab incumbent from round 39 (`research/curve25519_autolab_20261104/README.md`). The owner ruled on 2026-11-04 that the C = 2 seed arms with known-log G jumps count as index calculus. Because `seed=run` builds a fresh table per run, `tw2l-aa` gives a genuine noise floor, unlike the deterministic k-sum A/A.

The run uses 2 rounds, 1 warm-up and alternate order. It is L0: one local 4-core container, not isolated. Cost is counted, so host noise does not enter S.

## Predictions (registered)
- **S-a (identity):** every run of every arm verifies ([d]G = Q). Per target, `ksum5-ps` makes the same trials and relations as `ksum5-sib`; prune changes only the build.
- **S-b (STACK):** the paired S ratio `ksum5-ps / ksum5-sib` is below 1 at w = 0, 0.1 and 1 at every size, with the upper 95% end below 1. Point prediction about 0.90–0.95 at w = 0, because the build is about 40% of S and prune removes about 19% of build additions at P24.
- **F-b (FEWCOL):** the paired S ratio `tw2l / ksum5-sib` is at most 0.60 at w = 0 and at most 0.55 at w = 1 at every size. Point prediction: S(tw2l, w = 0) of 2.0–3.4, so a distance to `rho-neg` of 1.7–2.4×.
- **A/A:** the `tw2l-aa / tw2l` 95% interval contains 1 at every size.

## Decision
- **STACK** is an improvement over the Table B incumbent if S-a holds and S-b holds at all three sizes and all three w (`ecbench band` order `ksum5-ps < ksum5-sib`).
- **FEWCOL** is an improvement if every run verifies, the A/A holds, and the paired upper 95% end of `tw2l / ksum5-sib` is below 1 at w = 0, 0.1 and 1 at all three sizes. Missing the 0.60 / 0.55 point prediction while staying below 1 counts as an improvement with a falsified magnitude.
- **Kill:** any unverified run, an S-a mismatch, or a ratio of 1 or more at any w for that arm.
- Distance to `rho-neg` is reported as the distance remaining, never as pass or fail.
- No leaderboard or `benchmarks/README.md` row without the validation gate's replay certificate and validator report.
