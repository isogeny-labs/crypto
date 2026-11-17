"""Render exact recorded bridge evidence; no native field computation."""
from pathlib import Path
import json
import typst

h = Path(__file__).resolve().parent
r = json.loads((h / "results.json").read_text())
rows = r["runs"]
assert [x["status"] for x in rows] == ["rejected"] * 3 + ["accepted"]
accepted = rows[-1]
fields = dict(part.split("=", 1) for part in accepted["result_lines"][0].split()[1:])
assert fields["image_rank"] == fields["actual_degree"] == fields["old_basis_functions"] == "32"
data = {"source": "results.json and accepted bridge64-04 terminal line",
        "attempts": [{"attempt": x["attempt"], "status": x["status"],
                      "native_wall_seconds": x["native_wall_seconds"],
                      "error_count": x["error_count"]} for x in rows],
        "accepted": fields}
(h / "plot-data64.json").write_text(json.dumps(data, indent=2) + "\n")
svg = '''<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="170" viewBox="0 0 1000 170">
<defs><marker id="a" markerWidth="8" markerHeight="8" refX="7" refY="4" orient="auto"><path d="M0,0 L8,4 L0,8" fill="#205984"/></marker></defs>
<g stroke="#205984" stroke-width="2" fill="none" marker-end="url(#a)"><path d="M210,84 H260"/><path d="M465,84 H515"/><path d="M720,84 H770"/></g>
<g fill="#eef5fa" stroke="#205984" stroke-width="1.5"><rect x="10" y="34" width="200" height="100" rx="7"/><rect x="260" y="34" width="205" height="100" rx="7"/><rect x="515" y="34" width="205" height="100" rx="7"/><rect x="770" y="34" width="220" height="100" rx="7"/></g>
<g font-family="sans-serif" font-size="16" fill="#17384c" text-anchor="middle"><text x="110" y="64">R53 compact order</text><text x="110" y="91">32 basis vectors</text><text x="110" y="116">H · lift = D I</text><text x="362" y="64">Coordinate map</text><text x="362" y="91">z = 1/u</text><text x="362" y="116">V = z² v</text><text x="617" y="64">Actual R61 source</text><text x="617" y="91">both equations</text><text x="617" y="116">degree 32</text><text x="880" y="64">Mapped basis</text><text x="880" y="91">rank 32</text><text x="880" y="116">two inverse products</text></g></svg>'''
(h / "bridge64.svg").write_text(svg + "\n")
table = "\n".join(f'[{x["attempt"].replace("-", "‑")}], [{x["status"]}], [{int(x["native_wall_seconds"])}], [{x["error_count"]}],' for x in rows)
report = '''#set page(paper: "a4", margin: 17mm)
#set text(size: 10pt)
#set heading(numbering: none)
= R64: compact-order bridge to the actual field
The retained rank-32 compact PID order is now bound to the actual R61
degree-32 source under $z=1/u$, $V=z²v=v/u²$, and old $lambda=l$.
Both old source equations reduce to zero in the actual quotient. The
source matrix identity $H M = D I$, where M is the saved source lift, holds before and after
coefficient inversion. All 32 compact basis functions map to nonzero
actual functions; their complete coefficient matrix has rank 32 and
a checked two-sided inverse. Altered quadratic and matrix entries fail.

#image("bridge64.svg", width: 100%)

== Full attempt record
#table(columns: (2fr,1fr,1.2fr,1fr), inset: 4pt,
[Attempt], [Status], [Wall seconds], [Error lines],
''' + table + '''
)
The first attempt cast a source module after switching rings. The second
mapped the module before casting, creating a zero fourth image. The third
tested the original identity on that changed representation. All three
are rejected despite late markers. Attempt 04 uses the original R53
matrix construction order and passes the strict gate. Wall intervals
are receipts, not controlled performance comparisons.

== Exact boundary and next step
The accepted run serializes both source equations, all 32 actual basis
functions, the 32 by 32 image matrix and its inverse. Four frozen raw
attempts and 24 historical byte pins are in results.json. The bridge
identifies the generic field represented by the compact order. It does
not certify that the order is integrally closed.

The next exact calculation starts with its trace dual. The retained
Gram discriminant predicts a finite quotient of F2 dimension 160. At
each descending Frobenius step, construct the complete binary square
map on that quotient, solve its full kernel and reconstruct the PID
lattice. Equality certifies affine normalization under the checked
source hypotheses. Then calculate branch valuations on both proper
charts for the level-77 map and subgroup action.

Primary normalization source:
#link("https://www.math.purdue.edu/~iswanso/intclSinghSwanson.pdf")[Singh–Swanson, Theorem 1.1].
'''
(h / "report.typ").write_text(report)
typst.compile(str(h / "report.typ"), output=str(h / "report.pdf"))
print(json.dumps({"attempts": len(rows), "accepted": accepted["attempt"], "rank": fields["image_rank"]}))
