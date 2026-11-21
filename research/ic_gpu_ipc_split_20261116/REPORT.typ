#set page(margin: 2cm)
#set text(font: "Liberation Sans", size: 10.5pt)
#set par(justify: true)
= fes-f2-gpu on an A100: what the current call spends its time on

_Engineering note, not a result on the index-calculus cost. One Modal job on an A100 80GB PCIe, three interleaved runs per arm, two instances. Wall times are a practicality note (AGENTS.md section 6); the counted cost is unchanged. All figures are unverified assertions of the producing session (pcc:unverified)._

== Question

The 2026-11-13 A100 note left about 0.2 ms of the 0.37 ms worker round trip that no phase timer located, and guessed at pipe wake-up latency. Meanwhile PR #4101 landed a binary `--serve-bin` protocol and a single-copy CUDA upload, so that note's numbers describe the old protocol. Two questions: how much shorter is a call on current `main`, and where does the remaining time go?

== Method

One A100 container, `gpu/fes` built for `sm_80`, three arms of `ic bench` on the same `fes-f2-gpu` sweeps (24 unknowns: Koblitz n=23, 1878 calls; 26 unknowns: char-2 n=25, 2338 calls), arm order rotated per repeat, `FES_GPU_TIMING=1` and `FES_SERVE_TIMING=1`:

- *A*: `ic` before the direct pack (text `--serve` protocol) with the CUDA worker.
- *B*: current `main` `ic` (direct pack, binary `--serve-bin`) with the CUDA worker; the worker prints its busy time per call (end of the request read to the flushed reply) and the kernel's GPU time from CUDA events.
- *C*: current `main` `ic` against a null worker (`null_bin.cpp`) that parses the same binary request and answers "no solutions": the pipe round-trip floor in the same container. Its answers are wrong on purpose, so it is excluded from the identity check.

Smoke tests before the arms: the text and the binary protocol give byte-identical replies from the CUDA and the host worker; the compiled-in selftest ran. No A/A arm: the spread across the three repeats of each arm is the only noise estimate.

== Results

#table(columns: (auto, auto, auto, auto, auto), align: (left, right, right, right, right),
  [*per call, ms*], [*A, 24 unk.*], [*B, 24 unk.*], [*A, 26 unk.*], [*B, 26 unk.*],
  [whole solver call], [0.5009], [0.2028], [0.6310], [0.2753],
  [Rust prep (forms + pack; pack)], [0.2517], [0.0073], [0.3107], [0.0092],
  [worker round trip], [0.2461], [0.1919], [0.3166], [0.2615],
  [worker busy (B only)], [-], [0.0933], [-], [0.1685],
  [kernel, GPU events (B only)], [-], [0.0690], [-], [0.1416],
  [pipe floor, null worker (C)], [-], [0.0135], [-], [0.0136],
)

Whole call, A to B: 0.5009 to 0.2028 ms at 24 unknowns (59.5 percent less; B's three runs span 0.1969 to 0.2070) and 0.6310 to 0.2753 ms at 26 (56.4 percent less). Of the 0.298 ms saved at 24 unknowns, 0.244 ms is the Rust preparation (the direct pack) and 0.054 ms is the round trip (binary protocol, single-copy upload). Counted fields (verdicts, calls, counted ops, hit rates, relations, verification flags) are identical across arms A and B: 0 of 5 runs differ from the first A run at either instance (`results.json`).

#figure(image("fig_split.svg", width: 100%), caption: [Mean per-call time by phase, A100 80GB PCIe. "Unlocated" is the round trip minus the worker's busy time minus the pipe floor.])

*Where the current call goes (B, 24 unknowns, 0.203 ms):* the kernel is 0.069 ms (34 percent), the worker's copies and reply 0.024 ms, the pipe floor 0.0135 ms, Rust preparation and verification 0.010 ms, and 0.085 ms (42 percent) is in none of those: round trip 0.192 ms minus busy 0.093 ms minus floor 0.0135 ms. At 26 unknowns the same remainder is 0.079 ms of 0.275 ms (29 percent). It is nearly the same at both sizes, so it does not scale with the system.

*The pipe-latency guess is not supported.* The null worker's round trip is 13.5 µs at both sizes, an order of magnitude below the 85 µs remainder. A host measurement agrees (28 µs per call on the authoring machine). What the remainder is has not been measured. Candidates, none tested: the worker thread sleeping between requests and waking cold (the null worker's gap between requests is short, the CUDA worker's is not), `cudaStreamSynchronize` yielding or blocking instead of spinning, and a first-launch latency after the GPU idles between calls. Each can be switched on its own; a second job should A/B them against B.

== What this does not show

- *The A to B difference is not one change.* It spans PR #4062 (direct pack, mine), PR #4101 (binary protocol, single-copy upload, direct `from_poly`; another session) and whatever else reached `main` between the two `ic` builds. Only the phase table, not the totals, attributes it.
- *A different GPU from the 2026-11-13 note* (A100 80GB PCIe here, SXM4-80GB there) and a different host: absolute times are not comparable with that note.
- *No counted-cost effect.* S and the gap to the matched rho do not move; this is the wall time of a call, not a change in the repository's metric.
- One container, one job, three runs per arm, no A/A, no independent replay. The local client that launched the job was stopped at its time limit; the job itself ran to completion (exit code 0) and was fetched by token. The S3 checkpoint gate was waived by the owner for this one run.
- The null worker measures the pipe round trip with both processes hot; it is a floor, not an estimate of what a call pays between real requests.

== Files

`job_ipc.sh`, `run_arms.sh`, `null_bin.cpp`, `modal_ipc.py`, `analyze.py`, `make_fig.py`, sweeps `koblitz-n23-gpu.json` and `char2-n25-gpu.json`, `results.json`, `fig_split.svg`, and `raw/` (every run's log and JSON, `binaries.sha256`, GPU and host description, `launch.json`).
