from pathlib import Path
import json,subprocess
p=Path(__file__).parent;s=json.loads((p/'summary.json').read_text())
svg='<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="265"><rect width="1000" height="265" fill="white"/>'
for i,row in enumerate(s['controls']):
 y=8+i*29;rec=row['client_receipt'];code=str(rec['client_terminal_exit']) if rec else 'no child'
 svg+=f'<rect x="10" y="{y}" width="980" height="24" fill="#327b69"/><text x="25" y="{y+18}" font-family="sans-serif" font-size="15" fill="white">{row["case"]}: pass; terminal status {code}</text>'
svg+='<text x="15" y="260" font-family="sans-serif" font-size="13">Fixed local software controls. No sudo, Docker or scientific workload.</text></svg>'
(p/'flow.svg').write_text(svg)
subprocess.run(['rsvg-convert','-f','pdf','-o',str(p/'diagram.pdf'),str(p/'flow.svg')],check=True)
def esc(x):return x.replace('&',r'\&').replace('_',r'\_').replace('%',r'\%').replace('#',r'\#').replace('<=',r'$\leq$')
paras=(p/'REPORT.md').read_text().split('\n\n')
tex=r'\documentclass[10pt]{article}\usepackage[margin=.55in]{geometry}\usepackage[T1]{fontenc}\usepackage{graphicx}\setlength{\parindent}{0pt}\setlength{\parskip}{4pt}\sloppy\begin{document}'
tex+=r'{\large '+esc(paras[0][2:])+r'}\par '+esc(paras[1])+r'\par\includegraphics[width=.9\linewidth]{diagram.pdf}\par '+'\n\n'.join(esc(x) for x in paras[2:])+r'\end{document}'
(p/'report.tex').write_text(tex)
with (p/'render.log').open('w') as f:subprocess.run(['pdflatex','-interaction=nonstopmode','-halt-on-error','report.tex'],cwd=p,stdout=f,check=True)
