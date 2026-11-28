# M31 fixed DM matching minor: exact rank and direct-operator tradeoff

Status [pcc:unverified]; deterministic reference, not native Rust, timing A/B, or complete solve.

Scope notice #5122 comment 7134781539 before execution. Parent 62a96998536d060ad0fa0a6005571816207148ac. Existing ten-query B677 public capture, source blob 7fd27b9888058399d8814cef373d373243e6a1e5 at 577d6c2c1c4a527604d965e6fb62c44df7902f36, 303-pivot tape blob 72f2a2240994e3436ab8e5a1910a31cf1949bdcf, existing DM matching receipt at 6910c3f3f0448429045672161950df22da1a4679. Curve p=2147483647, a=847514846, b=2087444724, r=2147446831, h=1, G=(625037715,2100627251), Q=(976979610,1228979761). All matrix operations over F_r, not F_p. Historical seed 0x7033315f6d340101 unchanged. Study m31-dual-matching-minor-v1: no new RNG, master/replicate/child streams or randomized timing; intentional fixed capture and matching reuse. No candidate reselection. Producer binary and rejection-trace gaps remain; not qualified for general reproducible A/B or speedup.

Sources checked (KB): KB MCP/wrapper unavailable; bounded fallback current AGENTS, ACTIVE, main and branch dated logs, graph README/SOURCES, prior DM/Gram/Lanczos reports, #4917/#4905 terminal merged, #5076/#5120 open and scoped, cross-repo open PR search "square minor Wiedemann" (no hits, not absence evidence). Current main native source blob identities: large_prime_filter e111545df0d2f0684759a1fac7cb4c07d0af72e0; large_prime_filter_mst e747ae865b31e09b772769bf73b9279671d23346; gaudry_cubic_graph bde1d4af8a6b010f2b3f0842d5d3c1c8c8851aa8; jv_large_prime_rank_bound 2c0e1e04a31361573412568d9840bbeb1acd0469; pq_wiedemann b1712946b81487bf5c4c45fdb3e969e9edb70aec. Wiedemann 1986 section II and III (https://doi.org/10.1109/TIT.1986.1057137; https://www.enseignement.polytechnique.fr/informatique/profs/Francois.Morain/Master1/Crypto/projects/Wiedemann86.pdf): nonsingular square Krylov solve and handling dependencies/non-square systems; direct square route already exists in pq_wiedemann. No novelty claim. #5120 owns native tape/solver comparison; #5076 owns extension admission. Both have unresolved integration gates, so no production edit, collector or competing native run.

## Method and exact result

D is the 363-by-342 reduced point-constraint matrix with 2034 nonzeros, and c the reduced target functional. Choose the rows of D matched by the *already frozen* degree-three DM matching in its stored source-weight order. The resulting square matrix B has 1908 nonzeros. Unlike the previous DM census, this tests actual field rank, not merely the structural matching. If B is nonsingular, solve B^T y=c, scatter y into the matched point rows and zero the 21 unmatched rows, giving D^T u=c; reverse the pivot tape. A square solve or its failure alone is never a positive target certificate; verify the original equations. If B is singular, refuse this minor and use the existing rectangular method; no uncharged search.

Replay recipe (deterministic connector BigInt reference, no native executable):
1. Fetch the immutable source and tape blobs above, and degree3.matching from M31_DUAL_DM_20261124.json. Build point-indexed maps of signed coefficients using source row ordinal, and target map from query b joined by label; combine duplicates and remove exact zero mod r.
2. For each of the 303 tape entries, compute expression -other/pivot from current pivot point row, compare sorted maps to recorded expression, substitute into other point rows and target map, clear the pivot row, check recorded point nonzero delta. Retain the old eliminated-column coefficients and target coefficient for reverse lift.
3. Verify all matching edges, unique rows and coverage of the 342 remaining source-weight columns. Form B^T in ascending live source-weight order and stored matching order for its columns; append c. Eliminate columns in ascending order with first available nonzero pivot row, counting row swaps and retaining raw nonzero pivots. Compute determinant from their product and swap parity.
4. Back-substitute; scatter into the 677 point IDs with unmatched/removed points initialized zero. Check reduced equations then reverse the tape: for eliminated variable j, recover pivot point multiplier from (old target coefficient minus sum of the other old column coefficients times current multipliers)/pivot coefficient. Check all 645 original equations over F_r.
5. Compare against committed raw receipt (including exact matching, pivots and complete lifted vector); the coordinator reread the published receipt and independently recomputed its pivot product and all original equation residuals, but this is not independent external validation.

| Quantity | Result |
|---|---:|
| Pivot expressions and nonzero deltas | 303/303 |
| Minor dimensions / rank | 342 x 342 / 342 |
| Minor nonzeros | 1908 |
| Omitted rows / nonzeros | 21 / 126 |
| det(B) mod r | 1605794268 |
| Elimination swaps | 20 |
| Exact reference coefficient/RHS updates | 2221264 |
| Reference dense augmented entries | 117306 |
| Original compatibility checks | 645/645 |

Raw receipt M31_DUAL_MATCHING_MINOR_20261124.json at 5e5cb1b88d73a8a21f8eb5bb873d13088ac3537b, blob 0435e45bac77240f0ee7be09bd9cd14bd0aba305. The checked compatibility vector is not a point-log vector and differs in choice of free coordinates from Gram; no target recovery on this capture.

## Cost decision

At full recurrence degree n=342, ordinary direct square Wiedemann with 2n moments and n-1 reconstruction steps needs (3n-2)*1908=1953792 coefficient contributions before common verification. Existing matrix-free Gram Lanczos reference uses n*2*2034=1391256. The direct full-degree model is about 40.4% MORE contributions despite cheaper individual products. Neither direct recurrence degree nor native timings were measured here. A stored-frame, adaptive certified lower-degree, block, or different solver could change the comparison; charge setup, matching, projection, recurrence, vector work, inversion, storage, fallback, verification and reconstruction. The matching minor is useful as a deterministic alternate negative-control certificate, not a justified replacement for the primary Lanczos candidate. If a direct solve is already available, one checked attempt with a bounded budget may be appropriate, but do not add matching overhead solely to chase cheaper per-product arithmetic. Do not infer generic nonsingularity from matching.

One coordinator, zero workers. No worktree/cargo/rustc, Conductor lease, KB or independent workers exposed; no native dispatch, timing, fresh RNG, paid compute, merge or new source collection. Phase elapsed/CPU/RSS unavailable. Existing draft #5122 documentation-only, branch-only append; main reconciliation/independent PCC remain open. Next executable step remains #5120 native tape equivalence and fully charged fixed/held-out comparisons, #5076 extension admission; this reference supplies an alternate checked fixture, not another static optimization target. Preserve mandatory experiment-seeds/v1, M31, parallel/resource, publication/no-merge and stop/impact requirements.

Impact score: 3/10 — The preselected M31 structural matching minor is exactly full rank and its lifted compatibility witness satisfies all 645 equations; the full-degree direct scalar Wiedemann model would require more coefficient contributions than the existing Gram Lanczos reference. No actual direct Krylov timing or independent native validation.
