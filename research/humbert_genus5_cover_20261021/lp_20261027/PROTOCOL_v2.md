# Protocol v2: LA precheck, and single large prime against the fixed baseline (frozen 2026-10-27, before data)

Follows amendment 2 of `PROTOCOL.md`: in v1, every lp:0.5 win came from baseline
instances that needed several linear-algebra rounds.

## Arms (same instance per (q, seed); binary `bin/humbert-v2`, frozen before data, sha256 prefix 8251f2c37883cea8)

| tag | arm | env |
|:--|:--|:--|
| `v1` | `iota`, v1 retry policy (the incumbent) | — |
| `pre` | `iota` with the precheck: no Wiedemann solve while a column is empty | `HUMBERT_LA_PRECHECK=1` |
| `pre_aa` | `pre` again in a separate process (A/A noise floor) | `HUMBERT_LA_PRECHECK=1` |
| `lp50pre` | `lp:0.5` with the precheck | `HUMBERT_LA_PRECHECK=1` |

Grid: q ∈ {4001, 6007, 8009, 12007}, seeds 1–5, twist as in v1 (same instances as
v1). Arm order rotated per seed. Rho runs once per instance, in `v1`.

## Metrics

- M1 (baseline fix): t_ic(v1) / t_ic(pre) per instance; LA rounds and precheck
  extensions per arm.
- M2 (large primes vs fixed baseline): t_ic(pre) / t_ic(lp50pre) per instance.
- Noise floor: t_ic(pre) / t_ic(pre_aa).
- Correctness gates on every run as in v1. Every arm's IC / expected-steps rho.

## Predictions and falsification (fixed now)

- Q1: `pre` is faster than `v1` on every instance where v1 needed more than one
  LA round, and within the A/A band of v1 on one-round instances.
  **Falsified if** any multi-round instance is not faster, or any one-round
  instance is slower than the A/A band allows.
- Q2: against the fixed baseline, lp:0.5 is not faster at q ≤ 8009: median
  M2 < 1 at q = 4001, 6007 and 8009. **Falsified if** lp50pre is faster on at least
  4 of 5 seeds at any of these q.
- Q3 (exploratory, no decision): the M2 median at q = 12007.

## Limits stated in advance

Same host and route deviation as v1 (local 4-vCPU shared VM, L0; no isolab-eks).
Nothing else runs on the host during the campaign. Wall time only; §5a as in v1.
