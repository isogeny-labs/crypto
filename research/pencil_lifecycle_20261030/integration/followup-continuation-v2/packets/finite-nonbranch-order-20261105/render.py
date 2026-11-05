from pathlib import Path
import csv,html,typst
p=Path(__file__).resolve().parent;r=list(csv.DictReader((p/'steps.csv').open()))
s=['<svg xmlns="http://www.w3.org/2000/svg" width="760" height="300"><rect width="760" height="300" fill="white"/><g font-family="sans-serif"><text x="16" y="24" font-size="17">Finite local multiplicity: proposed proof dependencies</text>']
for i,x in enumerate(r):
 y=42+i*46;s.append(f'<rect x="14" y="{y}" width="730" height="38" fill="#edf2f7"/><text x="24" y="{y+15}" font-size="13">{html.escape(x["step"]+": "+x["mechanism"])}</text><text x="24" y="{y+31}" font-size="11">{html.escape(x["status"])}</text>')
s.append('<text x="16" y="289" font-size="11">[pcc:unverified] T-1613; no compiler, axiom readback or experiment execution.</text></g></svg>');(p/'steps.svg').write_text(''.join(s));typst.compile(str(p/'report.typ'),output=str(p/'report.pdf'))
