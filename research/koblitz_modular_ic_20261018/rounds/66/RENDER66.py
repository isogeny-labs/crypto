"""Render the complete second-lattice result from its accepted receipt."""
from pathlib import Path
import json
import typst

h = Path(__file__).resolve().parent
r = json.loads((h / "results.json").read_text())["runs"]
assert len(r) == 1 and r[0]["status"] == "accepted" and r[0]["error_count"] == 0
f = dict(x.split("=", 1) for x in r[0]["result_lines"][0].split()[1:])
assert (f["old_quotient"], f["square_rank"], f["survivor_dimension"], f["new_quotient_length"]) == ("80", "48", "32", "48")
data = {"source": "results.json, second-frobenius66-01 exact terminal line",
        "dimensions": {"V1/O": 80, "square_obstruction_rank": 48,
                       "V2/O": 32, "V1/V2": 48}, "binary_rows": 229}
(h / "plot-data66.json").write_text(json.dumps(data, indent=2) + "\n")
svg = '''<svg xmlns="http://www.w3.org/2000/svg" width="980" height="250" viewBox="0 0 980 250">
<rect width="980" height="250" fill="white"/>
<text x="18" y="27" font-family="sans-serif" font-size="18" fill="#17384c">Second complete descending Frobenius step</text>
<g fill="#edf5fa" stroke="#205984" stroke-width="2"><rect x="20" y="70" width="280" height="130" rx="8"/><rect x="350" y="70" width="280" height="130" rx="8"/><rect x="680" y="70" width="280" height="130" rx="8"/></g>
<g font-family="sans-serif" fill="#17384c" text-anchor="middle"><text x="160" y="112" font-size="20">V₁ / O</text><text x="160" y="147" font-size="30">80</text><text x="160" y="177" font-size="15">all quotient classes</text><text x="490" y="112" font-size="20">Square test</text><text x="490" y="147" font-size="30">rank 48</text><text x="490" y="177" font-size="15">229 binary rows</text><text x="820" y="112" font-size="20">V₂ / O</text><text x="820" y="147" font-size="30">32</text><text x="820" y="177" font-size="15">rank-32 PID lattice</text></g>
<g stroke="#205984" stroke-width="3" fill="none"><path d="M300 135 H345"/><path d="M630 135 H675"/></g>
<g fill="#205984"><path d="M345 135 l-12 -7 v14 z"/><path d="M675 135 l-12 -7 v14 z"/></g>
<text x="20" y="230" font-family="sans-serif" font-size="13" fill="#435568">Exact algebraic dimensions; run second-frobenius66-01. No timing comparison.</text>
</svg>'''
(h / "frobenius66.svg").write_text(svg + "\n")
report = '''#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= R66: second Frobenius lattice step
All 80 classes of $V_1/O$ are squared with the actual degree-32 source
multiplication table. Complete denominator clearing gives 229 binary
remainder rows of rank 48, hence 32 square survivors. Every remainder
is reconstructed, every kernel product vanishes, and an altered kernel
is rejected.

#image("frobenius66.svg", width: 100%)

The full second PID lattice has rank 32. The quotient $V_2/O$ has binary
dimension 32 and $V_1/V_2$ has dimension 48. Both module inclusions,
both field-basis inverse products, old-order inclusion, and all 32 new
basis squares back in $V_1$ pass. An altered inverse is rejected.

== Execution and evidence
One native attempt finished with exit 0, the exact success marker, and
no Singular error lines. Its UTC wall interval is 194 s, a receipt rather
than a controlled performance comparison. Six historical source byte pins
and six donor hashes are checked in results.json. The complete quotient,
binary kernel, and polynomial/field lattice bundles are saved.

This is the second descending step. The next calculation tests all 32
classes of $V_2/O$ and continues until the lattice stabilizes. Actual
proper branches and divisors follow that affine normalization.

The method follows
#link("https://www.math.purdue.edu/~iswanso/intclSinghSwanson.pdf")[Singh–Swanson, Theorem 1.1].
'''
(h / "report.typ").write_text(report)
typst.compile(str(h / "report.typ"), output=str(h / "report.pdf"))
print(json.dumps(data))
