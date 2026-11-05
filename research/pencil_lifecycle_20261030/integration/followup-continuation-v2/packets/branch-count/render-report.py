from pathlib import Path
import json,html,subprocess
p=Path(__file__).parent
data=json.loads((p/'graph-data.json').read_text())
colors=['#27649b','#c66a18','#21865f']
svg='<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="215"><rect width="1000" height="215" fill="white"/><text x="20" y="20" font-family="sans-serif" font-size="16">Exact counter increments per modeled edge; both cases return None</text>'
for i,case in enumerate(data['cases']):
 center=260+i*470
 for j,value in enumerate(case['counts']):
  x=center-110+j*80;y=145-value*75
  svg+=f'<rect x="{x}" y="{y}" width="45" height="{max(2,value*75)}" fill="{colors[j]}"/><text x="{x+22}" y="{y-7}" text-anchor="middle" font-family="sans-serif" font-size="16">{value}</text>'
 svg+=f'<text x="{center-5}" y="170" text-anchor="middle" font-family="sans-serif" font-size="17">{html.escape(case["label"])}</text>'
for j,label in enumerate(data['series']):
 x=90+j*300;svg+=f'<rect x="{x}" y="190" width="14" height="14" fill="{colors[j]}"/><text x="{x+22}" y="203" font-family="sans-serif" font-size="16">{label}</text>'
svg+='</svg>';(p/'flow.svg').write_text(svg)
subprocess.run(['rsvg-convert','-f','pdf','-o',str(p/'diagram.pdf'),str(p/'flow.svg')],check=True)
def esc(s):
 return s.replace('&',r'\&').replace('%',r'\%').replace('_',r'\_').replace('#',r'\#').replace('^',r'\textasciicircum{}').replace('<',r'$<$').replace('>',r'$>$')
paras=(p/'REPORT.md').read_text().split('\n\n')
tex=r'\documentclass[10pt]{article}\usepackage[margin=.65in]{geometry}\usepackage[T1]{fontenc}\usepackage{graphicx}\setlength{\parindent}{0pt}\setlength{\parskip}{7pt}\sloppy\begin{document}'
tex+=r'{\large '+esc(paras[0][2:])+r'}\par '+esc(paras[1])+r'\par\includegraphics[width=\linewidth]{diagram.pdf}\par '+'\n\n'.join(esc(s) for s in paras[2:])+r'\end{document}'
(p/'report.tex').write_text(tex)
with (p/'render.log').open('w') as f:subprocess.run(['pdflatex','-interaction=nonstopmode','-halt-on-error','report.tex'],cwd=p,stdout=f,check=True)
