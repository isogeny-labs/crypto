"""Render a source-linked diagram and report from the two terminal receipts."""
from pathlib import Path
import json
import typst

h = Path(__file__).resolve().parent
data = json.loads((h / "results.json").read_text())
rows = data["runs"]
assert [r["attempt"] for r in rows] == ["first-frobenius65-01", "lattice65-01"]
assert all(r["status"] == "accepted" and not r["error_count"] for r in rows)
fields = [dict(s.split("=", 1) for s in r["result_lines"][0].split()[1:]) for r in rows]
assert fields[0]["trace_dual_quotient"] == "160"
assert fields[0]["square_rank"] == fields[0]["square_survivor_dimension"] == "80"
assert fields[1]["new_trace_dual_length"] == "80"
plot = {
    "source": "results.json: exact accepted terminal lines",
    "runs": [r["attempt"] for r in rows],
    "dimensions": {"V0/O": 160, "square_obstruction_rank": 80,
                   "V1/O": 80, "V0/V1": 80},
}
(h / "plot-data65.json").write_text(json.dumps(plot, indent=2) + "\n")
svg = '''<svg xmlns="http://www.w3.org/2000/svg" width="980" height="250" viewBox="0 0 980 250">
<rect width="980" height="250" fill="white"/>
<text x="18" y="27" font-family="sans-serif" font-size="18" fill="#17384c">First complete descending Frobenius step</text>
<g fill="#edf5fa" stroke="#205984" stroke-width="2"><rect x="20" y="70" width="280" height="130" rx="8"/><rect x="350" y="70" width="280" height="130" rx="8"/><rect x="680" y="70" width="280" height="130" rx="8"/></g>
<g font-family="sans-serif" fill="#17384c" text-anchor="middle"><text x="160" y="112" font-size="20">V₀ / O</text><text x="160" y="147" font-size="30">160</text><text x="160" y="177" font-size="15">complete F₂ quotient</text><text x="490" y="112" font-size="20">Square test</text><text x="490" y="147" font-size="30">rank 80</text><text x="490" y="177" font-size="15">241 binary rows</text><text x="820" y="112" font-size="20">V₁ / O</text><text x="820" y="147" font-size="30">80</text><text x="820" y="177" font-size="15">full rank-32 PID lattice</text></g>
<g stroke="#205984" stroke-width="3" fill="none"><path d="M300 135 H345"/><path d="M630 135 H675"/></g>
<g fill="#205984"><path d="M345 135 l-12 -7 v14 z"/><path d="M675 135 l-12 -7 v14 z"/></g>
<text x="20" y="230" font-family="sans-serif" font-size="13" fill="#435568">Exact algebraic dimensions; runs first-frobenius65-01 and lattice65-01. No timing comparison.</text>
</svg>'''
(h / "frobenius65.svg").write_text(svg + "\n")
report = '''#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= R65: first Frobenius lattice step
The complete trace-dual quotient $V_0/O$ has binary dimension 160. The first
native run squares every class using the full rank-32 source multiplication
table. Complete denominator remainders produce 241 binary rows of rank 80,
leaving 80 survivors. Every remainder, kernel product, and whole-field
survivor is checked. An altered kernel is rejected.

#image("frobenius65.svg", width: 100%)

The second native run reconstructs $V_1$ as a rank-32 polynomial lattice.
It checks both module inclusions, both basis inverse products, all 32
basis squares back in $V_0$, and an altered-inverse negative control.
The exact binary quotient lengths are 80 for $V_1/O$ and 80 for
$V_0/V_1$. This is one descending step; equality with the
integral closure requires iteration to a stable lattice.

== Execution and evidence
Both attempts finished with native exit 0, exact markers, and zero Singular
error lines. The first and second UTC wall intervals are 76 s and 187 s,
respectively; these are receipts, not controlled performance comparisons.
Twelve historical source/protocol/driver byte pins and the saved quotient,
kernel, survivor, and lattice bundles are recorded in results.json.
The input field bridge is R64, while the Gram and multiplication table are
the retained R61 and R52 sources. The next full test is $V_2={x in V_1:x^2 in V_1}$
on every class of $V_1/O$.

The normalization method follows
#link("https://www.math.purdue.edu/~iswanso/intclSinghSwanson.pdf")[Singh–Swanson, Theorem 1.1].
'''
(h / "report.typ").write_text(report)
typst.compile(str(h / "report.typ"), output=str(h / "report.pdf"))
print(json.dumps({"runs": plot["runs"], "dimensions": plot["dimensions"]}))
