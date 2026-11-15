# Protocol: Cheon comb against matched rho on 14 sizes, 36 to 62 bits

Frozen before any session of this study runs. Amendments are appended with a
date and never edit this text.

## 1. Question

In [`research/cheon_ecbench_20261022/`](../cheon_ecbench_20261022/) and
[`research/cheon_ecbench_medium_20261106/`](../cheon_ecbench_medium_20261106/),
the Cheon comb arms and matched `rho.negation` were measured on six
medium-tier sizes (36 to 56 bits, 4-bit steps) with 8 planted targets each.

The fitted cost exponents (S against r) have overlapping 95 % intervals:

| arm | exponent |
|:--|:--|
| comb n−1 | 0.450 [0.406, 0.473] |
| rho n−1 | 0.496 |
| comb n+1 | 0.428 [0.389, 0.460] |
| rho n+1 | 0.513 |

So no exponent separation is established.

**Question:** with more sizes, a wider size range and more targets, do the
exponent intervals of comb and matched rho separate, in each family?

This is a cost of the DLP with auxiliary inputs (a counted static-DH oracle).
It makes no plain-ECDLP or index-calculus claim.

## 2. Design

**Curves.** `curves.json` is written by
`cheon_p224_shape gen --bits 36,38,40,42,44,46,48,50,52,54,56,58,60,62`
(`examples/cheon_p224_shape.rs`). The `--bits` option is added in this
study's commit; the construction is unchanged:

- p = next_prime(3·2^(B−2));
- a random (a, b) is drawn from the SplitMix64 stream seeded by (B, family);
- the first curve is kept with prime n, n ∓ 1 = d·q (q prime) and
  |log d / log n − ρ| ≤ 0.015, where ρ = 0.12943 (n−1) or 0.21106 (n+1).

There are 14 sizes per family. 62 is the largest size the generator accepts:
`prime_explicit` needs p < 2^62, so 64 bits is impossible.

**Check already done (P0, below).** Before this protocol was frozen, the
generated 36 to 56-bit curves were compared field by field with the curves of
`research/cheon_ecbench_20261022/spec-{minus,plus}.json`. All 6 of 6 per
family are equal, and all 12 equal `research/cheon_p224_shape_20261022/curves.json`.

**Specs.** `spec-minus.json` and `spec-plus.json`:

| | value |
|:--|:--|
| arms | `rho-neg` = `rho.negation` (reference), and `cheon-comb` = `cheon.pminus1_comb` with d = `shape:0.12943:0.015`, or `cheon.pplus1_comb` with d = `shape:0.21106:0.015` |
| targets | 24 per curve, `target_seed` 20261022 |
| measurement | 1 round, 0 warmup, alternate order, seed 1, L0, timeout 3600 s |

That gives 14 × 24 × 2 = 672 runs per session.

**Running.**

```sh
ecbench run --spec spec-minus.json --out sessions/minus-20261106 --cpus 2 --lock <scratch>/cheon-wide-minus.lock --allow-busy
ecbench run --spec spec-plus.json  --out sessions/plus-20261106  --cpus 3 --lock <scratch>/cheon-wide-plus.lock  --allow-busy
```

- **Concurrency.** The two sessions run at the same time on separate CPUs,
  each with its own lock, so they bypass the shared host lock.
- **Why this is safe here.** These are L0 counts-only sessions: operation
  counts are deterministic, and the medium study's P1 found 272 of 272 counts
  identical on re-run. No wall time is admitted, and every report states this
  deviation.
- **Audit.** Each session is then audited with
  `ecbench verify --dir <session> --replay-all --out sessions/<name>.audit.json`.

## 3. Analysis (fixed now)

**Fits.** `fit.sh` runs `ecbench bound fit` for each session, arm and w ∈ {0, 0.1, 1}:

- whole session, no `--tier` and no `--curve`, fit version 2;
- `--oracle` set to `static_dh:n-1:d=shape(0.12943,0.015)` or
  `static_dh:n+1:d=shape(0.21106,0.015)`;
- `--weight w` for w > 0.

That gives 12 records, `docs/bounds/records/cheon-ecbench-wide-<family>-<arm>[-w<w>].json`.
The frontier is then rebuilt with `ecbench frontier build`.

**Exponent.** The quantity compared is the fitted exponent α of `S` (the
record's `fit.alpha` and `fit.alpha_ci95`), at each weight.

## 4. Predictions and decision rules

- **P0 (curves).** Met before freezing: see §2.
- **P1 (positive control).** For targets 0–7 on the six sizes shared with the
  medium study, each run's counts equal the medium session's run with the same
  arm, curve and target index:
  - the counts compared are outcome, `cost.total_gae`, `S`, oracle queries,
    oracle server work and every per-phase count;
  - a mismatch is reported per run, and does not stop the study.
- **P2 (fits).** All 12 fits are accepted with no admissibility reason, and
  procedure 2 ranks them.
- **P3 (decision rule, per family).** "Exponent separated" means the lower end
  of rho's α interval is above the upper end of comb's α interval at all three
  weights. Otherwise the result is "not separated". Both outcomes are reported
  with the intervals.
- **Predictions, written before data:**
  - n+1 separates; n−1 is uncertain, about even odds;
  - rho α lies in [0.45, 0.55];
  - comb α lies in [0.40, 0.47] for n−1 and [0.36, 0.45] for n+1.
- **Also reported:** the gap to matched rho as a pooled paired ratio of `S`
  over the 12 cells (2 families × 3 weights × q ∈ {d, √r}), as the original
  report did; the per-size rho gate; and every timeout.
- **Stop rule.** A run that times out stays in the tables and the fits treat
  it as `ecbench bound fit` does. No size, target or arm is added or removed
  after any data is seen.

## 5. Outputs

- `sessions/` holds the two sessions and their audit receipts.
- `REPORT.md` and `report.pdf` give the outcome of P0–P3, the α intervals and
  the gap table.
- `alpha.svg`, a graph of α with its intervals per arm, family and weight,
  is drawn from the records.
- The run report goes in `reports/`.
- The 12 records, plus the rebuilt `docs/bounds/frontier.json` and `FRONTIER.md`.
- A `benchmarks/` row and `bench.record/v1` record.

## Amendment 1 (2026-11-07): restart after a host loss

The two sessions started at 2026-11-06T23:03Z
(`sessions/minus-20261106` and `sessions/plus-20261106`) were killed outright
when the container was reclaimed while idle, shortly after 00:45Z. By then
they had written 526 and 525 of 672 records, through part of the 58-bit size.

A killed runner cannot finalise its session, so both read `running`. They are
kept, never deleted:
- `ecbench verify --replay 0 --allow-interrupted` audits them as interrupted:
  `interruption: never_finalised`, 0 problems
  (`sessions/*-20261106.interrupted-audit.json`);
- they are not cited as complete;
- they enter no fit.

The same specs, with no change, are run again from scratch into
`sessions/minus-20261107` and `sessions/plus-20261107`, with the same binary
and running arrangement as §2. A watcher process keeps the host alive while
they run. §3 and §4 then apply to these sessions in place of the
`-20261106` ones.

The interrupted records are compared with the new runs as an extra
reproduction check:
- every record in the interrupted sessions should equal the new session's run
  with the same arm, curve and target index;
- mismatches are reported.

Nothing else changes: sizes, targets, arms, analysis, predictions and
decision rules are as frozen above.

## Amendment 2 (2026-11-07): one session per size

The restarted sessions `sessions/minus-20261107` and `sessions/plus-20261107`
were also killed by a container restart, about 1.3 h in, partway through the
58-bit size (550 and 547 of 672 records). Like the first pair, they are kept,
audited as interrupted with 0 problems (`*.interrupted-audit.json`), never
cited as complete, and left out of every fit.

This container is restarted every 1.3 to 1.7 hours, so a single 10-hour
session cannot finish here. With the owner's agreement, each family's spec is
therefore run as 14 sessions, one per size:
- `chunks/spec-<family>-b<bits>.json` is `spec-<family>.json` with only that
  size's curve. Everything else, arms, targets, seeds and measurement, is
  unchanged; this was checked with jq;
- the output is `sessions/<family>-b<bits>`;
- each chunk is audited with `--replay-all`, as in §2.

A chunk killed by a restart is renamed `<dir>.interrupted-<n>`, kept, audited
with `--allow-interrupted`, and run again from scratch.

**Fits.** The §3 fits are taken over all 14 complete chunk sessions of a family
together (`fit.sh`: one `--dir` and `--audit` per chunk, no `--tier` and no
`--curve`). This is the same set of runs as one whole session, with no filter.

**Extra reproduction checks.** The P1 positive control is unchanged. In
addition, every record of the four interrupted sessions is compared with the
chunk run that has the same arm, curve and target index.

**Concurrency.** One chunk runs per CPU in the container's CPU set, which is
CPUs 0 and 3 after the latest restart. Each has its own lock, under the §2
disclosure. Counts are deterministic at L0.

**A false start, disclosed.** At 04:15Z the queue started before this
amendment was pushed. It was set for CPUs 1, 2 and 3, which the restarted
container no longer had:
- it completed `minus-b36`, `plus-b36` and `minus-b38`, each audited with
  `--replay-all`. These are kept and used; their specs are this amendment's;
- the other chunks failed to pin and wrote nothing;
- `minus-b54` was stopped after 1 record and is handled as an interrupted
  chunk, as above.

**If a size never finishes.** If a size cannot finish here despite reruns,
the study stops at the last size that completed in both families. It reports
that size range, and amends again before any fit. No size is dropped after
its results are seen.

## Amendment 3 (2026-11-07): cap at 56 bits

The host was replaced at about 05:40Z, and its local disk went with it. The
chunk sessions completed between 04:18Z and 05:40Z had not been pushed, so
they are lost:
- `minus-b40` to `minus-b54`, except `minus-b54`'s 1-record interrupted
  start, which was pushed;
- `plus-b38` to `plus-b54`.

Their counts were never read for any analysis.

Compute here only advances while a session turn is active, and the 60- and
62-bit chunks need 2 to 4 hours each. With the owner's agreement, before any
further data, the study is capped at **36 to 56 bits in 2-bit steps (11 sizes
per family)**:
- the 58, 60 and 62-bit curves stay in `curves.json` and their chunk specs
  stay in `chunks/`, but they are not run;
- the fits in `fit.sh` use the 11 sizes 36 to 56;
- everything else is as frozen: targets, arms, seeds, analysis, the P3
  decision rule and the predictions, which stand as written for the narrower
  range.

**Expected effect.** The narrower range widens the exponent intervals. Before
any data, the planning estimate for rho is about ±0.05 with 24 targets over 11
sizes, against about ±0.034 over 14 sizes. P3 may therefore not separate where
the 14-size design would have.

**Pushing.** Chunks are pushed as soon as each is audited, so a host loss
costs at most the chunks in progress. A chunk lost before it was pushed is
listed in the report and run again.

The interrupted starts `minus-b54` and the two pairs of whole-spec sessions
are kept as before.
