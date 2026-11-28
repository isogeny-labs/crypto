# Validator report: multi-target 4-summand holdout (`prime_fast_j0_multi4`)

Head validated: cef1c311031c21c4a521f24fb716aa302a5b738f. All figures wall clock on one unisolated 4-vCPU host, `[pcc:unverified]`.

## Disposition
Validated with qualifiers. At 32 bits `multi4` beats in-row `ic_multi_solve_ex` at T=64 and T=512 and is slower than folded multi-target BSGS at every T measured (64, 512, 4096). Gate eligibility: `insufficient_evidence`.

## Binding and provenance
- Source blobs in the worktree equal head blobs (multi4, bsgs_multi, multi, prime_fast.rs, verify_pf_cert.py) and the freeze-commit blobs.
- Provenance gap: the multi4 binary mtime (02:02:09) precedes the source mtime (02:02:27). A differential build from head blobs gave byte-identical .text/.rodata/.eh_frame; only 66 panic-location line numbers differ (earlier source differed in non-code lines). bsgs_multi binary is bit-identical.
- Freeze commit a383db8868 (02:05:54Z) precedes holdout commit cef1c31103 (02:09:50Z); the holdout commit adds only holdout files and a session log. Tuning group (p,n)=(4294966981,4294939651); holdout (4294966177,4294835173).

## Numbers
Producer rows recompute: pooled base/sym4 4.257 (T=64, c=0.5), 2.179 (T=512, c=0.8); BSGS 24.7 / 106.5 ms/row. Exact CSKIP=1700 replay: 48/48 rows identical on all deterministic fields (cert, sym_rec, base_rec, ok flags, BSGS m/giant steps).

Fresh replay (frozen B4=44/52, CSKIP=1800+1900, 3 reps, 0 unverified rows):

| | T=64 | T=512 |
|---|---|---|
| better-c base/sym4 | 4.21 [3.79, 4.47] (c=0.5) | 2.145 [2.08, 2.21] (c=0.8) |
| frozen prediction / abandon | >=2.5 / <1.3: met | >=1.8 / <1.3: met |
| fixed c=0.5 / 0.8 | 4.21 / 3.82 | 3.02 / 2.145 |
| CSKIP=2000 (extra) | 3.87 | 2.08 |
| sym4/BSGS (1800+1900) | 2.06 [1.99, 2.13] | 1.36 [1.33, 1.39] |

Baseline c sweep (CSKIP=1800): frozen grid {0.5, 0.8} brackets the optimum, best off-grid at most 1.4% faster (ratios vs best baseline 3.84 and 2.18). `extra=6` was not swept.

## Soundness and timing scope
- Relations verified by exact point addition in `scan4`; the final answer checked by kG=Q and x==k. The solver does not use k (zeroed-k control: every sym_rec equals true k; 4096/4096 and 512/512).
- Timer scopes match: sym4 total covers build, collection, elimination, per-target phase, verification; baseline wraps all of `ic_multi_solve_ex` including its verification; BSGS covers baby table, giant steps, verification.
- In-process arm order (sym4 first) does not flatter sym4 within ±20% noise (baseline-first ratios 4.57/4.22/3.07/2.53 vs frozen 4.42/3.81/3.18/2.13).

## Certificates and negative controls
Independent strict checker (own Miller-Rabin and Jacobian arithmetic): 700 rows, 593,152 scalars, 0 failures; rejected 15/15 mutations. `tools/verify_pf_cert.py` accepted 8 of 15: unreduced sym_rec, one/all-null sym_rec, truncated sym_rec, null base_rec variants, zeroed sym4_ok counter. A whole-null sym_rec crashes with TypeError. The multi-target null/truncation/counter hole is confirmed open (commit dce8863f66 only fixed single-target rows).

## T=4096 (exploratory, CSKIP=1800, 4 rows, all targets verified)
sym4 ms/row vs baseline: B=52 c=0.8 474 vs 653; B=64 c=0.8 399 vs 654 (base/sym4 1.38 to 2.25 over configs). BSGS 235.9 / 233.9 ms: sym4/BSGS about 2.0 (B=52), 1.6 to 1.7 (B=64). No claim at T=4096.

## Not parity
Slower than folded BSGS at every T. No matched priced rho exists in the rows. Context only: in-repo folded multi-target rho 25.2 ms (T=64), 102.3 ms (T=512), 1641 ms (T=4096); not priced under 5a, no ranking drawn. Result is a constant-factor improvement over the library baseline, not a floor advance.

## Qualifiers
1. multi4 source mtime postdates binary (resolved by differential build).
2. Producer's "fresh curves" are one (p,n) group with 2 twists; CSKIP=1800 shares it. Only 1900 and 2000 are new groups.
3. Better-c estimator uncertainty about ±10%; all alternative estimators stay above prediction (T=64 3.82 to 4.27; T=512 2.13 to 2.23).
4. Producer host 25% to 40% slower than validator host; ratios stable.
5. Head has 14 of 36 check runs failing (not diagnosed by validator).

## Missing for docs/VALIDATION_GATE.md
Replay certificate file with independent:true; validator-report.json; matched priced rho (5a band 0/0.1/1 GAE); VALIDATION.json and pcc claim id; bench.record/v1 and S3 raw data (holdout files ~231 KB and tuning files ~173 KB exceed the 64 KiB Git data threshold); isolab-eks/L2/A-A admission; scope limited to 32 bits, T in {64, 512}, 3 groups.
