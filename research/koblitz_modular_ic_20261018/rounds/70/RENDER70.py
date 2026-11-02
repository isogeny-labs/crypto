"""Render the exact six-step normalization chain and the full attempt record."""
from pathlib import Path
import json
import matplotlib.pyplot as plt
import typst

h = Path(__file__).resolve().parent
rounds = h.parent
r70 = json.loads((h / "results.json").read_text())
assert [r["status"] for r in r70["runs"]] == ["rejected", "accepted", "rejected", "accepted", "accepted"]
assert r70["accepted_rank_minor"] == [[1, 0], [1, 1]]
lengths = [160, 80]
for n, expected in [(66, 32), (67, 8), (68, 4), (69, 2)]:
    data = json.loads((rounds / str(n) / "results.json").read_text())
    accepted = [r for r in data["runs"] if r["status"] == "accepted"]
    assert len(accepted) == 1
    fields = dict(x.split("=", 1) for x in accepted[0]["result_lines"][0].split()[1:])
    assert int(fields["survivor_dimension"]) == expected
    lengths.append(expected)
accepted70 = next(r for r in r70["runs"] if r["attempt"] == "sixth-frobenius70-02")
f = dict(x.split("=", 1) for x in accepted70["result_lines"][0].split()[1:])
assert f["square_rank"] == "2" and f["survivor_dimension"] == "0" and f["new_order_quotient"] == "0"
lengths.append(0)
plot = {"source": "accepted exact terminal lines in rounds/65-70/results.json",
        "lattice_labels": [f"V{i}" for i in range(7)],
        "binary_quotient_lengths": lengths,
        "rank_minor": r70["accepted_rank_minor"],
        "attempts": [{"id": r["attempt"], "status": r["status"]} for r in r70["runs"]]}
(h / "plot-data70.json").write_text(json.dumps(plot, indent=2) + "\n")
fig, ax = plt.subplots(figsize=(9.6, 3.25))
ys = [0, 1, 2, 4, 8, 16, 32]
ax.plot(range(7), lengths, color="#205984", marker="o", linewidth=2.5, markersize=6)
for i, y in enumerate(lengths):
    ax.annotate(str(y), (i, y), textcoords="offset points", xytext=(0, 9), ha="center", fontsize=10)
ax.set_yscale("symlog", linthresh=2, base=2)
ax.set_ylim(-0.3, 220)
ax.set_yticks([0, 2, 4, 8, 16, 32, 64, 128])
ax.set_yticklabels(["0", "2", "4", "8", "16", "32", "64", "128"])
ax.set_xticks(range(7), [f"V{i}" for i in range(7)])
ax.set_ylabel("F₂ dimension of Vᵢ / O")
ax.set_xlabel("Complete descending Frobenius step")
ax.grid(axis="y", alpha=0.25)
ax.spines[["top", "right"]].set_visible(False)
fig.tight_layout()
fig.savefig(h / "frobenius70.svg", format="svg")
plt.close(fig)
svg_path = h / "frobenius70.svg"
svg_path.write_text("\n".join(line.rstrip() for line in svg_path.read_text().splitlines()) + "\n")
report = '''#set page(paper: "a4", margin: 17mm)
#set text(size: 10pt)
= R70: affine order reaches the integral closure
The complete descending Frobenius chain on the rank-32 level-77 source
order has binary quotient lengths $160, 80, 32, 8, 4, 2, 0$. The last
two-class square map has 24 binary rows and rank two. Its first two
rows are $(1,0)$ and $(1,1)$, a determinant-one witness that the kernel
is zero. The sixth lattice is checked as the old order $O$ by a full
32-by-32 field-coordinate identity and all 32 old-order basis squares.

#image("frobenius70.svg", width: 100%)

== Why this proves affine normality
The retained rank-32 order has all 528 source multiplication columns
and an actual degree-32 field identification. Its nondegenerate trace
Gram gives a trace-dual over-lattice $V_0$ containing every element
integral over $F_2[z]$. The integral closure is square-closed, so it
lies in each $V_i$ by induction. As $V_6=O$, it lies in $O$; conversely
$O$ lies in its integral closure. The corresponding conditional
six-step implication is kernel-checked in Lean. The native source
identities and trace-dual containment are the stated external premises.

== Full attempt record and boundaries
The first native attempt is rejected despite a late marker: zero-column
Singular errors occur after rank two. Fresh attempt 02 passes exit 0,
the exact marker, zero error lines, and an explicit two-row witness.
The first Lean compile failed on a field instance and set inclusion;
the repaired second run compiled and audited its four authored claims.
The third run added the direct six-step implication and audited 12
declarations with zero failing and zero trusted. All five attempts,
including failures, are bound to historical source bytes in results.json.

This is the affine $F_2[z]$ chart. Proper branches, valuations of
$b,c,lambda,nu$ and denominator functions at the omitted finite fibers
and infinity, the level-77 map, and its order-11 action remain next.
The general descending method is described by
#link("https://www.math.purdue.edu/~iswanso/intclSinghSwanson.pdf")[Singh–Swanson, Theorem 1.1].
'''
(h / "report.typ").write_text(report)
typst.compile(str(h / "report.typ"), output=str(h / "report.pdf"))
print(json.dumps({"quotient_lengths": lengths, "attempts": len(r70["runs"])}))
