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
three release binaries,
container/compiler identity, and scientific environment must be frozen and
committed. Generation uses `--unsat 4 --min-sat 2`; the draw files must then
pass the exact prefix audit and be committed and pushed before any closure
step. A cell lacking the registered four UNSAT and two natural SAT records is
unmeasured. The seed is never changed to obtain a desired panel.

The execution order is:

1. verify the frozen protocol, registered source tree, current pushed
   descendant ref, binaries, and sanitized scientific environment;
2. run three restartable, atomic generator invocations and commit the audited
   draw files together with each `.log` and `.generation.json` custody
   sidecar;
3. replay the historical `(m,n,ell,draw)=(4,12,5,24)` identity control;
4. run the first two committed satisfiable draws per new cell;
5. run the first four committed unsatisfiable draws per cell; and
6. use the frozen native analyzer to classify the five packages and create an
   `experiment.export/v1`, source-linked Markdown report, SVG, and inspected
   PDF.

Every closure invocation names exactly one logical draw and starts at most one
native closure process. Detached jobs therefore have a 24-hour maximum rather
than a monolithic multi-day stage. Evidence is append-only under
`runs/<logical-name>/attempts/<NNNN>/`; checkpoints are shared only by that
logical name. Retries never overwrite an earlier stdout, stderr, status, or
metadata record.

## Durable workers and current launch block

Use a full repository checkout: the Cargo workspace and path dependencies are
not compatible with the earlier sparse-path launch sketch. On an ephemeral
worker, set `DREG_OUTPUT_ROOT=$EPHEMERAL_OUT`. The runner rejects a non-absolute
or non-durable output root. On resume it imports and validates the prior tree
from `$EPHEMERAL_RESUME_DIR` into an empty `$EPHEMERAL_OUT` before opening a new
attempt; it never treats the old and new roots as the same directory.

The application checkpoint interval is 300 seconds. The worker sends `TERM`
and allows 60 seconds before forced termination. A signal checkpoint is best
effort; durability comes from periodic checkpoints and the append-only attempt
record. Resumed algebraic state remains evidence, but resumed wall and CPU time
are never metrics.

Launch is currently blocked: the available AWS identity resolves to account
`590183823895`, while the deployed ephemeral stack is in account
`104186811093`, and `/ephemeral/config` is absent under the current identity.
This is an infrastructure obstruction only. Do not launch under the wrong
account, and do not interpret it as mathematical evidence.

Commands and exact decision rules are recorded in
[PREREGISTRATION.md](PREREGISTRATION.md). The canonical machine-readable
contract is [protocol.json](protocol.json).
