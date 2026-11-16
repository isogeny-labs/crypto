# Prime IC relation phase: why wide batching stops paying, and the joint tuning (2026-10-28)

Extension of lead `ICP-prime-ic-constant-engineering` (stacked on PR #2783, which measured 16/64/256/1024 lanes).
Report: [`REPORT.pdf`](REPORT.pdf) (source `report.html`, figure `grid.svg` / `grid.png`). Benchmarks entry:
[`benchmarks/20261028-ic-wide-ext`](../../benchmarks/20261028-ic-wide-ext/).

## What was done

1. **Lane-stage diagnosis** (`diagnosis/`): one timer-instrumented session per curve (28 and 32 bit), every width 4..1024 and
   `walk-prime-fast`, with the four stages of a lane round and the loop body (target draw, guard insert, oracle probe) timed
   (marker file `/dev/shm/lane_diag.on`; off by default, no counter without it). The timers are wall-clock counters, so these two
   sessions are descriptive, not replayable, and not in the certified headline.
2. **Old binary against new binary** (`sessions/ab-old-*`, `ab-new-*`): three alternated sessions of `walk`, `walk-prime-fast` and
   16/32 lanes at 28 bits. The new binary has round 2's three `ic_boundary.rs` speedups restored (the 10-26 merge `99165c165d` had dropped
   them from the #2462 branch) and a 32-bit word-Euclid path in `Fp64::inv` for moduli below 2^32.
3. **Joint tuning grid** (`sessions/grid-*`): base size x (walk, 16 lanes, 32 lanes) x oracle (`mitm-prime-batch`, `mitm-prime-blocked`)
   at 28 bits (bases 256-2048) and 32 bits (512-4096), two sessions each.

## Results

- **Cost split, 28-bit, ns/trial at 16 lanes:** additions with the shared inversion 45.9, touch 32.9,
  lane preparation 18.0, queue 4.9, loop body 157.1
  (draw 19.9, guard insert 55.1, oracle probe 82.0), total 330.0; the plain walk is 370.4.
- **The shared inversion is amortised by about 32 lanes**; the additions stay at about 40-46 ns/trial beyond that.
- **Two costs grow with the width:** lane start-up (two scalar multiplications a lane, about 12 microseconds; 1024 lanes cost about 12 ms, a quarter
  of a 28-bit relation phase, under 5% of a 32-bit one) and cache reach (the round's touched lines no longer stay in L1; the guard insert and
  oracle probe slow down at 1024 lanes). Neither is present at 32 lanes, so neither can beat the knee; a smaller lane layout was not built.
- **The floor is hash-table work:** the guard insert and the oracle probe are about 137 ns/trial at 28 bits, 140 at 32, and do not depend on the width.
- **`walk-prime-fast` is fixed.** Relation-phase time against the plain walk: 1.658 (interval 1.58-1.74) before, 1.042 (0.996-1.090) after;
  new/old relation phase 0.625 (interval 0.594-0.659). Cause: `Fp64::inv` used the binary extended Euclid on 64-bit words,
  the plain walk's step uses 32-bit word Euclid for small moduli. 240 of 240 records are identical outside timing;
  the walk and the lane arms are unchanged within noise.
- **Joint tuning: the optimal base does not move.** 28 bits: base 512 is best for every method (51.4 ms with 16 lanes,
  62.5 ms for the walk). 32 bits: base 1024 (332 ms with 32 lanes, 335 with 16, 533 for the walk;
  32 against 16 lanes is within noise). The cache-blocked table halves the build at 2048 and above but probes slower; it wins only at base 4096.

## Not done

- No 34/36-bit ladder: the harness's `prime_search` takes 8 <= bits <= 32; larger curves need another curve family.
- Chunked inversion and software prefetch: the pair table is a std hash map whose slot addresses are not exposed, and the touch loop is already
  the overlapped-load form; an adaptive width has no room above a flat 16-48 optimum.
- The grid has two sessions per size (not three) to fit the CPU budget; its intervals are therefore wide. The diagnosis has one session per size.
- The 28-bit grid's "incumbent" column is the base-1024 walk (the base the round-3 default uses), which is not the 28-bit optimum; compare same-base cells.

## Validation

- Every run verified; `ecbench verify --replay-all` on the ten certified sessions ([`replay-certificate.json`](replay-certificate.json), receipts in `verify/`):
  1,176 replays, all identical; the old/new sessions are replayed by the other binary.
- The section 5a band is identical from both binaries (`band/ab-old.json`, `band/ab-new.json`; tables in `band/raw/`).
- [`validator.py`](validator.py) recomputes all 100 headline numbers from the raw records without sharing code with `summarize.py` ([`validator-report.json`](validator-report.json): pass).
- `python3 scripts/validation_gate.py evidence benchmarks/20261028-ic-wide-ext`: valid.

## Reproduce

```sh
python3 make_specs.py
DEADLINE=$(date -d +40min +%s) ./run_batch.sh     # needs BIN tags ecbench-l2-diag (old) and ecbench-l2-split (new) in /dev/shm/bins
for s in sessions/*/; do ecbench verify --dir $s --replay-all --exit-code --out verify/$(basename $s).verify.json; done
python3 make_certificate.py && python3 summarize.py && python3 validator.py
```

Host: Intel Xeon @ 2.10 GHz (family 6 model 207, AVX-512), 4 vCPU cloud container, `--cpus auto --allow-busy` (below L2); every wall-time session ran under an exclusive CPU lock.
