# L08 DEVIATIONS (executor)

PROTOCOL.md and SPEC.md were not edited (`sha256sum -c frozen.sha256` OK before work; re-checked at the end).
No threshold, arm, seed rule or decision rule was changed.

Deviations from PROTOCOL/SPEC: none in the specified computation. Items below are additions or
interpretations needed to make the text executable; none alters a counted quantity.

1. Additive output keys (not in SPEC 8): `g4_absdiff`, `g4_expected`, `rel_dup_lines` in `run` rows; in `gate` rows
   `g4_absdiff`, `g4_expected`, `eligible`, `n_relations_fb`, `N`, `fb_exhausted`, `g5_exhausted`, `g5_g1_fail`, `g5_g2_fail`.
   They are uncounted diagnostics. `rel_dup_lines` counts relations whose line (sorted point-index set) repeats an earlier relation.
   Reason: SPEC 5.4 does not deduplicate lines, so a line meeting the factor base in k >= 2 points can be produced by C(k,2) pairs
   and each production counts as a relation/edge (rank-deficient duplicates inflate `useful`). Implemented as specified; the count is reported (RESULTS 4).
2. G4 in the gate stage: SPEC 6 evaluates G4 only for p >= 1031 and eligible >= 20000, while PROTOCOL 7 lists it for the gate stage at p in {131, 257}.
   Followed SPEC (g4 = "n/a" at those p); the split fraction and |diff| are reported anyway.
3. Unspecified details fixed minimally: (a) the Gaussian elimination used for the counted determinant (n = 4 prefilter and the genus test) counts one
   INV per pivot, one M for the elimination factor, one M per row-update product, one M per pivot accumulated into the determinant, and swaps rows
   when a pivot is zero; (b) G3-PC replacement points are drawn after all 200 quadruples are formed, with `rng.choice` on the candidate list in
   index order; (c) quadruple draws use `rng.sample(range(n_rel), 4)` and repeated quadruples are allowed; (d) in the analyzer, percentile = element
   round(q*(n-1)) of the sorted 2000 replicates; `n_valid` in the TSV counts non-exhausted rows without g1/g2 failures while means are over all present rows of a cell;
   (e) `seeds_tried` is the seed number of the curve (seeds examined up to and including it) and `reject_counts` the rejections before it.
4. Execution: commands were run through `code/runlog.sh` (adds `timeout 600`, logs command, exit code and sha256 in `raw/COMMANDS.log`; stdout/stderr go to
   `<out>.stdout`/`<out>.stderr`). The selftest was run once before the logged run to debug (no gate or data command was run before the implementation passed it).
   `raw/data.sha256` was written with `sha256sum raw/*` after all runs.
5. No timeouts (exit 124) and no hook refusals occurred (`raw/timeouts.txt` absent; `raw/hook_refusals.txt` not needed). All grid cells were run; no instance size was reduced.
6. The PROTOCOL 10 loop for stage 1 was split over a driver script (`code/stage1.sh`, `code/stage2.sh`) containing exactly the listed commands; the p = 131 stage-1 runs were issued by hand with the same command text before the driver (which starts at p = 257).
