# Judgments, rows 1-24 (README order). Fields: c claim, ok recompute (True/False/"partial"/None),
# cc class claimed, ca class actual, iso isolation tier, aa A/A control, v verdict, n notes.
V, C, U, X = "validated", "validated_with_caveats", "unvalidated", "contradicted"
J = {}
def row(n, *claims): J[n] = list(claims)
def cl(c, ok, cc, ca, iso, aa, v, n): return dict(c=c, ok=ok, cc=cc, ca=ca, iso=iso, aa=aa, v=v, n=n)

row(1,
 cl("Folded (negation-class) walk uses 0.67-0.89x the group operations of the plain walk at N<=6e4 on hyperelliptic Jacobians", True, "unlabelled (registered hypothesis R1)", "engineering (constant)",
    "counts (isolated_bench CPU 3, uncontended; irrelevant to counts)", "none (deterministic counts; no A/A arm)", C,
    "10/10 headline numbers recompute from run records. Protocol and first raw record share one calendar day; the card/spec commit order is not provable from git. Instances are small (N<=6e4) and the effect vanishes at N>=1e5."),
 cl("At N>=1e5 the median folded/plain ratio is 0.968, registered R1 (<=0.80) falsified", True, "unlabelled", "null result", "counts", "none", V,
    "Honest falsification reported; recomputes."),
 cl("Plain DP walk S_walk 1.82 and 2.10 vs ideal 1.2533 at the two largest rows", True, "unlabelled", "baseline diagnostic: the rho reference is itself 45-68% above ideal at large N", "counts", "none", C,
    "Matters for every IC-vs-rho ratio quoted elsewhere that used this walk as the rho reference: the reference was not the strongest available (see row 3)."))
row(2,
 cl("SplitMix64-mixed hash lowers plain S_walk to median 1.406 at N>=1e5 (H1 <=1.55 falsified at the two largest rows 2.240/1.696)", True, "unlabelled (registered H1)", "engineering (rho-walk constant) with a negative registered outcome", "counts (isolated_bench CPU 3, uncontended)", "small-N control H3 (|median mixed - median plain| 0.023) serves as the null", C,
    "7/7 recompute (1 partial: interval not reproducible). Two registered hypotheses honestly reported falsified."),
 cl("Mixed fold/plain median 0.960 (H2 <=0.80 falsified)", True, "unlabelled", "null result", "counts", "H3 control", V, "Recomputes; falsification reported."))
row(3,
 cl("Full-width branch coefficients cut Jacobian plain S_walk: median 1.390 at N>=1e5, largest row 2.100 -> 1.481", True, "accounting correction (against IC)", "engineering: the rho walk changed (branch width) and counted steps fell 29% at the largest row; this strengthens the rho baseline", "counts (isolated_bench CPU 3)", "H3/W3 small-N control moves 0.025", C,
    "6/6 recompute. Label differs: a change to the walk is not 'accounting' (numbers changed AND algorithm changed). Valuable for rho fairness: it shows the 8-bit rho reference used in earlier rounds understated rho at large N."))
row(5,
 cl("Index calculus costs 38-159x the callgrind Ir of rho on the same curves (28.1-34.6 M vs 0.22-0.84 M Ir)", "partial", "tooling entry, no speedup claimed", "accounting / baseline measurement", "counts (callgrind Ir)", "none", U,
    "Only the rendered docs/perf/PERFORMANCE_INDEX.md is in the repo; the per-child callgrind outputs are not (8 of 10 claims no_raw_in_repo). Headline says 9 factor bases but the committed child list has 8 ic.pipeline children (+3 rho). Cannot be recomputed from raw."))
row(8,
 cl("Branch-free Fp64 leaves the two_sylow halving search neutral (ratios 0.98-1.08), positive control 0.44, negative control within 4.5%", True, "neutral (no speedup claimed)", "engineering (null)", "L0 (stated: KVM, no PMU)", "yes: negative control FpWide<1> kernel within 4.5%; positive control fp64 kernel 0.44", C,
    "7/7 recompute; gate checksums equal 64/64. A null result, which is all an L0 wall run can support. Protocol and raw first appear in one commit, so 'frozen before timing' is the study's assertion, not verifiable from git. Duplicate of README row 22."))
row(14,
 cl("BSGS one-target wall total_s baseline/candidate 1.30-1.44x chain, 1.74-1.98x paired at 44-52 bits", True, "engineering", "engineering", "L0 (study label; isolated_bench CPU 3 on KVM, 26/438 contended runs excluded)", "yes: A' same-binary arm, 7 rotated rounds", U,
    "Wall-time speedup at L0 cannot be validated. Direction is robust: effect far exceeds the recorded A/A' spread and every output is identical (266/266 pairs). Magnitude unsupported at this tier."),
 cl("rho_parallel solve_s 5.84x at 32, 3.75x at 36, 1.82x at 40, 1.14x at 48 bits (1 thread)", False, "engineering", "engineering (startup cost removal, not walk speed)", "L0", "yes (A')", U,
    "Recomputed 3.691x at 36 bits (claimed 3.75x), a 1.6% difference. The gain is the removal of a one-off Jacobian mul/combine start-up cost that outweighs the walk at small sizes, so it shrinks to 1.14x by 48 bits and is within noise at 44 and 52 bits; it does not speed up the asymptotic walk."),
 cl("Candidate/PARI elllog 0.020-0.167 (baseline/PARI 0.022-0.974) on the same curves and targets", None, "external comparison", "engineering vs external tool", "L0", "no", U,
    "PARI ratio aggregation rule not stated and not recomputable (cannot_define x2). PARI elllog is a generic BSGS/rho-style solver; whether it is the strongest external reference at these sizes (e.g. a tuned Pollard rho with negation) is not shown. Counted outputs identical, 266/266 pairs."))
row(15,
 cl("Pohlig-Hellman total_steps 284,364 -> 10,682 (26.62x) by per-prime BSGS tables instead of repeated scans; all 48 answers correct and identical", True, "correctness fix (nothing timed)", "engineering (algorithmic constant in a counted unit) on a smooth-order curve", "counts (nothing timed)", "n/a (deterministic counts, before/after on the same targets)", V,
    "8/8 recompute, 51/51 outputs identical. Applies only to smooth group orders (3*19^4*151*181*13171); not a speedup for prime-order targets."),
 cl("pollard_collab negation-map dead trails 50 -> 1, 106 -> 0, 197 -> 15", True, "correctness fix", "engineering (bug fix)", "counts", "n/a", V, "No-negation arm byte-identical, as claimed."))
row(16,
 cl("Rho below IC on all 800 curves at every section-5a weight (IC ~30-60x rho)", "partial", "null/cost study", "baseline measurement (IC loses)", "L0 (shared 4-vCPU VM), counted units only", "yes: rho A/A seeds 1.001 [0.965, 1.039]", C,
    "Counted S, ecbench replays 408/408 exact; the direction is unambiguous (30-60x). Recount: 48,680 non-warmup records recomputed vs 46,912 claimed (the claimed number does not match the committed sessions); bootstrap/permutation statistics need a tool not in the repo (6 cannot_define). The study's other findings (floor/crater ratio, coordinate effect) are null or exploratory and make no speedup claim."))
row(18,
 cl("Testing a candidate secret degree takes chain length e vs full decapsulation chain E (569/1076, 832/1594, 1087/2106)", True, "attack-cost accounting, not a KEM speedup", "accounting", "counts (chain lengths)", "n/a", V, "Chain-length ratios are deterministic and recompute 10/10."),
 cl("pk_degree_test wall 0.79 vs 1.28 s (L1), 2.72 vs 5.54 s (L3), 6.84 vs 13.34 s (L5)", True, "attack-cost accounting", "accounting", "L0 (shared VM, unpinned), n=7/3/3", "no", U,
    "Numbers recompute but n=3 at L3/L5 on an unpinned shared VM cannot support a wall-time comparison. The deterministic chain-length ratio above carries the finding."))
row(19,
 cl("P-256 rho seeding 0.66 vs 11.53 ms/seed (17.4x) by an inversion-free Jacobian ladder; inversions per seed ~750 -> 1", "partial", "op-count reduction, wall corroboration", "engineering (setup-only)", "L0 (stated: no isolation receipt, no PMU)", "no", C,
    "The inversion count reduction (750 -> 1) is structural and the ratio recomputes from benchmarks/.../data.csv. The wall figure at L0 with no A/A is corroboration only, as the row says. Identical-output check: 400 rows compared, 0 differ, versus 3,000 seeds claimed (only 400 in the committed data). Row stresses it is setup-only."))
row(21,
 cl("Direct pack on by default: 1.006-2.173x on every F5 cell", False, "engineering", "engineering", "L0 (shared, unpinned, wall)", "no explicit A/A arm; 'on' vs 'off' arms, 3 then 15 rounds", U,
    "Recomputed min over 28 F5 cells is 0.9974 (f5_n12_m12_d4, 4 threads), not 1.006: at least one cell regresses by 0.3%, contradicting 'every cell'. Wall at L0 with no A/A. Outputs exact (77/77 equal). Decision rule committed before run 2 per README; order not provable from git."))
row(22,
 cl("Duplicate of README row 8: branch-free Fp64 neutral on two_sylow_search", True, "neutral", "engineering (null)", "L0 (stated)", "yes: controls K1 0.442 / C1 0.045", C, "Same entry listed twice in benchmarks/README.md (rows 8 and 22); identical evidence, 5/5 recompute, 6/6 gate lines equal."))
row(23,
 cl("BSGS24/32/40 per-call 61.7->31.5, 327->118, 1937->592 us (paired 1.99x, 2.79x, 3.24x)", True, "engineering", "engineering", "pinned CPU 4 on a shared EPYC VM, movable-thread eviction, PSI unavailable (L1 at best)", "yes: five A/A and five A/B pairs; each ratio exceeds its workload's A/A envelope", C,
    "10/10 recompute from evidence/run-20261016T0205Z/runs.jsonl. The only row in this batch with a documented A/A envelope per workload. Caveat: shared host and PSI unavailable, so contention was not fully observable; frozen protocol is in the repo but lands in one commit with data."),
 cl("SEA40 0.81x, SEA192 1.02x, cold API 1.005x all within the A/A envelope", True, "null result", "null result", "same", "yes", V, "Correctly reported as no effect."))
row(24,
 cl("Online median IC/precomputed-rho: K3300 3.74x time-matched, 40.61x memory-matched; K600 81.5x, 214.7x, 395.5x; precomputed rho faster in every matched pair", "partial", "accounting correction (reverses earlier 113x and 12.29x 'IC beats rho' readings)", "accounting / fairer baseline: rho against a Bernstein-Lange precomputed DP table", "L0 hosted runner (ubuntu-24.04, 3 CPU models across runs; ratios within one runner)", "yes: positive control empty-table rho; replays 288/288 recovered scalars agree", C,
    "Per-target medians reproduce; bootstrap CIs cannot (analyzer seeds not committed). This is the strongest rho-fairness result in the README: it shows the earlier IC-beats-rho online ratios (rows 104, 108, 110) used an empty-table rho that is not the strongest reference. Counts unranked per section 5a."))
