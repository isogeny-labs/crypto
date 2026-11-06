# Stopped XOR-SAT heuristic factorial and matrix admission smoke

The [frozen ten-arm protocol](protocol.json) was pushed at
`8e8eaaafbeb3d868ef93e0832f507bc98d215c5e` before timing. Its runner
verified the same two 5+1 XCNF files, three compressed S3 archives, worker and
CryptoMiniSat 5.14.7 binary hashes. The first hash baseline reached
1,000,001 conflicts in 87.425 seconds of L0 local process wall and returned
`INDETERMINATE`. The second, Gray baseline reached the 140-second external
watchdog without a final solver status. Its last printed restart row showed
about 950,000 conflicts; the exact final counter is unknown. The runner killed
the solver process group, retained its partial stdout and empty stderr, and
stopped as the protocol required. No later factorial arm ran.

| Arm | Branch | Auto-disable Gauss | Process wall (s) | Exit | Final conflicts | Status | Peak child RSS upper bound (bytes) |
| --- | --- | ---: | ---: | ---: | ---: | --- | ---: |
| hash A1 | vmtf+vsids | 1 | 87.425 | 15 | 1,000,001 | `INDETERMINATE` | 403,832,832 |
| Gray A1 | vmtf+vsids | 1 | 140.054 | -9 | unknown | `WATCHDOG` | 296,910,848 |

The 227.479-second sum is only these two solver process intervals. Formula
export, S3 GET, worker build, useful relation yield, graph closure, rank,
linear algebra, target descent and rho are outside it. The incomplete A/A
controls and factorial prevent choosing a solver setting or factor base. The
120-second CryptoMiniSat `--maxtime` flag did not produce a terminal solver
status within the 140-second process watchdog on this host; whether that
reflects CPU-time accounting, long inprocessing, or host contention remains
unresolved. This observation does not establish a solver defect.

The separately labelled exploratory [matrix-admission smoke](evidence-01/matrix-smoke/summary.json)
tested CryptoMiniSat's `--maxmatrixcols 25000 --maxnummatrices 10` on the same
two formulas, at 20,000 conflicts. Its logs show that the setting admits
332×20,916 XOR matrices that the default 1,000-column limit excludes. All
six smoke calls returned `INDETERMINATE`. Process walls in seconds were:

| Formula | Wide first | Default | Wide repeat |
| --- | ---: | ---: | ---: |
| Hash 5+1 | 6.371 | 6.193 | 14.624 |
| Gray 5+1 | 16.613 | 10.572 | 24.430 |

The repeated wide calls varied substantially, and variant order was not
randomised or isolated. The supported conclusion is matrix admission, not
a runtime improvement. The [independent replay](evidence-01/replay.json)
checks the frozen protocol hash, S3 compressed bytes, XCNF and binary hashes,
all retained raw-output hashes, capped statuses, watchdog classification and
the different matrix-admission traces. Raw stdout/stderr are stored as
deterministic gzip files in `evidence-01/`.

Next, freeze an isolated Linux resource envelope and a cap that allows all
matched arms to finish, with adjacent A/A controls. A larger matrix-column
limit merits a separate paired experiment because it changes the actual XOR
matrices used, not merely the wall cap. Any SAT assignment must pass the
original CNF, XOR, field, domain and group verifier before graph/rank work.
The [selection problem](../../../../problems/n83-mixed-orbit-factor-base-selection.md)
remains open, and `selected_best_total_runtime` is null.
