#set page(paper: "a4", margin: 16mm)
#set text(size: 9pt)

= Run report: Part B, the stage diagnostic on the Frobenius closure

*Descriptive run report*; interpretation is in the study README.

== What ran

- `resume3.sh` (amendment 3), started 2026-10-18 20:28 UTC; cells one process each, `pdp_split_dreg --cells CELL --base closure --slots-per-target 4 --targets 4 --guesses 0,2,4 --seed 20261012 --budget-secs 300 --max-free 24 --lambda-targets 256 --lambda-systems 512`.
- Binary `pdp_split_dreg` SHA-256 `58bfbbd5…` (`BINARIES.sha256`), commit in `COMMIT`.
- Cells (m:a:n:ℓ): 3:1:11:4, 3:0:13:4, 3:1:17:4, 3:1:19:4, 3:0:13:5, 3:1:17:5; plans unsplit `S₄` and the `S₃` chain; four targets, four slot tuples each, three guess counts: 96 runs per cell, 576 in all. The 3:1:17:5 cell finished (96/96, `.done` stamp) minutes before the fourth container reclaim.
- Host: the cloud container (Intel Xeon @ 2.10 GHz, 4 vCPU), no isolation: F4 wall time is descriptive only.

== Outcome

All 576 runs are complete and exact against an exhaustive enumeration of the decompositions over each slot tuple: no timed-out branch, no false or unliftable solution accepted, no decomposition missed.

== Measured values

Per (cell, plan, g): solving degree per run, F4 word XORs, yield per system λ (exhaustively counted over 512 random (target, slot-tuple) pairs), orbit-folded columns and the relation-phase projection are in `analysis.json` / `analysis.md` beside this page.

#figure(image("fig1-solving-degree.svg", width: 100%), caption: [F4 solving degree per run on the closure, by plan and guess count (records `runs-B/`).])
#figure(image("fig2-word-xors.svg", width: 100%), caption: [F4 word XORs per run, all `2^g` branches summed (log).])
#figure(image("fig3-projection-vs-rho.svg", width: 100%), caption: [Relation-phase projection in S, `(columns + 1)/λ · X̄ · ns_per_word_xor / √r` (derived; pinned ratio from another host; linear algebra and descent omitted), against measured rho from session `ECBS1he23809badf5b`.])

== Raw records

`runs-B/*.jsonl` (one line per cell header and per run), `runs-B/*.stderr`, `runs-B/*.done`, `progress.txt`.
