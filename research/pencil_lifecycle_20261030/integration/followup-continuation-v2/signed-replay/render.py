from pathlib import Path
import json,subprocess
p=Path(__file__).parent;s=json.loads((p/'summary.json').read_text())
svg='<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="470"><rect width="1000" height="470" fill="white"/><text x="16" y="22" font-family="sans-serif" font-size="18">Recorded edge-prefix ranks: fixed p739, target2G, seed47 software fixtures</text>'
svg+='<text x="16" y="44" font-family="sans-serif" font-size="13">Blue: nuisance rank. Orange dashed: nuisance + target rank. Green marker: target identified.</text>'
for i,r in enumerate(s['results']):
 col=i%3;rr=i//3;x0=35+col*330;y0=80+rr*195
 title=('target-rank' if r['rank'] else 'projective' if r['projective'] else 'affine')+' cap '+str(r['cap'])
 svg+=f'<text x="{x0}" y="{y0}" font-family="sans-serif" font-size="15">{title}</text>'
 data=r['replay']['prefixes'];maximum=max(1,max(d['nuisance_and_target_rank'] for d in data));n=len(data)
 svg+=f'<path d="M {x0} {y0+15} V {y0+145} H {x0+275}" fill="none" stroke="#777"/>'
 for key,color,dash in [('nuisance_rank','#265da8',''),('nuisance_and_target_rank','#c86c12',' stroke-dasharray="5 3"')]:
  pts=' '.join(f"{x0+275*(j+1)/n},{y0+145-125*d[key]/maximum}" for j,d in enumerate(data))
  svg+=f'<polyline points="{pts}" fill="none" stroke="{color}" stroke-width="2"{dash}/>'
 for j,d in enumerate(data):
  if d['target_determined']:svg+=f'<circle cx="{x0+275*(j+1)/n}" cy="{y0+145-125*d["nuisance_and_target_rank"]/maximum}" r="4" fill="#187049"/>'
 svg+=f'<text x="{x0}" y="{y0+164}" font-family="sans-serif" font-size="12">0 to {n} edge rows; rank 0 to {maximum}</text>'
svg+='</svg>';(p/'flow.svg').write_text(svg)
subprocess.run(['rsvg-convert','-f','pdf','-o',str(p/'diagram.pdf'),str(p/'flow.svg')],check=True)
def esc(x):return x.replace('&',r'\&').replace('_',r'\_').replace('%',r'\%').replace('#',r'\#').replace('<=',r'$\leq$')
paras=(p/'REPORT.md').read_text().split('\n\n')
tex=r'\documentclass[10pt]{article}\usepackage[margin=.55in]{geometry}\usepackage[T1]{fontenc}\usepackage{graphicx}\setlength{\parindent}{0pt}\setlength{\parskip}{4pt}\sloppy\begin{document}'
tex+=r'{\large '+esc(paras[0][2:])+r'}\par '+esc(paras[1])+r'\par\includegraphics[width=.9\linewidth]{diagram.pdf}\par '+'\n\n'.join(esc(x) for x in paras[2:])+r'\end{document}'
(p/'report.tex').write_text(tex)
with (p/'render.log').open('w') as f:subprocess.run(['pdflatex','-interaction=nonstopmode','-halt-on-error','report.tex'],cwd=p,stdout=f,check=True)
