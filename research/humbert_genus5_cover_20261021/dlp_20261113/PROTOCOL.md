# Protocol: double large primes on the genus-5 cover (frozen 2026-11-13, before data)

Lead: `LEAD-TRF-quartic-genus5-humbert-cover` (crypto-autoresearcher atlas). Follows
`../lp_20261027/` (PROTOCOL.md v1 and PROTOCOL_v2.md, PR #2710).

## Question

Does a double-large-prime arm (relations with at most two points outside the small
base, reduced by an incremental elimination over the large-prime columns) cut the cost
of index calculus on the descended genus-5 curve against the fixed baseline (iota +
precheck) and against single large primes, and does it lower the distance to matched rho?

## Arms (same instance per (q, seed); binary `bin/humbert-v3`, sha256 prefix a8ff53874ff869a9)

All arms run with `HUMBERT_LA_PRECHECK=1`.

| tag | arm |
|:--|:--|
| `pre` | `iota` (fixed baseline of PROTOCOL_v2) |
| `pre_aa` | `iota` again in a separate process (A/A noise floor) |
| `lp50` | `lp:0.5` (single large prime) |
| `dlp50` | `dlp:0.5` (double large prime) |
| `dlp35` | `dlp:0.35` |

Grid: q ∈ {4001, 8009, 12007, 16001}, seeds 1–5, twist +1 for odd seeds and −1 for even
(as before; the q ≤ 12007 instances are the PROTOCOL_v2 instances). Arm order rotated
per seed. Rho runs once per instance, in `pre`; every arm is compared with that
instance's expected-steps rho sqrt(pi r / 2) × measured s/step.

Before data: the refactored `iota` and `lp:0.5` arms reproduce recorded runs exactly
(iota q = 1009 seed 1 from PR #2580; lp:0.5 q = 2003 seed 1 from PR #2710), and the
crate's 6 tests pass. A dlp smoke run at q = 1009 and 4001 (seed 1, not part of the
grid) solved and passed the checks; it is not used in any analysis.

## Metrics

- Counted (host-independent): relation candidates per arm; pivots, dropped residuals,
  large primes with known logs.
- Wall time (exploratory, unisolated): t_ic per arm and phase; ratios on the same
  instance; IC / expected-steps rho.
- Correctness gates on every run: solved, pushforward 8P, T(P) ≠ 0, iota pairs
  constant over known points, X_oo = 4c.

## Predictions and falsification (fixed now)

- R1 (counted): dlp50 needs fewer relation candidates than lp50 on every instance.
  **Falsified if** any instance has dlp50 candidates ≥ lp50 candidates.
- R2 (wall time): dlp50 is faster than pre on at least 4 of 5 seeds at each of
  q = 8009, 12007, 16001. **Falsified if** fewer than 4 of 5 at any of them.
- R3: the median IC / expected-steps-rho ratio of dlp50 is below that of pre at each
  q ≥ 8009. **Falsified if** not below at any of them.
- R4 (exploratory, no decision): the log–log slope of median t_ic over q = 4001 → 16001
  for dlp50 against pre.

## Limits stated in advance

Local 4-vCPU shared VM, no isolation (L0): every wall-time number is exploratory
(AGENTS.md requires isolated execution for wall-clock evidence; isolab-eks is not
reachable from this session). R1 is a counted metric and does not depend on the host.
§5a as before: IC does no group operations, arms are compared on paired wall time only.
Nothing else runs on the host during the campaign.
