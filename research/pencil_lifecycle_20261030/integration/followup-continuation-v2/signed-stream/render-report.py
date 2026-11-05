from pathlib import Path
import subprocess,hashlib,json,html
p=Path(__file__).parent
rows=json.loads((p/'controls.json').read_text())["rows"]
svg='<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="310"><rect width="1000" height="310" fill="white"/>'
svg+='<text x="15" y="22" font-family="sans-serif" font-size="17">Fixed software fixture: candidate and edge event counts</text>'
svg+='<text x="250" y="46" font-family="sans-serif" font-size="14" fill="#244b88">Blue: candidates</text><text x="440" y="46" font-family="sans-serif" font-size="14" fill="#287a61">Green: inserted edges</text>'
for i,row in enumerate(rows):
 label=('target-rank' if row['rank_policy'] else 'projective' if row['projective'] else 'affine')+' / cap '+str(row['candidate_cap'])
 y=63+i*35
 svg+=f'<text x="12" y="{y+14}" font-family="sans-serif" font-size="15">{label}</text>'
 for j,(key,color) in enumerate([('candidate','#244b88'),('edge','#287a61')]):
  n=row['events'].get(key,0); yy=y+j*13
  svg+=f'<rect x="250" y="{yy}" width="{n*3.5}" height="10" fill="{color}"/><text x="{260+n*3.5}" y="{yy+10}" font-family="sans-serif" font-size="13">{n}</text>'
svg+='<text x="12" y="297" font-family="sans-serif" font-size="13">One target, seed47; six configurations. Counts are deterministic controls, not timing or cost measurements.</text></svg>'
(p/'flow.svg').write_text(svg)
subprocess.run(['rsvg-convert','-f','pdf','-o',str(p/'diagram.pdf'),str(p/'flow.svg')],check=True)
def esc(s):
 return s.replace('&',r'\&').replace('%',r'\%').replace('_',r'\_').replace('#',r'\#').replace('^',r'\textasciicircum{}').replace('<',r'$<$').replace('>',r'$>$')
paras=(p/'REPORT.md').read_text().split('\n\n')
tex=r'\documentclass[10pt]{article}\usepackage[margin=.65in]{geometry}\usepackage[T1]{fontenc}\usepackage{graphicx}\setlength{\parindent}{0pt}\setlength{\parskip}{7pt}\sloppy\begin{document}'
tex+=r'{\large '+esc(paras[0][2:])+r'}\par '+esc(paras[1])+r'\par\includegraphics[width=\linewidth]{diagram.pdf}\par '+'\n\n'.join(esc(s) for s in paras[2:])+r'\end{document}'
(p/'report.tex').write_text(tex)
with (p/'render.log').open('w') as f:subprocess.run(['pdflatex','-interaction=nonstopmode','-halt-on-error','report.tex'],cwd=p,stdout=f,check=True)
