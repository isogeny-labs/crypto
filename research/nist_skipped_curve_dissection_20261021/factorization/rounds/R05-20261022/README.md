# R05: sieving checkpoint retained after exit 139

The bounded local run ended at **2026-10-22 11:28:15 UTC with exit 139**.
Stderr reports a segmentation fault in the native command chain; the exact
failing component and cause are unknown. No outer-timeout signal was
logged and no new factor was returned. The total remains **eight of nine
integers fully factored, thirteen of fourteen timeout fields resolved**.
No factorization worker was running at finalization.

A separate [native timeout diagnostic](../../native-timeout-diagnostic-20261023/README.md)
subsequently reproduces original exit 139 versus patched exit 0 on the
same input and a bounded small Q range. It verifies a timeout printf
defect and a two-line correction; the production backtrace remains absent.
The diagnostic and patched binary are separate from this unchanged R05 run.

The main relation file contains **3,164,757 lines**, matching the native
log's final raw-relation counter. A separate worker output contains
**32,684 lines** and remains separate. Both files and the range ledger
are retained and individually hashed. Duplicate removal, relation
validity/rank analysis, filtering and final NFS linear algebra were not
completed. The configured 46,000,000-relation target is neither a measured
count nor a guarantee of enough usable relations.

The [result](result.json), [summary](summary.json),
[native receipt](native-receipt.json), and [independent replay](independent-verification.json)
separate failed execution from the unchanged certified partial partition.
Native failure stopped automatic refinement. Finalization explicitly
refined the empty factor-hint file and independently replayed the original
512-bit target and all three proven primes, without a new search.
Both native polynomial formats pass fresh exact input/common-root checks.
The raw reader-format failure and corrected framing are retained; the
native job itself was preserved unchanged.

The [FactorDB snapshot](../../factordb-20261023/README.md) checks all nine
original targets and the exact residual. It supplies no additional factors.
All returned products and whole-piece groupings pass native arithmetic
checks. The residual remains `C` with no proper factor returned.

Read the [completion report](reports/completion-20261023T0100Z/REPORT.md),
[inspected final PDF](reports/completion-20261023T0100Z/report.pdf), and
[frozen protocol](PROTOCOL.md). The earlier [observation](OBSERVATION.json),
[interim report](reports/run-20261022T1027Z/REPORT.md), and PDF remain
unchanged as evidence of the worker running at their recorded timestamp.

## Archive and continuation

The deterministic full raw archive is identified by its
[manifest](archive-manifest.json), sibling SHA-256, and
[validation record](archive-validation.json). Its actual publication status
is explicit; no remote object or store entry is implied by a local manifest.
Bulk publication has the same [artifact-store access blocker](../../../../../problems/nist-factorization-raw-archive-publication.md)
as R02. Results, receipt, three prime certificates, selected polynomial
and verifier inputs stay loose for review under AGENTS.md section 13a.

With the checksum-bound full archive available, the existing job can
restore the relation checkpoint instead of restarting from R04:

```bash
sha256sum -c runs.tar.xz.sha256
NIST_FACTOR_CHECKPOINT="$PWD/runs.tar.xz" GP_BIN=gp \
  GNFS_BIN_DIR=/opt/yafu/factor/lasieve5_64/bin NFS_POLY_BATCH=120 \
  YAFU_PATCHED_BIN=/opt/yafu-timeout-fix/yafu \
  YAFU_TIMEOUT_CONTROL_DIR=/opt/yafu-timeout-fix/native-controls \
  bash ../../native-timeout-diagnostic-20261023/resume_with_checked_tool.sh \
  skipped_256_876-twist2 /tmp/nist-p256-next 1800 --resume
```

This launches a new computation; finalization launched no later allocation.
Use the binary that passes the separate native timeout controls. Keep
the separate worker file intact; its presence does not establish safe
merge semantics. A prepare-only restore and independent arithmetic replay
verify portability without launching another search. The distributed card
remains draft and unqueued. The complete prime factorization and historical
skip reason remain unresolved.
