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
| + lever 4: 4 threads | **1.685** [1.61, 3.57] | **65x** | **20.7x** |

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
1.7 ns unexplained inside the pipeline. The likely cause is L2 pressure:
the 16-lane `pre` array is 128 KiB and shares the core's 2 MiB with the
filter, and this host has no PMU to confirm it. **The probe, not the
arithmetic, is now the bound**, so further vector work on the arithmetic
cannot pay much.

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

## Next levers (in order)

1. **Probe-side.** Software prefetch or a batched, range-grouped probe so
   the filter's L2 misses overlap, plus a shrunken `pre` array (block the
   forward pass). Target: the pipeline toward its 1.9 + 2.3 ns component
   floor, about 4.5 ns on one thread and about 1.2 ns on 4.
2. **Explicit-intrinsics lane kernel**, for robustness and for 52-bit IFMA.
3. **Land the kernel as an oracle id** with witness recovery and the ledger
   parity test (PLAN.md order of work, step 1). That parity test is what lets
   this kernel replace `rootfind-subtract` in ecbench sessions.

## Files

- `runs.jsonl`, `summary.json`, `summ.py`: the 10-run evidence and its
  summary.
- `floor-thishost.jsonl`, `baseline-thishost.jsonl`: lever 1 and the library
  arms on the same host.
- `exploratory/`: the first five runs, made while arms were being added
  (scalar-target build, then AVX-512, then fused). They are kept for the
  record but are not part of the summary.
- `stack.svg`, `stack-256.svg`, `lanes.svg`, the PNG renders and
  `report.pdf`: built by `make_report.py` from the rows above.
