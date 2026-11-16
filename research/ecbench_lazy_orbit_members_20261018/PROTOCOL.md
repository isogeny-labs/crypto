# Protocol: lazy orbit-member index in the Frobenius-folded pair table (2026-10-18)

Frozen before any claim data. Class: **engineering** (a constant moves; the
algorithm, its relations and its exponent do not).

## 1. Question

`FrobeniusPairTable::build` (the table behind oracle `mitm-frobenius-counted`)
fills `member_of[i·n + t]`, the base index of `σ^t(P_i)`, for every base point
and every rotation: `|F|(n−1)` Frobenius maps and as many hash lookups, charged
as `oracle_setup`. Only a probe that *hits* reads two entries of it. On the
development curves a hit is 1–2 % of the probes (4 relations in ~200–300
trials), so almost all of that set-up is dead work. At n = 31, `c = 3` columns
it is 5 580 maps and 5 580 lookups ≈ 530 of 1 853 GAE (29 %).

## 2. Change under test

New oracle id `mitm-frobenius-lazy-counted` (`FrobeniusMitmOracle::new_lazy_counted`,
`FrobeniusPairTable::build_lazy`): the same table entries, keys and probe, with
no member index; a hit names its two summands `σ^t(P)` itself with `t`
Frobenius maps and one index lookup each, charged to the relations phase
(`OracleCounters::frobenius_maps`, `lookups`). `t` maps are charged per summand,
the same unit price the eager table paid per step. The eager oracle and every
committed session are unchanged. A unit test
(`lazy_frobenius_table_returns_the_eager_pairs_and_charges_the_resolution`)
checks the lazy and eager probes return identical pairs for every base pair sum.

## 3. Exploration (not evidence)

Scratch sessions on target seeds 2026101850+n and 2026101870+n+a (2026-10-18,
not kept; sessions in the scratchpad) screened the change on n = 23, 31, 37
(a = 0) and 17, 29 (a = 1) and swept the column count under it. The claim
below uses **the per-curve columns of the 2026-10-14 protocol, not retuned**,
and fresh target seeds 202610181001–202610181015. Exploratory ratios (eager ÷
lazy mean S): 1.19 (n23), 1.37 (n31), 1.02 (n37). The column sweep suggested
that fewer columns suit the lazy oracle; that is a *next* experiment, not part
of this claim.

## 4. Sessions

One session per curve, specs from `make_specs.sh`
(`docs/ecbench/specs/claim-koblitz-lazy-members-a<a>-n<n>.json`): 32 public
targets × 3 rounds (+1 warm-up) for n ≤ 31, 16 × 3 at n = 37, 8 × 2 at n = 41;
alternate order, L0 (`--cpus none --allow-busy`; no CPU reservation, so wall
times are descriptive). Arms, all `*_full`: `rho-frob` (reference),
`rho-frob-aa` (A/A), `rho-strong`, `ic-orbit-c<c>-renew` (the frozen baseline),
`ic-orbit-c<c>-renew-lazy` (candidate; the same factor base, targets and
`stride-renew`, only the oracle id differs). `run.sh` runs, replays
(`verify --replay-all`), tabulates and saves the comparisons.

## 5. Hypotheses and decision rules

Every record must verify, every replay match, `lower_bound = false` for all
arms, and `rho-frob-aa ÷ rho-frob` contain 1 per curve (else the curve is
reported unreliable).

- **H1 (same algorithm).** Per workload, `trials`, `relations`, `matrix_rank`,
  the recovered logarithm and the table's `pair_table_entries` are equal across
  the two IC arms. Fails otherwise; then the arms are not the same algorithm
  and no improvement is claimed.
- **H2 (improvement, per curve).** `ic-orbit-renew ÷ ic-orbit-renew-lazy`
  (paired by target, cluster bootstrap 95 %): an improvement on a curve iff the
  lower end > 1. The size of the ratio is the improvement; a curve whose
  interval contains 1 is a null. Prediction: > 1 on every curve, largest where
  `|F|·n` is large against the rest of S.
- **H3 (distance to rho).** `ic-orbit-renew-lazy ÷ rho-strong` and `÷ rho-frob`
  are reported per curve as the distance remaining, not as a gate. No ranking
  against rho is made unless the interval excludes 1 and the order survives
  AGENTS.md §5a's pricing band.
- **H4 (where the saving is).** The saving per curve is accounted by phase
  (`oracle_setup` down, `relations` up by the resolution cost); the net, not the
  setup line, is the improvement.

## 6. What cannot be claimed

This removes dead set-up work from one oracle. It does not change the exponent,
beat rho on any curve unless H3 says so, or say anything about ECC2K-130 or
deployed curves. Prices are one host's microbenchmark ratios
(`src/cryptanalysis/ecbench/full_prices.json`).
