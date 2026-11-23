"""Render the exact two-fiber result from the accepted terminal receipt."""
from pathlib import Path
import json
import typst

h = Path(__file__).resolve().parent
r = json.loads((h / "results.json").read_text())["runs"]
assert len(r) == 1 and r[0]["status"] == "accepted" and r[0]["error_count"] == 0
parse = lambda s: dict(x.split("=", 1) for x in s.split()[1:])
rows = [parse(x) for x in r[0]["result_lines"]]
assert [(x["fiber"], x["reduced_dimension"], x["branches"]) for x in rows] == [("0", "4", "2"), ("1", "8", "4")]
data = {"source": "results.json, rational-fibers71-01 exact terminal lines", "fibers": rows,
        "degree_profiles": [parse(x) for x in r[0]["degree_lines"]]}
(h / "plot-data71.json").write_text(json.dumps(data, indent=2) + "\n")
svg = '''<svg xmlns="http://www.w3.org/2000/svg" width="980" height="300" viewBox="0 0 980 300">
<rect width="980" height="300" fill="white"/>
<text x="30" y="36" font-family="sans-serif" font-size="22" fill="#17384c">Normal order: two complete rational fibers</text>
<g fill="#edf5fa" stroke="#205984" stroke-width="2"><rect x="30" y="70" width="430" height="170" rx="10"/><rect x="520" y="70" width="430" height="170" rx="10"/></g>
<g fill="#17384c" font-family="sans-serif" text-anchor="middle">
<text x="245" y="105" font-size="21">z = 0</text><text x="245" y="149" font-size="31">F₄ × F₄</text><text x="245" y="186" font-size="17">2 branches, degree 2 each</text><text x="245" y="215" font-size="15">fiber length 32; reduced dimension 4</text>
<text x="735" y="105" font-size="21">z = 1</text><text x="735" y="149" font-size="31">F₄⁴</text><text x="735" y="186" font-size="17">4 branches, degree 2 each</text><text x="735" y="215" font-size="15">fiber length 32; reduced dimension 8</text></g>
<text x="30" y="274" font-family="sans-serif" font-size="15" fill="#435569">Full specialized Frobenius matrices; separate local lengths and ramification are next.</text>
</svg>'''
(h / "fibers71.svg").write_text(svg + "\n")
report = '''#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= R71: rational omitted fibers in the normal affine order
The full rank-32 source multiplication table is specialized at both rational omitted primes. At each point, all 32 basis-square columns define Frobenius on the entire 32-dimensional fiber algebra. Iterating it removes the nilradical. The stable image has dimension four at $z=0$ and eight at $z=1$.

#image("fibers71.svg", width: 100%)

== Exact checks and interpretation
The $z=0$ image stabilizes after five squares and is $bb(F)_4 times bb(F)_4$. The $z=1$ image stabilizes after four squares and is $bb(F)_4^4$. The program computes fixed dimensions for all 32 Frobenius powers, inverts the gcd-degree equations over the integers, verifies nonnegative degree counts, and reconstructs every fixed dimension. The source unit squares to itself in both fibers; Frobenius preserves each stable image.

One native run passed exit 0, the exact terminal marker, and zero Singular error lines. Six historical executed-source byte pins and five donor hashes match. The four-second UTC interval is only a run receipt.

The reduced algebra determines two residue-degree-two branches over $z=0$ and four over $z=1$. The unreduced fiber dimension is 32 at each prime. Primitive idempotents and their full local-factor lengths are required to separate the ramification indices. Local uniformizers and valuations of the source coordinates and registered denominators, plus the degree-five and infinity fibers, remain next.
'''
(h / "report.typ").write_text(report)
typst.compile(str(h / "report.typ"), output=str(h / "report.pdf"))
print(json.dumps(data))
