# Wake-up probe for the fes-f2-gpu worker: preregistration (frozen before any GPU data)

Question. On an A100 (PR #4380, `research/ic_gpu_ipc_split_20261116/`), the `ic` round trip to the CUDA serve worker
is 0.192 ms per call at 24 unknowns, of which the worker's own busy time is 0.093 ms and the null-worker pipe floor
0.0135 ms; about 0.085 ms (U) is in neither. On the host (`ic` against a CPU worker with the same timer) the same
remainder is about 0.03 ms. What is U?

Instrument. `wake_probe.cpp` replays real captured 24-unknown requests against a worker, one at a time, with an idle
gap between calls, the client either blocking or spinning for the reply, and the worker either blocking or spinning
for the next request (`FES_SERVE_SPIN_US`, `gpu/fes/fes_io.hpp`, default off). Workers: `null_bin` (answers no
solutions: the floor) and `fes_cuda`. Cells: worker {null, cuda} x reply {block, spin} x worker spin {0, 2000} us x
gap {0, 100, 1000} us spinning; 1500 calls per cell, 3 repeats, cell order rotated per repeat. Per cell: client round
trip mean and median; for `fes_cuda` its busy time per call (`FES_SERVE_TIMING`). Derived:
U(cell) = round trip mean - busy - null-worker round trip mean in the same (reply, wspin, gap) cell.

Why this isolates the cause. Both the real run and the null floor use the same pipe, protocol and request bytes; what
differs is that the CUDA worker takes ~0.09 ms to answer while the Rust side blocks in `read`, and then idles for the
inter-call gap (76 to 150 us in the real runs, from row wall time minus solver wall time). On the host, blocking cold
wake-ups cost 70 to 80 us only after about a millisecond of idle; gaps of 76 to 150 us cost at most 15 us there. So a
cold wake-up in the Modal container is a candidate, not an established cause.

Predictions and decision rules (thresholds fixed now; 40 us is about half of U).
- Reproduction check. In the cell reply=block, wspin=0, gap=100 the replay must show U between 50 and 120 us. If not,
  the replay does not reproduce what `ic` sees: the cause is in `ic`'s own work between calls (H4), not in the pipe.
- H1, the client's wait for the reply. U(reply=block) - U(reply=spin) >= 40 us at wspin=0, gap=100. Then the next
  change is an opt-in spin on the reply in `ServeWorker::solve`.
- H2, the worker's wait for the next request. U(wspin=0) - U(wspin=2000) >= 40 us at gap=100 and gap=1000. Then the
  next change is an opt-in spin in the worker.
- H3, inside the CUDA process but outside its busy timer. U(reply=spin, wspin=2000, gap=0) >= 40 us, with both sides
  hot and no idle. Then neither spin helps and the next probe is timestamps inside the worker (read, launch, return).
- If H1 and H2 both hold, U should fall by about the sum when both are spun; if the drops are not additive the two
  waits overlap and either one suffices.
- Null result. If every cuda cell's U is within 20 us of every other and below 40 us, U was specific to `ic`'s run
  and not reproduced here.

What this cannot show. Wall time only, one container and job, three repeats (no A/A beyond the spread of repeat
means), replay of one sweep's requests; spinning in the gap hides what `ic`'s own work does to the caches between
calls. A cell that reduces U is a candidate fix, not a speed-up of the whole `ic` run until measured there.
