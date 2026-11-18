# 2026-11-18: root-finding kernel, levers 2–4 (PR #4708)

## 08:10Z: levers 2–4 measured

### What was done
- Wrote `examples/rootfind_lever_probe.rs`. It contains:
  - a lockstep K-target lever-1 kernel (K = 1, 2, 4, 8, 16) with a
    branch-free, u128-free Montgomery reduction;
  - AVX-512 variants detected at runtime;
  - filter-only timing at 16, 8 and 4 bits per key, plus the exact set;
  - a fused L1-block probe and a multiply-mix exact set;
  - threads over targets.
- Ran it 10 times at A = 256 and A = 1024 on the 32-bit prime instance, with
  4096 targets and best of 5 passes (commit `5209aebb91`). In the same
  session on the same host, also ran the lever-1 floor probe 5 times and the
  library arms (`rootfind_baseline`, 5 rounds).
- Wrote `research/rootfind_levers_20261118/`: RESULTS.md, the raw JSONL,
  summary.json, three SVG/PNG graphs and report.pdf (built by
  `make_report.py` and `render.js`).
- Added a status section to `research/rootfind_baseline_100x_20261117/PLAN.md`.

### Results (`[pcc:unverified]`, shared host, medians over 10 runs)
- Correctness: 0 group-law mismatches, 0 lane mismatches, 0 mismatches
  between the AVX-512 and scalar builds; every pipeline found the reference
  hits (78 and 5163).
- Lever 2:
  - The scalar-target build is falsified: 4 lanes at 4.92 ns is above the
    4 ns falsification line.
  - The AVX-512 build with 16 lanes reaches 1.89 ns arithmetic, against
    5.64 ns for one lane.
- Lever 3 confirmed: the 8-bit/key (1 MiB) filter probes in 2.29 ns, against
  5.99 ns for the 16-bit/key (2 MiB) filter.
- Lever 4 confirmed: 3.9x at 4 threads.
- Stack at A = 1024: 1.685 ns/point, against 109.9 ns for `rootfind-mitm`
  (65x) and 34.9 ns for `rootfind-subtract` (20.7x), both measured on the
  same host.

### Decisions made
- Re-measured the library arms on this host, because the container restart
  moved the session to a different CPU (2.30 GHz, 504 MiB L3). The #4619
  numbers are not comparable across hosts.
- Labelled the fused-probe and mix-hash arms post hoc: they were added after
  the exploratory runs and were not in PLAN.md.
- Added no leaderboard or `benchmarks/README.md` row. S does not change, and
  the validation gate needs a replay certificate and an isolated runner,
  neither of which is available here.

### Open threads
- The probe is now the bound: 1.9 ns arithmetic out of 6.2 ns on one thread.
  Next step: prefetch or range-batched probes and a smaller `pre` working
  set.
- An explicit-intrinsics lane kernel. The compiler vectorized 16 lanes but
  not 8, so the current gain depends on its choices.
- Land the kernel as an oracle id with witness recovery and the ledger
  parity test.
- Replay on an isolated runner when AWS works.

### Errors
- No checkpoint upload path (`scripts/checkpoint.sh push`: no AWS profile,
  no token). The probe was run locally without a pushed checkpoint, and the
  resume note was kept in the scratchpad, because the tracked
  `.resume/RESUME.md` holds another session's note.
- A git worktree for #4691 filled the disk. Removed it and freed the debug
  target (449 MB).
- Python Playwright is absent, so the renders used the Node package
  (`render.js`).

## 09:35Z: probe-side rounds 2–3

### What was done
- Added probe-side arms to `examples/rootfind_lever_probe.rs`, interleaved
  round by round:
  - software prefetch of each block's filter words;
  - a 32-bit prefix array;
  - block sizes 8, 32 and 128;
  - a one-multiply filter slot function.
- Ran 10 runs of round 2 (commit `fbe3f39088`) and 10 runs of round 3
  (`31bfc33071`, thread sweep on the fastest configuration). Data is in
  `research/rootfind_levers_20261118/probe-side/`; RESULTS.md, the graphs
  (new `probe-side.svg`) and report.pdf are regenerated.

### Results (`[pcc:unverified]`, A = 1024, medians of 10)
- Prefetch: 6.85 against 6.71 ns, slower.
- 32-bit `pre`: 6.29 ns, minor.
- One-multiply hash: 5.09 ns, 1.32x. The filter alone goes from 2.32 to
  1.71 ns per probe at the same 3.4 % false-positive rate.
- With the one-multiply hash, 4 threads reach 1.365 ns/point: 80.5x
  `rootfind-mitm` and 25.5x `rootfind-subtract`, both on the same host.
- At A = 256 the gain is on one thread only (4.36 to 3.72 ns); 4 threads
  stay at 1.18 ns.

### Decisions made
- Rewrote the round-1 "L2 pressure" explanation in RESULTS.md: the controls
  refute it, and the probe is instruction-bound.

### Open threads
- Vectorize the slot computation and use a gather (AVX-512) for the probe.
- The explicit-intrinsics arithmetic kernel.
- The oracle id with witness recovery and the parity test.

### Errors
- None.
