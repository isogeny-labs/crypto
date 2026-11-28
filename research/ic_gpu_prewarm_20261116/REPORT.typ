#set page(margin: 2cm)
#set text(font: "Liberation Sans", size: 10.5pt)
#set par(justify: true)
= Starting the fes-f2-gpu worker early: host A/B of the mechanism

_Engineering note, not a result on the index-calculus cost. Wall times are a practicality note (AGENTS.md section 6). A host run with a stand-in for the GPU's start-up, not a GPU measurement. All figures are unverified assertions of the producing session (pcc:unverified)._

== Question

`research/ic_gpu_wake_probe_20261116/` found that the CUDA serve worker takes about 150 ms to initialise before it reads its first request, and that the first `fes-f2-gpu` call waits for it: the remainder that earlier looked like 0.085 ms per call. Can that wait be hidden by starting the worker earlier, while `ic bench` builds the instance, calibrates and runs the rho reference?

== Change

`prewarm_shared_server()` in `src/cryptanalysis/fes_gpu.rs` starts the process-wide serve worker without waiting for it (spawning a process returns before the worker has initialised); it does nothing if a worker is running or none is found, and `ICX_FES_PREWARM=0` turns it off, so one binary can be compared with itself. `ic bench` calls it right after its configurations are parsed, only when a configuration will use the worker (oracle `descent-algebraic` with solver `fes-f2-gpu`). A test starts a worker that sleeps 0.4 s before it execs the real one and checks that the prewarm returns in under 0.3 s, that a second prewarm reuses the same process, and that the first solve after it equals the CPU search.

== Method

One release `ic` binary, the host worker `gpu/fes/fes_solve` behind a wrapper that sleeps 150 ms before it execs it (standing in for a CUDA context's initialisation; no GPU), the Koblitz n=19 sweep with two `fes-f2-gpu` rows (310 and 476 calls), `--rho-runs 1`. Three arms interleaved over four repeats with the order rotated: prewarm on, prewarm off, and prewarm off again as the noise floor. `run_ab.sh`, `analyze.py`.

== Results

#table(columns: (auto, auto, auto, auto, auto), align: (left, right, right, right, right),
  [*wall, s*], [*off*], [*off again*], [*on*], [*off - on*],
  [first `fes-f2-gpu` row (310 calls)], [0.3352], [0.3345], [0.1624], [0.1728],
  [second row (476 calls; worker already running in every arm)], [0.2791], [0.2501], [0.2378], [0.0413],
  [whole run (first "on" run 0.863, cold; the other three 0.62 to 0.63)], [0.8608], [0.8313], [0.6837], [0.177],
)

The first row is 0.173 s shorter with the early spawn; two runs of the same "off" arm differ by 0.0007 s, so the noise floor on this row is small against the effect. The saving is a little larger than the 150 ms stand-in delay, which is plausible because the early spawn also overlaps the process start. The second row differs by 0.041 s against a spread of 0.029 s between the two "off" arms: no effect is claimed there, and none is expected, since the worker is already running in every arm. Counted fields (calls, counted ops, points, hit counts, verification tests) are identical in 24 of 24 compared rows (against the first run of the same row, whatever the arm).

#figure(image("fig_prewarm.svg", width: 80%), caption: [Wall time of the first `fes-f2-gpu` row per run, early spawn on and off.])

== What this does not show

- *No GPU measurement.* The 150 ms is a sleep standing in for a CUDA context's initialisation (about 150 ms per process on the A100, from the fits in the wake-up note). A real CUDA worker initialises on the CPU and driver while the parent runs the rho reference on other threads, so contention may reduce the gain; the A100 value needs its own A/B, with the first call timed directly.
- *Only short runs gain.* The saving is one start-up per `ic` process. It is most of a short `ic bench` row (a 1006-call A100 row took 0.19 s) and negligible for a long run.
- No effect on the repository's counted metric. Four repeats per arm, one host.

== Files

`run_ab.sh`, `analyze.py`, `make_fig.py`, `results.json`, `koblitz-n19-gpu.json`, `fig_prewarm.svg`, `raw/`.
