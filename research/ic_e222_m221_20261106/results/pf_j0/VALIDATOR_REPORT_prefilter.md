# Validator report: prefilter + retuned B (saved from the validator agent's message)

Disposition: validated with qualifiers; not leaderboard-eligible. [pcc:unverified]
Commit 94b6c2db62965f0d5cfcd7786b47fe3fb1fa7208; binary sha256 d1ac9ff0c2291861007304aae08d53bc7293fe2c144d5f308f47076ef3bf4d98; 4-vCPU Xeon @2.10GHz VM, no isolation; CSKIP 300 and 400, two replicates, ON/OFF order alternating; 384 rows, manifest sha256 621fe4e33a8e6ed0dc5530ed20454aa5c549b81f28f275683b08d70d46d75db2 (raw rows were in the validator scratchpad).

## Verified
- Prefilter is lossless: bit insertion of k0, beta*k0, beta^2*k0 matches the raw-x3 probe (same hash, shift, mask); instrumented copy: 143,886,684 probes, 4.4% pass, 0 false negatives.
- 384/384 rows verified (sym_ok, plain3_ok); target counts, rels, inversions identical ON vs OFF in 192 paired instances; pure-python certificate checker passes 96 rows (instrumented copy, deterministic fields equal the pristine rows).
- Timing scope equal (bitset build inside sym_ms); order swap neutral (e.g. 3.99/3.92, 4.42/4.36, 4.24/4.25); within-run A/A on plain3 medians 0.98-0.99, single rows 0.70-1.37.

## Ratios plain3(c=0.5)/sym, prefilter ON, pooled over 4 curves x 16 rows per size
28 bits: median 3.89 [sum ratio 3.81], CI [3.80, 3.99]; 30 bits: 4.20 [4.21], CI [4.06, 4.35]; 32 bits: 4.21 [4.14], CI [4.12, 4.28]; all 192 rows: 4.07 [4.12], CI [4.01, 4.17]. Cells 3.6x-4.3x, row range 2.4x-6.2x. Prefilter OFF at the same B: 2.81 / 2.80 / 2.40. Prefilter alone at fixed B: 1.37x / 1.54x / 1.77x. The 2.8-3.0x -> ~4.1x jump is prefilter plus B retuning.

## Findings
- F1: the stated rule round(0.176*2^(bits/3)) gives 114/180/286, not 116/184/292 (constant actually used ~0.1797); optimum is flat, impact small.
- F2: "never used for tuning" holds for the b values only; the producer's tuning and holdout curves share one (p, n) per size; validator CSKIP=300 at 28/32 bits is the same (p, n), CSKIP=400 new; ratios agree for fresh and tuned groups.
- F3: c=0.5 is not the best ic_solve baseline: 5-point c grid (0.3-0.8, CSKIP=400, n=32 per point) gives best c=0.8 (28b, 57.0 ms vs 64.2), c=0.6 (30b 162.3 vs 171.6; 32b 504.1 vs 533.3); ratio vs best c is about 3.4x-4.1x (3.44-3.49, 4.10, 4.03).
- F4: the rule and prediction were not pre-registered in a hashed frozen file; 28 bits extrapolated from the 30-bit sweep.
- F5: with default prefilter and summands=2 the code panicked (bits empty); fixed in the follow-up commit (prefilter requires summands >= 3).
- F6: reporting contract not met (no S column, no section 5a band, no isolation/A/A, no bench.record, pcc id or VALIDATION.json; rows lack commit/CSKIP/C3/argv).
- F7: sym vs the example's negation-only rho median: 4.6x (28b), 8.0x (30b), 16.2x (32b) behind; the strongest matched rho (folded) is further ahead.

## Suggested claim wording
"On j=0 curves at 28/30/32 bits with B about 0.18*2^(bits/3) (116/184/292), the prefiltered symmetric 3-summand IC solves ECDLP about 4.1x faster than ic_solve (3 summands, c=0.5) on wall clock; pooled medians 3.9x at 28 bits and 4.2x at 30 and 32; against the best of a 5-point c grid about 3.4x-4.1x. The prefilter alone is 1.4x-1.8x at fixed B and relations are identical. Single host, no isolation."

## Remaining for the validation gate
Isolated L2 run with A/A interval; replay certificate with commit/env/argv/G/Q/x emitted by the binary; hashed pre-registration of B and a per-size-tuned baseline c; matched strongest rho in the same unit with the section 5a band at 0/0.1/1 GAE and the unpriced counters for both arms; VALIDATION.json, bench.record, pcc claim id.
