"""Lay out the three frozen GLS run records; never compute research results."""
import csv
import shutil
from pathlib import Path

import typst

HERE = Path(__file__).resolve().parent
RUNS = {
    "v1": ("20261021T1530Z", "initial audit-heavy implementation", "1a1f39d56acf1d66fa70f61563900f7d6cabd4c1c15d0186c0b9cf400b597164", "27f9f04ba4cd4a500ef70ee79c8f243750ef070dfe948cc7105c97bde7ec2800"),
    "v2": ("20261021T1533Z", "post hoc early-stop implementation", "3483134315e51797b956df8eb29410252e65e86418285855e94d7b463bc92626", "ec4d77908b179b44a758af3e7b8b0539019e41b96faa2653acd45bc3b85bf23b"),
    "v2_aa": ("20261021T1536Z", "repeated v2 A/A control", "3483134315e51797b956df8eb29410252e65e86418285855e94d7b463bc92626", "d64ab66dc2a38adf4a13beded258acb76308e31db2343d3cc47b32a2b44c5a87"),
}

for run, (stamp, description, source_hash, record_hash) in RUNS.items():
    directory = HERE / "reports" / f"run-{stamp}-gls-symmetry"
    directory.mkdir(parents=True, exist_ok=True)
    for suffix in ("svg", "png"):
        shutil.copyfile(HERE / f"gls_fold_cost_{run}.{suffix}", directory / f"gls_fold_cost_{run}.{suffix}")
    rows = list(csv.DictReader((HERE / f"gls_symmetry_summary_{run}.csv").open()))
    flat = next(r for r in rows if (r["p"], r["b"], r["arm"]) == ("211", "12", "flat"))
    fold = next(r for r in rows if (r["p"], r["b"], r["arm"]) == ("211", "12", "fold"))
    rho = next(r for r in rows if (r["p"], r["b"], r["arm"]) == ("211", "12", "rho"))
    title = f"GLS symmetry {run} run record"
    graph = f"gls_fold_cost_{run}.png"
    markdown = f"""# {title}

UTC start directory: `{stamp}`. Description: {description}. Host:
`ip-172-31-26-135.us-east-2.compute.internal`, Intel Xeon 6975P-C, 32
logical CPUs. Isolation: L0 shared host. The run exited successfully; the
[stderr](../../gls_symmetry_run_{run}.stderr) file is empty. The exact
[protocol](../../GLS_SYMMETRY_PROTOCOL.md) and its explicit v2 amendment
define the three curves, bases, relation oracle, targets and stopping rules.

The command was native `rustc -D warnings -O` compilation of
[`gls_symmetry_ic{'_v1' if run == 'v1' else ''}.rs`](../../gls_symmetry_ic{'_v1' if run == 'v1' else ''}.rs)
followed by a single complete producer execution with stdout redirected to
[`gls_symmetry_results_{run}.ndjson`](../../gls_symmetry_results_{run}.ndjson)
and stderr to the linked file. Source SHA-256: `{source_hash}`. Raw output
SHA-256: `{record_hash}`. Producer snapshot commit: `17405f4ef` (v1 source
and v2 source were frozen there). No run was substituted or discarded.

Requested/completed: 3 curve records, 9 full signed-triple censuses, 9
corrupted-orbit controls, 54 cold one-target IC solves and 27 same-target
negation-rho solves: **102/102 records**, 0 failures or timeouts. The
[independent audit](../../gls_symmetry_audit_{run}.json) passed all group,
curve-order, orbit, rank and scalar checks. The
[native summary](../../gls_symmetry_summary_{run}.csv) has all 27 cell-arm
rows, medians and min/max L0 times. Three target scalars repeat the same
setup trajectory in each cell; setup trial counts are one deterministic
trajectory, not three independent samples.

Example measured cell `p=211,r=44221,B=12`: flat/fold collection queries
`{flat['median_trials']}/{fold['median_trials']}`; cold recorded `Fp²`
multiplications `{flat['median_f2_mul']}/{fold['median_f2_mul']}`; rho
`{rho['median_f2_mul']}`. L0 median time (three targets):
`{flat['median_ns']}/{fold['median_ns']}/{rho['median_ns']}` ns for flat/fold/rho.
The displayed ratio of flat/fold multiplication counts is
`{fold['flat_over_arm_w0']}`; it is derived by dividing the two recorded
counts. Min/max times, all other cells and separate operation categories
are in the CSV and NDJSON. No uncertainty interval was recorded beyond the
three-target min/max; setup count variation is absent by construction.

![All-cell count ratio for this run](gls_fold_cost_{run}.svg)

The graph is rendered by [`render_gls_visuals.py`](../../render_gls_visuals.py)
from the native CSV without recomputing results. [PNG](gls_fold_cost_{run}.png)
is the matching 150 dpi render. [Editable PDF source](README.typ) and
[PDF](README.pdf) include this graph. Field additions, inner base-field
arithmetic, hashing and allocations are not exhaustively priced; times are
L0 descriptive. This run report records observations only.
"""
    (directory / "README.md").write_text(markdown)
    typ = f"""#set page(paper: "a4", margin: 20mm)
#set text(font: "Liberation Sans", size: 9pt)
= {title}
UTC start: `{stamp}` · {description}. Host: Intel Xeon 6975P-C, 32 logical CPUs; L0 shared host.

The native producer compiled with `rustc -D warnings -O` and completed one execution. Source SHA-256: `{source_hash}`. Raw NDJSON SHA-256: `{record_hash}`. Raw records: `gls_symmetry_results_{run}.ndjson`; native summary: `gls_symmetry_summary_{run}.csv`. All are in the parent study directory.

There are 102/102 expected records: 3 curves, 9 exact relation censuses, 9 negative controls, 54 cold index-calculus solves, and 27 matched rho solves. Zero recorded failures or timeouts. Independent order, group-action, witness-orbit, rank and recovered-scalar audit passed.

At $p=211$, $r=44221$, $B=12$: flat/fold collection queries `{flat['median_trials']}/{fold['median_trials']}`; flat/fold/rho recorded $F_(p^2)$ multiplications `{flat['median_f2_mul']}/{fold['median_f2_mul']}/{rho['median_f2_mul']}`. L0 median nanoseconds for three targets: `{flat['median_ns']}/{fold['median_ns']}/{rho['median_ns']}`. The flat/fold multiplication-count ratio is `{fold['flat_over_arm_w0']}`. All cell values and min/max times are in the native summary CSV. These are descriptive counts and shared-host times.

#image("{graph}", width: 100%)

The graph displays the nine recorded flat/fold multiplication-count ratios, with three target scalars per cell and one deterministic setup trajectory. Field additions, base-field arithmetic inside field operations, hashing and allocations are not exhaustively priced. This run report makes no speed claim. See `README.md` for links, commands and limitations.
"""
    (directory / "README.typ").write_text(typ)
    typst.compile(str(directory / "README.typ"), output=str(directory / "README.pdf"))
