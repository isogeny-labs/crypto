# Protocol amendment 2: the host changed after L4, and the R2 cards run longer

Frozen 2026-10-27, after card R2-L1 and before any other R2 card. It changes
nothing in `PROTOCOL.md` or amendment 1 except what is stated here.

## What happened

The session's worker restarted between card L4 and card R2-L1, and the
container came back on another host. The `lscpu` record of each card shows it:

| | host A (L1–L4) | host B (R2-L1 on) |
|:--|:--|:--|
| CPU | Intel Xeon @ 2.10 GHz, 4 vCPU | Intel Xeon @ 2.80 GHz, 4 vCPU |
| L2 / L3 | 8 MiB (4 instances) / 260 MiB | 4 MiB (4 instances) / 33 MiB |

The binaries are the same files with the same hashes; `target/` survived.

The arms do not slow down alike on B. A fast-only probe outside the cards,
and R2-L1 itself, show it on `gaudry-d4`, δ = 512:

| arm | A → B |
|:--|:--|
| `sparse` | about 100 → 300 ms |
| `dense` | about 0.6 → 1.8 s |
| `fast` | about 12 → 16 ms |

So `sparse/fast` depends on the host.

## Changes

1. **Hosts are reported separately, never pooled.**
   - Host B is the full corpus: every R2 card.
   - Host A is the first L1–L4 runs, on the systems they measured; amendment
     1's bug skipped n6d3 and two n3d10 systems.

   Amendment 1 marked L1–L4 as superseded and gave them only as a run-to-run
   check. They are now kept as a second host, which the bug did not affect for
   the systems they measured. Every prediction is scored on host B. The README
   gives host A's per-family ratios beside host B's, as a check on host
   dependence. If the host changes again during R2, the cards run after the
   change are reported as a third host. They are re-run on one host only if
   the corpus would otherwise be split.
2. **Longer cards.** On host B the remaining cards take about 2–3 times their
   protocol estimates, beyond the 1700 s run timeout and the executor's 30-minute
   shell.
   - The `run` timeout of R2-L3, R2-L4, R2-L5, R2-L6, R2-A1, R2-A2, R2-D1 and
     R2-D2 is raised to 7000 s. Nothing else on those cards changes.
   - The executor runs R2-L2. The main session runs the eight long cards with
     `tools/task_card.py` from its own shell, which has a 2-hour background
     limit, as it ran card 8 of the exhaustive planner study. Each such card's
     launcher log goes in `cards/`.

## Unchanged

Arms, corpus, seeds, rounds, measures, predictions P1–P5 (scored on host B),
the decision rule, and the dense-arm cap.
