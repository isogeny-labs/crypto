# Primary N83 hash 4+2 wide-XOR natural access result

The [single-arm protocol](protocol.json) was frozen and pushed in commit
`48610033170` before timing. It used the exact first public trial and
all-affine identity pattern on the primary `a=0` curve, with the retained
K600 sequential hash small base and disjoint K600 hash residual pool.
The formula, compressed S3 archives, public corpus, worker and CryptoMiniSat
5.14.7 binary matched their frozen SHA-256 digests at preflight. The
20,000-conflict exploratory smoke and the full call both admitted the
332×20,916 XOR matrices under `--maxmatrixcols 25000
--maxnummatrices 10 --autodisablegauss 0`.

The full call reached its **1,350.063-second external process watchdog**.
The runner killed and reaped the solver process group; the exit code was
`-9`. CryptoMiniSat produced no terminal status line and no final conflict
counter. Its last visible restart row said about **3.397 million**
conflicts, but that is only a partial progress marker, not a final count.
The process receipt therefore records `status: WATCHDOG` and
`conflicts: null`. It observed a 655,212,544-byte Darwin child-RSS upper
bound and an empty stderr stream. There was no SAT assignment to send to
the original-XCNF, field, orbit-domain and curve-group verifier.

The [independent replay](evidence-01/replay.json) checks the frozen protocol
SHA-256, prior export receipt, corpus and archive bytes, formula and binary
hashes, the 20,000-conflict matrix smoke, decompressed raw solver streams,
matrix-admission trace, process exit, watchdog classification, and null
selection. It returned `PASS_BOUNDED_REPLAY`. Adding a false
`s SATISFIABLE` line to a scratch copy of the stdout was rejected at the
raw-stream digest. The raw stdout/stderr are preserved as deterministic gzip
files in `evidence-01/`, alongside the process and summary receipts.

This is an **L0 shared-host access gate**, not a measured factor-base
improvement. The during-run host sample had load averages 43.93, 25.53 and
28.58 on a 14-logical-CPU Mac; the after-run sample was also high.
The earlier default-solver 4+2 arm returned `INDETERMINATE` at roughly two
million conflicts in 133.704 seconds, but host and solver settings differ,
so dividing those walls would not be a valid throughput comparison.
The widened run extends the bounded search on one pattern and remains
censored. It supplies no natural partial, graph closure, independent rank,
linear algebra, unseen-target descent, complete cold cost or matched rho.
`selected_best_total_runtime` remains null.

Further work should use an admitted isolated host and a paired protocol to
test solver settings, while retaining natural useful relation yield as the
first selection gate. After that, full-rank and target stages must be charged
to the same one-target cold runtime before any factor base is selected.
