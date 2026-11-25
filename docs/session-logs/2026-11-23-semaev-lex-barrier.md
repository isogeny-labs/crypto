# 2026-11-23 — Signed-fiber elimination-degree obstruction

Branch: automation/semaev-lex-degree-barrier-20261123; draft PR #5068. This follows merged #5023 (projective S3), #5044 (Frobenius sector), and #5058 (complete guarded charts); do not maintain merged PR branches.

## 19:07 UTC — What was done (commands, files, features)
- Read AGENTS.md, docs/ACTIVE.md, summation_split/{core.py,solver.py,harness.py,FINDINGS.md,SECTOR_COMPLETE.md}, and earlier per-task log; checked PR merge states.
- Read primary Kousidis–Wiemers 2019 (Weil-descent first-fall bound) and Kosters–Yeo 2015 (FFD is not a substitute for solving degree). Research-KB MCP and wrapper unavailable in this runtime; GitHub source and literature fallback used.
- Ran a local finite-field sign-orbit exploration (not committed): p=2147483647, E:y²=x³+x+1, all-residue x=(0,6,11,12,14,21,22,25,26,31), exact signed sums yielded 2^(m−1) distinct abscissae for each m=2..10. Direct fixed-degree Sylvester resultant/interpolation independently validated every listed root for m=3..6, degrees 4,8,16,32.
- Exhaustively searched small odd primes p>3 and found a minimal-field witness: p=5, a=b=1, x=(0,3,4), S4(X)=−X⁴−X³−X²−X over F5, roots {0,2,3,4}; the four sign orbits are distinct.
- Opened draft PR #5068 with tracking line before committing the reusable verifier.

## Decisions made (and why)
- Study exact **lex elimination-degree obstruction**, not a first-fall, graded-F4 solving-degree, or runtime lower bound. The count of distinct roots forces a univariate elimination polynomial of at least that degree for any radical exact Fp-fiber projection.
- Keep this work disjoint from the completed sector-chart circuit, and do not mislabel generator degree ≤4 as a bounded solving degree.

## Open threads
- Publish executable exact-resultant/signed-fiber cross-checks for 50 random instances in each cell (p31, m=3..6), record CSV and findings.
- Independently replay, use Sage/Singular on common unspecialized slices, and obtain signed PCC standing. Neither F4 nor independent replay is available here.

## Errors
- No Sage, Singular, Magma, FGb or msolve executable in the current container; only SymPy. No F4 metric can be honestly filled.
- The shared docs/session-logs/2026-11-23.md is very large and heavily concurrent. Use a separate dated per-task session file rather than replacing others' entries.


## 19:18 UTC — What was done (commands, files, features)
- Added `summation_split/degree_barrier.py`, `test_degree_barrier.py`, `DEGREE_BARRIER.md`, `degree_barrier_summary.csv`, and `degree_barrier_vs_m.svg` to draft PR #5068. Code deliberately reuses existing `core.specialized_summation` and `sector.add` instead of changing either source module.
- Local Python 3/SymPy fallback ran `PYTHONPATH=/mnt/data python -m unittest -q summation_split.test_degree_barrier`: **3 tests passed** in 1.919 s (run wall ~3.37 s, high-water RSS 128840 KiB including interpreter/SymPy).
- The gate enforced **50 full-degree randomized all-rational-input instances per each of 8 (p,a,b,m) cells** at p=2^31-1, a,b=(1,1)/(1,4), m=3,4,5,6. **400/400** cases checked all coefficients of the exact resultant against the product of distinct signed-sum roots; the identity p=5, (a,b)=(1,1), m=3 was also checked.
- Locally regenerated `degree_barrier_gate.csv`, 401 lines including header, SHA-256 `b85cbf643200db00d3db7f4881b727266389a49e2df668e5db9d73421d2c8f50`. The remote module regenerates this CSV; the remote summary CSV freezes the 8 cells. No F4 run or d_F4 claimed.

### Decisions made
- Present this as a **negative lex-elimination control** only, not an asymptotic GB runtime lower bound. The existence of an exponential number of distinct projected roots forces high-degree *univariate output*, but does not control grevlex Macaulay degree.
- No new claim of "novel decomposition" or first-fall stabilization. Keep first-fall/d_F4 fields empty.

### Open threads
- Exact-source GitHub CI, independent signed PCC and a common unspecialized Sage/Singular F4 benchmark are required before any stronger performance statement.

### Errors
- No external F4 backend installed. The local minimal import shim for core/sector mirrors the fetched GitHub public module interfaces, but the remote CI remains the authoritative original-module execution control.
