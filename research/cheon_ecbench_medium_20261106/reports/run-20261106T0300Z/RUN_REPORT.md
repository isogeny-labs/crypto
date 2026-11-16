# Run report: Cheon ecbench medium-tier sessions, 2026-11-06T03:00Z

Descriptive only; the interpretation is in [`../../REPORT.md`](../../REPORT.md).

## What ran

Three specs, run in turn with `ecbench run --spec <spec> --out sessions/<name>-20261106 --allow-busy --quiet`
at isolation level L0, recording operation counts only. The host is a shared
cloud VM with 4 vCPUs, and no wall time is admitted.

| session | spec id | session id | arms | curves (log₂ r) | started | finished |
|:--|:--|:--|:--|:--|:--|:--|
| minus | `ECS1h363c3f2358e9` | `ECBS1he3401ce241ca` | `rho.negation`, `cheon.pminus1_comb` | 35.6 to 55.6 (6) | 03:00:44Z | 03:13:41Z |
| plus | `ECS1h925c4cd120b9` | `ECBS1h64874297871c` | `rho.negation`, `cheon.pplus1_comb` | 35.6 to 55.6 (6) | 03:13:41Z | 03:31:31Z |
| minus-naive | `ECS1h2934f0735cb6` | `ECBS1hb3aa04e7a9c6` | `rho.negation`, `cheon.pminus1_naive` | 35.6 to 51.6 (5) | 03:31:31Z | 03:53:01Z |

- **Specs.** `../../spec-*.json`. Each is
  `research/cheon_ecbench_20261022/spec-*.json` without its 32-bit curve; the
  arms, shapes, targets (8 per curve, seed 20261022) and measurement block are
  unchanged.
- **Binary.** sha256 `f561cc6c…`. It was built from the sources of
  `ac79a4b04f`, which equal those of `b83ccff8ce`, the clean commit checked
  out at run time. The recorded `git_commit` is stale; see
  [`../../sessions/BINARY.txt`](../../sessions/BINARY.txt).

## Completion

| session | runs requested | completed | verified | failures / timeouts |
|:--|--:|--:|--:|:--|
| minus | 96 | 96 | 96 | none |
| plus | 96 | 96 | 96 | none |
| minus-naive | 80 | 80 | 80 | none |

## Audit

Each session was replayed in full with `ecbench verify --dir <session> --replay-all`.

| receipt | runs reproduced | problems | sha256 |
|:--|--:|--:|:--|
| `sessions/minus-20261106.audit.json` | 96 / 96 | 0 | `91b8644598434800d4c5205a6f1df052a58262608ee8afde384c3fefc894a287` |
| `sessions/plus-20261106.audit.json` | 96 / 96 | 0 | `59baa234ed6a2f9d012aaf21f40e1f6c630c36e4f4d8508fb56352277e4862b0` |
| `sessions/minus-naive-20261106.audit.json` | 80 / 80 | 0 | `d76b80e728e415725698b32c80b4dee1100727eba080a62d922483a2eb6027e4` |

## Against the original sessions

[`../../compare.sh`](../../compare.sh) compares every run with the original
session's run of the same arm, curve and target index. The output is
[`../../results-p1.jsonl`](../../results-p1.jsonl). The fields compared are:
- outcome, recovered scalar and `matches_planted`;
- `total_gae` and `S`;
- oracle queries and oracle server work;
- per-phase adds, doubles, scalar multiplications and GAE.

| session | runs | identical | absent from the original | differing |
|:--|--:|--:|--:|--:|
| minus | 96 | 96 | 0 | 0 |
| minus-naive | 80 | 80 | 0 | 0 |
| plus | 96 | 96 | 0 | 0 |

## Measured values

| arm (session) | runs | S (mean, from the bound) | 95 % |
|:--|--:|--:|:--|
| `cheon.pminus1_comb` (minus) | 48 | 0.574 | [0.449, 0.722] |
| `rho.negation` (minus) | 48 | 0.987 | [0.744, 1.231] |
| `cheon.pminus1_naive` (minus-naive) | 40 | 12.243 | [9.382, 14.909] |
| `rho.negation` (minus-naive) | 40 | 1.055 | [0.802, 1.336] |
| `cheon.pplus1_comb` (plus) | 48 | 0.582 | [0.417, 0.772] |
| `rho.negation` (plus) | 48 | 1.013 | [0.758, 1.312] |

`S` is the w = 0 constant of each whole-session fit-version-2 bound, from
`docs/bounds/records/cheon-ecbench-medium-*.json` (fitted by
[`../../fit.sh`](../../fit.sh)). The graph is [`../../ranks.svg`](../../ranks.svg).
Raw records are in `../../sessions/<session>/records.jsonl`.
