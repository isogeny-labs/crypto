from pathlib import Path
import json,subprocess
p=Path(__file__).parent;s=json.loads((p/'input-check.json').read_text())
svg='<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="240"><rect width="1000" height="240" fill="white"/>'
for i,(label,status,color) in enumerate([('Retained file bindings','63 verified','#327b69'),('Corrected runtime context','19 members verified','#327b69'),('Replacement assembly','Source amendment / review pending','#a87535'),('Replacement qualification','Image / review / admission pending','#a87535')]):
 y=10+i*48
 svg+=f'<rect x="10" y="{y}" width="980" height="38" fill="{color}"/><text x="25" y="{y+25}" font-family="sans-serif" font-size="17" fill="white">{label}: {status}</text>'
svg+='<text x="15" y="230" font-family="sans-serif" font-size="14">Read-only input checks. Counts overlap. No new assembly, qualification or scientific result.</text></svg>'
(p/'flow.svg').write_text(svg)
subprocess.run(['rsvg-convert','-f','pdf','-o',str(p/'diagram.pdf'),str(p/'flow.svg')],check=True)
def esc(x):return x.replace('&',r'\&').replace('_',r'\_').replace('%',r'\%').replace('#',r'\#').replace('<=',r'$\leq$')
paras=(p/'REPORT.md').read_text().split('\n\n')
tex=r'\documentclass[10pt]{article}\usepackage[margin=.55in]{geometry}\usepackage[T1]{fontenc}\usepackage{graphicx}\setlength{\parindent}{0pt}\setlength{\parskip}{4pt}\sloppy\begin{document}'
tex+=r'{\large '+esc(paras[0][2:])+r'}\par '+esc(paras[1])+r'\par\includegraphics[width=.9\linewidth]{diagram.pdf}\par '+'\n\n'.join(esc(x) for x in paras[2:])+r'\end{document}'
(p/'report.tex').write_text(tex)
with (p/'render.log').open('w') as f:subprocess.run(['pdflatex','-interaction=nonstopmode','-halt-on-error','report.tex'],cwd=p,stdout=f,check=True)
