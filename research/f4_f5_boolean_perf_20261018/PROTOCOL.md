# Boolean F4/F5 performance, F6 and root solvers

Requested: improve F4/F5 on the linked `perf-history` Boolean matrix report,
and compare our F6 and both generic polynomial-system and Semaev root solving.
Baseline: main `1e2bf0715a2a927afc92e6730d6e65b59f15ab9e`.

The primary matrix fixture is copied from `examples/perfbench/bool_gb.rs`:
K_1 over GF(2^23), divisor index 0, two summands, one planted target,
StdRng seed 230, 22 Boolean variables, degree 4. Construction, reduction,
readback are included; verification and fixture generation are excluded.
Holdouts use seeds 231 and 232. Scaling controls use n=17 and n=19.
F4/F5 matrix outputs must have the same canonical row space. Each algorithm's
before/after output digest and word-XOR accounting must agree exactly.

Three boundaries are kept separate:

* Matrix step: F4 and F5 only. F6 is a decomposition oracle, with no
  standalone matrix-step interface. Matrix rows are not solver roots.
* Complete Boolean roots: matrix F4/F5, inherited F4, generic Boolean F4
  basis plus readback, and generic quadratic Möbius, Gray and Monica solvers.
  Compare sorted complete root sets against independent Gray enumeration.
* Decomposition: F4/F5, F6, indexed F6 and Semaev pair-table root finding on
  the same target and factor base. Verify witnesses by group addition and
  require verdict agreement. Pair-table reusable setup is reported separately;
  cold totals add that setup to the query. This is a decomposition-stage
  comparison, not an end-to-end IC/rho measurement.

One Rayon worker. Runtime attempts use the existing native `isolated_bench`:
exclusive `/tmp/crypto-bench.lock`, physical-core reservation with sibling
excluded, preflight unrelated CPU and PSI gate, affinity restoration, and
postflight unrelated CPU accounting. Immovable tasks are retained in records.
Only uncontended successful runtime samples are eligible. Builds use the same lock
through `flock`. Preserve every failure and timeout; a missing result is not
a negative decision. Shell timeout is 60 seconds per diagnostic command and
300 seconds per final multi-repeat comparison. No builds/tests overlap timing.

Before modification preserve a baseline binary and profile the primary case.
After modification run A/A, then at least five AB rounds. Retain all samples,
minimum, median and spread. Claim a gain only beyond the observed A/A range
and 2%; otherwise report neutral. Profile timings are components of complete
calls, not independent end-to-end gains. External references: Magma, msolve,
FGb, Singular and PolyBoRi; availability and exact computation compatibility
must be disclosed. No claim of being fastest without those measurements.

Current requirement status and completed commands will be recorded in REPORT.

Preflight update before candidate measurement: the host refused runtime with
32.30 unrelated CPU-seconds/second and CPU PSI avg10 78.26. Runtime work is
therefore unresolved here, not relaxed into an unisolated speed claim.
Use Callgrind 3.19, merged threads and explicit client requests around each
complete call, for deterministic Ir comparisons. Profiled wall times are not
native runtime measurements. Callgrind exposes AVX2 rather than this host's
native AVX-512, so instruction conclusions are limited to that execution path.

Candidate hypothesis, from the baseline profiles: memset accounts for
17.55% of F4 Ir and 17.32% of F5 Ir. Opt-in single-write Gray tables avoid
zeroing entries subsequently overwritten and omit the unused end of a partial
last table. Preserve row outputs and word-XOR counts. Keep production defaults
unchanged without qualified native speed measurements. Compare same-binary
KIC_GF2_WRITE_TABLE_ONCE=0/1 controls, plus the saved original binary.
