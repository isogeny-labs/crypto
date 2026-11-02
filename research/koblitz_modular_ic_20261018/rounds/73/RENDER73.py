"""Render the full degree-five residue profile from accepted receipts."""
from pathlib import Path
import json
import typst

h = Path(__file__).resolve().parent
rs = json.loads((h / "results.json").read_text())["runs"]
assert len(rs) == 5 and [x["status"] for x in rs] == ["rejected", "accepted", "rejected", "rejected", "accepted"]
parse = lambda s: dict(x.split("=", 1) for x in s.split()[1:])
row = parse(rs[1]["result_lines"][0])
degrees = [parse(s) for s in rs[1]["degree_lines"]]
assert (row["fiber_dimension_over_F2"], row["reduced_dimension_over_F32"], row["branches"]) == ("160", "20", "6")
assert [(d["relative_degree"], d["multiplicity"]) for d in degrees] == [("1", "4"), ("2", "1"), ("14", "1")]
data = {"source": "results.json, degree-five73-02 exact terminal lines", "fiber": row,
        "degree_profile": degrees, "lean": {"authored_theorems": 3, "audit": "0 failing, 0 trusted"}}
(h / "plot-data73.json").write_text(json.dumps(data, indent=2) + "\n")
svg = '''<svg xmlns="http://www.w3.org/2000/svg" width="980" height="280" viewBox="0 0 980 280">
<rect width="980" height="280" fill="white"/>
<text x="20" y="31" font-family="sans-serif" font-size="21" fill="#17384c">Complete degree-five fiber over F₃₂</text>
<text x="20" y="68" font-family="sans-serif" font-size="17" fill="#17384c">Reduced residue degrees: 20 of 32 fiber dimensions</text>
<g stroke="white" stroke-width="3"><rect x="20" y="90" width="40" height="75" fill="#68a4bb"/><rect x="60" y="90" width="40" height="75" fill="#68a4bb"/><rect x="100" y="90" width="40" height="75" fill="#68a4bb"/><rect x="140" y="90" width="40" height="75" fill="#68a4bb"/><rect x="180" y="90" width="80" height="75" fill="#347caa"/><rect x="260" y="90" width="560" height="75" fill="#205984"/></g>
<g font-family="sans-serif" text-anchor="middle"><text x="100" y="134" font-size="18" fill="#17384c">4 × 1</text><text x="220" y="134" font-size="18" fill="white">2</text><text x="540" y="134" font-size="24" fill="white">14</text></g>
<text x="20" y="196" font-family="sans-serif" font-size="17" fill="#17384c">Absolute F₂ residue degrees: 5, 5, 5, 5, 10, 70</text>
<text x="20" y="229" font-family="sans-serif" font-size="16" fill="#435569">Discriminant exponent 12 = 32 − 20: all branches are tame;</text>
<text x="20" y="253" font-family="sans-serif" font-size="16" fill="#435569">the degree-14 relative branch is unramified.</text>
</svg>'''
(h / "fiber73.svg").write_text(svg + "\n")
report = '''#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= R73: a degree-70 branch in the normal level-77 source
The complete $f_C=z^5+z^3+z^2+z+1$ fiber is rank 32 over $bb(F)_32$ (rank 160 over $bb(F)_2$). All 32 source square columns define absolute Frobenius; five semilinear iterations give the full $bb(F)_32$-linear relative Frobenius. Scalar linearity, commutation, source unit and stable-image invariance all pass.

#image("fiber73.svg", width: 100%)

The reduced image has dimension 20 over $bb(F)_32$. Fixed dimensions of all 32 powers and integer gcd-degree inversion give four branches of relative degree one, one of degree two and one of degree 14. The last has absolute $bb(F)_2$ residue degree 70. Every fixed dimension is reconstructed from the saved profile.

R65 gives discriminant exponent 12 at $f_C$. The fundamental equality and relative residue sum give $sum f_i(e_i-1)=32-20=12$. Dedekind's different bound $d_i >= e_i-1$, with equality exactly for tame branches, forces all six branches tame. In characteristic two the degree-14 branch must have index one. The Lean theorem checks that last weighted-budget arithmetic under explicit premises; three authored declarations compile and audit with zero failing and zero trusted.

Five terminal attempts are preserved: the first native run had source-ring errors despite a late marker; the repaired second run passes exit, marker and zero-error gates. Two Lean import attempts failed compilation before a final pinned compile/audit passed. The next experiment splits all 63 nonzero absolute idempotents to determine the other local lengths and ramification indices. Uniformizers, valuations and infinity remain separate.

The different inequality is stated in #link("https://research.sabanciuniv.edu/id/eprint/577/1/3011800001170.pdf")[Garcia–Stichtenoth, Section 2].
'''
(h / "report.typ").write_text(report)
typst.compile(str(h / "report.typ"), output=str(h / "report.pdf"))
print(json.dumps(data))
