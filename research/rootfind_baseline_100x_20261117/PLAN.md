# Plan: chasing 100x in the root-finding kernel

Date: 2026-11-17. Based on `RESULTS.md`. Predictions are computed from the
measured shares there; each has a test that would falsify it.

## What "100x" is measured against

Two reference points on the 32-bit prime curve at A = 1024, one thread:

- the default oracle `rootfind-mitm`: 133.6 ns per signed point;
- the best existing CPU arm `rootfind-subtract`: 45.1 ns per signed point.

100x means about 1.3 ns per point against the first and about 0.45 ns against
the second.

One caveat governs all of this. These levers change the time and the field
work per addition. They do not change the number of charged additions, so
the harness's counted cost S and the leaderboard rows do not move. Under the
repository's success criterion a wall-time gain with unchanged S must be
reported as a constant-factor engineering result, in time and in priced field
operations, and not as an index-calculus improvement.

## Ranked levers

| # | Lever | Predicted | Basis | Cheapest falsifying test |
|---|---|---|---|---|
| 1 | Shared denominators, abscissa only, Montgomery-resident base, one inversion per target | 2.2–3.1x over `rootfind-subtract` | **Measured** by the prototype | Done: 10.0–14.4 ns against 24.9–45.1 ns |
| 2 | Run 8 targets side by side (AVX-512; the host has `avx512ifma`) | arithmetic 7.7 ns to 1–2 ns per point | The inverse chain is serial, so the scalar code waits on multiply latency; 3.5 products per point at vector throughput is about 1 ns | Interleave 4 targets in scalar code first. If arithmetic stays above 4 ns per point, latency is not the bound and the vector work is not worth doing |
| 3 | Keep the probe in the L2 cache at large A: a 9-bit-per-key fuse filter, or probes grouped by key range across a batch of targets | probe 6.7 ns to about 2 ns per point at A = 1024 | Probe cost is 2.0 ns at A = 256 (128 KB filter) and 6.7 ns at A = 1024 (2 MB filter) | Time the filter alone on recorded keys at both sizes. If a 1.2 MB filter still costs over 4 ns, the bound is not cache size |
| 4 | Threads over targets (the table is read-only) | up to 3.5x on this 4-vCPU host | Targets are independent; earlier multithread runs gave 0.4–6.2x (`SURVEY.md`), so this is not assured | Run the prototype on 1, 2 and 4 threads at A = 1024. Below 2.5x at 4 threads means memory bandwidth binds |
| 5 | Table build: batched abscissa-only additions written straight into the compact table | build 92–105 ns to about 10–15 ns per pair | A batched addition costs 11–29 ns; the rest is the hash-map insert | Build the prototype's key array with the batched kernel and time it |
| 6 | Binary curves: use the batched builder for the `rootfind-mitm` table, and the same filter in front of the probe | build 86–122 ns to about 15 ns per pair; per point 25–41 ns to about 15 ns | Batched addition is 11–12 ns; the probe is the larger share | Swap in `build_negation_folded_binary_batch` and time `prepare` |
| 7 | GPU | 21–121x one CPU thread on the relation phase | Already measured on an A100 (`SURVEY.md`) | Re-measure against the faster CPU kernel, since the baseline it was compared with moves |

## Where the stack lands

Multiplying the levers at A = 1024, prime 32-bit:

| Stage | ns per point | vs default | vs best existing |
|---|---|---|---|
| Default `rootfind-mitm` | 133.6 | 1x | — |
| Best existing `rootfind-subtract` | 45.1 | 3.0x | 1x |
| Lever 1 (measured) | 14.4 | 9.3x | 3.1x |
| Levers 1–3 (predicted) | 3–4 | 33–45x | 11–15x |
| Levers 1–4 on this 4-vCPU host (predicted) | about 1 | 115–155x | 40–50x |

So 100x against the default oracle looks reachable on CPU alone, if levers 2
to 4 hold. 100x against the best existing arm does not: the CPU stack predicts
40–50x here, and the rest would have to come from more cores or the GPU.
None of the predicted rows is a result until measured.

## Order of work

1. Land lever 1 as a new oracle id beside `rootfind-subtract`, with witness
   recovery and the ledger, and a parity test against `rootfind-mitm` (same
   verdicts, same witnesses, same charged additions).
2. Run the three cheap tests for levers 2, 3 and 4 in the prototype. Each is a
   few dozen lines and decides whether the larger work is worth doing.
3. Lever 5 and lever 6 (table builds), which are independent of the above.
4. Extend the field to 52 bits (IFMA lanes) and to 64 bits, and measure how
   much of the gain survives. The prototype covers only primes below 32 bits.
5. Replay the baseline and the winning arm on the lab's isolated runner once
   AWS is available, so the figures can stop being `[pcc:unverified]`.

## Risks

- **Field size.** Every measured number is for a prime below 32 bits. Wider
  fields cost more per product and may shift the shares.
- **Hits.** The prototype times refutations. A workload with many hits spends
  time in witness recovery, which none of this speeds up.
- **Noise.** The host is shared; A/A spread was 0.89–1.06x. Predicted factors
  near that band need an isolated host to resolve.
- **The exponent.** Earlier studies found that root finding's gap to rho is
  set by its exponent (`SURVEY.md`). A 100x constant moves the crossover by
  a few bits of group size and does not close the gap at cryptographic sizes.

## Status, 2026-11-18 (levers 2–4 tested)

Measured in `research/rootfind_levers_20261118/RESULTS.md` (`[pcc:unverified]`,
shared host, 10 runs, library arms re-measured on the same host):

- Lever 2: falsified in scalar-target code (4 lanes 4.92 ns > 4 ns), but an
  AVX-512 build with 16 targets in lockstep reaches 1.89 ns/point arithmetic,
  inside the predicted 1–2 ns.
- Lever 3: confirmed. The 8-bit/key 1 MiB filter probes in 2.29 ns against
  5.99 ns for the 16-bit/key 2 MiB one, at A = 1024.
- Lever 4: confirmed, 3.9x on 4 threads.
- Stack at A = 1024 on that host: 1.685 ns/point, which is 65x the default
  `rootfind-mitm` (109.9 ns) and 20.7x `rootfind-subtract` (34.9 ns). The
  prediction was about 1 ns.
- The probe is now the bound (arithmetic is 1.9 of 6.2 ns on one thread).
  Probe-side rounds 2–3 found that prefetch is slower and a 32-bit prefix
  array is minor, but a one-multiply filter hash gives 1.32x. The stack is
  now 1.365 ns/point on 4 threads: 80.5x `rootfind-mitm` and 25.5x
  `rootfind-subtract`. The probe is instruction-bound.
- Round 4 vectorizes the probe with AVX-512 (hash, gather and compare 8 keys
  at a time). On 4 threads, against library arms re-measured at the same
  time, it reaches 0.867 ns/point at A = 256 (116x `rootfind-mitm`, 22.4x
  `rootfind-subtract`) and 1.172 ns at A = 1024 (96.6x and 32.8x). The
  median crosses 100x at A = 256. The run spread (about 75–120x) and the
  4-thread against 1-thread comparison mean it is not settled.
- Next: a smaller or partitioned filter at large A (the 1 MiB gather is
  L2-latency bound), the explicit-intrinsics arithmetic, then step 1 of the
  order of work (oracle id, witness recovery, parity test).

## Status, 2026-11-18 (next 100x: the k-sum relation phase)

`research/ksum_fast_ic_20261118` re-implements the generic-track incumbent,
`ksum:m=5`, end to end on batched Montgomery/AVX-512 arithmetic. All
numbers are `[pcc:unverified]`: 48 targets, 2 rounds, the same curve and
host. Against the library pipeline (97.2 ms a solve):
- with the table charged to every solve, as the pipeline does: 21x on one
  thread, 38x on 4 threads at A = 19, and 51x at the re-tuned A = 17;
- with one table per curve (multi-target accounting): 140–168x.

The counted cost S is unchanged (7.7x rho remains).
