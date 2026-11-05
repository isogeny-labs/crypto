# Round 7: memory-bounded solve of m5_t4_k0_T

All result lines [pcc:unverified]. Input sha256 verified: 0ae81d1c...8488 (matches). /usr/bin/time absent; peak RSS read by hand from `ps` samples (the
sampler in run1.sh/run2.sh also matches another agent's msolve, so its summary.txt is not used). Scripts: run1.sh (cap 12 GiB), run2.sh (cap 10 GiB).
Cap change: coordinator asked mid-task for 10 GiB on new runs; run A (already running, 12 GiB) was left alone; run B used 10 GiB.

## Attempts
A. `ulimit -v 12582912; timeout 7200 msolve -f m5_t4_k0_T.ms -g 1 -t 2 -m 10000 -v 2` (log logs/A_m10000_t2.log)
 - exit 139 (address-space cap), wall ~1.5 h, peak RSS sampled >= 9.9 GB. [pcc:unverified]
 - Highest degree 9. -m 10000 splits the 48509-pair deg-9 round into 10000-pair rounds (matrices 131k x 262k ... 243k x 488k, density 0.2-0.7%); five such deg-9 rounds finished, pair list not exhausted (353k pairs), death in a deg-8 round (98k x 256k). Degree-9 NOT completed. [pcc:unverified]
B. `ulimit -v 10485760; timeout 7200 msolve ... -g 1 -t 2 -m 10000 -l 44 -r 0 -v 2` (logs/B_l44_m10000_t2_r0.log)
 - exit 139, wall ~45 min, peak RSS sampled >= 8.5 GB. Same deg-9 trajectory but linear algebra faster (e.g. 193 s vs 888 s for the 186k x 432k round); died at pairs 371k in deg-8 rounds (140k x 343k). Degree 9 NOT completed. [pcc:unverified]

Not run: -l 1, -l 2 with -t 2 alone, -q 1, smaller -m. Reason: memory growth is dominated by the basis/hash table and fill-in of the growing
basis (RSS ~6 GB before the big rounds in both runs), not by one matrix; -m only reshapes the matrices. Time per attempt is 1-1.5 h.
Earlier history (6 GiB, 14.5 GiB, 13.5 GiB timeouts) agrees. [pcc:unverified]

## Rust engine (step 2), estimate only, not built
f4_fp.rs reduces sparse pair rows but hands the residual to a DENSE u64 echelon (see its doc and RrefSnap rows: Vec<Vec<u64>>); a 1e5 x 4e5 residual
is 320 GB. examples/ic_next10_03_pkm_degree.rs `convert` reads .magma, not .ms. Does not plausibly fit 12 GiB; not built. [pcc:unverified]

## Answer
D_T(5,4) still UNDECIDED. No run finished a degree-9 pass; degree 10 was never reached. The only fact: no degree-10 round was started in
either run, and the pair list at deg 9 shrank slowly (354k pairs remaining, ~1000 new elements per deg-8/9 round), so completion in deg 9 is neither confirmed nor excluded. [pcc:unverified]
Suggested next: substitute the 18 linear-in-one-variable relations (s0=h0r1_1 etc.) by hand to cut 23 variables to ~7 before F4 (changes the encoding, not
a "fixed system" run), or a machine with >= 32 GiB.
