# LANE cuckoo: cuckoo filter / cuckoo table for the pair-sum table (p24, |F|=128)

All numbers below are exact counts or measurements from `raw_output.txt` / `model_output.txt`, label **[pcc:unverified]**
(no pcc claim id; 1 shared-CPU host, no replay certificate, no matched-rho band; not a benchmark-record row).

## Verdict
* Lever class: **memory-only** (and a small native-work *cost* at w>0). It does **not** reach counted GAE at w=0:
  the cuckoo structure holds the same 16,256 pair-sum keys (16,232 distinct) that the incumbent's oracle phase
  already computed (16,256 group adds, unchanged), the hit rate per trial is unchanged (exact lookup semantics after
  1-GAE verification), and false positives only *add* verification GAE. Setup GAE and trial count are therefore identical
  or slightly worse. [pcc:unverified]
* Versus the other lanes' structures it is the **densest dynamic** structure (load 0.95 -> 1.05x overhead vs xor 1.23x),
  but a static ribbon (1.05-1.10x) is as dense and cheaper to probe than 2 buckets. Deletion works (zero false negatives
  when deleting *recomputed true keys with value check*) but enables nothing useful in IC: dynamic FB eviction/growth
  measured **worse** than a static base (below). [pcc:unverified]
* Gap to matched rho is unchanged by this lane (incumbent p24 ~56.6k GAE vs rho.negation ~5.0k, from records.jsonl);
  no distance was closed.

## Method
`examples/ic_alt_cuckoo.rs` (self-contained; curve/factor-base/Bloom helpers copied from the xor-ribbon lane's example).
p24: p=16777199, a=16777196, b=31, G=(2,5412647) (asserted on-curve). Base = first 128 on-curve prime abscissae (max x=1801,
279 primes scanned). Keys = x(Pi+Pj), x(Pi-Pj), i<j: 16,256 emitted, 16,232 distinct (24 duplicate abscissae).
Cuckoo: 4-slot buckets, partial-key hashing (fingerprint f bits, nonzero; i2 = (g(fp) - i1) mod nb, an involution, so any nb works),
random-walk kick-out, max 500 kicks unless stated. Table mode stores v=15 value bits (pair index (i,j,sign)) beside the
fingerprint. Queries: 8,000,000 random on-curve x (7,984,442 non-members). Comparators: std HashMap<u32,u32>, Bloom sized for the
same measured FPR (k=round(log2 1/FPR), m=1.4427kn).

Commands (build only this example, no ecbench, no tests):
```
cargo build --release --example ic_alt_cuckoo
cd research/ic_sketch_mixed_solver_20261115/alt/cuckoo
../../../../target/release/examples/ic_alt_cuckoo 8000000 200 > raw_output.txt      # 11.2 s CPU
python3 cuckoo_model.py > model_output.txt   # imports ../xor-ribbon-build/memory_model.py, edits nothing
```

## Results (E1-E5, see raw_output.txt)
**Load factor reached (fill to first failure, 4096 buckets = 16,384 slots, 200 seeds):** mean 0.967-0.969 for f=8/12/16, min 0.958,
max 0.976 at 500 kicks. Max-kick sensitivity (f=12): 100 kicks 0.939, 500 kicks 0.968, 2000 kicks 0.976. Finite size: 512 buckets 0.973,
65,536 buckets 0.963. On the **actual 16,232 pair keys** (f=12, 200 hash seeds): success 200/200 up to load 0.95, 199/200 at 0.96, 45/200 at 0.97, 0/200 at 0.98.

**Kick-outs per insert (random keys, by load band at insertion):** <0.5: 0.004; 0.5-0.8: 0.275; 0.8-0.9: 2.24; 0.9-0.95: 10.9; >=0.95: ~51 (cap 500).
Real key set, whole build: 0.106 (load .80), 0.343 (.90), 0.898 (.95), 1.229 (.96) kick-outs/insert.

**Bits per entry / FPR / probes (real key set, n=16,232, f=12, load 0.95, nb=4272):** filter 12.63 bits/key, table (fp+15 value bits) 28.42 bits/key;
FPR 1.856e-3 (theory 1.854e-3, 14,816 FPs); negative lookup 1.999 buckets / 7.99 slot compares (12.3 ns); positive lookup 1.43 buckets / 4.16 compares.
Other points (FPR measured): f=8 load .95 FPR 2.93e-2, 8.42 b/key; f=16 load .95 FPR 1.18e-4, 16.84 b/key (table 32.63); load 0.5 halves FPR and doubles bits/key.
Comparators: std HashMap 145.4 bits/key (294,928 B est.), packed-ideal exact 24+15=39 bits/key, 9.5 ns/lookup;
equal-FPR Bloom: f=8 (k=5) 7.21 b/key vs cuckoo 8.42 (cuckoo 1.17x *larger*); f=12 (k=9) 12.98 vs 12.63 (0.97x); f=16 (k=13) 18.76 vs 16.84 (0.90x). Bloom negative reads ~2.0 (cache lines) vs cuckoo 2.0 buckets (1 line each).
Exact (FPR=0) quotient-cuckoo: needs nb=2^12 at load 0.991 (infeasible) or 2^13 at load 0.495 = 52.5 bits/key incl. value, i.e. **worse** than packed-exact 39.

**E4 deletion:** evicting FB element #17 means recomputing its 254 keys (254 group adds), then `delete(key,value)`: 254/254 deleted, 0/16,002 surviving
entries lost; inserting a replacement element (254 adds) 254/254 succeeded, load back to 0.8999. Filter-only (no value check) is unsafe: deleting 2,000,000 never-inserted keys
removed 2,868 real entries (1.43e-3, = FPR-order) and left 2,860/16,232 members false-negative.

**E5 dynamic-FB policies (random on-curve R, table f=16, 20 reps; GAE = setup adds + verification adds + 1.0479 x trials, the incumbent c_t):**
| policy | GAE to 87 relations (mean) | trials to 87 |
|---|---|---|
| static F=64 | 201,016 | 187,896 |
| static F=96 | 89,959 | 77,060 |
| **static F=128** | **63,209** | 44,723 |
| static F=160 | 55,770 | 28,860 |
| static F=192 | 57,751 | 20,032 |
| grow 64->, +1 elem/100 trials | 66,563 | 25,456 |
| grow 32->, +1 elem/100 trials | 69,785 | 28,531 |
| grow 64->, +1 elem/500 trials | 83,043 | 52,323 |
| swap F=128, replace random elem / 2000 trials | 74,610 | 44,890 |
| swap F=128, replace least-hit elem / 2000 trials | 74,649 | 44,999 |
Static F=128 hit rate: 15,374 hits / 8.0e6 trials = 1.92e-3 (model 1.938e-3). Split-half per-element hit correlation r=-0.016 (n=2,560): no persistent
per-element signal, so "evict the weak element" has nothing to select on; swaps cost ~11k extra GAE for the same trials. Growth never beats the best static F (same total pair adds, but
hit rate lower earlier); it only helps if F* is unknown in advance (anytime), and still lost to static F=128 here. The static F=160 row is the incumbent model's known fixed-R optimum, not a cuckoo effect.
(The 63.2k static mean vs the incumbent's single-run 56.6k is sampling spread of the 87-hit stopping time plus verification adds; same order.)

**Cost-model translation (cuckoo_model.py; reuses the xor-ribbon lane's model: setup F^2, trials 0.68F/h(F), c_t=1.0479; cuckoo f=12 table = (12+v)/0.95 bits/key, 2 bucket probes/neg lookup, FP verify 1 GAE).** Ratios are vs exact-packed at the same memory budget:
* w=0, p24: budgets >=1 MiB: 1.00x (F*=144 unconstrained; memory does not bind), total 6.26e4 vs 6.25e4 GAE (slightly worse by FP verification). Only at 32 KiB (L1): F=96, 7.22e4 GAE = **1.12x** vs exact-packed 8.05e4 (xor-table 1.06x).
* w=0, p83 (memory-capped regime): 32 KiB 1.83x, 1 MiB 1.74x, 32 MiB 1.64x, 16 GiB 1.58x, 1 TiB 1.49x (xor-table 1.69/1.61/1.52/1.46/1.40x). These are model extrapolations, not measurements, and are exactly the memory-capped-|F| gains that count only where memory binds.
* w=0.1 and w=1 (native probes priced), p24 at >=1 MiB: 0.94x and 0.77x (worse than exact-packed, because 2 buckets/neg lookup vs 1 probe). In the memory-capped L1 case at w=0.1: 1.04x, at w=1: 0.78x.
* So: reaches counted GAE at w=0: **no** (identical setup and trials). Native work (w>0): **negative**. Memory: **yes, 1.1-1.8x model gain only where memory caps |F|**, never at p24 with >=1 MiB.

## Caveats
* Hash functions are a splitmix64 stand-in; real-world placement may differ. Timing (ns) came from a shared 3-core host with other lanes running and is indicative only.
* Fingerprint hits must be verified by recomputing one group add (1 GAE); the model charges it. Membership-only mode needs F adds per candidate (model row [4] is worse everywhere).
* E5 uses random on-curve x as the trial distribution (matches the incumbent's uniform R model: measured hit rate matches theory), not actual aG+bQ trials; relations needed fixed at 87 or 0.68|F|, linear algebra not modelled.
* Not preregistered, no replay certificate, no matched rho band, no benchmark record: **[pcc:unverified]**; no leaderboard or `benchmarks/` change.
