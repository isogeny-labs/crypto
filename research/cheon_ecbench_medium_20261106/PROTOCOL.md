# Protocol: Cheon ecbench sessions on one tier (medium)

Frozen before any session of this study runs. Amendments are appended with a
date and never edit this text.

## 1. Question

Can the Cheon arms of [`research/cheon_ecbench_20261022/`](../cheon_ecbench_20261022/)
be ranked by frontier procedure 2 (`docs/bounds/README.md`)?

Procedure 2 ranks only fit-version-2 bounds fitted on a whole session. Each
original session spans the 32-bit curve, which is toy tier, and medium-tier
curves of 36 to 56 bits. `ecbench bound fit` refuses a fit that mixes tiers, and
procedure 2 never ranks a `--tier` fit. So no Cheon bound from those sessions
can be ranked. This study re-runs the medium-tier curves as sessions of their
own.

This is a re-measurement of the same workloads for ranking. It tests no new
method and makes no plain-ECDLP or index-calculus claim.

## 2. Design

Three specs:
- `spec-minus.json`
- `spec-minus-naive.json`
- `spec-plus.json`

Each equals the original `research/cheon_ecbench_20261022/spec-*.json` with one
change: its 32-bit curve (`r` ≈ 2^31.6) is removed. The label and description
also differ. Everything else is byte-identical in content:
- the arms (`rho-neg` = `rho.negation`, against `cheon-comb` or `cheon-naive`);
- the `d` shape parameters;
- `targets_per_curve` 8 and `target_seed` 20261022;
- the measurement block (1 round, 0 warmup, alternate order, seed 1, L0, timeout 1800 s).

| session | curves (log₂ r) | arms | runs |
|:--|:--|:--|--:|
| minus | 35.6, 39.6, 43.6, 47.6, 51.6, 55.6 | `rho.negation`, `cheon.pminus1_comb` | 96 |
| minus-naive | 35.6, 39.6, 43.6, 47.6, 51.6 | `rho.negation`, `cheon.pminus1_naive` | 80 |
| plus | 35.6, 39.6, 43.6, 47.6, 51.6, 55.6 | `rho.negation`, `cheon.pplus1_comb` | 96 |

**Running.**
- Each session runs with `ecbench run --spec <spec> --out sessions/<name>-20261106`
  at L0, counting operations only, on this shared 4-core host. No wall time is
  admitted.
- Each session is then audited with `ecbench verify --dir <session>
  --replay-all --out sessions/<name>-20261106.audit.json`.

## 3. Analysis (fixed now)

**Fits.** For every session, arm and weight w ∈ {0, 0.1, 1}:
- run `ecbench bound fit` over the whole session, with no `--tier` and no
  `--curve`, at fit version 2;
- use the same `--oracle` label as the original record of that arm. The labels
  are `static_dh:n-1:d=shape(0.12943,0.015)` for minus and minus-naive, and
  `static_dh:n+1:d=shape(0.21106,0.015)` for plus;
- pass `--weight w` for w > 0.

This gives 18 records, `docs/bounds/records/cheon-ecbench-medium-<session>-<arm>[-w<w>].json`.

**Frontier.** Rebuild with `ecbench frontier build --bounds docs/bounds/records
--out docs/bounds/frontier.json --markdown docs/bounds/FRONTIER.md`. The
ranking is read from that page as procedure 2 writes it.

## 4. Predictions and decision rules

- **P1 (reproduction).** Every run's counts equal the original session's run
  with the same arm, curve and target index. The counts compared are
  `outcome`, `cost.total_gae`, the per-phase counts and
  `counters.oracle_queries`. A mismatch is reported per run and does not stop
  the study.
- **P2 (fit).** All 18 fits are accepted, with no tier refusal and no
  admissibility reason. Procedure 2 lists each of them as ranked, not under
  "listed, not ranked".
- **P3 (frontier, n−1 and n+1 comb domains).** `rho.negation` and the comb arm
  are both on the frontier at every weight: comb has fewer operations, and rho
  queries no oracle. This is the procedure-1 reading of the original report.
- **P4 (naive).** `cheon.pminus1_naive` is dominated by `rho.negation` at every
  weight.

Each prediction is reported as met or not met, with the frontier entries that
decide it. No result is withheld.

## 5. Outputs

- `sessions/`: the three sessions and their audit receipts.
- `REPORT.md`: the outcome of P1–P4 and the ranked frontier rows.
- `report.pdf`.
- `ranks.svg`: one graph of `S` per arm with intervals, by session and weight.
- The 18 records, and the rebuilt `docs/bounds/frontier.json` and `FRONTIER.md`.
