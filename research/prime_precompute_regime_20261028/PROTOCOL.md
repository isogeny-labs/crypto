# Prime-field precomputation regime: known-log k-sum table against preprocessing rho (pre-registration, 2026-10-28)

**Status.** Registered before any timed run. Public synthetic curves; no
security claim. **Class: control / accounting.** Nothing here can count as an
improvement under the success criterion. The study measures which
precomputation trade a prime-field table method sits on and what a fair
generic rival costs. Stacked on `research/prime_ksum_table_20261028/`
(isogeny-labs/crypto#2808). Leads 2 and 3 of the 2026-10-28 prime-field list:

- **lead 2:** compare at equal memory;
- **lead 3:** share the table across targets.

## Question

The k-sum table (`ksum:m=4`) is built before any target arrives. Two things
are therefore unmeasured:

1. **The fair rival.** At the same table size, the right comparison is
   preprocessing rho (Bernstein–Lange 2013), not plain rho. At the same
   precomputation work, it is preprocessing rho with a smaller table.
2. **Many targets.** Once the base logarithms are known, each further target
   costs one decomposition, not a whole index calculus. What is that online
   cost, and after how many targets does the table pay for itself?

## Arms (new methods, this PR)

- **`ic.known_log_ksum_v1`** (`kl-ksum4`). The known-log base of
  `ic.known_log_mitm2_v1` (seeded scalars `s_i`, points `[s_i]G`, both signs,
  every log known and charged as a scalar multiplication), with the k = 4
  sum table in place of the pair table.
  - Online: query `Q + [a]G`, `a` advancing by one, one probe per query.
  - A hit `Q + [a]G = Σ ±[s_i]G` gives the log directly. No matrix.
  - Target-blind: the base and the table.
  - Online: the queries, `k − 1` additions per witness check, and verification.
- **`rho.preprocessing`** (`pre-eqS`, `pre-eqP`). Bernstein–Lange on
  `{P, −P}` with the tuned walker of `rho.negation` and jumps in `G` only.
  - Precomputation: walks from `[k·t]G` until `entries` distinct
    distinguished points are stored.
  - Online: walks from `[k]([t_a]G + [t_b]Q)` until one ends at a stored
    point.
  - One walker and one jump table serve both phases, escapes included, so
    they are the same map. This is checked by
    `preprocessing_rho_table_is_target_blind_and_online_walks_are_short`.
  - Walk length `2^d`, `d = round(log₂ √(N/S))`, `N = r/2`.
- Kept for reference: `rho-neg` (reference), `rho-neg-aa` (A/A control),
  `ic-ksum4` (the cold IC row of #2808, frozen `A`).

`kl-ksum4` uses `size = A`, the frozen #2808 base size: 30, 44, 64 at 32,
36, 40 bits. That gives a table of about `E = 2³·C(A,4)` entries:
219,240, 1,086,008 and 5,083,008.

- **`pre-eqS`** (equal memory): `entries = E`.
- **`pre-eqP`** (equal precomputation): `entries = S_P = round(P² / (r/2))`.
  `P` is `kl-ksum4`'s target-blind GAE (factor base plus table), measured on
  the train panel. Being target-blind, it is the same for every target. `S_P`
  is frozen in `params.json` before the test panel.

## Measurements

All are native `ecbench` phase ledgers.

- `P`: target-blind GAE.
  - `kl-ksum4`: `factor_base` + `oracle_setup`.
  - `pre-*`: `setup` + `precompute`.
- `T`: online GAE.
  - `kl-ksum4`: the `target_*` phases.
  - `pre-*`: `search` + `internal_verification`.
- `E`: entries kept (`ksum_distinct_signed_classes`, `table_entries`).

Derived, each from those three numbers per arm:

| quantity | formula |
|:--|:--|
| online `S` | `T/√r` |
| set-up `S` | `P/√r` |
| break-even against plain rho | `L* = P / (T_rho − T)` targets |
| amortised `S` at `L` targets | `P/(L√r) + T/√r` |
| crossover between two arms | `L` where their amortised costs meet |
| precomputation–time product | `P·T/N` |
| advice product | `E·T²/N` |

Every derived number is reported at `w = 0, 0.1, 1`, pricing the unpriced
lookups. `kl-ksum4` makes one lookup per query; preprocessing rho makes one
per distinguished point.

## Predictions (derived from the cost laws)

The table assumes `T_rho/√r` = 0.79, 0.85, 0.88, as measured in #2808.

| bits | `kl-ksum4` set-up `S` | `kl-ksum4` online `S` | `pre-eqS` set-up `S` | `pre-eqS` online `S` | `pre-eqP` `S_P`, online `S` | `L*` kl | `L*` pre-eqS | kl/pre-eqS crossover `L` |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 32 | 3.7 | 0.15 | 331 | 0.003 | 27, 0.27 | 5 | 376 | 2.2 k |
| 36 | 4.6 | 0.12 | 737 | 0.0014 | 42, 0.22 | 6 | 837 | 6.1 k |
| 40 | 5.3 | 0.10 | 1,594 | 0.0006 | 57, 0.19 | 7 | 1,812 | 15 k |

In words:

- **Equal memory.** Preprocessing rho's online walk is about 50–160× cheaper
  than the known-log table's queries. Its table costs about 90–300× more to
  build.
- **Equal precomputation.** The known-log table's online cost is about half
  of preprocessing rho's at `w = 0`. Both sit near `P·T ≈ N`: `kl` at about
  1, preprocessing rho at about 2.
- **Advice.** `kl` sits far above `E·T² ≈ N` (by `N/E`); preprocessing rho
  sits on it.

## Falsifiers and registered readings

1. **Equal memory, online.** `pre-eqS` online `S` below `kl-ksum4`'s by at
   least 10× at 40 bits, at every `w`. If not, the advice comparison is
   wrong.
2. **Products.** `P·T/N` in `[0.5, 2]` for `kl-ksum4` and in `[1.5, 6]` for
   `pre-eqS` and `pre-eqP` at 40 bits. If not, the stated law is wrong for
   that arm.
3. **Equal precomputation.** "`kl-ksum4` online `S` ≤ `pre-eqP`'s" is
   reported as a ranking only where it holds across the whole band (§5a).
   The prediction is that it holds at `w = 0` and may flip at `w = 1`,
   because each query's lookup is priced there.
4. **Break-even.** `L*` for `kl-ksum4` within 2× of the prediction at 40
   bits.

None of these readings is an improvement. The headline must say that
multi-target and precomputation numbers are accounting, not a cheaper
single-target attack.

## Selection (train panel)

Target seed `20261015100`, 4 targets, one round, arms `rho-neg` and
`kl-ksum4` at 32, 36 and 40 bits. Read `P` from `kl-ksum4`'s target-blind
phases and freeze `S_P` in `params.json`. Nothing else is tuned.

## Test panel

The frontier's 32-, 36- and 40-bit `a = −3` curves, target seed
`20261015200`:

- 32 and 36 bits: 16 targets × 2 rounds;
- 40 bits: 16 targets × 1 round, because `pre-eqS` precomputes about
  1.7·10⁹ operations per run there.

Warm-up 1, alternate order. Arms: `rho-neg`, `rho-neg-aa`, `ic-ksum4`,
`kl-ksum4`, `pre-eqS`, `pre-eqP`. Every answer is verified;
`verify --replay-all` runs on every session; `band` at `w = 0, 0.1, 1`.
The derived table comes from [`derive.mjs`](derive.mjs), reading only the
native records.
