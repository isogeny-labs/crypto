# Publication integration

Round13 was constructed and sealed against parent
7fde2b9498ec30ea2e111f1b178ae000e465331d. Before publication, the PR branch
advanced to9de9a2beb3d5f6c7048956463ec1446a6cf6ee54 by merging main.
Preserve that merge and all unrelated incoming work. No shared branch
rebase or force push is used.

The incoming merge does not change the study, the pinned binary-field and
curve-construction sources, identity-certificate implementation, hashing
implementation or Lean dependencies. It overlaps three canonical files:
docs/index-calculus-scoreboard.html, docs/ic/dashboard-overview-data.json
and problems/README.md. Retain both sets of frontier/problem changes and
regenerate the bounded dashboard from its merged source.

Acceptance for the integrated publication is unchanged source-hash pins,
clean canonical-document integration, native tests, exact round13 replay,
root serial library tests, site/workflow tests and a dashboard check.
The sealed runs archive, its270-theorem audit and mathematical outputs are
immutable. Integration receipts are separate; this is validation of incoming
repository work, not a new mathematical experiment or IC performance run.

The local integration of the unpublished checkpoint had one adjacent-row
conflict in problems/README.md: the round13 Koblitz update and an incoming
Spot-recovery update. It was combined by retaining the round13 Koblitz
row and the incoming Spot row. The other incoming problem updates remain.
This was local unpublished work; the remote PR already contained the main
merge and did not report a dirty merge state. No shared history was rebased
or force-pushed. The final diff against the remote head changes only the
Koblitz row of that index. Both canonical HTML changes merged cleanly,
and dashboard JSON was regenerated with the merged tooling.

Completed validation on the integrated tree:

- All93 source SHA256 pins match the sealed ledger before and after tests.
- Native:110 passed,1 ignored;exact replay reproduces all15 JSON/checkpoint
  files and all three generated Lean modules byte for byte.
- Root serial library suite:4464 passed,0 failed,119 ignored. The frozen
  construction's earlier4405/118 result remains separately recorded.
- Site49 and workflow4 tests pass. Merged dashboard render/check and
  whitespace checks pass. No unmerged index entries remain.
- The original270-theorem Lean audit is retained:its sources and pinned
  dependencies are unchanged. This does not claim another full audit run.

Separate integration receipts are in integration-runs.tar.xz,with member
and archive SHA256 files. Extraction uses the integration-runs prefix and
does not overwrite the sealed mathematical runs directory. No arithmetic,
rank,divisor support or IC efficiency claim changes during integration.

Whitespace-scope correction:the initial integration receipt's unqualified
claim that the index whitespace check passed was too broad. Comparing
against the old local checkpoint also includes incoming main files,whose
existing SVG/PDF formatting produces warnings. Those warnings are retained
in integration-runs/whitespace-check.log;they are not introduced by round13.
The actual publication diff is checked against remote head9de9a2b and
passes. Incoming files are preserved. The sealed receipt is left intact
with this explicit correction rather than rewritten.
