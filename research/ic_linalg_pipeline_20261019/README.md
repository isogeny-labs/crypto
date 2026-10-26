# The relation matrix inside `ic.pipeline`: sparse-cado against dense full-rank, |F| = 1,024 to 16,384

Measured 2026-10-19 under [`PROTOCOL.md`](PROTOCOL.md), frozen in `a75e9b407`
before any recorded session. PDF: [`REPORT.pdf`](REPORT.pdf). Run reports:
[`reports/run-20261019T1629Z/`](reports/run-20261019T1629Z/report.pdf) (session A),
[`reports/run-20261019T1816Z/`](reports/run-20261019T1816Z/report.pdf) (session B).

It answers the open item in `research/ic_leads_review_20261018/README.md` §3.3
("The relation matrix has never been measured"). The question is what choosing
`linalg=sparse-cado` (CADO-style filter, then block Wiedemann; PR #1874) over
`linalg=incremental-gauss-full-rank` changes in a full ecbench `ic.pipeline` run,
as the factor base grows.

## Result

**For `S`, nothing.** On every target and at every size, the two arms read the same
relations and give the same `S` (ecbench `ops` ratio 1.0000, interval [1, 1], 40
pairs per size, 8 at 16,384). That is by construction: `ic.pipeline` leaves the
matrix's `row_ops` unpriced. Priced across the AGENTS.md §5a band, the two tie at
w = 0, and dense is cheaper by at most 0.27% at w = 0.1 and 1 GAE per operation.
The order does not hold across the band, so the two arms are **not ranked** in `S`.

**For the matrix itself, the two count and cost differently:**

| N | `row_ops` dense | `row_ops` cado | cado/dense | LA wall dense | LA wall cado | cado/dense, Σ (per-pair range) |
|--:|--:|--:|--:|--:|--:|:--|
| 1,024 | 28,749 | 43,991 | 1.53 | 61 ms | 44 ms | 0.72 (0.59–0.95) |
| 2,048 | 61,209 | 92,929 | 1.52 | 223 ms | 113 ms | 0.51 (0.41–0.63) |
| 4,096 | 129,316 | 194,343 | 1.50 | 793 ms | 355 ms | 0.45 (0.39–0.57) |
| 8,192 | 281,907 | 422,323 | 1.50 | 5.77 s | 1.44 s | 0.25 (0.21–0.32) |
| 16,384 | 613,491 | 918,135 | 1.50 | 22.1 s | 6.12 s | 0.28 (0.23–0.32) |

`row_ops` are means per run (deterministic; identical across rounds). Wall times
are means per run; the ratio is Σ cado / Σ dense over (workload, round) pairs.
A/A control (`cado-4096-aa` against `cado-4096`): `row_ops` identical,
LA wall ratio 1.02, per-pair range 0.83–1.31.

- sparse-cado does about **1.5× more counted work** than dense at every size.
  Two-term relations barely fill in, so the dense elimination's counted work stays
  near-linear in N.
- It spends **2–4× less linear-algebra wall time** from N = 2,048 up. The dense
  method's scan of every column of every pivot row is not counted in `row_ops`
  (`ic_framework/linalg.rs`, module doc), and that scan is the wall-time gap.
  From 2,048 up, the per-pair ranges lie entirely below the A/A range; at 1,024
  they overlap it.
- **Every wall figure is descriptive.** Each index-calculus run earned L1, not L2,
  because of 41–47 ticks of hypervisor steal on the run CPU; ecbench admitted no
  wall pair (`wall.status: descriptive`).
- **End to end, the matrix is a small share.** The oracle's |F|² pair-table
  set-up takes 84% of a dense run's wall time at 8,192 and 89% at 16,384 (37 s and
  195 s), so the whole-run wall median ratio (ecbench) is only 0.89 at 8,192 and
  0.90 at 16,384 (descriptive).

## Distance remaining to rho

`ops` ratio of `cado-N` to `rho-neg` (S, w = 0; ecbench cluster intervals). It
equals dense's at every N:

| N | 1,024 | 2,048 | 4,096 | 8,192 | 16,384 |
|:--|--:|--:|--:|--:|--:|
| S_IC / S_rho | 156 [124, 203] | 149 [119, 190] | 335 [270, 428] | 1,201 [970, 1,542] | 4,054 [2,649, 7,479] |

The best size here is N = 2,048. Beyond it the |F|² table set-up grows faster than
relation collection shrinks. Priced at w = 1 the gaps narrow to 150–2,052×, because
rho's unpriced canonicalisations are then charged too (`analysis/*.json`). This is
the distance remaining, not a pass/fail gate.

## Predictions (PROTOCOL.md)

- P1 (identical `S`, trials, relations): **held**.
- P2 (cado ≈ 1.5× dense's `row_ops`): **held**, at 1.50–1.53.
- P3 (cado less LA wall time for N ≥ 4,096, about 4× at 8,192): **held,
  descriptively**: 2.2× at 4,096, 4.0× at 8,192, 3.6× at 16,384 (L1).
- P4 (set-up dominates; neither matrix moves the gap to rho): **held**.

## Classification and decision

Engineering, not an advance (AGENTS.md §3). The relation matrix does not change
`S` in `ic.pipeline` at these sizes. sparse-cado cuts the matrix's wall time
(descriptively) at the price of more counted multiply-subtracts. Neither reaches
the bottleneck: the oracle's |F|² set-up, and the gap of at least 149× to rho.

Consequences:

- `row_ops` is the wrong proxy for the dense matrix's cost. A pricing of
  `row_ops` would rank dense ahead while it runs 2–4× slower. Charging the
  matrix in `S` would need a unit that counts the dense scan (or the stored
  `rank × |F|` words); that is an accounting change and needs its own protocol.
- For `ic.pipeline` at large factor bases, sparse-cado is the better default for
  wall time, but this does not move the IC frontier. The next lever is the oracle
  set-up, not the linear algebra.
- No leaderboard change: no `S` moved, and the wall figures are descriptive
  (L1). The leaderboard's existing linear-algebra section
  (`research/la_engines_20261014`) stays the reference for engine speed.

## Deviations from the protocol

- The protocol did not fix a replay count. The audits replayed 24 of session A's
  480 records and 6 of session B's 36 (`--replay`, not `--replay-all`); all
  reproduced exactly.
- The two sessions recorded different environment class ids
  (`ECBENV2h43ce23acf702`, `ECBENV2h6639ae09b95e`) on the same container,
  binary and commit. Within-size comparisons never cross sessions.
- Wall time: the protocol required L2 for admitted wall figures; none reached it.
  Reported as descriptive, as the protocol says.

## Evidence

- Specs: [`specs/A-1k-8k.json`](specs/A-1k-8k.json) (`ECS1h529376370841`),
  [`specs/B-16k.json`](specs/B-16k.json) (`ECS1hab3596c535bb`).
- Sessions: `sessions/A-1k-8k` (`ECBS1hf4d23305c958`, 480 records, 480 verified,
  2026-10-19 16:29–18:16 UTC) and `sessions/B-16k` (`ECBS1h0c4c69816435`, 36 records,
  36 verified, 18:16–19:41 UTC). Commit `a75e9b407`, clean tree; binary
  `797ea0fb…e91b`.
- Audits: `sessions/A-1k-8k.audit.json` (OK, 24 replays reproduced, SHA-256
  `a7ffcfe926e4ef9742fabeb7a59cf93083a430144bfefe99fe4aaa77b4f37220`) and
  `sessions/B-16k.audit.json` (OK, 6 replays reproduced, SHA-256
  `d18752eadac714a2e7c71ec7b6eb073c76711790e38cdde3b821e0582f7cde08`).
- Comparisons: `sessions/*/comparisons/` (ecbench `compare --save`). Tables:
  `analysis/table-A.md` and `analysis/table-B.md` (ecbench `table`).
- Derived figures: `analysis/A-1k-8k.json` and `analysis/B-16k.json`, from
  `scripts/analyze.jq` (jq over the records; no Python computation). Figures:
  `figures/*.svg` and `.png`, from `scripts/plots.py` (layout only).
- Host: 4-vCPU Intel Xeon @ 2.80 GHz VM (AVX-512), Linux 6.18, 15 GiB, one
  NUMA node; `ecbench run --cpus auto`, run CPU 3 reserved.
