# Native prime-field factor-base size and decomposition-oracle tuning

Status: **draft**. No experiment has run or been scheduled. A runnable
protocol now exists as a DRAFT: [`PROTOCOL.md`](PROTOCOL.md), with
[`run.sh`](run.sh), [`specs/`](specs/) and the sizing-probe specs in
[`probes/`](probes/). It is not frozen. Blocker 1 below must land first. This
card records the next bounded engineering experiment after historical
evidence PR #1898. See the
[evidence-linked handoff](../../research/factor_base_evaluation_20261015/followup_20261016/README.md)
for the decision sequence and sources.

## Question and decision value

Can joint selection of factor-base size and an exact native decomposition
oracle reduce the complete one-target IC cost against the current verified
prime-field incumbent, on fresh prime-order instances?

A robust holdout improvement becomes the next baseline at that size, even
above rho. A null or pricing-sensitive outcome keeps the incumbent. Low-x is
the incumbent's base, so it is always measured. The Kummer coset base is a
matched-column control on the two prime-order PKM-ladder curves. No random
base plug-in exists, so a strided-abscissa base stands in as the
small-x-structure control. This task does not replace the open PKM
reduced-presentation degree check, and it makes no NIST-size claim.

## The pinned incumbent and design (summary of PROTOCOL.md)

- **Incumbent.** The IC frontier campaign's `ic-pair` arm
  (`research/ic_frontier_campaign_20261015`, frozen 2026-10-15, commit
  `9bef5733`): `ic.pipeline`, `prime-abscissa:size=N` with
  `mitm:negation_folded=1`, `targets=walk`, `linalg=incremental-gauss`. The
  values of `N` and `max_trials` are frozen per size: 32 / 162 / 407 / 1,024
  at 20 / 24 / 28 / 32 bits. Its Phase 1 band `S` at `w = 0` is 12.5, 14.4,
  20.6 and 32.1. Matched `rho.negation` on the same curves is 1.27, 1.19,
  1.11 and 0.79, so IC is 9.8× to 41× rho. The incumbent is re-run unchanged,
  with the same method ids, on every new workload.
- **Grid.** On fresh `prime_search` train curves (seed `20261018101`,
  20/24/28/32 bits, 8 public targets), it crosses five sizes
  `round(N_inc·2^{k/2})`, `k = −2..2`, with every exact native prime oracle:
  `mitm` m = 2 (the incumbent), `mitm-prime-batch`, `mitm` m = 3,
  `rootfind-mitm` m = 3 and `rootfind-subtract` m = 3. The edge rule extends
  the grid to `|k| ≤ 4`. The memoryless `subtract`, `rootfind-subtract`
  m = 2 and `rootfind-pairs` m = 3 run as controls at 20 and 24 bits only.
- **Selection.** The selected cell has the smallest train mean `S` at
  `w = 0.1` among cells verified 8/8. Selection is hash-frozen before the
  holdout runs.
- **Holdout.** On other fresh curves (seed `20261018202`), 48 public targets ×
  2 rounds per size. The arms are the incumbent, the selected cell, the
  size-only cell, a stride control, matched `rho.negation`, and A/A arms for
  IC and rho.
- **Decision, per size.** All measured runs must verify and the replays must
  be identical. The paired ratio-of-totals 95% upper bound must be below 1 at
  each of `w = 0, 0.1, 1`. The gap to rho is reported as the distance
  remaining.

## Cost derivation (this host)

Every input is *recorded* (a committed session) or *measured* (this card's
pre-freeze probes, PROTOCOL.md §11). Every total is *derived*. The probes
ran on one core while another experiment loaded the host.

**Per-run wall.** All figures are process wall, one thread:

| size | incumbent (*recorded*, frontier Phase 1 mean) | grid cells (*measured*) | memoryless controls (*measured*) |
|--:|--:|:--|:--|
| 20 | 0.035 s, 29 MiB | — | about ¼ of 24-bit (cost ∝ r) |
| 24 | 0.042 s, 29 MiB | m = 2, 3: 0.032–0.048 s, 34 MiB | `rfsub2` 0.20–0.28 s; `sub2` 0.62–0.79 s; `rfpairs3` 2.1 s |
| 28 | 0.092 s, 40 MiB | — (interpolated: 0.1–0.2 s) | retired |
| 32 | 0.579 s, 90 MiB | `mitm2` 0.42–0.76 s; `mitm3` 0.63–1.08 s, 42–190 MiB; `mitm3` N = 4096 2.84 s, 658 MiB | retired |

**Executions** (from `ecbench plan`):

| stage | executions | derived wall |
|:--|--:|--:|
| identity: replay-all of frontier P20–P32 | 4 × 160 measured | about 1 min (P32's 48 IC runs at 0.58 s dominate) |
| train | 232 + 232 + 208 + 208 = 880 | 20 + 35 + 30 + 140 s ≈ 4 min |
| edge extensions | at most 5 families × 2 × 4 sizes × 16 | at most about 5 min (worst case is ten 32-bit cells at 2.84 s × 8) |
| holdout | 4 × 1,008 (7 arms × 48 targets × 3 rounds incl. warm-up) | 32 bits: 5 IC arms × 144 × ≈0.6 s ≈ 7 min; 28: ≈1.5 min; 24 and 20: ≈1 min together; about 10 min; under 15 min if `mitm3` at N ≈ 2,900 were selected |
| panel K | 2 × 160 | under 1 min |
| audit (replay-all of every measured run) | about 4,600 | ≈ the train + holdout time, about 20 min |
| analyse (84 compares × 4,000 resamples, band, table) | — | under 5 min |
| process overhead | about 6,000 spawns × ≈0.03 s | about 3 min |

**Totals** (*derived*):

- About 50 minutes of one core, sequential.
- The card's `outputs.verify` (`run.sh verify`) costs another ≈20 minutes.
- The card asks for **2 core-hours** and **2 wall-hours**. That is about 2×
  margin for a contended spot host. The 4-hour shard timeout is a ceiling,
  not an estimate.
- **RAM:** the largest process is ≈0.66 GiB (`mitm3`, N = 4096, 32 bits).
  An `mitm2` extension to N = 4096 is about the same (the table is 2N²
  entries; *derived* from 188 MiB at N = 2048). The card asks for 4 GB.
- **Disk:** about 5 KB per record (*recorded*: frontier sessions are 1.2 MB
  for 240 records). About 6,000 records come to ≈30 MB, and receipts add
  less than that again. The release build of `ecbench` and `backlog` is
  ≈1 GB. The card asks for 10 GB.

**What fits here.** The host is a 32-vCPU Xeon 6975P-C (AVX-512), 124 GB
RAM, 2 TB EBS spot instance. The whole experiment needs one core, under
1 GB of RAM and well under 1 GB of output, so it fits many times over. It
can run beside the multi-day experiment from the other checkout. Use its
own worktree, `PBO_CPU` set to a core that experiment does not use, and
`CARGO_TARGET_DIR` inside the worktree.

Counts do not depend on the host, so L0 is enough for the decision. Wall
figures from this shared host stay descriptive. A spot interruption costs at
most one session (minutes). Rerunning the same command (or `--resume`)
skips complete sessions and keeps the interrupted directory. Running the
four sizes on four cores would cut wall time to ≈20 min, but `run.sh` is
sequential by design so its log order is the audit order.

## Blockers (also in `card.json`)

1. `ecbench compare` has no `--weight`. It is at `src/bin/ecbench.rs:228`
   and `src/cryptanalysis/ecbench/compare.rs:288`. `band.rs:187` ranks means
   without intervals. So the protocol's paired intervals at `w = 0.1` and `1`
   cannot be computed natively. This blocks the decision rule.
2. There is no seeded random prime factor-base plug-in
   (`src/cryptanalysis/ecbench/methods.rs:2125–2137`). This is a control gap,
   not a blocker of the primary question. Land the plug-in, or freeze with
   the stride control and record the gap.
3. Freezing still needs three steps: merge the card and blocker 1, pin
   `code.commit` to that merge, and record `protocol.blob`.

## Extension (not in this card)

The frontier ran 36- and 40-bit rungs from `prime_fast::find_a3_curve(bits, 3)`
(`examples/ic_frontier_prime_curves.rs`, which hard-codes residue 3). A fresh
36/40-bit pair would need a `--residue 1` option, or a seeded `a = −3`
search. The `prime_search` constructor stops at 32 bits. 36 bits fits easily
here: 5 s and 0.4 GB per incumbent run (*recorded*, frontier P36).

## Resources and funding

`resources` in `card.json` are this derivation's integers: 1 CPU, 4 GB,
10 GB, 2 core-hours, 2 wall-hours. They are estimates, not measurements. An
overflow of the cap is a recorded incomplete result, never a reason to
shrink the holdout. Funding stays 0 while the card is a draft. Given about 2
core-hours, a reviewer may set it at or below the 131,072 floor of the seed
cards once it is frozen.

## Completion

Land the frozen protocol, the run directory `research/prime_base_oracle_tuning_<run-date>/`
(PROTOCOL.md §13), and the curve registrations. The run directory contains
the host record, sessions, audits, selection with hashes, analyses, report,
graph and PDF, and benchmark copies. A promotion also needs the leaderboard
update and a second-host replay. Draft planning alone does not complete this
experiment.

`backlog validate XP-20261016-prime-base-oracle-tuning --no-git` passes on
this draft. TODO warnings are allowed only in `cairn.env.recipe`.
