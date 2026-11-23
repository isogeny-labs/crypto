# RESIDUES (SYNTHESIS.md section 5 items 2 and 5), started 2026-11-02
Budget <= ~8 CPU-h, nice, <= 4 concurrent solver processes. Conflicts are the unit; CPU s descriptive (host load 30-60+).
Outputs: accounting/{analysis.md,raw/*}, cube/{analysis.md,raw/*}. Writing order: PRE-REGISTRATION first (below), then runs.

## What / how to rerun
Shared run set: patched copy of the L11 sweeper (cube/sweeper; env KEEP_CNF_DIR dumps CNF + window-bit variables), MULMODE=school,
kissat 4.0.4 default, m=3 diagonal (b=3s, l=s), small-x, bits-grouplaw, sym=1, seeds 1..10 (= 10 curves/primes), 8 planted + 8 uniform targets/curve,
KISSAT_CONFLICTS=100000.  cmd per (b,seed):
  cd cube; MULMODE=school KISSAT_CONFLICTS=100000 nice -n 19 sweeper/target/release/prime_pdp_sat_sweep sweep --m 3 --bits B --two-adicity 2 --families small-x --encodings bits-grouplaw --ls S --planted 8 --uniform 8 --timeout 120 --seed SEED --out raw/sweep/bBB-sSEED
  (the sweeper refuses to overwrite its --out). Then: python3 accounting/analyze_A.py ; python3 cube/cube_driver.py ; python3 cube/analyze_B.py
A uses raw/sweep (b=6,9,12) + L12 raw TSVs (re-analysed, no new solves). B uses b=9 CNFs in cube/cnf.
Safety: every process started here is nice'd and wrapped in `timeout`; nothing is left running at hand-back (see end of analysis.md files).
