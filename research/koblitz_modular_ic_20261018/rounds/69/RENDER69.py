"""Render the complete second-lattice result from its accepted receipt."""
from pathlib import Path
import json
import typst

h = Path(__file__).resolve().parent
r = json.loads((h / "results.json").read_text())["runs"]
assert len(r) == 1 and r[0]["status"] == "accepted" and r[0]["error_count"] == 0
f = dict(x.split("=", 1) for x in r[0]["result_lines"][0].split()[1:])
assert (f["old_quotient"], f["square_rank"], f["survivor_dimension"], f["new_quotient_length"]) == ("4", "2", "2", "2")
data = {"source": "results.json, fifth-frobenius69-01 exact terminal line",
        "dimensions": {"V4/O": 4, "square_obstruction_rank": 2,
                       "V5/O": 2, "V4/V5": 2}, "binary_rows": 25}
(h / "plot-data69.json").write_text(json.dumps(data, indent=2) + "\n")
svg = '''<svg xmlns="http://www.w3.org/2000/svg" width="980" height="250" viewBox="0 0 980 250">
<rect width="980" height="250" fill="white"/>
<text x="18" y="27" font-family="sans-serif" font-size="18" fill="#17384c">Fifth complete descending Frobenius step</text>
<g fill="#edf5fa" stroke="#205984" stroke-width="2"><rect x="20" y="70" width="280" height="130" rx="8"/><rect x="350" y="70" width="280" height="130" rx="8"/><rect x="690" y="70" width="280" height="130" rx="8"/></g>
<g font-family="sans-serif" fill="#17384c" text-anchor="middle"><text x="160" y="112" font-size="20">V₄ / O</text><text x="160" y="147" font-size="30">4</text><text x="160" y="177" font-size="15">all quotient classes</text><text x="490" y="112" font-size="20">Square test</text><text x="490" y="147" font-size="30">rank 2</text><text x="490" y="177" font-size="15">25 binary rows</text><text x="820" y="112" font-size="20">V₅ / O</text><text x="820" y="147" font-size="30">2</text><text x="820" y="177" font-size="15">rank-32 PID lattice</text></g>
<g stroke="#205984" stroke-width="3" fill="none"><path d="M300 135 H345"/><path d="M630 135 H695"/></g>
<g fill="#205984"><path d="M345 135 l-12 -7 v14 z"/><path d="M695 135 l-12 -7 v14 z"/></g>
<text x="20" y="230" font-family="sans-serif" font-size="13" fill="#435569">Exact algebraic dimensions; run fifth-frobenius69-01. No timing comparison.</text>
</svg>'''
(h / "frobenius69.svg").write_text(svg + "\n")
report = '''#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= R69: fifth Frobenius lattice step
All four classes of $V_4/O$ are squared with the actual degree-32 source
multiplication table. Complete denominator clearing gives 25 binary
remainder rows of rank two, hence two square survivors. Every remainder
is reconstructed, every kernel product vanishes, and an altered kernel
is rejected.

#image("frobenius69.svg", width: 100%)

The full fifth PID lattice has rank 32. The quotient $V_5/O$ has binary
dimension two and $V_4/V_5$ has dimension two. Both module inclusions,
both field-basis inverse products, old-order inclusion, and all 32 new
basis squares back in $V_4$ pass. An altered inverse is rejected.

== Execution and evidence
One native attempt finished with exit 0, the exact success marker, and
no Singular error lines. Its UTC wall interval is 171 s, a receipt rather
than a controlled performance comparison. Six historical source byte pins
and six donor hashes are checked in results.json. The complete quotient,
binary kernel, and polynomial/field lattice bundles are saved.

This is the fifth descending step. The next calculation tests both
classes of $V_5/O$ and continues until the lattice stabilizes. Actual
proper branches and divisors follow that affine normalization.

The method follows
#link("https://www.math.purdue.edu/~iswanso/intclSinghSwanson.pdf")[Singh–Swanson, Theorem 1.1].
'''
(h / "report.typ").write_text(report)
typst.compile(str(h / "report.typ"), output=str(h / "report.pdf"))
print(json.dumps(data))
