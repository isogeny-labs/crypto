"""Render native recorded summaries only; no solver or comparison computation."""
import json
import shutil
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.ticker import EngFormatter
import typst

HERE = Path(__file__).resolve().parent
data = json.loads((HERE / "summary.json").read_text())
plt.rcParams.update({"font.family": "DejaVu Sans", "font.size": 10})

def save(fig, name):
    fig.savefig(HERE / f"{name}.svg", bbox_inches="tight")
    fig.savefig(HERE / f"{name}.png", dpi=150, bbox_inches="tight")
    plt.close(fig)

primary = [r for r in data["comparisons"] if r["case"][1:3] == [23, 230]]
fig, ax = plt.subplots(figsize=(8, 3.4))
for i, r in enumerate(primary):
    arms=[s for s in data["summaries"] if s["key"][:4]==r["case"] and s["key"][5]=="paired"]
    before=next(s for s in arms if s["key"][4] is False)
    after=next(s for s in arms if s["key"][4] is True)
    ax.bar(i - .17, r["reference_median_ir"], .32, yerr=[[r["reference_median_ir"]-before["min_ir"]],[before["max_ir"]-r["reference_median_ir"]]], capsize=3, color="#476e98", label="reference" if i == 0 else None)
    ax.bar(i + .17, r["candidate_median_ir"], .32, yerr=[[r["candidate_median_ir"]-after["min_ir"]],[after["max_ir"]-r["candidate_median_ir"]]], capsize=3, color="#c48236", label="single-write tables" if i == 0 else None)
ax.set_xticks(range(len(primary)), [r["case"][0] for r in primary])
ax.yaxis.set_major_formatter(EngFormatter())
ax.set_ylabel("Callgrind instructions (Ir)")
ax.set_title("Complete matrix step · 22 variables · five alternating pairs")
ax.legend(frameon=False)
fig.tight_layout()
save(fig, "matrix-instructions")

root_rows = [r for r in data["summaries"] if r["key"][1:4] == [23, 230, False] and r["key"][5] == "comparison" and
             r["key"][0] in ["solve-f4", "solve-f5", "solve-inherited", "roots-gray", "roots-moebius", "roots-monica", "generalized-native"]]
fig, ax = plt.subplots(figsize=(9, 3.8))
ax.barh([r["key"][0] for r in root_rows], [r["first_ir"] for r in root_rows], color="#476e98")
ax.set_xscale("log")
ax.xaxis.set_major_formatter(EngFormatter())
ax.set_xlabel("First complete-call instructions (Ir, logarithmic axis)")
ax.set_title("Every Boolean root · same planted n23 system · outputs checked equal")
fig.tight_layout()
save(fig, "complete-roots")

fig, axes = plt.subplots(1, 2, figsize=(10, 3.6), sharex=False)
for ax, random in zip(axes, [False, True]):
    rows = [r for r in data["cold_summaries"] if r["key"][1:4] == [23, 230, random] and r["key"][5] == "comparison" and
            r["key"][0] in ["decomp-f4", "decomp-f5", "decomp-f6", "decomp-generalized", "rootfind-mitm", "rootfind-table"]]
    ax.barh([r["key"][0] for r in rows], [r["median_ir"] for r in rows], color="#668968")
    ax.set_xscale("log")
    ax.xaxis.set_major_formatter(EngFormatter())
    ax.set_xlabel("Cold instructions (setup + query)")
    ax.set_title("generator control" if random else "planted target")
fig.tight_layout()
save(fig, "cold-decomposition")

matrix_table = "\n".join(f"| {r['case'][0]} | {r['case'][1]} | {r['case'][2]} | {r['reference_median_ir']:,.0f} | {r['candidate_median_ir']:,.0f} | {r['reduction_percent']:.3f}% | {r['reference_samples']}/{r['candidate_samples']} |" for r in data["comparisons"])
aa_text = "; ".join(f"{r['key'][0]}: {r['min_ir']:,}–{r['max_ir']:,} Ir ({r['spread_percent']:.3f}% spread; n={r['n']})" for r in data["summaries"] if r["key"][5] == "aa")
root_table = "\n".join(f"| {r['key'][0]} | {r['first_ir']:,} | {r['n']} |" for r in root_rows)
decomp_table = "\n".join(f"| {r['key'][0]} | {'generator' if r['key'][3] else 'planted'} | {r['median_ir']:,} |" for r in data["cold_summaries"] if r["key"][1:3] == [23, 230] and r["key"][5] == "comparison" and r["key"][0].startswith(("decomp-", "rootfind-")))
report = f"""# Native generalized roots and Boolean F4/F5/F6 comparison

Correction dated2026-10-19: the old `random=true` rows are fixed generator
controls. The original raw flags and numeric results are preserved. See
[the correction](../f4_f5_boolean_runtime_20261019/TARGET_LABEL_CORRECTION.md)
and [the native continuation](../f4_f5_boolean_runtime_20261019/REPORT.md).

The opt-in single-write Gray-table builder reduces complete matrix-F4/F5
instruction work on the linked 22-variable fixture, with identical reduced
rows, rank and word-XOR accounting. These are Callgrind instruction results,
not admitted native runtime speedups. The production default is unchanged.

The missing bounded generalized-rootfinding execution path is now native Rust:
certified prime/binary/odd-extension fields; polynomial, affine-linearized and
partial rational-composition domains; complete enumeration, component MITM and
Boolean Gray dispatch; and a complete S3 separator retaining all original eight
coordinates. Ten independent tests pass, including poles, repeated and
inseparable roots, GF(2^83) extraction, a wide prime certificate, and literal
eight-coordinate separator enumeration. CLI fixtures return exactly two
Boolean roots and eight separator tuples. This completes these implementation
contracts; it does not establish the old hard-family 8x research target.

## Workload and measurement boundary

Baseline main: `1e2bf0715a2a927afc92e6730d6e65b59f15ab9e`.
The primary fixture is exactly `bool_gb/matrix_f4_deg4_m2_n23` and its F5
counterpart: icv1-f2m23-tm5197-1f85e9e1, m=2, divisor index 0, seed 230, 22 Boolean unknowns,
degree 4. Its matrix has 8,449 columns, F4 5,842 rows, F5 5,566 rows, rank 5,289.
F5 prunes 276 rows. F4 performs 90,002,176 word XORs; F5 performs 83,289,643,
including 856 criterion XORs. Neither count changes under the candidate.

Same-binary controls use `KIC_GF2_WRITE_TABLE_ONCE=0/1`. Five A/A calls are
controls, and five alternating A/B pairs provide the primary comparison.
Screening calls are not pooled with confirmation. Seeds 231 and 232 are fresh
matrix holdouts; n17 and n19 are separate attempted scaling controls. The n17
pair completes; both n19 arms return None under the existing matrix limits in
both configurations, leaving four failed comparisons and no speed result.
Every successful matrix pair checks raw and canonical row-space output and
accounting. Root fixtures compare sorted complete sets against independent
Gray enumeration. Decomposition witnesses are re-added in the group and all
completed algorithms must agree on the verdict.

Host: Intel Xeon 6975P-C, x86-64, 32 logical CPUs, 132,915,908,608 bytes RAM,
Linux 6.18.54; rustc 1.99.1. Callgrind 3.19 uses merged worker threads and
explicit per-call regions. Its simulated ISA exposes AVX2, not the host's
native AVX-512. The local runtime preflight refused 32.30 unrelated CPU
seconds per second and CPU PSI avg10 78.26. Profiled elapsed times are never
used as native performance evidence. Clean-CI runtime qualification is a
separate run, analyzed by the native `runtime.rs` with A/A and the exact
3,125-resample paired bootstrap; it is not presumed successful here.

## Complete matrix instruction work

![Complete matrix instructions](matrix-instructions.svg)

| engine | field degree | seed | reference Ir | candidate Ir | reduction | samples A/B |
|:--|--:|--:|--:|--:|--:|:--|
{matrix_table}

A/A controls, retained separately: {aa_text}. Matrix error bars show the
recorded minimum/maximum of each arm's five confirmation samples.

The profile identified redundant zero initialization in the shared elimination
table. The candidate initializes only the zero entry and each subsequently
built entry, omitting unaddressable trailing entries of a partial final table.
The byte-level work changes; the mathematical rows and counted XORs do not.
The reused-storage test poisons buffers and independently checks every subset
XOR across partial groups, suffix widths and table sizes. The option remains
off until native runtime and regression evidence justify changing policy.

## Complete root sets

![All roots](complete-roots.svg)

| solver, n23 planted fixture | first complete-call Ir | retained samples |
|:--|--:|--:|
{root_table}

The general API includes field/domain certification, input normalization,
backend dispatch and its complete result. Direct Gray search is cheaper;
the new solver dispatches to that existing backend for Boolean quadratics.
This is integration and completion, not a novel exhaustive-search algorithm.
Four root fixtures cover one planted and one unplanted generator control at each n17/n23.
The generic Boolean F4-basis arm hit its 50-second profiling budget on n23's
planted fixture; its cost is unknown, and the failed run is retained. Its
other three completed root sets match. A profile timeout is not a no-root
answer. The n17 solver fixture here is a scaling control, not the old
eight-target `boolean-solver.pdf` workload.

## Same-target decomposition, including reusable setup

![Cold decomposition](cold-decomposition.svg)

| method | target kind, n23 | cold Ir |
|:--|:--|--:|
{decomp_table}

F6 uses exact factor-base geometry inside inherited F4; it has no standalone
matrix-step entry point. Its first verified decomposition is compared at
the decomposition boundary. The generalized arm includes encoding, complete
root solving and lifting. Rootfind-mitm is the actual `ic_framework/rootfind`
backend, using the exact same signed points. Rootfind-table is the existing
Koblitz pair table. Their setup is profiled separately and added once for the
primary cold comparison; online-only query figures are retained in JSON.
At m=2 the indexed F6 arm also ran and agreed; no m=3-only memoryless backend
is presented as an m=2 result. These are stage measurements, not full IC/rho.

## Validation, limits and replay

See [validation](VALIDATION.md), [protocol](PROTOCOL.md), [native summary](summary.json),
[native analyzer](analyze.rs), [runtime qualification](runtime.rs), and
[runner](run-profile.sh). The frozen first driver is in `records/driver-snapshot.rs`;
binary and source hashes, initial profiles, isolation refusal and failed runs
are preserved. The final generic driver and all field/solver sources are
versioned with the study. Raw run records are archive members:

```sh
cd research/f4_f5_boolean_perf_20261018
sha256sum -c runs.tar.xz.sha256
tar -xJf runs.tar.xz
```

Rebuild `analyze.rs` against the repository's serde_json/blake3 dependencies
and pass this study directory; it rechecks outputs and merged-thread profiles.
`summary.json` records relative archive-member paths and provenance hashes.
No Python algorithm, harness, verifier or comparison calculation is used;
this rendering script only lays out native recorded values.

Magma, msolve, FGb, Singular and PolyBoRi are unavailable on this host; their
matched external comparisons have not run. No fastest-implementation claim
is made. Arm64, GPU and native AVX-512 runtime are not measured by this study.
The original hard implicit-domain generalized-rootfinding qualification stays
open in [its problem record](../../problems/generalized-rootfinding-all-roots-8x.md).

| requested requirement | status |
|:--|:--|
| Improve F4/F5 for the linked matrix | Implemented opt-in and instruction-verified; native speed qualification separate |
| Compare F6 | Verified same-target decomposition instructions and witnesses |
| Compare generic and Semaev root solvers | Verified complete root sets and cold/query decomposition costs; one generic GB timeout retained |
| Find and finish missing native generalized solver | Bounded native solver and complete separator implemented; ten independent tests and CLI fixtures pass |
| Old hard-family all-roots 8x research qualification | Unresolved; no new claim or relaxed gate |
| Full library verification and clean runtime | See validation; no unsupported full-suite success is inferred |

The component panel and all leaderboard twins are regenerated from the native
summary. Tables A-C, the IC/rho progress timeline, curve facts and aggregate
historical performance series remain unchanged: this study supplies no new
whole-pipeline ratio, curve identity or exponent. Its own plots and PDFs are
copied to `benchmarks/20261018-f4-f5-boolean-perf/`.
"""
(HERE / "REPORT.md").write_text(report)

def typ_escape(text):
    return text.replace("\\", "\\\\").replace('"', '\\"')

source = '''#set page(paper: "a4", margin: 17mm)
#set text(font: "DejaVu Sans", size: 9pt)
#set heading(numbering: "1.")
= Boolean F4/F5/F6 and native generalized roots
Instruction measurements, 2026-10-18. Native runtime qualification is separate.
'''
source += '\n' + '#text("' + typ_escape(report.split("## Workload")[0].split("\n\n", 1)[1]) + '")\n'
for title, graph, description in [
    ("Complete matrix step", "matrix-instructions.svg", "ICV1 n23 (icv1-f2m23-tm5197-1f85e9e1), 22 Boolean variables, degree 4. Five alternating pairs; exact output and XOR accounting. Callgrind 3.19, merged workers, simulated AVX2. Fresh-seed and scaling records are retained in summary.json."),
    ("Every Boolean root", "complete-roots.svg", "First complete call on the same n23 planted system. All original roots checked equal. The general API includes certification and normalization; direct Gray enumeration remains cheaper. Generic F4-basis profiling timed out on this fixture."),
    ("Cold same-target decomposition", "cold-decomposition.svg", "One planted and one unplanted generator n23 target. Table setup is added once; online query data remain separate. F6 and the generic solver can return different verified witnesses for the same verdict. No IC/rho result follows.")]:
    source += '\n#pagebreak()\n= ' + title + '\n' + description + '\n\n#image("' + graph + '", width: 100%)\n'
    if title == "Complete matrix step":
        source += '\n#table(columns: 4, [Engine], [Reference Ir], [Candidate Ir], [Reduction],\n'
        for r in primary:
            source += f'[{r["case"][0]}], [{r["reference_median_ir"]:,.0f}], [{r["candidate_median_ir"]:,.0f}], [{r["reduction_percent"]:.3f}%],\n'
        source += ')\n'
source += '\n= Reproduction and limits\nSource: REPORT.md, PROTOCOL.md, summary.json, analyze.rs and runtime.rs in research/f4_f5_boolean_perf_20261018. Raw member hashes and every failed attempt are retained. Ten native solver tests and CLI fixtures pass; full-library status is in VALIDATION.md. Magma, msolve, FGb, Singular and PolyBoRi were not available. Native wall, external fastest, hard all-roots 8x and whole-pipeline claims are not established.\n'
(HERE / "REPORT.typ").write_text(source)
typst.compile(str(HERE / "REPORT.typ"), output=str(HERE / "REPORT.pdf"))
destination = HERE.parents[1] / "benchmarks" / "20261018-f4-f5-boolean-perf"
destination.mkdir(parents=True, exist_ok=True)
for name in ["REPORT.pdf", "matrix-instructions.svg", "matrix-instructions.png", "complete-roots.svg", "complete-roots.png", "cold-decomposition.svg", "cold-decomposition.png"]:
    shutil.copyfile(HERE / name, destination / name)
