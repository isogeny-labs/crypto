from pathlib import Path
import csv,subprocess
p=Path(__file__).parent
rows=list(csv.DictReader((p/'socket-paths.csv').open()))[:5]
svg='<svg xmlns="http://www.w3.org/2000/svg" width="900" height="265"><rect width="900" height="265" fill="white"/><g font-family="sans-serif"><text x="15" y="24" font-size="20">Known Unix socket pathname bytes (both phases)</text>'
for i,row in enumerate(rows):
 y=45+i*35;n=int(row['pathname_bytes']);svg+=f'<text x="15" y="{y+17}" font-size="15">{row["kind"]}</text><rect x="190" y="{y}" width="{n*5}" height="24" fill="#397d69"/><text x="{200+n*5}" y="{y+17}" font-size="14">{n}</text>'
svg+='<path d="M725 40V225" stroke="#ad4728" stroke-dasharray="4 3"/><text x="735" y="60" font-size="13">107 bytes</text><text x="735" y="78" font-size="13">+ terminator</text><text x="15" y="250" font-size="13">Source-derived paths; 29 producer controls and 29 independent controls passed. No runtime execution.</text></g></svg>'
(p/'flow.svg').write_text(svg);subprocess.run(['rsvg-convert','-f','pdf','-o',str(p/'diagram.pdf'),str(p/'flow.svg')],check=True)
def esc(x):return x.replace('&',r'\&').replace('_',r'\_').replace('%',r'\%').replace('#',r'\#')
paras=(p/'REPORT.md').read_text().split('\n\n');tex=r'\documentclass[10pt]{article}\usepackage[margin=.55in]{geometry}\usepackage[T1]{fontenc}\usepackage{graphicx}\setlength{\parindent}{0pt}\setlength{\parskip}{4pt}\sloppy\begin{document}'
tex+=r'{\large '+esc(paras[0][2:])+r'}\par '+esc(paras[1])+r'\par\includegraphics[width=.96\linewidth]{diagram.pdf}\par '+'\n\n'.join(esc(x) for x in paras[2:])+r'\end{document}'
(p/'report.tex').write_text(tex)
with (p/'render.log').open('w') as f:subprocess.run(['pdflatex','-interaction=nonstopmode','-halt-on-error','report.tex'],cwd=p,stdout=f,check=True)
