# Protocol: filtered double-large-prime matrices on the genus-5 cover (frozen 2026-11-15, before data)

Lead: `LEAD-TRF-quartic-genus5-humbert-cover`. Follows `../dlp_20261113/` (PR #4065), which
found that full elimination of the large primes halves the relation candidates against
single large primes, but makes the reduced rows about twice as heavy. At equal unknowns it
does 2.1× the linear-algebra mulmods, so its lead over `lp:0.5` falls from 1.71× to 0.97×
between q = 4001 and 16001.

## Question

Holding the relation set fixed, does building the matrix differently remove the fill-in
cost of double large primes? The `dlp:0.5` collection is kept, and only the matrix handed
to Wiedemann changes. Does that make double large primes beat single large primes at the
top of the grid?

## Arms (same instance per (q, seed); one frozen binary, sha256 prefix recorded in `BINARY.sha256`)

All arms run with `HUMBERT_LA_PRECHECK=1`.

| tag | arm | matrix |
|:--|:--|:--|
| `pre` | `iota` | full base, no large primes (baseline of PROTOCOL_v2); rho runs here |
| `lp50` | `lp:0.5` | single large primes, pairs combined |
| `dlp50` | `dlp:0.5` | double large primes, full incremental elimination (as in `dlp_20261113`) |
| `dlpm50` | `dlpm:0.5` | same collection and stopping rule as `dlp50`; raw rows with the large primes as extra columns, singletons removed (no merges) |
| `dlpf50` | `dlpf:0.5` | as `dlpm50`, then greedy merges: a large prime of degree 2–12 is eliminated through its lightest row only while (columns − 1) × (nnz + Δ) < columns × nnz |
| `dlpf50_aa` | `dlpf:0.5` again, own process | A/A |

The rows `dlpm50` and `dlpf50` collect are exactly `dlp50`'s. The reducer still runs, with
its time recorded as `t_reduce`, and it decides when to stop. The raw rows (every fresh
relation with at most two large primes) are kept as well. Large-prime logs come from the
solution, for large primes that stay columns, or by back substitution over the pruned and
pivot rows. Wiedemann's counted mulmods are 2·nnz per matrix–vector product over about 3N
iterations, so they are proportional to N × nnz, which is the quantity the merge rule
estimates.

Grid: q ∈ {8009, 12007, 16001, 24001}, seeds 1–5, twist +1 for odd seeds and −1 for even.
The q ≤ 16001 instances are those of `dlp_20261113`. Arm order is rotated per seed
(`(seed + k) % 6`). q = 4001 is left out because its matrices are below the dense threshold
(1200 columns), so Wiedemann is not used there.

Before data:
- `dlp:0.5` at q = 8009, seed 1 reproduces the recorded `dlp_20261113` run exactly (all
  counted fields).
- The crate's 7 tests pass, including a planted-solution test of the filter.
- A smoke run of `dlpm:0.5` and `dlpf:0.5` on that instance solved and passed every check.
  It is not used in any analysis.

## Metrics

- Counted (host-independent): relation candidates; LA columns, rows and nnz; LA mulmods;
  pruned rows and merges; large primes with known logs.
- Wall time (exploratory, unisolated L0): t_ic and its phases; paired ratios on the same
  instance; IC / expected-steps rho.
- Correctness gates on every run: solved, pushforward 8P, T(P) ≠ 0, iota pairs constant,
  X_oo = 4c.

## Predictions and falsification (fixed now)

- C0 (control, counted):
  - Every `pre`, `lp50` and `dlp50` run at q ≤ 16001 reproduces the counted fields of its
    `dlp_20261113` record (rel_candidates, la_mulmods, x_recovered).
  - On every instance, `dlpm50`, `dlpf50` and `dlpf50_aa` have the same rel_candidates as
    `dlp50`.
  - On every instance, `dlpf50_aa` has the same counted fields as `dlpf50`.
  - **Failure** of any part invalidates the comparisons it supports.
- F1 (counted): dlpf50 LA mulmods < dlp50 LA mulmods on every instance. **Falsified if** any
  instance has ≥.
- F2 (counted): dlpf50 LA mulmods < dlpm50 LA mulmods on every instance. **Falsified if**
  any instance has ≥.
- F3 (wall time): dlpf50 t_ic < lp50 t_ic on at least 4 of 5 seeds at each of q = 12007,
  16001, 24001. **Falsified if** fewer than 4 of 5 at any of them.
- F4 (wall time): the median IC / expected-steps-rho of dlpf50 is below the lowest median
  among pre, lp50 and dlp50 at each q ≥ 12007. **Falsified if** not below at any of them.
- F5 (exploratory, no decision): the log–log slope of median t_ic over q = 8009 → 24001 for
  every arm.

## Limits stated in advance

- Local 4-vCPU shared VM, no isolation (L0). Every wall-time number is exploratory and an
  unverified assertion; isolab-eks is not reachable from this session (no AWS identity).
- F1, F2 and C0 are counted and do not depend on the host.
- §5a: index calculus does no group operations, so arms are compared on paired wall time
  only.
- Nothing else runs on the host during the campaign.
- `dlpm50` and `dlpf50` pay for the reducer they do not use (reported as `t_reduce`), which
  biases their wall time against them.
- The merge rule is greedy and uses one fixed degree cap (12). It is not tuned on this grid.
