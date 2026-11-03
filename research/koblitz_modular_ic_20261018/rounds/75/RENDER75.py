"""Render all six finite-fiber uniformizer searches from accepted receipts."""
from pathlib import Path
import json
import typst

h = Path(__file__).resolve().parent
rs = json.loads((h / "results.json").read_text())["runs"]
assert len(rs) == 2 and all(x["status"] == "accepted" and x["error_count"] == 0 for x in rs)
parse = lambda s: dict(x.split("=", 1) for x in s.split()[1:])
factors = [parse(s) for r in rs for s in r["factor_lines"]]
ramified = [x for x in factors if x["e"] != "1"]
assert [(x["e"], x["first_basis"], x["successes"]) for x in ramified] == [
    ("14", "5", "9"), ("2", "1", "16"), ("7", "5", "11"),
    ("7", "17", "10"), ("7", "1", "26"), ("7", "1", "25")]
data = {"source": "results.json, two accepted exact native receipts", "ramified": ramified,
        "unramified_count": 6, "candidate_total": 192}
(h / "plot-data75.json").write_text(json.dumps(data, indent=2) + "\n")
labels = ["z: e14", "z: e2", "z+1: e7 A", "z+1: e7 B", "fC: e7 A", "fC: e7 B"]
bars = []
for i, (label, row) in enumerate(zip(labels, ramified)):
    y = 65 + 39 * i
    width = 15 * int(row["successes"])
    bars.append(f'<text x="18" y="{y+20}" font-family="sans-serif" font-size="16" fill="#17384c">{label}</text>'
                f'<rect x="185" y="{y}" width="480" height="25" fill="#edf5fa"/>'
                f'<rect x="185" y="{y}" width="{width}" height="25" fill="#205984"/>'
                f'<text x="680" y="{y+20}" font-family="sans-serif" font-size="16" fill="#17384c">{row["successes"]} / 32; first {row["first_basis"]}</text>')
svg = '<svg xmlns="http://www.w3.org/2000/svg" width="980" height="350" viewBox="0 0 980 350">' + \
      '<rect width="980" height="350" fill="white"/><text x="18" y="32" font-family="sans-serif" font-size="21" fill="#17384c">Complete uniformizer-class search at six ramified branches</text>' + \
      ''.join(bars) + \
      '<text x="18" y="328" font-family="sans-serif" font-size="15" fill="#435569">Every one of 192 candidates has vanishing e-th power; all six branches have index-e candidates.</text></svg>'
(h / "uniformizers75.svg").write_text(svg + "\n")
report = '''#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= R75: uniformizer classes at every finite omitted branch
The R72/R74 primitive idempotents are rebound to the complete specialized 528-product source table. Four rational-fiber and two degree-five ramified branches each receive all 32 projected source-basis candidates $t_j=y_j^q-y_j$: 192 candidates in total. Every candidate lies in its local factor and has vanishing $e$-th power. The chart counts those whose $(e-1)$-st power survives.

#image("uniformizers75.svg", width: 100%)

The saved first candidates use source basis indices 5, 1, 5, 17, 1, 1 for indices 14, 2, 7, 7, 7, 7. In the normal local DVR modulo the base prime, nilpotency index exactly $e$ means valuation one. The six unramified factors have local rank equal to residue degree and use the base prime itself as uniformizer. Both full native programs passed exit 0, exact marker and zero-error gates; 12 historical source byte pins and nine donor hashes match.

These coordinate columns are uniformizer classes in the finite fiber algebras. The next computation lifts them and resolves valuations and leading residues of $b,c,lambda,nu$ and every registered denominator at sufficient prime-power precision. The normal infinity chart is also needed before actual pole divisors and a complete prescribed Riemann–Roch space can be fixed.
'''
(h / "report.typ").write_text(report)
typst.compile(str(h / "report.typ"), output=str(h / "report.pdf"))
print(json.dumps(data))
