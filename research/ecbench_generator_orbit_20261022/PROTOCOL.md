# Protocol: orbit-layout pair table and generator-orbit base (2026-10-22)

Frozen before any claim data. Class: **engineering** (constants move; the
exponent does not). Baseline: the lazy orbit-member IC of
`research/ecbench_lazy_orbit_members_20261018/` (PR #2158).

## 1. Question

Two changes to the measured IC baseline (`compact-orbit-scan` +
`mitm-frobenius-lazy-counted` + `stride-renew`), each removing counted work:

1. **Orbit-layout table** (oracle `mitm-frobenius-layout-counted`,
   `FrobeniusPairTable::build_layout`). `compact-orbit-scan` pushes every orbit
   seed-first as `(σ^t S, −σ^t S)`, `t = 0..n` (its Frobenius maps are already
   charged to the factor base), and now records that
   (`FactorBase::frobenius_orbit_layout`). The table then names `σ^t(P_i)` by
   index arithmetic, so a hit spends no Frobenius map or member lookup. And
   since `ε·σ^{n−k}(P + ε·σ^k P) = P + ε·σ^{n−k} P`, the build adds a
   representative only to the rotations `k ≤ (n−1)/2` of its own orbit, which
   saves `n − 1` additions and canonicalisations per orbit. Entries, keys and
   recovered pairs are those of the lazy table.
   Unit test: `layout_table_returns_the_lazy_pairs_with_no_maps_and_half_the_same_orbit_sums`.
2. **Generator-orbit base with the generator stride**
   (`compact-orbit-scan:…,generator_orbit=1`, `targets=stride-generator`).
   Column 0 is the signed Frobenius orbit of `G` itself. Its logarithm is
   known, so one fewer relation is needed, with no lift, projection or order
   check for that column. Targets are `R_k = Q + [k]G`: one addition a target,
   no scalar multiplication, and no repeat before `k = r`, so they carry no
   guard probe. The run stops at `k = r`.
   Unit test: `generator_orbit_base_pins_the_target_with_one_relation`.

## 2. What it is at c = 1

With one column, (2) is a Frobenius-folded baby-step giant-step: the baby steps
are the pair sums of `G`'s orbit, the giant steps `Q + [k]G`. The mechanism is
generic. A win there against rho is a small-`r` effect (rho's own set-up), not
evidence that index calculus scales. It is reported as such, whatever the
numbers.

## 3. Exploration (not evidence)

Scratch sessions (target seeds 990010221002, 990010221004 and the pilot-3
seeds; 2026-10-22; not kept) at n = 29 (16 targets) and n = 13, 23, 31, 37 (4
targets). They gave the §5a band for ic-gen (lazy oracle) and ic-gen-layout
against rho-frob.

| cell | arm | S w=0 | S w=0.1 | S w=1 | rho-frob |
|:--|:--|--:|--:|--:|--:|
| n29 c1 | ic-gen-layout | 0.668 | 0.756 | 1.549 | 1.043 |
| n29 c1 | ic-gen (lazy) | 0.806 | 0.923 | 1.976 | 1.043 |
| n23 c3 | ic-gen (lazy) | 1.047 | 1.237 | 2.943 | 0.465 |
| n23 c3 | ic-lazy | 1.079 | 1.239 | 2.687 | 0.465 |

At n29 the progression's coverage is clustered: the expected first hit from a
uniform start is 33.7 trials, against 28.2 for a random set of the same size.
The pilot targets averaged 58.7. Nothing here is retuned. The claim uses the
2026-10-14 per-curve columns and fresh seeds 202610221001–202610221013.

## 4. Sessions

One session per curve, specs from `make_specs.sh`
(`docs/ecbench/specs/claim-koblitz-generator-orbit-a<a>-n<n>.json`):

- Targets: 32 public targets × 3 rounds (+1 warm-up) for n ≤ 31, except
  64 × 3 at n = 29 (the cell closest to rho), and 16 × 3 at n = 37.
- n = 41 (90 columns) is left out: at that column count one known column is
  noise, and the session is the slowest.
- Order alternate, L0 (`--cpus none --allow-busy`), so wall times are
  descriptive.

Arms, all `*_full`:

| arm | role | what it is |
|:--|:--|:--|
| `rho-frob` | reference | |
| `rho-strong` | baseline | |
| `ic-lazy-c<c>` | baseline | #2158 |
| `ic-layout-c<c>` | candidate | `ic-lazy` with the oracle id changed |
| `ic-gen-layout-c<c>` | candidate | `ic-layout` with `generator_orbit=1` and `stride-generator` |
| `rho-frob-aa` | control | A/A |

`run.sh` runs, replays (`verify --replay-all`), tabulates, writes the §5a band
(`ecbench band`, Markdown and JSON) and saves the comparisons.
`check_same_algorithm.sh` tests H1.

## 5. Hypotheses and decision rules

Every record must verify and every replay must match, with
`lower_bound = false` for all arms. `rho-frob-aa ÷ rho-frob` must contain 1 on
each curve; a curve that fails this is reported unreliable. Ratios are paired
by target, with a 95 % cluster bootstrap (`ecbench compare`).

- **H1 (same algorithm).** On every workload and round, `ic-lazy` and
  `ic-layout` agree on `trials`, `relations`, `rank`, the recovered logarithm
  and `pair_table_entries` (`check_same_algorithm.sh`). If not, no layout
  improvement is claimed.
- **H2 (layout improvement, per curve).** `ic-lazy ÷ ic-layout`. It is an
  improvement iff the lower end is > 1. Under H1 the layout arm does a subset
  of the lazy arm's counted work, so it should also be ≤ at every band weight.
  The band order is reported.
- **H3 (generator orbit, per curve).** `ic-layout ÷ ic-gen-layout`:
  - an improvement iff the lower end is > 1;
  - a regression iff the upper end is < 1;
  - otherwise a null.
  The order at w = 0, 0.1 and 1 is reported. An improvement that does not hold
  across the band is reported as band-dependent, not as an improvement.
- **H4 (net against the baseline).** `ic-lazy ÷ ic-gen-layout`, under the same
  rules and the band.
- **H5 (distance to rho).** `ic-gen-layout ÷ rho-frob` and `÷ rho-strong` per
  curve, reported as the distance remaining. An IC arm is ranked ahead of rho
  on a curve only if:
  - the ratio's upper end is < 1, and
  - the IC arm's band S is below rho's at w = 0, 0.1 **and** 1 (AGENTS.md §5a).

  Prediction from §3: at n = 29 below rho at w = 0 and 0.1 and above it at
  w = 1, so unresolved. Even a crossing on one curve would not be "faster than
  rho" (that needs a growing-n trend), and at c = 1 it is the generic mechanism
  of §2.

## 6. What cannot be claimed

- These change no exponent.
- They say nothing about ECC2K-130 or deployed curves.
- They are not a rho crossing unless H5 holds at every band weight.

Prices are one host's microbenchmark ratios
(`src/cryptanalysis/ecbench/full_prices.json`). A row reaches the leaderboard
or `benchmarks/README.md` only through the validation gate
(`docs/VALIDATION_GATE.md`: replay certificate, validator report, matched-rho
band, named by `VALIDATION.json`). Until then the study is reported as not
validated.

## Amendment 1 (2026-10-22, before any claim data was read)

Card attempt 1 (`cards/attempt-1/`: card, receipt and logs) stopped at the
first curve. `run.sh` called `ecbench band --json` without the output path
that option takes, so the script exited 2 after the n = 13 session and its
replay audit. `run.sh` now passes the path in a single `band` call. Nothing
else changed. The attempt's partial outputs were set aside unread. The
re-run uses the same frozen specs and seeds, and its n = 13 session replaces
them. Hypotheses, arms, seeds and decision rules are unchanged.
