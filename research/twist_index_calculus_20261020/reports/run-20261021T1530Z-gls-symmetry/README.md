# GLS symmetry v1 run record

UTC start directory: `20261021T1530Z`. Description: initial audit-heavy implementation. Host:
`ip-172-31-26-135.us-east-2.compute.internal`, Intel Xeon 6975P-C, 32
logical CPUs. Isolation: L0 shared host. The run exited successfully; the
[stderr](../../gls_symmetry_run_v1.stderr) file is empty. The exact
[protocol](../../GLS_SYMMETRY_PROTOCOL.md) and its explicit v2 amendment
define the three curves, bases, relation oracle, targets and stopping rules.

The command was native `rustc -D warnings -O` compilation of
[`gls_symmetry_ic_v1.rs`](../../gls_symmetry_ic_v1.rs)
followed by a single complete producer execution with stdout redirected to
[`gls_symmetry_results_v1.ndjson`](../../gls_symmetry_results_v1.ndjson)
and stderr to the linked file. Source SHA-256: `1a1f39d56acf1d66fa70f61563900f7d6cabd4c1c15d0186c0b9cf400b597164`. Raw output
SHA-256: `27f9f04ba4cd4a500ef70ee79c8f243750ef070dfe948cc7105c97bde7ec2800`. Producer snapshot commit: `17405f4ef` (v1 source
and v2 source were frozen there). No run was substituted or discarded.

Requested/completed: 3 curve records, 9 full signed-triple censuses, 9
corrupted-orbit controls, 54 cold one-target IC solves and 27 same-target
negation-rho solves: **102/102 records**, 0 failures or timeouts. The
[independent audit](../../gls_symmetry_audit_v1.json) passed all group,
curve-order, orbit, rank and scalar checks. The
[native summary](../../gls_symmetry_summary_v1.csv) has all 27 cell-arm
rows, medians and min/max L0 times. Three target scalars repeat the same
setup trajectory in each cell; setup trial counts are one deterministic
trajectory, not three independent samples.

Example measured cell `p=211,r=44221,B=12`: flat/fold collection queries
`829/254`; cold recorded `Fp²`
multiplications `242562/201581`; rho
`25635`. L0 median time (three targets):
`9716891/7534945/823465` ns for flat/fold/rho.
The displayed ratio of flat/fold multiplication counts is
`1.203298`; it is derived by dividing the two recorded
counts. Min/max times, all other cells and separate operation categories
are in the CSV and NDJSON. No uncertainty interval was recorded beyond the
three-target min/max; setup count variation is absent by construction.

![All-cell count ratio for this run](gls_fold_cost_v1.svg)

The graph is rendered by [`render_gls_visuals.py`](../../render_gls_visuals.py)
from the native CSV without recomputing results. [PNG](gls_fold_cost_v1.png)
is the matching 150 dpi render. [Editable PDF source](README.typ) and
[PDF](README.pdf) include this graph. Field additions, inner base-field
arithmetic, hashing and allocations are not exhaustively priced; times are
L0 descriptive. This run report records observations only.
