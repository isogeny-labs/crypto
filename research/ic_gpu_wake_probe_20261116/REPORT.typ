#set page(margin: 2cm)
#set text(font: "Liberation Sans", size: 10.5pt)
#set par(justify: true)
= fes-f2-gpu on an A100: the "unlocated" 0.085 ms is a one-time start-up, not a per-call cost

_Engineering note, not a result on the index-calculus cost. Wall times are a practicality note (AGENTS.md section 6); the counted cost is unchanged. One Modal job (A100 80GB PCIe, 72 timed cells) plus a re-reading of the 12 runs of the earlier job. All figures are unverified assertions of the producing session (pcc:unverified)._

== Question and result in brief

The earlier A100 note (`research/ic_gpu_ipc_split_20261116/`) reported that on current `main` about 0.085 ms of each call's 0.192 ms round trip is in none of the measured phases, and that a pipe floor of 13.5 µs rules out pipe latency. This note asks what that remainder is. *It is not per call.* The cumulative mean round trip printed every 500 calls follows b + F/n to within 1 µs, with a one-time cost F of about 150 ms per process, the CUDA worker's initialisation, which the first call waits for. Spread over 1878 and 2338 calls that is 80 and 64 µs per call, which is the "remainder". The steady-state call on `main` at 24 unknowns is 108 µs of round trip, which is the worker's busy time (93) plus the pipe floor (13.5) to within 1.5 µs.

== The probe, and what the preregistered rules say

`PREREG.md` (frozen before any GPU data) listed three causes inside a per-call wake-up story: the client's wait for the reply (H1), the worker's wait for the next request (H2), and something inside the CUDA process but outside its busy timer (H3). `wake_probe.cpp` replays 1500 captured 24-unknown requests against the null worker and the CUDA worker, with the idle gap, the client's wait (block or spin) and the worker's wait (block or spin for up to 2 ms) varied: 24 cells, 3 repeats. The unlocated time U is round trip mean minus the worker's busy time minus the null worker's round trip in the same cell.

#table(columns: (auto, auto, auto), align: (left, right, left),
  [*rule*], [*measured*], [*outcome*],
  [reproduction: U in 50 to 120 µs at reply=block, spin 0, gap 100], [94.4 µs], [reproduced],
  [H1: U(block) - U(spin) >= 40 µs at spin 0, gap 100], [7.6 µs], [not supported],
  [H2: U(spin 0) - U(spin 2000) >= 40 µs at gap 100 and 1000], [3.2 and 13.4 µs (block); 1.0 and -8.4 µs (spin)], [not supported],
  [H3: U >= 40 µs with both sides spinning and no gap], [88.7 µs], [as stated, supported],
)

U is between 80 and 108 µs in all 12 CUDA cells, whatever the waits and the gap: nothing about waking up changes it. H3 as preregistered says only that the time is not a wake-up cost; it does not say what it is. The medians do: the median round trip is 97 to 128 µs against a mean of 190 to 240 and a worker busy mean of 90 to 97 µs, so a typical call has almost no unlocated time. The excess sits in a few slow calls, outside the worker's busy window. One slow call of about 150 ms in 1500 adds 100 µs to a mean and nothing to a median.

== The start-up fit

The worker allocates its pinned and device buffers and creates its CUDA context before it reads its first request, so the first call's round trip includes that. `FES_GPU_TIMING` prints the cumulative mean every 500 calls, and both arms of the earlier job logged it. Fitting mean(n) = b + F/n by least squares to the 4 or 5 points of each run (`fit_startup.py`):

#table(columns: (auto, auto, auto, auto, auto), align: (left, right, right, right, right),
  [*arm, instance*], [*b, µs per call*], [*F, ms*], [*runs*], [*max residual*],
  [A (text protocol), 24 unknowns], [168.1], [146], [3], [0.3 µs],
  [B (binary, main), 24 unknowns], [107.9], [158], [3], [0.6 µs],
  [A, 26 unknowns], [251.5], [153], [3], [0.9 µs],
  [B, 26 unknowns], [184.9], [178 (145 to 236)], [3], [0.5 µs],
)

F is the same in both arms (about 150 ms), so it belongs to the worker's start-up, not to a protocol. Each b equals the worker's busy time plus the pipe floor: 93.3 + 13.5 = 106.8 against 107.9 at 24 unknowns, and 168.5 + 13.6 = 182.1 against 184.9 at 26. F is a fit, not a measurement: the first call's latency was not recorded, and the fit assumes a constant per-call cost after it. The next run should time the first call directly.

#figure(image("fig_startup.svg", width: 100%), caption: [Left: cumulative mean round trip against calls with the b + F/n fit. Right: the steady-state call on `main`.])

== What this changes in the earlier note

- *The steady-state call is shorter than reported.* The reported whole-call means (0.501 to 0.203 ms at 24 unknowns, 0.631 to 0.275 ms at 26) include the one-time start-up in both arms. Removing F/n from each mean gives about 0.423 to 0.119 ms (72 percent less) and 0.566 to 0.199 ms (65 percent less). These are derived from the fits and from the earlier means, not new measurements.
- *The steady-state call is fully accounted for.* At 24 unknowns, 118 µs: kernel 69, the worker's copies, launch and reply 24, pipe floor 13.5, Rust pack and verification 10, remainder 1.5. At 26 unknowns, 198 µs: kernel 142, worker other 27, floor 13.6, Rust 13, remainder 2.
- *Where a gain would come from.* In steady state the kernel is 58 to 72 percent of a call. The 150 ms start-up is a per-process cost: it is 40 to 75 percent of the wall of the short `ic bench` rows of these sweeps (a 1006-call row took 0.19 s) and negligible for a long run. It can be overlapped with other work by starting the worker early; that change is not made here and needs its own A/B on the GPU.

== What this does not show

- *The identity check on the CUDA worker is void.* The probe's reply checksum is identical across all null-worker runs and different in all 36 CUDA runs, because the kernel appends solutions in a nondeterministic order that `ic` sorts afterwards. A byte checksum is the wrong test there; equality of answers across modes was not checked on the GPU.
- *The pre-registered decision rules were written on means*; the median and fit analysis above is exploratory and was written after seeing the medians. The fit rests on 4 or 5 points per run, one container, one job, three repeats, no A/A.
- A100 80GB PCIe, a different GPU variant from the 2026-11-13 note. No effect on the repository's counted metric.
- The spin option `FES_SERVE_SPIN_US` added to `gpu/fes/fes_io.hpp` for this probe is off by default and, per the rules above, does not help; it is kept so that the probe reproduces.

== Files

`PREREG.md`, `wake_probe.cpp`, `run_cells.sh`, `job_wake.sh`, `modal_wake.py`, `null_bin.cpp`, `analyze.py`, `fit_startup.py`, `make_fig.py`, `results.json`, `startup_fit.json`, `startup_fit_A.json`, `fig_startup.svg`, and `raw/`.
