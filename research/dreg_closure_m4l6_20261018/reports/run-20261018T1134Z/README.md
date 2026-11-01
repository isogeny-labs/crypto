# Run report: dreg closure (4,6), run 20261018T1134Z

Observations only. The reading under PROTOCOL.md §4 is in the study's
`RESULTS.md`; this report says what ran and what was recorded.

## What ran

- Study: `research/dreg_closure_m4l6_20261018/` (PROTOCOL.md §§1–10). Command:
  `pod/boot.sh closure` with `FRESH=4:15:6`, which runs `run.sh host`, `gen`
  (skipped: the committed draws exist), `measure`, `deg5`, `control`.
- Commit built on the host: `9d4541e2` (`runs/host-326a1f5b2d8c.txt`;
  `dreg_ladder` sha256 `473f511e…`, `dreg_closure` sha256 `de547a61…`,
  rustc 1.99.1).
- Host: RunPod pod `wfh2xovuq5aahv`, 2×RTX 4090 flavour used as a CPU host
  (GPUs idle), 25 vCPU, cgroup limit 200,000,000,000 bytes, `prlimit` address
  cap A = 184,000,000,000 bytes (0.92 × limit), `RAYON_NUM_THREADS=64`,
  MemTotal 528 GB on the machine. Isolation: none (shared cloud host); no
  isolation level is claimed.
- Draws: `draws/m4-15-6.jsonl`, 251 draws generated on the first host
  (`jxomnqsqefy6gb`, 06:29–11:30Z, about 70 s per draw), sha256
  `d472512a…` (`draws/sha256.txt`), committed as `fc1eea79` before any
  measurement; 2 unsatisfiable draws (42 and 214), 249 satisfiable.
- Timeline (UTC, `runs/queue.log`): 11:34:35 measure A1 draw 42; 11:59:02
  draw 214; 12:22:39 deg5 C1 draw 0; 12:40:07 control A1 draw 0; 13:14:42
  `CLOSURE_DONE`. The two A1 draws ran one after the other (§8's "side by
  side" paired A1 with A2, and A2 was deferred by §10).

## Runs requested and completed

| run | cell | draw | `--to` | outcome | wall | peak RSS | record |
|---|---|--:|--:|---|--:|--:|---|
| measure | A1 (4; 15, 6) | 42 (unsat) | 4 | exit 0, `closure_refuted` at 4 | 24:27 min | 25,846,376 kB | `runs/fresh-m4-15-6.d42.jsonl` |
| measure | A1 | 214 (unsat) | 4 | exit 0, `closure_refuted` at 4 | 23:36 min | 25,716,896 kB | `runs/fresh-m4-15-6.d214.jsonl` |
| deg5 | C1 (3; 19, 7) | 0 (unsat) | 5 | **failed**: "memory allocation of 94976 bytes failed", SIGABRT (exit 134) after 17:28 min; no record written | 17:28 min | 169,306,092 kB | `runs/deg5-m3-19-7.d0.log` only |
| control | A1 | 0 (satisfiable, 1,226 solutions) | 4 | exit 0, `refuted_satisfiable: false` | 34:36 min | 21,361,800 kB | `runs/control-m4-15-6.d0.jsonl` |

Wall and RSS are GNU `time` values from the `.log` files (measured). CPU
use was 1,391–1,429 % of one core on the A1 runs (measured).

## Recorded values (from the `.jsonl` scans)

| draw | degree | columns | XL rank | closure rank | XL refuted | closure refuted | closure products | closure word ops | secs |
|--:|--:|--:|--:|--:|---|---|--:|--:|--:|
| 42 | 3 | 9,022 | 854 | 2,748 | no | no | 5,539 | 24,105,851 | 0.197 |
| 42 | 4 | 341,326 | 23,757 | 1 | no | **yes** | 1,181,122 | 2,873,336,798,759 | 1,464.66 |
| 214 | 3 | 9,022 | 854 | 2,748 | no | no | 5,542 | 24,281,639 | 0.214 |
| 214 | 4 | 341,056 | 23,757 | 1 | no | **yes** | 1,178,721 | 2,455,694,015,797 | 1,413.40 |
| 0 (control) | 3 | 11,662 | 854 | 2,748 | no | no | 5,643 | 26,860,286 | 0.217 |
| 0 (control) | 4 | 342,537 | 23,757 | 214,391 | no | no | 1,094,741 | 2,712,062,896,535 | 2,072.76 |

All values measured by `examples/dreg_closure` and copied from the records.
"Closure rank 1" is the engine's refutation signal (the constant lies in the
degree-4 span). Every scan row has `saturated: true`.

Sample size: 2 unsatisfiable draws and 1 control draw for A1; 0 completed
runs for C1. No spread is recorded; the two unsatisfiable draws agree on
every categorical field and differ by 0.1 % in columns and 17 % in word ops.

## Against the protocol's projections (§2, §5)

- A1 peak RSS 25.8 GB against a 44 GB projection (3·cols²/8 bytes): 0.59×.
- C1 at degree 5 ran out of memory at 169 GB resident under the 184 GB cap;
  the projection was 217 GB, so the failure is consistent with the
  projection and the cell is **capped** on a 200 GB host (§4: "mixed or
  capped: inconclusive").
- Unsatisfiable-draw rate at S = −9: 2 in 251 (§5 expected `--unsat 4`
  within about 150 draws; recorded in PROTOCOL.md §10).

## Graph

`closure-a1.svg` / `closure-a1.png` (source `plot.py`, recorded values only):
left, columns, XL rank and closure rank at degree 4 per draw; right, peak RSS
and wall per run with the §2 projections and the 184 GB cap.

## Missing

- A2, B1, B2, C2: not run (deferred, PROTOCOL.md §§8, 10).
- C1: no completed run; the abort left no `.jsonl`.
- A1: draws 2–4 of the unsatisfiable list do not exist (only 2 were found).

Raw records: `runs/` in the study directory; the first host's generation log
is `draws/m4-15-6.log`.
