"""Render recorded exact counts only; no new field calculations or trials."""
from pathlib import Path
import json
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import typst

h = Path(__file__).resolve().parent
result = json.loads((h / "results.json").read_text())
runs = {row["attempt"]: row for row in result["runs"]}
assert len(runs) == 5
assert {name for name, row in runs.items() if row["status"] == "accepted"} == {
    "generator112-01", "check112-01", "half-descent63-02", "half-lean63-01"}

def fields(line):
    return dict(part.split("=", 1) for part in line.split()[1:])

gen = next(fields(line) for line in runs["generator112-01"]["result_lines"]
           if line.startswith("GEN112_RESULT "))
replay = next(fields(line) for line in runs["check112-01"]["result_lines"]
              if line.startswith("CHECK112_RESULT "))
linear = {row["family"]: row for row in
          (fields(line) for line in runs["half-descent63-02"]["result_lines"]
           if line.startswith("HALF63_LINEAR "))}
source = {row["family"]: row for row in
          (fields(line) for line in runs["half-descent63-02"]["result_lines"]
           if line.startswith("HALF63_RESULT "))}
data = {"source": "results.json accepted terminal lines",
        "degree112": {"standard_monomials": int(gen["standard_monomials"]),
                      "whole_source_identities": int(gen["whole_source_identities"]),
                      "fresh_replay_identities": int(replay["whole_source_identities"]),
                      "derivative_inverses": int(gen["derivative_inverses"])},
        "half_spaces": [{"family": family,
                         "root_dimension": int(linear[family]["root_dimension"]),
                         "fixed_dimension": int(linear[family]["fixed_dimension"]),
                         "trace_rank": int(linear[family]["trace_rank"]),
                         "actual_source_bindings": int(source[family]["whole_source_root_bindings"]),
                         "source_involution_identities": int(source[family]["source_involution_identities"]),
                         "trace_one": int(source[family]["one_traceable"])}
                        for family in ("mixed", "cubic")],
        "workflow_status": {status: sum(row["status"] == status for row in result["runs"])
                            for status in ("accepted", "rejected")}}
assert sum(x["actual_source_bindings"] for x in data["half_spaces"]) == 138
assert sum(x["source_involution_identities"] for x in data["half_spaces"]) == 276
(h / "plot-data63.json").write_text(json.dumps(data, indent=2) + "\n")

fig, axes = plt.subplots(1, 2, figsize=(8.1, 2.7))
for ax, row, limit in zip(axes, data["half_spaces"], (12, 145)):
    fixed, trace = row["fixed_dimension"], row["trace_rank"]
    ax.bar([0], [fixed], width=.55, color="#205984", label="Fixed")
    ax.bar([0], [trace], bottom=[fixed], width=.55, color="#d38127", label="Trace image")
    ax.text(0, fixed / 2, str(fixed), ha="center", va="center", color="white", fontsize=10)
    ax.text(0, fixed + trace / 2, str(trace), ha="center", va="center", color="white", fontsize=10)
    ax.set_xlim(-.8, .8)
    ax.set_ylim(0, limit)
    ax.set_xticks([0], [f'{row["family"]}: {row["root_dimension"]} roots'])
    ax.set_ylabel("F2 dimension")
    ax.spines[["top", "right"]].set_visible(False)
fig.suptitle("Blue: fixed subspace; orange: trace image", fontsize=11)
fig.tight_layout(rect=(0, 0, 1, .92))
fig.savefig(h / "half-spaces63.svg")
plt.close(fig)

fig, ax = plt.subplots(figsize=(8.1, 2.2))
names = ["Generator", "Fresh replay", "Half-space source", "Lean audit"]
counts = [124, 124, 138, 5]
ax.barh(names[::-1], counts[::-1], color=["#77a8bb", "#d38127", "#477b9e", "#205984"])
for i, count in enumerate(counts[::-1]):
    ax.text(count + 2, i, str(count), va="center", fontsize=9)
ax.set_xlim(0, 160)
ax.set_xlabel("Recorded complete inputs or authored theorems")
ax.set_title("Accepted stage sizes (different counted units)")
fig.tight_layout()
fig.savefig(h / "stages63.svg")
plt.close(fig)

table = []
for row in result["runs"]:
    wall = str(int(row["native_wall_seconds"])) if "native_wall_seconds" in row else "—"
    table.append(f'[{row["attempt"].replace("-", "‑")}], [{row["status"]}], [{wall}],')
report = '''#set page(paper: "a4", margin: 16mm)
#set text(size: 10pt)
#set heading(numbering: none)
= R63: degree 112 source and half-space descent
The actual marked-23 source has degree 112 over F2(x). Its 112 standard
monomials and 124 complete source functions have square pairs built from
two checked generator formulas and derivative inverses. A fresh source
replay binds all 124 saved inputs and roots, both generator bundles,
both inverse products, and altered-inverse/root controls.

#image("stages63.svg", width: 100%)
Each bar counts a different exact unit. The two 124-function checks are
producer and fresh replay of the same cohort, not independent functions.

== Terminal flow
#table(columns: (2fr,1fr,1.2fr), inset: 4pt,
[Attempt], [Status], [Native wall seconds],
''' + "\n".join(table) + '''
)
The first half-space attempt is rejected despite a late marker: an
unexported ambient constant caused Singular question-mark errors. Version 5
reconstructs the constant from retained R61 matrices; attempt 02 passes.
Wall intervals are receipt timing, not a performance comparison.

== Accepted half-space restriction
#image("half-spaces63.svg", width: 100%)
The mixed space has fixed dimension 6 and trace rank 4. The cubic space has
fixed dimension 64 and trace rank 64. Both contain an actual function with
source involution trace one. The 10 mixed roots lie inside the 128 cubic
roots; the two dimensions do not add.

#pagebreak()
= Exact checks and next geometry
The half-space run binds all 138 saved roots to the actual R61 source and
verifies $g^2=f$ for each. It checks 20 mixed and 256 cubic root/square
involution identities, all 70 fixed roots, and both trace-one functions
in the full source quotient. Each restricted matrix has a checked left
inverse, intertwines the ambient action, squares to the identity, and
rejects an altered action.

The pinned Lean 4.34.1/mathlib d13f23b audit compiles five authored
conditional implications: square-root uniqueness, fixedness of a root
over a fixed square, integer pole-cap halving, restriction of an
involution, and a trace separator. The audit reports zero failing and
zero trusted native decisions. The actual source equations are checked
by the native field calculations, and proper pole valuations still need
their own construction.

== New normalization route
The retained rank-32 trace Gram discriminant predicts a quotient of
F2 dimension 160 between the trace-dual lattice and the affine order.
The Singh–Swanson descending Frobenius algorithm turns each normalization
step into a complete binary kernel on that finite quotient. Equality is
an exact affine stopping certificate after the source-field bridge is
verified. The old PID variable is z=1/u relative to the actual FIELD61
coordinate. The quadratic equations match under V=z²v; the degree-16
equation and all 32 basis functions require explicit reduction. See
R64/NORMALIZATION-LEAD.md for the source-bound algorithm and proof scope.

Proper normalization, branch valuations at zero/one/infinity, complete
Riemann–Roch spaces, the level-77 map and order-11 action, quotient
saturation, arbitrary reverse divisors, node187, and selected-991 factor
action remain separate exact construction tasks.

== Reproduction
Five immutable raw attempts; four accepted and one rejected. The 29
historical source byte pins and every terminal receipt are in results.json.
Singular used seed 63, one thread, 1200 seconds and 4 GiB virtual memory
per run; Lean compile and axiom audit had separate 1200-second caps and
the pinned toolchain. Raw archive custody is recorded separately.

Primary normalization source:
#link("https://www.math.purdue.edu/~iswanso/intclSinghSwanson.pdf")[Singh–Swanson, Theorem 1.1].
'''
(h / "report.typ").write_text(report)
for name in ("half-spaces63.svg", "stages63.svg"):
    path = h / name
    path.write_text("\n".join(line.rstrip() for line in path.read_text().splitlines()) + "\n")
typst.compile(str(h / "report.typ"), output=str(h / "report.pdf"))
print(json.dumps({"attempts": len(result["runs"]), "status": data["workflow_status"],
                  "half_spaces": data["half_spaces"]}))
