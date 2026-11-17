from j_part5 import *
# Orphan directories: benchmarks/<dir>/ exists but has NO row in benchmarks/README.md (row_no = 1000 + n).
row(1002,
 cl("Gate-first geometry with reduced root bases: original/candidate word-XOR ratios 1.64/1.93/2.19/2.41; process-CPU previous/candidate 2.85-3.69", True, "improvement (as reported)", "engineering (decomposition stage)", "word-XORs: counts; CPU: L0 (shared Apple M4 Pro, unpinned)", "no", C,
    "6/6 recompute: the per-round paired median CPU upper bound is 3.69 (arm-median ratio would be 3.79). Counted XOR ratios stand; CPU ratios are L0 and unvalidated. No README row in benchmarks/README.md; directory only."))
row(1003,
 cl("Zero-minor ECDLP vs rho.negation: counted field-operation cost 260-6,715x rho; search-phase slope 0.905 [0.769, 1.021]", "partial", "negative result", "baseline measurement (method loses)", "counts (L0 macOS)", "no", C,
    "Mean-over-seeds reproduces all six values (260.4 ... 6,715.0); median-over-seeds gives 273.7 ... 7,547: the study does not state how the 4 rho seeds per target reduce (cannot_define x3). Slope 0.904887 recomputes; interval not reproducible. Direction (method loses to rho by 2-4 orders) is robust."))
row(1004,
 cl("GLV-GLS-4 D=-619 confirmation on a second host: projective GLV-GLS-4 median 105,273 ns vs GLS-2 114,823 ns: 1.091x; A/A 0.989", True, "confirmation", "engineering (scalar multiplication, variable-time)", "L0 (taskset core 3, another build starting, not isolated)", "yes: A/A 0.989", U,
    "4/4 recompute. The 9% gain sits within 1.1% of the A/A reading 0.989 on a host with another build starting: a wall ratio at L0, unvalidated. Variable-time arithmetic; the row (and the lattice row 1023) state that no unknown-scalar-recovery improvement is claimed. No README row."))
row(1007,
 cl("Native Boolean F5 hashing: 3.715% fewer complete-F5 instruction references (95% 3.714-3.726%) and 45.136% fewer standalone criterion references over 11 frozen systems, 5 paired A/B/A' rounds", True, "improvement", "engineering", "counts (callgrind Ir, deterministic; AMD EPYC 9V74, one Rayon worker)", "yes: A' arm", V,
    "5/5 recompute; 627/627 outputs identical; two interval claims not recomputable. Instruction counts are deterministic, so isolation is irrelevant. No README row."))
row(1010,
 cl("Split/hybrid PDP: closest relation-phase projection 22.8x strong rho or 391x signed-Frobenius rho; amended 192-run grid exact", True, "stage diagnostic/projection; no end-to-end IC speedup", "baseline (projected method loses to matched rho)", "matched rho L2 on most records; projection uses calibration from another host", "no", C,
    "4/4 recompute. A projection from a stage cost to rho via calibration from a different host: derived, not measured end to end; the sign of the comparison (IC loses) is robust. No README row."))
row(1011,
 cl("Scalar group-law PDP kernel vs baseline: group-scalar/baseline per-pair ratios 0.0668-0.1225; 29.8-40.8 ns per own pair; 400 corpus root sets agree", True, "improvement (partial: library promotion pending)", "engineering (kernel)", "L1-class (CPU 3 pinned on shared 4-vCPU Xeon, 6/125 runs contended, retained with sensitivity table)", "yes: PS0 A/A (largest 8.81%)", C,
    "3/3 recompute; 20/20 root sets equal in the recomputed sample. A 8-15x kernel gain is far above the 8.8% A/A. VM neighbours outside the pin; contended runs retained with a sensitivity table (not excluded). No README row."))
row(1012,
 cl("Group-law vs derived baseline gate count: 7,410-14,608 fused ternary-logic issues per task vs 70,695-190,275 (9.5-13.0x)", "partial", "program-model accounting, no wall time", "accounting (model-derived)", "gate-count model (no timing)", "n/a", C, "2/3 recompute. A program-model count, not a measured speedup; identity-certificate requirement remains blocked (README)."))
row(1014,
 cl("Frobenius auxiliaries cost 57-701x more MatrixF4 word operations; convolution splitting 1.21-2.07x more despite lower degree", "partial", "model diagnostics, no IC/rho claim", "negative result", "counts (word ops)", "no", C, "README table cells match (57-701x) but frob.json has a fourth cell at 30.1x outside the quoted range; negative result either way."))
row(1015,
 cl("Direct quotient-curve m=2 solve vs symmetrised control: F4 word-operation ratios 1.340-1.642; identical root digests; five interleaved rounds", True, "improvement (decomposition stage)", "engineering", "counts (word ops, Apple M4 Pro L0)", "no explicit A/A; root digests 200/200 equal", V, "2/2 recompute; deterministic counts, outputs identical. No README row."))
row(1016,
 cl("Gate-first original/candidate word-XOR ratios 1.5675-1.7788; lane roots cost 2.6-4.0x the sequential control (negative)", "partial", "improvement + negative lane result", "engineering; lane batching negative", "counts", "no", C, "XOR ratios recompute; the lane-root range 2.6-4.0x holds for f6p rows only (f4 rows give 2.18x), so the stated range is narrower than the data."))
row(1017,
 cl("Early-abort probes: mode 1 original/candidate 1.00083-1.00378 (no change); mode 2 0.99714-0.99928 (negative)", "partial", "null/negative", "null result", "counts", "no", C, "Ranges cover the f4 rows only; null result stated as such. 2 partial, no mismatch."))
row(1019,
 cl("PinX3 vs geometric e3 per call: 0.052/0.154 ms vs 9.702/123.595 ms (n=17/19) and n=31 full solves 33.5/45.9 s", True, "diagnostic (no end-to-end sub-rho claim)", "engineering (stage presentation); a ~190x per-call gain", "L0 (shared 4-vCPU container, wall indicative only)", "no", U, "3/3 recompute. Per-call wall at L0 unvalidated; the study states the diagnostics make no end-to-end sub-rho claim."))
row(1020,
 cl("n=17 per-call e3 0.000116/0.00141/0.0216 s at l=3/4/5; symmetric l=5 timed out (>300 s for four probes)", True, "stage diagnostic", "descriptive stage timing with a timeout", "L0 (indicative only)", "no", U, "2/2 recompute; wall at L0; a timeout is not evidence of a rate."))
row(1021,
 cl("Closed-form Koblitz 4E table check 0.124/0.122 us at n=131/233 vs tau-NAF 160/98 us (order-of-magnitude only)", True, "historical cost comparison", "engineering (counted multiplications are primary)", "x86_64, 4 cores shared with ~6 other agents, no isolated timing", "no", U, "4/4 recompute. A timing of a table lookup against a full scalar multiplication on a contended host with an example field backend and no on-curve check: not like-for-like; the study itself restricts the claim to counted multiplications."))
row(1022,
 cl("sqrt-elu vs Velu at l~2^24: 100,663,797 vs 24,454,729 M-equivalents (4.12x); counted exponent 0.822 [0.805, 0.838] at l>=2^16", True, "historical counted comparison", "engineering (algorithmic exponent l^0.82 vs l)", "counts (61-bit Fp counted M/I); wall contended (4 cores, ~6 agents)", "no", C, "4/4 recompute. A counted-cost comparison of two published algorithms; the exponent interval comes from the study's fit."))
row(1023,
 cl("Derived GLS-2/GLV-GLS-4 cost-model ratio 1.220x at D=-619; char-2 Verschiebung correction gives cost ratio >=2.5 (derived loss)", "partial", "derived model", "accounting (derived)", "no timing", "n/a", C, "Derived model ratios; not a measurement; model vs measured wall 1.147x (row 1024) and 1.091x (row 1004) lower."))
row(1024,
 cl("D=-619 projective GLV-GLS-4 median 71,172 ns vs GLS-2 81,665 ns: 1.147x; across D 1.062-1.214", True, "improvement (variable-time)", "engineering", "L0 (taskset core 3, load 0.02-0.39, not isolated)", "no (A/A 0.989 appears only in the confirmation row 1004)", U, "4/4 recompute; a 15% wall gain at L0 whose confirmation on another host reads 9.1% with A/A 0.989. Unvalidated as a wall claim."))
row(1025,
 cl("Fiber-aware IC vs matched rho: envelope IC/rho grows from 8.4-8.8 to 35.1-37.0 (h=4) and 13.4-20.1 to 60.4-63.1 (h=72); negative whole-method result", True, "negative result", "baseline measurement (IC loses and the gap widens)", "counted S: L2/L1 on most records (231/105, 197/139, 88/56)", "matched rho in the same sessions", V, "5/5 recompute; 816 audited records; extrapolation to deployed sizes labelled derived."))
row(1026,
 cl("Boolean F4/F5 row building: cumulative requested bytes -42.47% (F4), -45.47% (F5), -50.30% (criterion); requests -0.25%/-7.70%/-35.12%; output and counters identical", True, "improvement (allocation traffic; not runtime or peak memory)", "engineering (allocation counts)", "counts (allocator-requested bytes; wall null)", "A/A/replication controls", V, "6/6 recompute; outputs identical 96/96 recorded. Row correctly states allocation traffic is not runtime."))
row(1035,
 cl("E4 direct/quotient F_p row-reduction multiplication ratios 4.12-4.26 (order 2), 17.6-19.6 (order 4); E5 combined/blind F4 operations 234-529x", "partial", "diagnostic, no rho comparison; 600-700x prediction failed (stated)", "engineering diagnostic; E5 negative", "E4 unisolated wall observations, counted ops primary; E5 L1/L2 on 8 records", "no", C, "4/6 recompute; the E4 ranges hold on random targets only (not all targets). Prediction failure retained."))
row(1036,
 cl("Frozen matched profiler panel: instruction reductions 67.958860% (screen), 5.292412% (search), 0.147796% (replay) for P192/P224 isogeny campaign", True, "improvement (instruction counts)", "engineering (callgrind user-space instructions, ARM Linux guest)", "counts (callgrind)", "no", V, "4/4 recompute from research/isogeny_catalogue_setup_20261009, not the listed study directory. 'Operational intervals are not campaign throughput' (README); no campaign-total gain claimed."))
