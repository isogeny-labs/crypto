from pathlib import Path
import json,html
import typst
p=Path(__file__).resolve().parent
r=json.loads((p/'review.json').read_text())
s=['<svg xmlns="http://www.w3.org/2000/svg" width="760" height="300"><rect width="760" height="300" fill="white"/><g font-family="sans-serif"><text x="12" y="24" font-size="17">Independent source review: seven recorded findings</text>']
for i,f in enumerate(r['findings']):
 y=42+i*33;s.append(f'<text x="14" y="{y}" font-size="12">{html.escape(f["id"]+": "+f["finding"])}</text><text x="34" y="{y+14}" font-size="10" fill="#335577">{html.escape(f["disposition"])}</text>')
s.append('<text x="14" y="289" font-size="11">Source inspection only. No wrapper, compiler, unit or tests executed. [pcc:unverified]</text></g></svg>')
(p/'findings.svg').write_text(''.join(s));typst.compile(str(p/'report.typ'),output=str(p/'report.pdf'))
