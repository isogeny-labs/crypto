# Item 2 results: BLOCKED before launch, no pod created, no data [pcc:unverified]

Date 2026-11-15. ml-cryptanalysis e8aecdd, crypto 44d35033fa.

## Step 1 (credential)
`test -n "$RUNPOD_API_KEY"`: set (value never printed). `POD_STATE_DIR` was unset; use `/home/user/.pod-state-item02`
(outside every repository, tokens live there, mode 700). Runpod REST reachable: `ic_bounds_big_pod.py prices` answered.
`create ... --dry-run` for H200 and B200 printed the request bodies; nothing was provisioned.

## Why I did not launch (two measured/planned blockers, either one is enough)
1. Memory. `python3 scripts/ic_bounds_big_run.py plan --k 15 --D 6 --device gpu --dev-gb {105,135,170} --work-gb 20 --tile-mb 1024`:
   target part worst case 145.5 GB (both parts worst: 290.6 GB matrices, predicted peak 313.1 GB, 192,192 / 191,900
   rank at the worst point). `spill.feasible` = False for dev-gb 105 and 135, True only for dev-gb 170. The engine moves
   the whole active part to the device, and an H200 has 141 GB, so the run as specified (one H200) cannot hold the
   active part at its worst rank. Host spill need 143 to 208 GB depending on dev-gb. This is the engine's own planner
   (a worst-case bound), not a trial; the final rank (about 300k per part) is below the worst case, but the rank passes
   through the worst point on the way.
2. Time. The k = 14 H200 closure (proper p = 114689, 499,178 columns, rank 448,922) took 11,677.8 s = 3.24 h
   (relaunch/fetched/proper-p114689-k14-D6.json, measured). estimate.py's madds grow x3.1 per k step at D = 6
   (2.87e14 to 8.90e14 from k = 13 to 14), so k = 15 extrapolates to about 10 h on an H200 (derived, band maybe 8 to 12 h),
   above both the 9 h cap in the request and the "about 6 H200-hours" estimate. A 9 h cap would likely end before the
   sweep, and a pod stopped at the cap loses a container-disk checkpoint.
The request approved H200 spend; a B200 (or other 180 GB device) is a different SKU, an unverified engine target
(cupy build for sm_100, untested here), and a likely larger bill, so I did not switch hardware on my own.

## What is needed from the owner
Decision on (a) B200-class device with 300+ GB host RAM and a 12 h cap (card B in LAUNCH_CARD.md), after a cheap
self-test on that SKU, or (b) a engine change to spill part of the active part (not available in the current code).
Exact next command: see LAUNCH_CARD.md section "Launch".
