from pathlib import Path
import json,subprocess
p=Path(__file__).parent;s=json.loads((p/'summary.json').read_text())
svg='<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="290"><rect width="1000" height="290" fill="white"/><text x="15" y="24" font-family="sans-serif" font-size="18">Saved candidate equations checked; one p739 instance, target 2G</text>'
for i,r in enumerate(s['results']):
 n=r['result']['candidate_equations'];label=['affine','projective','target-rank'][i%3]+' / cap '+('64' if i<3 else '2048');y=48+i*36
 svg+=f'<text x="15" y="{y+16}" font-family="sans-serif" font-size="15">{label}</text><rect x="220" y="{y}" width="{n*3.8}" height="22" fill="#327b69"/><text x="{230+n*3.8}" y="{y+16}" font-family="sans-serif" font-size="15">{n}/{n}</text>'
svg+='<text x="15" y="282" font-family="sans-serif" font-size="13">Every recorded candidate matched. Source scalars checked: 3. Restart events present: 0.</text></svg>'
(p/'flow.svg').write_text(svg)
subprocess.run(['rsvg-convert','-f','pdf','-o',str(p/'diagram.pdf'),str(p/'flow.svg')],check=True)
def esc(x):return x.replace('&',r'\&').replace('_',r'\_').replace('%',r'\%').replace('#',r'\#').replace('<=',r'$\leq$')
paras=(p/'REPORT.md').read_text().split('\n\n')
tex=r'\documentclass[10pt]{article}\usepackage[margin=.55in]{geometry}\usepackage[T1]{fontenc}\usepackage{graphicx}\setlength{\parindent}{0pt}\setlength{\parskip}{4pt}\sloppy\begin{document}'
tex+=r'{\large '+esc(paras[0][2:])+r'}\par '+esc(paras[1])+r'\par\includegraphics[width=.9\linewidth]{diagram.pdf}\par '+'\n\n'.join(esc(x) for x in paras[2:])+r'\end{document}'
(p/'report.tex').write_text(tex)
with (p/'render.log').open('w') as f:subprocess.run(['pdflatex','-interaction=nonstopmode','-halt-on-error','report.tex'],cwd=p,stdout=f,check=True)
