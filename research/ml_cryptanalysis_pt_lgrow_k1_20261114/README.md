# K1 pair-table factor base sized for n: frozen evidence from ml-cryptanalysis

This directory is a frozen copy of a validated ecbench experiment from the
sibling repository, so that the IC leaderboard (which reads only this
repository) can show the two rows it supports.

| | |
|:--|:--|
| Source repository | `github.com/aburan28/ml-cryptanalysis` |
| Source path | `experiments/fb-pt-lgrow-k1` (plus the four sibling files its validator report reads) |
| Source commit | `7d4e8e0fccb43148c80fd2004821fd7e48f9df76` (main; merge of ml-cryptanalysis PR #310), with the validator files replaced by their redacted versions from `b504b611` (ml-cryptanalysis PR #314) |
| Validation | `VAL-20261114-d9190c`, `fb-pt-lgrow-k1/VALIDATION.json` |
| ecbench binary | sha256 `a0be1b2a3a02f5385c9b9654622b01923f3f54a52904bd157861e0cbe8796d29` |
| Isolation | L0, counted cost only |

## Layout

The copy keeps the source's relative layout, so every path in
`VALIDATION.json` and in the validator report's `raw_data` resolves unchanged:

- `fb-pt-lgrow-k1/`: the experiment, byte-identical to the source commit
  (protocol, specs, sizing sessions, the eight measured sessions, `results.json`,
  `report.md`, figure, `VALIDATION.json`, `validation/`). This is the evidence
  directory the validation gate reads.
- `fb-pt-l7-k1-n23/scripts/score.py`, `fb-oracle-compare-k1-eks/runpod/run2.sh`,
  `fb-cost-model-k1-n23-eks/eks/sessions/*/session/records.jsonl`: the files the
  validator report lists as `../…` raw data.
- The 16 session logs `fb-pt-lgrow-k1/sessions/*/s-*.{run,verify}.log` are
  git-ignored in the source repository, so they are not in its commit. They
  were copied from the source worktree; each matches the sha256 pinned in its
  session's committed `SHA256SUMS` and in the validator report.
- `summarize.py` and `summary.json`: written here. `summary.json` is what the
  leaderboard builder reads (`SOURCES["k1_pt_lgrow"]`); it is recomputed from the
  session records with the AGENTS.md §5a band, and the builder cross-checks it
  against `VALIDATION.json` and the curve registry. `python3 -I summarize.py --check`
  fails when it is stale.

Check: `python3 scripts/validation_gate.py evidence research/ml_cryptanalysis_pt_lgrow_k1_20261114/fb-pt-lgrow-k1`.

## What it supports

Two one-target, cold rows in ecbench counted GAE, each against strong
signed-Frobenius rho on the same 8 targets × 5 rounds in the same session, with
fixed pair-table dimension 7 on the same targets as the before mark:

| curve | n | r | recipe | gap to rho, bits (w = 0 / 0.1 / 1) | before (dimension 7) |
|:--|--:|:--|:--|:--|:--|
| `icv1-f2m23-tm5197-1f85e9e1` | 23 | 2^22.00 | `koblitz-standard-subspace:dimension=9`, `pdp3-koblitz:m=3,engine=rootfind-negfold` | 5.59 / 5.66 / 5.95 | 6.91 / 6.94 / 7.09 |
| `icv1-f2m29-tm40309-30c52b96` | 29 | 2^15.37 (cofactor 12646) | `koblitz-standard-subspace:dimension=10`, same oracle | 9.47 / 9.52 / 9.86 | 10.36 / 10.46 / 11.06 |

Status of these numbers: recomputed here from the copied records; the claim is
recorded as `PCC-20261114-8ba038` and is unverified until CI signs a verdict.

Caveats carried from the validator report:

- Rho is cheaper than both rows at every weight; the gaps are the distance remaining.
- The gain over dimension 7 is an engineering improvement (parameter re-selection) at fixed n.
- The slope statistic D in the source report is `(R_n23 + 7 R_n29) / 84` by construction, an offset rather than a bent slope. It is not a row.
- The leaderboard's existing K1 n = 23 figure (table C) is a 32-target batch, pre-gate figure in another unit; these rows are not ranked against it.
- K1 n = 29 has r = 42457 = 2^15.37, smaller than K1 n = 17's subgroup, so n is the field degree here, not a problem size.

## Redaction

`fb-pt-lgrow-k1/VALIDATION.json`, `validation/VALIDATOR.md` and `validation/validator-report.json` come from ml-cryptanalysis `b504b611` (PR #314). That commit replaced the validator's model id with a redaction note and re-pinned the sha256 values; `validation/REDACTION.md` lists the old and new hashes. No number, check or verdict changed.
