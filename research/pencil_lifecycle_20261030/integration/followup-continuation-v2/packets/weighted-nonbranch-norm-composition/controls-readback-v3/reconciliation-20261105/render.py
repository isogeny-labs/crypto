from pathlib import Path
import csv,html
import typst
p=Path(__file__).resolve().parent
rows=list(csv.DictReader((p/'phases.csv').open()))
s=['<svg xmlns="http://www.w3.org/2000/svg" width="720" height="240" viewBox="0 0 720 240">','<rect width="720" height="240" fill="white"/>','<g font-family="sans-serif" fill="#172b4d"><text x="18" y="28" font-size="18">Phase 7 recorded outer wall time (seconds)</text>']
for i,r in enumerate(rows):
 y=56+i*48;v=float(r['seconds']);s += [f'<text x="18" y="{y+17}" font-size="14">{html.escape(r["phase"])}</text>',f'<rect x="110" y="{y}" width="{v*3.3}" height="25" fill="#377eb8"/>',f'<text x="{120+v*3.3}" y="{y+18}" font-size="13">{v:.3f} s; exit {r["outer_exit"]}</text>']
s+=['<text x="18" y="222" font-size="12">T1538-phase7; one sequence; no uncertainty recorded; not a performance comparison.</text></g></svg>'];(p/'phases.svg').write_text(''.join(s))
typst.compile(str(p/'report.typ'),output=str(p/'report.pdf'))
