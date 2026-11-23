from pathlib import Path
import csv,html
import typst
p=Path(__file__).resolve().parent
rows=list(csv.DictReader((p/'observations.csv').open()))
s=['<svg xmlns="http://www.w3.org/2000/svg" width="760" height="238"><rect width="760" height="238" fill="white"/><g font-family="sans-serif"><text x="15" y="23" font-size="17">Preserved evidence and outstanding checks</text>']
for i,r in enumerate(rows):
 y=43+i*43
 s.append(f'<rect x="14" y="{y}" width="730" height="35" rx="4" fill="#e6edf4"/><text x="24" y="{y+22}" font-size="13">{html.escape(r["component"])}: {html.escape(r["observed_status"].replace("_"," "))}</text>')
s.append('<text x="15" y="231" font-size="11">2026-11-05 reconciliation; historical outcomes; no new experiment or comparative timing claim.</text></g></svg>')
(p/'status.svg').write_text(''.join(s));typst.compile(str(p/'report.typ'),output=str(p/'report.pdf'))
