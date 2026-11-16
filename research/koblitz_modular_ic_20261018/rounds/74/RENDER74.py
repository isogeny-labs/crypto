"""Render the exact six-factor partition from the accepted R74 receipt."""
from pathlib import Path
import json
import typst

h = Path(__file__).resolve().parent
rs = json.loads((h / "results.json").read_text())["runs"]
assert len(rs) == 2 and [x["status"] for x in rs] == ["rejected", "accepted"]
parse = lambda s: dict(x.split("=", 1) for x in s.split()[1:])
factors = [parse(s) for s in rs[1]["factor_lines"]]
assert [(x["absolute_degree"], x["local_length"], x["ramification"]) for x in factors] == [
    ("10", "2", "1"), ("70", "14", "1"), ("5", "1", "1"),
    ("5", "7", "7"), ("5", "7", "7"), ("5", "1", "1")]
data = {"source": "results.json, degree-five-local74-02 exact factor lines", "factors": factors}
(h / "plot-data74.json").write_text(json.dumps(data, indent=2) + "\n")
svg = '''<svg xmlns="http://www.w3.org/2000/svg" width="980" height="300" viewBox="0 0 980 300">
<rect width="980" height="300" fill="white"/>
<text x="20" y="33" font-family="sans-serif" font-size="21" fill="#17384c">Full degree-five fiber: six primitive local factors</text>
<text x="20" y="68" font-family="sans-serif" font-size="17" fill="#17384c">Local lengths over F₃₂, total 32</text>
<g stroke="white" stroke-width="3"><rect x="20" y="90" width="50" height="85" fill="#68a4bb"/><rect x="70" y="90" width="350" height="85" fill="#347caa"/><rect x="420" y="90" width="25" height="85" fill="#89bbc7"/><rect x="445" y="90" width="175" height="85" fill="#205984"/><rect x="620" y="90" width="175" height="85" fill="#205984"/><rect x="795" y="90" width="25" height="85" fill="#89bbc7"/></g>
<g font-family="sans-serif" text-anchor="middle" fill="white"><text x="45" y="139" font-size="16">2</text><text x="245" y="139" font-size="22">14</text><text x="532" y="139" font-size="19">7</text><text x="707" y="139" font-size="19">7</text></g>
<g font-family="sans-serif" fill="#17384c"><text x="20" y="215" font-size="17">Absolute residue degrees: 10, 70, 5, 5, 5, 5</text><text x="20" y="246" font-size="17">Ramification indices: 1, 1, 1, 7, 7, 1</text><text x="20" y="276" font-size="15" fill="#435569">The two index-seven branches contribute the full discriminant exponent 12.</text></g>
</svg>'''
(h / "local74.svg").write_text(svg + "\n")
report = '''#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= R74: all six degree-five local factors
The complete $f_C$ fiber of the normal rank-32 order is split by absolute Frobenius-fixed idempotents. A six-dimensional fixed basis generates all 63 nonzero binary combinations. Each passes direct squaring and a full 528-product multiplication-operator identity. The complete Boolean order test identifies six primitive idempotents, checks pairwise orthogonality and reconstructs the source unit.

#image("local74.svg", width: 100%)

Each local operator rank gives the full fiber length; the projected relative Frobenius rank gives the relative residue degree. Their ratios yield indices $1,1,1,7,7,1$. The local lengths $2,14,1,7,7,1$ sum to 32, and the residue-degree multiset matches R73. The absolute degree-70 branch is unramified. The two index-seven degree-five branches contribute $6+6=12$, exactly R65's $f_C$ discriminant exponent. R72 found another tame index-seven pair above $z=1$, a concrete pattern to test for an intermediate degree-seven field.

The first native attempt is rejected despite exit 0 and a late marker because Singular rejected an `intmat` increment. Fresh version two passes exit, exact marker, zero-error gate, full idempotent tests and all rank/different checks. Source and raw attempt hashes are retained. Uniformizers, coordinate and denominator valuations, and infinity remain before pole divisors and the level-77 map search.
'''
(h / "report.typ").write_text(report)
typst.compile(str(h / "report.typ"), output=str(h / "report.pdf"))
print(json.dumps(data))
