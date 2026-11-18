# Root-finding kernel, levers 2–4: falsifying tests

Date: 2026-11-18. Follows `research/rootfind_baseline_100x_20261117/PLAN.md`
(merged in #4619), whose predictions and falsification lines are used here
unchanged. Every number is `[pcc:unverified]`. The host was shared and
unisolated, and nothing has been independently replayed.

**What this is.** Constant-factor engineering of the time per signed point
in the root-finding decomposition oracle, on a 32-bit prime curve. The
number of charged additions is the same, so the counted cost S, the matched
rho gap and the index-calculus leaderboard do not move. This is not an
index-calculus improvement under the success criterion, and no leaderboard
or `benchmarks/README.md` row is added: the validation gate needs a replay
certificate and an isolated runner, and neither is available to this
session.

## Setup

- **Code.** `examples/rootfind_lever_probe.rs` at commit `5209aebb91`
  (`source-commit.txt`; binary hashes in `probe.sha256`). The lever-1 rows
  come from `examples/rootfind_floor_probe.rs` and the library arms from
  `examples/rootfind_baseline.rs`, all built and run on the same host in the
  same session.
- **Instance.** `prime_instance_for(32, 59297)` (p = 3466697909), base
  `prime-abscissa:size=256` and `size=1024`, 4096 random targets (refutations),
  each timing the best of 5 passes.
- **Repetitions.** 10 full runs of the probe (`runs.jsonl`, summarized in
  `summary.json` by `summ.py`), 5 runs of the lever-1 probe
  (`floor-thishost.jsonl`), and 5 interleaved rounds of the library arms,
  including the A/A copy (`baseline-thishost.jsonl`).
- **Host** (`host.txt`). 4 vCPU Intel Xeon at 2.30 GHz, AVX-512 (f, dq, vl,
  ifma), 2 MiB L2 per core, shared and with no PMU. This is not the host of
  #4619's baseline (2.1 GHz, 260 MiB L3), which is why the library arms were
  re-measured here and every ratio below is taken against this host's own
  rows.

## Correctness controls (all 10 runs, both sizes)

- The scalar kernel matched the library's group law on every abscissa of the
  first 64 targets: 0 mismatches.
- Every lockstep lane matched the scalar kernel: 0 mismatches. The AVX-512
  build matched the default-target build bit for bit: 0 mismatches.
- Every timed pipeline, fused or not, through every filter and with every
  thread count, found exactly the reference number of table hits: 78 at
  A = 256 and 5163 at A = 1024.

## Results against the preregistered falsification lines

All values are medians over 10 runs, with [min, max] in brackets.

| Lever | Prediction (PLAN.md) | Falsified if | Measured, A = 1024 | Verdict |
|---|---|---|---|---|
| 2: targets side by side | arithmetic 7.7 → 1–2 ns/point | 4 interleaved lanes stay above 4 ns | default target: 4 lanes 4.92 ns, best 4.91 (8 lanes), 16 lanes 21.1 ns. AVX-512 build: 16 lanes **1.89** [1.81, 1.95] ns against the 5.64 ns single lane | the scalar-target version is **falsified** (latency is not the only bound). The AVX-512 16-lane version **meets the prediction** |
| 3: L2-resident filter | probe 6.7 → ~2 ns at A = 1024 | a ~1 MB filter still costs over 4 ns | filter alone: 16-bit/key (2 MiB) 5.99 [4.03, 6.60] ns, 8-bit/key (1 MiB) **2.29** [1.83, 3.15] ns, 4-bit/key (512 KiB) 1.66 ns but 15.7 % false positives | **confirmed**. The 2 MiB filter is exactly this core's L2 size |
| 4: threads over targets | up to 3.5x | under 2.5x at 4 threads | fused pipeline 6.57 → **1.685** [1.61, 3.57] ns/point, **3.9x** (2 threads 1.95x). At A = 256: 3.75x | **confirmed** |

Two arms were added after the first exploratory runs (`exploratory/`) and
were not in PLAN.md, so treat them as post hoc:

- **Fused L1 probe blocks.** Every 32 abscissae per lane are probed while
  they are still in L1, instead of being written to a `2 A K` row buffer
  that competes with the filter for L2. This took the one-thread pipeline
  from 7.84 [6.60, 8.69] to 6.65 [5.33, 7.77] ns/point.
- **Multiply-mix hasher for the exact set.** It replaces SipHash, which
  costs 21.7 ns a probe at 1M keys. It took the pipeline from 6.65 to
  6.18 [5.12, 7.11] ns/point. Without the filter, the exact set alone is
  slower: 9.19 ns/point.

## The stack, A = 1024, same host

| Stage | ns per signed point | vs default `rootfind-mitm` | vs `rootfind-subtract` |
|---|---|---|---|
| `rootfind-mitm` (library default) | 109.9 | 1x | — |
| `rootfind-subtract` (best existing arm) | 34.9 | 3.2x | 1x |
| lever 1: shared denominators, scalar, 16-bit filter | 11.6 [9.85, 12.44] | 9.5x | 3.0x |
| + lever 2: 16 targets, AVX-512 (16-bit filter) | 9.49 [6.85, 10.17] | 11.6x | 3.7x |
| + lever 3: 8-bit filter | 7.84 [6.60, 8.69] | 14.0x | 4.5x |
| + fused L1 probe blocks (post hoc) | 6.65 [5.33, 7.77] | 16.5x | 5.2x |
| + mix-hash exact set (post hoc) | 6.18 [5.12, 7.11] | 17.8x | 5.6x |
| + lever 4: 4 threads | 1.685 [1.61, 3.57] | 65x | 20.7x |
| round 3: one-multiply filter hash, 1 thread (post hoc) | 5.18 [4.46, 5.36] | 21.2x | 6.7x |
| round 3: + 4 threads | 1.365 [1.15, 2.04] | 80.5x | 25.5x |
| round 4: AVX-512 gather probe, 1 thread (post hoc) | 3.97 [3.75, 4.26] | 27.7x | 8.8x |
| round 4: + 4 threads | **1.172** [1.04, 1.46] | **93.8x** | **29.7x** |

The two round-3 rows come from a later build (`probe-side/`, commit
`31bfc33071`) on the same host and in the same session. In that build, the
same fused kernel with the old hash ran at 6.96 [5.53, 7.25] ns on one
thread.

At A = 256 (`stack-256.svg`) the same stack runs from 97.0 ns (default) and
18.8 ns (`rootfind-subtract`) down to 1.156 [1.10, 1.81] ns/point on 4
threads: 84x and 16x. At this size the filter fits in L2 either way, so
the 16-bit filter wins (4.10 ns fused against 4.34 for the 8-bit one).

PLAN.md predicted "about 1 ns" for levers 1–4 on this host, which would be
115–155x against the default arm. The measured 1.685 ns falls short of that
by about 1.7x, for the reason in the next section.

## Where the time goes now

At A = 1024 on one thread, the arithmetic takes 1.89 ns of the 6.18 ns
point. Probing the 1 MiB filter on its own costs 2.29 ns, and the exact
check on 3.4 % false positives costs about 0.3 ns. That leaves roughly
1.7 ns unexplained inside the pipeline. After round 1 I suspected L2
pressure (the 16-lane `pre` array is 128 KiB and shares the core's 2 MiB
with the filter). Rounds 2–3, below, mostly refute that: prefetch slows the
pipeline, and halving `pre` gains only 0.4 ns. A cheaper hash gains 1.6 ns,
so the probe is bound by its instruction count. **The probe, not the
arithmetic, is the bound**, so further vector work on the arithmetic cannot
pay much. This host has no PMU, so these are inferences from controlled
arms, not counter readings.

## Not settled

- **AVX-512 vectorization depends on the compiler.** The compiler vectorized
  16 lanes but not 8 lanes (4.98 ns). An explicit-intrinsics kernel (u32
  lanes with `vpmuludq`, or IFMA for 52-bit fields) would make the gain
  robust.
- **Field size.** Everything here is below 32 bits. 52- and 64-bit fields
  change the product cost and may shift the bound back to the arithmetic.
- **Hits.** The timings are for refutations. Witness recovery on hits is not
  measured.
- **Noise.** Single-run spread was up to 2x (lever 4 at 4 threads:
  [1.61, 3.57]). The medians are stable, but the ratios should be replayed
  on an isolated runner before anyone cites them.

## Rounds 2–3: the probe side (post hoc, after round 1)

Round 1 left the one-thread pipeline at 6.2 ns against component costs of
1.9 + 2.3 ns. I tested three hypotheses for the gap, one change at a time.
The arms were interleaved within each repetition, with 10 runs each
(`probe-side/round2-runs.jsonl`, commit `fbe3f39088`; thread sweep in
`round3-runs.jsonl`, commit `31bfc33071`).

| Arm, A = 1024, one thread | ns per point | Reading |
|---|---|---|
| fused (round-1 kernel) | 6.71 [6.25, 7.01] | reference |
| + software prefetch of each block's filter words | 6.85 [6.45, 7.07] | **slower**: latency was not the bound |
| + 32-bit prefix array (halves the 128 KiB `pre`) | 6.29 [5.98, 6.58] | small; L2 pressure is minor |
| prefetch + 32-bit + 128-abscissa blocks | 6.72 [6.36, 7.09] | no gain |
| prefetch + 32-bit + 8-abscissa blocks | 6.34 [6.05, 6.64] | no gain over 32-bit alone |
| fused + **one-multiply filter hash** | **5.09** [4.84, 5.25] | **1.32x**: the probe was instruction-bound |

The one-multiply hash takes the slot and four bits from a single
odd-constant product of the key. The keys are uniform field elements
already, so the second mixing multiply and the loop over bit positions
were wasted work. Measured alone, the filter probe falls from 2.32 to
1.71 ns, with the same false-positive rate (3.4 %). With it, 4 threads
reach 1.365 ns/point at A = 1024. At A = 256 the gain is on one thread only
(4.36 to 3.72 ns): 4 threads stay at 1.18 ns, against 1.16 ns in round 1.

## Round 4: the probe in AVX-512 (post hoc)

Rounds 2–3 showed the probe is bound by instructions. Round 4 does the
one-multiply probe 8 keys at a time: one `vpmullq` for the hashes, one
gather for the filter words, variable shifts for the four bits, one
compare. The keys that pass (about 3.4 %) go to the exact set. Data:
`probe-side/round4-runs.jsonl`, 10 runs, commit `214d4e4b11`.

| Arm (16 lanes, 8-bit one-multiply filter) | A = 256 | A = 1024 |
|---|---|---|
| filter alone, scalar probe (ns/probe) | 1.27 [1.23, 1.90] | 1.89 [1.71, 2.43] |
| filter alone, AVX-512 probe (ns/probe) | **0.73** [0.70, 0.89] | 2.08 [1.74, 2.29] |
| pipeline, 1 thread, scalar probe | 3.71 [3.54, 3.88] | 4.73 [4.47, 5.37] |
| pipeline, 1 thread, AVX-512 probe | **3.03** [2.83, 3.20] | **3.97** [3.75, 4.26] |
| pipeline, 4 threads, scalar probe | 1.113 [0.97, 1.57] | 1.378 [1.33, 1.92] |
| pipeline, 4 threads, AVX-512 probe | **0.867** [0.84, 1.34] | **1.172** [1.04, 1.46] |

The filter-only rows show two regimes. At A = 256 the filter (64 KiB) sits
in L1/L2, and the vector probe is 1.7x the scalar one. At A = 1024 (1 MiB)
it is no faster: the gather waits on L2. Inside the pipeline the vector
probe still gains 1.19x at A = 1024, because it frees issue slots for the
arithmetic it is interleaved with.

**Against the library arms re-measured at the same time**
(`probe-side/round4-baseline.jsonl`, two 5-round sessions run right after
the 10 probe runs):
- `rootfind-mitm`: 99.3 and 101.9 ns at A = 256; 113.4 and 112.9 ns at
  A = 1024.
- The A/A copy: 98.8 and 101.0 ns at A = 256; 115.7 and 112.2 ns at
  A = 1024.
- `rootfind-subtract`: 19.0 and 19.8 ns at A = 256; 39.1 and 37.8 ns at
  A = 1024.

The 4-thread AVX-512 probe then stands at:

| | vs `rootfind-mitm` | vs `rootfind-subtract` |
|---|---|---|
| A = 256 | **116x** | 22.4x |
| A = 1024 | **96.6x** | 32.8x |

Read this as a median on a shared host, not a settled 100x:
- The 4-thread spread over the 10 runs is wide (A = 256: 0.84–1.34 ns, so
  about 75–120x).
- One more run taken just after the baselines came in at 1.07 and 1.57 ns
  (`round4-extra-run-after-baseline.jsonl`).
- The reference arms run on one thread; the stack uses 4. PLAN.md defines
  the stack that way, but it is not a like-for-like single-core comparison.

On one core, the kernel is 33x (A = 256) and 28.5x (A = 1024) faster than
`rootfind-mitm`.

## Next levers (in order)

1. **A smaller filter at large A.** With the probe vectorized (round 4),
   the 1 MiB filter is gather-latency bound at A = 1024. A 4- or 6-bit
   filter in L1/L2 plus a cheaper exact stage, or probes partitioned by
   filter range across a batch of targets, is the next thing to test.
2. **Explicit-intrinsics lane kernel**, for robustness and for 52-bit IFMA.
3. **Land the kernel as an oracle id** with witness recovery and the ledger
   parity test (PLAN.md order of work, step 1). That parity test is what lets
   this kernel replace `rootfind-subtract` in ecbench sessions.

## Files

- `runs.jsonl`, `summary.json`, `summ.py`: the 10-run evidence and its
  summary.
- `floor-thishost.jsonl`, `baseline-thishost.jsonl`: lever 1 and the library
  arms on the same host.
- `exploratory/`: the single runs made while arms were being added (runs 1
  to 8). They are kept for the record but are not part of any summary.
- `probe-side/`: rounds 2 to 4 with their binary hashes, source commits and
  host notes, plus the round-4 same-time library baseline.
- `stack.svg`, `stack-256.svg`, `lanes.svg`, `probe-side.svg`, the PNG
  renders and `report.pdf`: built by `make_report.py` (with `render.js`)
  from the rows above.
