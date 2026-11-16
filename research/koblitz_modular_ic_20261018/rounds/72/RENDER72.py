"""Render exact local-length partitions from the accepted R72 receipt."""
from pathlib import Path
import json
import typst

h = Path(__file__).resolve().parent
r = json.loads((h / "results.json").read_text())["runs"]
assert len(r) == 1 and r[0]["status"] == "accepted" and r[0]["error_count"] == 0
parse = lambda s: dict(x.split("=", 1) for x in s.split()[1:])
factors = [parse(x) for x in r[0]["factor_lines"]]
assert [(x["fiber"], x["local_length"], x["ramification"]) for x in factors] == [
    ("0", "28", "14"), ("0", "4", "2"), ("1", "2", "1"), ("1", "14", "7"), ("1", "2", "1"), ("1", "14", "7")]
data = {"source": "results.json, rational-local72-01 exact factor lines", "factors": factors}
(h / "plot-data72.json").write_text(json.dumps(data, indent=2) + "\n")
svg = '''<svg xmlns="http://www.w3.org/2000/svg" width="980" height="300" viewBox="0 0 980 300">
<rect width="980" height="300" fill="white"/>
<text x="20" y="34" font-family="sans-serif" font-size="22" fill="#17384c">Primitive local factors in the full 32-dimensional fibers</text>
<g font-family="sans-serif" font-size="19" fill="#17384c"><text x="22" y="99">z = 0</text><text x="22" y="210">z = 1</text></g>
<g stroke="white" stroke-width="3"><rect x="125" y="66" width="700" height="60" fill="#205984"/><rect x="825" y="66" width="100" height="60" fill="#68a4bb"/>
<rect x="125" y="177" width="50" height="60" fill="#89bbc7"/><rect x="175" y="177" width="350" height="60" fill="#205984"/><rect x="525" y="177" width="50" height="60" fill="#89bbc7"/><rect x="575" y="177" width="350" height="60" fill="#205984"/></g>
<g fill="white" font-family="sans-serif" font-size="17" text-anchor="middle"><text x="475" y="102">e = 14, f = 2; length 28</text><text x="350" y="213">e = 7, f = 2; length 14</text><text x="750" y="213">e = 7, f = 2; length 14</text></g>
<g fill="#17384c" font-family="sans-serif" font-size="14" text-anchor="middle"><text x="875" y="102">e = 2</text><text x="150" y="260">e = 1</text><text x="550" y="260">e = 1</text></g>
<text x="125" y="284" font-family="sans-serif" font-size="14" fill="#435569">Widths are exact local-factor dimensions; each row sums to 32.</text>
</svg>'''
(h / "ramification72.svg").write_text(svg + "\n")
report = '''#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= R72: rational-branch ramification in the normal affine order
The complete 528-product table gives the entire 32-dimensional fiber algebra at each omitted rational prime. All nonzero Frobenius-fixed idempotents are enumerated: three above $z=0$ and 15 above $z=1$. The primitive ones are selected by the complete Boolean order test, checked orthogonal, and checked to sum to the source unit.

#image("ramification72.svg", width: 100%)

At $z=0$, the two residue-degree-two branches have local lengths 28 and 4, hence ramification indices 14 and 2. At $z=1$, four residue-degree-two branches have local lengths 2, 14, 2, 14, hence indices 1, 7, 1, 7. Each local-length partition sums to 32. The passage from local fiber length to $e f$ uses R70 affine normality.

R65's discriminant exponent at $z+1$ is 24. The two index-seven branches are tame in characteristic two, and their different exponents contribute $2(7-1)+2(7-1)=24$, a separate consistency check. The $z=0$ exponent 76 includes wild excess beyond the $e-1$ lower bound.

The sole native attempt passed exit 0, exact marker, and zero-error gates. Six historical source pins and eight donor hashes match. Full square, stable-image, fixed-basis and idempotent matrices are retained. Uniformizers, coordinate and denominator valuations, the degree-five fiber and infinity remain the next geometric calculations.
'''
(h / "report.typ").write_text(report)
typst.compile(str(h / "report.typ"), output=str(h / "report.pdf"))
print(json.dumps(data))
