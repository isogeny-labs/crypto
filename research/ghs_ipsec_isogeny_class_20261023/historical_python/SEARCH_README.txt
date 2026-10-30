RESUMABLE IPsec F_(2^155) ISOGENY-CLASS SEARCH
Started 2026-10-23, following the user's request to proceed despite the scale.

This is a real, unbounded local computation. Its lifetime is limited to the
execution workspace. No scheduler or external hosting has been provisioned.
The live database and status file record actual progress; no estimate is a
promise of completion. Earlier PDFs and archives are historical snapshots.

READ STATUS (also checks whether a worker holds the database lock):
  python resumable_search.py --status

RESUME in the foreground after a stop, using the same database:
  python resumable_search.py

RUN A FINITE BATCH instead (counts expansions in this invocation):
  python resumable_search.py --max-nodes 100

STOP GRACEFULLY:
  Send SIGTERM to the PID in live-search/worker.pid. The current prime-degree
  expansion finishes and commits, then the worker exits. --status detects
  an absent worker even if it was killed before updating status.json.

Redirect stdout/stderr to a log when starting a background worker. A
background process does not survive destruction, suspension, or expiration
of its execution workspace.

WHAT IS BEING ENUMERATED
Vertices are j-invariants modulo absolute Frobenius. Each orbit has 155
members in this class. GHS ranks and the necessary Hess bounds are invariant
under this field automorphism. The smallest polynomial-basis integer is the
canonical representative. Modular roots of degrees 3,17,31 supply edges;
each representative has two rational neighbors for each degree. Every edge
stores its original modular root and the Frobenius exponent taking that root
to the canonical target. Reverse modular equations are checked explicitly.
This records a path with isogeny correspondences and field automorphisms;
it does not construct explicit maps at degrees 17 or 31.

The old degree-3/7 search did not generate the whole class. The replacement
generator set addresses that coverage obstruction. Whole-class coverage of
this set is conditional on the GRH-dependent completeness of the computed
class group. The unconditional subgroup lower bound is independently
certified in full-class/. The database never labels a component exhausted
as a completed whole-class search without a separate coverage review.

WHAT IS TESTED
For every visited Frobenius orbit the database records direct GHS ranks and
descent genera for every proper subfield F2, F32, and F_(2^31). It also records
the necessary genus bound from the Hess two-factor support screen over F32.
Small values identify candidates for further analysis. A necessary bound is
not a constructed descent, a complete security assessment, or a DLP solution.
Candidate thresholds: direct m<=6 over F2 or F32; direct m<5 over F_(2^31);
or Hess necessary genus bound <=31 over F32. Candidates are also exported to
candidates.jsonl. Database rows remain authoritative after any interruption.

PERSISTENCE AND VERIFICATION
SQLite transactions commit each completed degree expansion and each screen.
A crash can require recomputing the current unfinished expansion. Previously
committed vertices and edges are retained, and unique constraints prevent
duplicate insertion. An exclusive file lock prevents two simultaneous
workers. Configuration, source hashes, and polynomial hashes must match on
restart. Changing the source requires an explicit database migration or a
new search directory; the worker refuses a silent mismatch.

verify_resumable.py kills a test worker mid-expansion, checks SQLite integrity,
restarts, and compares all logical tables with an uninterrupted run. It also
checks lock exclusion, the seed's known GHS ranks, Frobenius invariance,
modular-edge witnesses, and inactive-worker detection. Test results are in
resumable_verification.json. Tests use temporary databases, not the live job.

PORTABILITY
Python 3 with the standard library is sufficient on Linux. Copy these files
and preserve the live-search/ directory to resume elsewhere:
  analyze.py, isogeny_search.py, resumable_search.py
  sources/phi_j_3.txt, sources/phi_j_17.txt, sources/phi_j_31.txt
For a consistent live SQLite backup use sqlite3.Connection.backup(), or stop
the worker before copying the database. A raw copy during writes may omit
WAL contents. Live databases and checkpoints are excluded from this source package.

Modular-polynomial data: Andrew Sutherland, MIT,
  https://math.mit.edu/~drew/ClassicalModPolys.html
  https://math.mit.edu/~drew/modpolys/jfiles/phi_j_17.txt
  https://math.mit.edu/~drew/modpolys/jfiles/phi_j_31.txt
See prior reports for mathematical references and class-group certificates.
