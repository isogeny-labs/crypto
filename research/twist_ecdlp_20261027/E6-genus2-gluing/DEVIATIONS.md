# E6-genus2-gluing: deviations from PREREGISTRATION.md (none silent; also summarised in RESULTS.md)

PREREGISTRATION.md and the five frozen files (e6lib.gp, e6_main.gp, e6_controls.gp, e6_summary.gp, run_all.sh) are unchanged;
their sha256 values equal the ones recorded in the preregistration and `out/SHA256SUMS_v1_orig.txt` verifies every frozen file.

1. Driver. `run.sh` replaces `run_all.sh` (not executed). Same commands, seeds and grid; controls run before the main runs so the
   positive control precedes the data. Each p is seeded independently, so order changes no number. `run.sh` v1 (the version that
   produced out/*) is kept as `run_v1.sh`; `run.sh` v2 adds modes (`E6_REDUCED=1|2`, `E6_OUT`, `E6_POSTONLY=1`) and the post-processing
   steps below. Default mode of v2 runs the identical experiment commands in the identical order as v1.
2. p=101: the grid asks for N=20 curves per pattern, but only 16 distinct j-invariants with pattern [1,1,1] exist
   (exhaustive count 34/49/16 at p=101, 70/105/34 at p=211, out/diag_distinct_j.txt, re-derived independently in out/xcheck/xcheck2.out and
   by the Legendre lambda formula: 16, 34). The frozen sampler therefore loops forever on the 17th curve of that pattern; `main p=101`
   ended at its 900 s timeout (rc=124, no DONE line) after printing all records of 56 curves (20+20+16); the 64 brute-force records
   (4 curves per pattern) were complete. Not rerun. 56 curves are reported at p=101 (221 curves in total).
3. p=20011 has 3 curves (N=1 per pattern), as registered.
4. Post-hoc helper scripts (frozen ones untouched): e6_totals.sh (pooled sums; v2 only parameterises the output directory,
   e6_totals_v1.sh is the original), e6_wilson.gp (v2 reads k/n pairs from the environment and no longer contains the two n=0 rows that
   errored by design in v1, e6_wilson_v1.gp is the original; the errors printed in the v1 out/wilson_totals.txt are gone), e6_diag_p101.gp,
   plot_e6.py, e6_opcounts.sh (operation counts, added in this finalisation), e6_xcheck1-3.gp (independent cross-checks, see RESULTS.md).
   None of them produces a number that enters P1-P8; out/SMOKE_notdata_* files are excluded from every number.
5. Single seed, no repeat. Wall-clock is not a metric (contention-affected, shared 4 cores).
6. Operation counts (poly evaluations, candidate tuples, charpoly calls, model rho/IC group operations) were not part of the
   preregistered metric list; they are added as a derived, read-only secondary metric from the raw files (out/opcounts.txt).
   Field multiplications inside PARI (hyperellcharpoly, ellcard, factormod) are NOT counted; the unlogged rejection-sampling attempts of
   `sampleE` are NOT counted (only NC2 reports `tries`).
7. `verify_posthoc_firstrun/` holds the outputs of the first post-hoc cross-check session (identical to what run.sh v2 now regenerates
   in out/xcheck/); kept for provenance.
8. Replication-diagnosis pass (2026-10-27; PREREGISTRATION.md, the five frozen files, out/ and results.json untouched).
   The independent replication (replicate2/, VERIFICATION.json "replication") listed "not complete" only because its
   quick, reduced and full runs had been started in the background and not yet compared. Compared afterwards: the full
   replica replicate2/out_full is byte-identical to out/ in every raw file (main_p*.txt, ctrl_p*.txt, *.err), summary.txt,
   totals.txt, wilson_totals.txt, opcounts.txt and rc.txt (including main p=101 rc=124), and the sha256 column of
   SHA256SUMS_v2.txt is identical (only the path prefix differs); the reduced and quick replicas have byte-identical raw
   files and differ in summary/totals/opcounts/rc only because they cover a subset of the primes. Every experiment is
   seeded (setrand from E6_SEED+p), so there is no nondeterminism in any scientific number; wall time never enters one.
   One driver defect found: run.sh called the post-hoc cross-checks as `gp -q ../../e6_xcheck$i.gp`, which resolves only
   when E6_OUT is a direct child of the experiment directory; with E6_OUT=replicate2/out_full the three xcheck runs failed
   ("error opening input file") and left empty xcheck*.out. Fixed by calling "$HERE/e6_xcheck$i.gp" (HERE = script
   directory). No experiment command, seed or grid changed. The fixed commands reproduce out/xcheck/xcheck{1,2,3}.out
   byte for byte from replicate2/out_full/xcheck/raw_*.txt (6 s). Consequently the run.sh hash in out/SHA256SUMS_v2.txt
   no longer matches (the original is saved as scratchpad/run.sh.orig_e6); the original-run run.sh is the v2 that produced
   out/, and the frozen preregistered hashes are unaffected.
