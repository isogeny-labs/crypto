# High-arity Boolean closure-degree discriminator

Status: **preregistered draft; no scientific run has started**.

This study executes the still-unmeasured native closure cells identified by
`research/factor_base_arity_review_20261024/README.md`:

| cell | chained variables | degree-4 columns |
|---|---:|---:|
| `(m,n,ell) = (5,11,2)` | 43 | 136,698 |
| `(m,n,ell) = (5,13,3)` | 54 | 342,541 |
| `(m,n,ell) = (6,11,2)` | 56 | 396,607 |

The question is deliberately narrow: does fixed-degree MutantXL closure
refute four exactly-unsatisfiable draws by degree four in each cell? The
study is not a timing comparison, an end-to-end index-calculus result, or a
claim that any constant degree persists at cryptographic scale.

## Registered work units

The export has five logical packages, each with one registered instance:

| logical package | retained native subprocesses |
|---|---:|
| historical identity replay | 1 |
| satisfiable controls across all cells | 6 |
| four UNSAT draws at `(5,11,2)` | 4 |
| four UNSAT draws at `(5,13,3)` | 4 |
| four UNSAT draws at `(6,11,2)` | 4 |

Thus the campaign contains 19 closure subprocesses, not 20. Three additional
native generator invocations produce the deterministic cell streams. Package
rows summarize their fixed finite panels; the native analysis retains and
validates every subprocess and attempt. No package average can hide a failed,
missing, malformed, or censored subprocess.

Before any `run.sh gen-m...` step, the protocol, registered source commit,
three scientific release binaries, the report release binary,
container/compiler identity, and scientific
environment must be frozen and committed. Generation uses
`--unsat 4 --min-sat 2`; the frozen ladder audit must independently replay the
seed stream and recompute every scientific field, not merely check structural
self-consistency. Each generation sidecar binds the JSONL and generator log by
SHA-256 and byte length. The replay-audited files and sidecars must be
committed and pushed before any closure step. A cell lacking the registered
four UNSAT and two natural SAT records is unmeasured. The seed is never
changed to obtain a desired panel.

The execution order is:

1. verify the frozen protocol, registered source tree, binaries, sanitized
   scientific environment, and a fully qualified `DREG_EXECUTION_REF` whose
   live origin object ID equals `HEAD`;
2. run three restartable, atomic generator invocations and commit the audited
   draw files together with each `.log` and `.generation.json` custody
   sidecar;
3. replay the historical `(m,n,ell,draw)=(4,12,5,24)` identity control;
4. run the first two committed satisfiable draws per new cell, in the listed
   cell and ascending-draw order, until every control is terminal;
5. run the first four committed unsatisfiable draws per cell in listed-cell
   and ascending-draw order, even if a shared control is resource-unmeasured;
   a contradictory or malformed shared control still stops execution;
6. use the frozen native analyzer to classify and retain all 19 subprocesses
   in the five-package `summary.json`; and
7. use the separately frozen native `dreg_high_arity_report` binary to create
   the `experiment.export/v1`, source-linked Markdown report, SVG, Typst
   source, and deterministic PDF without an external renderer; text extraction,
   visual inspection, and independent review remain separate publication
   gates.

Every closure invocation names exactly one logical draw and starts at most one
native closure process. Detached jobs therefore have a 24-hour maximum rather
than a monolithic multi-day stage. Evidence is append-only under
`runs/<logical-name>/attempts/<NNNN>/`; checkpoints are shared only by that
logical name. Retries never overwrite an earlier stdout, stderr, status, or
metadata record. Each directory begins with an atomic `allocated.json`
(`dreg.high-arity-allocation/v1`). A final allocation containing only that
marker and recognized setup temporaries is reused and does not consume one of
the four attempts; metadata or process evidence commits it to the retry count.
Metadata, status, and finished records are installed atomically; a torn setup
temporary without process evidence is retryable, not invalid scientific
evidence.

## Durable workers and current launch block

Use a full repository checkout: the Cargo workspace and path dependencies are
not compatible with the earlier sparse-path launch sketch. On an ephemeral
worker, set `DREG_OUTPUT_ROOT=$EPHEMERAL_OUT`. The runner rejects a non-absolute
or non-durable output root. On resume it hashes the prior evidence tree,
copies it from `$EPHEMERAL_RESUME_DIR` into an initially empty
`$EPHEMERAL_OUT`, recomputes the destination manifest, and requires exact
agreement before opening a new attempt. The checkpoint transport must also
supply `DREG_RESUME_ANCHOR`, an absolute ordinary manifest outside both trees
and outside the repository that it retained independently before transfer.
The runner recomputes the immutable source manifest and requires byte equality
with this anchor, so an
attempt removed before import cannot silently reset the retry budget. A v3
import record binds the copy, immutable, and external-anchor manifests; the
old and new roots are never treated as the same directory.

After an attempt stops and before checkpoint transport snapshots
`DREG_OUTPUT_ROOT`, run the frozen `seal-resume-anchor` step with an absolute
`DREG_RESUME_ANCHOR_OUT` outside that tree and the repository. It validates
the retained bindings and atomically creates the exact immutable manifest
without overwriting an existing anchor. Transport retains that file separately
from the output-tree snapshot and supplies its downloaded path as
`DREG_RESUME_ANCHOR` on restore.

The application checkpoint interval is 300 seconds. The worker sends `TERM`
and allows 60 seconds before forced termination. A signal checkpoint is best
effort; durability comes from periodic checkpoints and the append-only attempt
record. A corrupt, malformed, or wrong-system retained checkpoint is recorded
as resource exit `125` / `checkpoint_invalid`: it remains unmeasured and never
becomes a mathematical disproof. Resumed algebraic state remains evidence, but
resumed wall and CPU time are never metrics. For every successful row, the
analyzer requires the native `checkpoint_segments` count to agree with the
validated selected-attempt metadata: exactly one for a fresh store and at
least two after restore. Shell and native validation also require each
committed manifest to hash exactly all ordinary generation files, including a
unique `catalog.json`, and bind its `spec_sha256` to the catalog's system
identity; omitted, extra, or misbound state is never restored.

The checked-in [launch-request.json](launch-request.json) is deliberately
`blocked` and does not authorize spending. It caps the entire campaign at USD
20.00, one 16-vCPU/64-GiB-or-smaller worker at a time, one build, three
generator attempts, 76 closure attempts in total (19 first attempts plus at
most 57 retries), and the original absolute deadline
`2026-12-01T00:00:00Z`. Build, queue/idle, successful, failed, interrupted,
Spot-restart, checkpoint/storage, and transfer costs all charge that one
ledger; retries do not reset it. Unknown or insufficient account-budget or
lane-ledger headroom stops submission.

Launch is currently blocked for independent reasons: the available AWS
identity resolves to account `590183823895`, while the deployed ephemeral
stack is in account `104186811093`; `/ephemeral/config` is absent under the
current identity; and the 2026-11-01 account inspection found both USD 25
daily budget thresholds already exceeded. The exact draw-specific closure
commands do not exist until the frozen generators produce replay-audited draw
inputs. Before any build or job, those 19 ordered commands and the exact
backend submission command must be added to the request, all code, protocol,
inputs/manifests and configuration must be committed and pushed, and the
listed GitHub custody check must produce a retained receipt bound to the
request and image. Current budget and lane headroom must then be positively
verified. Do not switch account, region, pool, provider, or host type to avoid
any stop. These are infrastructure obstructions only and are not mathematical
evidence.

Commands and exact decision rules are recorded in
[PREREGISTRATION.md](PREREGISTRATION.md). The canonical machine-readable
contract is [protocol.json](protocol.json). The exact publication mapping is
specified in [REPORT_EXPORT_CONTRACT.md](REPORT_EXPORT_CONTRACT.md). Its code
and consistency tests pass; binary hash binding remains required before
freeze. The runner also requires a byte-identical fresh frozen-analyzer replay
immediately before publication. The analyzer itself emits only
`dreg_high_arity.summary/v1`,
and no export or report from a registered run is claimed to exist.

A campaign whose v3 resume custody is invalid still publishes an
invalid/no-conclusion bundle so the failure remains visible. Missing or invalid
frozen protocol/binary provenance is different: the publisher cannot establish
its own trust boundary and must refuse publication. Independent review records
live in `reports/reviews`, outside each strict five-file publication directory.
The derived `reports/` namespace is excluded from recursive raw-evidence
hashing, so filing a review cannot change an already generated export.
