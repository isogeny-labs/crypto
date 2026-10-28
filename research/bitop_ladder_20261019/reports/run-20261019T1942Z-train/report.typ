#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
= Run report: Phase A train panel, 2026-10-19 19:42–22:01 UTC

*Study* `research/bitop_ladder_20261019` (`PROTOCOL.md` Phase A).

*Commands* `run_sessions.sh` via task cards 1–6, `cards/CARD-bitop_ladder-20261019-{1..6}.yaml`, with receipts and assessments beside them.
- Card 1 was stopped at 30 min by the harness; its four interrupted sessions were set aside and re-run unchanged by card 2 (Deviation 1).
- Cards 4–6 ran the selection extensions.

*Binary* `target/release/ecbench`, sha256 `372ed2eb2f19c5fd17168188c68cae57abc581edbf4d2a07ad0e3e8a0950a21a`.

*Host* env class `ECBENV2he1389885ccd0`: Intel Xeon \@ 2.10 GHz, 4 vCPU, Linux 6.18. Isolation L0 (`--cpus none`), up to four sessions at once.

*Workloads*
- Curves: `curves.jsonl`, one per size from 16 to 40 bits.
- Targets: public, target seed 20261019100; 8 a curve up to 34 bits, 4 above.
- Measurement seed: 20261019101.

*Arms*
- `rho-neg`;
- `pair-N<size>`: `mitm:negation_folded=1`;
- `rf3-N<size>`: `rootfind-subtract:m=3`, 16–28 bits.

The candidate grids are in `PROTOCOL.md`.

*Runs*
- 46 sessions and 2,296 runs: 2,295 verified, 1 crashed.
- The crash was `train-P39`, `pair-N617`, target 0: exit signal 9 after 284 s at a peak of 6.8 GB, most likely the kernel's out-of-memory killer while four sessions shared 15 GB.
- Every session's `verify.json` is `ok`.

*Measured values* (bitop/v1 from `ecbench work`; all mean bitops per candidate are in `select/SELECTION.md`): the selected base per size is in `params.json`.

#figure(image("train_curves.svg", width: 95%), caption: [Mean bitop/v1 per solve against base size, pair table at 20, 28, 34 and 40 bits; 8 train targets per point (4 at 40 bits). Sessions `sessions/train-P20`, `-P28`, `-P34`, `-P40` and their extensions.])

*Raw records* `sessions/train-P*/records.jsonl`.
