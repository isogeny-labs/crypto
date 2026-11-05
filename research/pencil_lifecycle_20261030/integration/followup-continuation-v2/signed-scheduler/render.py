from pathlib import Path
import json,subprocess
p=Path(__file__).parent;s=json.loads((p/'summary.json').read_text())
svg='<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="290"><rect width="1000" height="290" fill="white"/>'
for i,row in enumerate(s['results']):
 n=row['result']['choices'];label=['affine','projective','target-rank'][i%3]+' / cap '+('64' if i<3 else '2048');y=15+i*40
 svg+=f'<text x="15" y="{y+18}" font-family="sans-serif" font-size="16">{label}</text><rect x="230" y="{y}" width="{n*7}" height="26" fill="#327b69"/><text x="{240+n*7}" y="{y+18}" font-family="sans-serif" font-size="16">{n}/{n}</text>'
svg+='<text x="15" y="280" font-family="sans-serif" font-size="14">Verified choices in saved streams. One p739 instance; no collector rerun or timing observation.</text></svg>'
(p/'flow.svg').write_text(svg)
subprocess.run(['rsvg-convert','-f','pdf','-o',str(p/'diagram.pdf'),str(p/'flow.svg')],check=True)
def esc(x):return x.replace('&',r'\&').replace('_',r'\_').replace('%',r'\%').replace('#',r'\#').replace('<=',r'$\leq$')
paras=(p/'REPORT.md').read_text().split('\n\n')
tex=r'\documentclass[10pt]{article}\usepackage[margin=.55in]{geometry}\usepackage[T1]{fontenc}\usepackage{graphicx}\setlength{\parindent}{0pt}\setlength{\parskip}{4pt}\sloppy\begin{document}'
tex+=r'{\large '+esc(paras[0][2:])+r'}\par '+esc(paras[1])+r'\par\includegraphics[width=.9\linewidth]{diagram.pdf}\par '+'\n\n'.join(esc(x) for x in paras[2:])+r'\end{document}'
(p/'report.tex').write_text(tex)
with (p/'render.log').open('w') as f:subprocess.run(['pdflatex','-interaction=nonstopmode','-halt-on-error','report.tex'],cwd=p,stdout=f,check=True)
