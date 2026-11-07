from pathlib import Path
import json,subprocess
p=Path(__file__).parent;s=json.loads((p/'summary.json').read_text())
svg='<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="400"><rect width="1000" height="400" fill="white"/><g font-family="sans-serif" fill="#202030"><text x="20" y="25" font-size="20">First target-information attempt by retained stream</text><text x="20" y="48" font-size="14">64 fixed controls; p739, target 2G; no new collector execution</text>'
for tick in [0,64,128,192,256]:
 y=330-tick;svg+=f'<path d="M70 {y} H950" stroke="#ddd"/><text x="30" y="{y+5}" font-size="12">{tick}</text>'
for i,r in enumerate(s['records']):
 x=80+i*13.4;y=330-(r['first_attempt'] or 256);color='#296d9c' if '-affine' in r['stream'] else '#a45a1d'
 if r['first_attempt'] is None:svg+=f'<path d="M{x-3} {y-3} l6 6 m-6 0 l6 -6" stroke="{color}" stroke-width="2"/>'
 else:svg+=f'<circle cx="{x}" cy="{y}" r="3" fill="{color}"/>'
svg+='<text x="70" y="350" font-size="12">Streams ordered by seed 00 through 31, affine then target-rank</text><text x="70" y="371" font-size="13">Blue: affine; brown: target-rank. Dot: first determined attempt = native terminal attempt.</text><text x="70" y="390" font-size="13">Cross at 256: terminal undetermined (25). 39 dots; delay after first information = 0 in all 39.</text></g></svg>'
(p/'flow.svg').write_text(svg);subprocess.run(['rsvg-convert','-f','pdf','-o',str(p/'diagram.pdf'),str(p/'flow.svg')],check=True)
def esc(x):return x.replace('&',r'\&').replace('_',r'\_').replace('%',r'\%').replace('#',r'\#')
paras=(p/'REPORT.md').read_text().split('\n\n');tex=r'\documentclass[10pt]{article}\usepackage[margin=.55in]{geometry}\usepackage[T1]{fontenc}\usepackage{graphicx}\setlength{\parindent}{0pt}\setlength{\parskip}{4pt}\sloppy\begin{document}'
tex+=r'{\large '+esc(paras[0][2:])+r'}\par '+esc(paras[1])+r'\par\includegraphics[width=.96\linewidth]{diagram.pdf}\par '+'\n\n'.join(esc(x) for x in paras[2:])+r'\end{document}'
(p/'report.tex').write_text(tex)
with (p/'render.log').open('w') as f:subprocess.run(['pdflatex','-interaction=nonstopmode','-halt-on-error','report.tex'],cwd=p,stdout=f,check=True)
