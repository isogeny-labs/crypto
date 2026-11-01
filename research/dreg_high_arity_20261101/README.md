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
refute four exactly-unsatisfiable draws by degree four in each cell?  The
study is not a timing comparison, an end-to-end index-calculus result, or a
claim that any constant degree persists at cryptographic scale.

Before `run.sh gen`, the protocol must be frozen and committed.  The draw
files produced by `gen` must then be audited, committed, and pushed before
`identity`, `controls`, or `measure` may run.  Each cell must contain at least
four unsatisfiable and two satisfiable natural draws; otherwise that cell is
unmeasured and the seed is not changed.

The execution order is:

1. verify `sources.sha256` and the frozen source/build identity;
2. generate and commit the three deterministic draw files;
3. replay the historical `(m,n,ell,draw)=(4,12,5,24)` identity control;
4. replay the first two committed satisfiable draws per new cell;
5. scan the first four committed unsatisfiable draws per cell, one process at
   a time, with a 32 GB address-space cap and application checkpoints; and
6. create an `experiment.export/v1`, source-linked Markdown report, SVG, and
   inspected PDF, retaining every timeout, cap, OOM, resume, and failure.

On an ephemeral worker, set `DREG_OUTPUT_ROOT=$EPHEMERAL_OUT` before the
identity, control, and measurement steps.  The runner continues to read the
committed inputs from `research/dreg_high_arity_20261101/draws`, while raw
runs and checkpoint generations go to the worker's durable collection volume.
`DREG_DRAWS_DIR` may override the immutable input location when a fetched
checkout is laid out differently.  Never place the only copy of a checkpoint
inside an ephemeral checkout.

Commands are recorded in [PREREGISTRATION.md](PREREGISTRATION.md).  The
canonical machine-readable contract is [protocol.json](protocol.json).

