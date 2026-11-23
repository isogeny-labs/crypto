from pathlib import Path
import subprocess
p=Path(__file__).parent
labels=[("Reviewed recipe + retained payload", "Existing source review and hash ledger", "#dceaf8"),("Exact private context staged", "19 members / 10,509,133 bytes; readback passed", "#d9eee3"),("Base metadata + admission", "Still required; no Docker execution", "#fff0cd"),("Replacement image + Q0-Q4", "Not executed; final identity remains null", "#f3e2e2")]
svg='<svg xmlns="http://www.w3.org/2000/svg" width="960" height="280"><rect width="960" height="280" fill="white"/>'
for i,(a,b,c) in enumerate(labels):
 y=10+i*65
 svg+=f'<rect x="15" y="{y}" width="930" height="55" rx="6" fill="{c}"/><text x="30" y="{y+23}" font-family="sans-serif" font-size="19">{a}</text><text x="30" y="{y+44}" font-family="sans-serif" font-size="15">{b}</text>'
svg+='</svg>'
(p/'flow.svg').write_text(svg)
subprocess.run(['rsvg-convert','-f','pdf','-o',str(p/'diagram.pdf'),str(p/'flow.svg')],check=True)
def esc(s): return s.replace('&',r'\&').replace('_',r'\_').replace('%',r'\%').replace('#',r'\#')
paras=(p/'REPORT.md').read_text().split('\n\n')
tex=r'\documentclass[10pt]{article}\usepackage[margin=.65in]{geometry}\usepackage[T1]{fontenc}\usepackage{graphicx}\setlength{\parindent}{0pt}\setlength{\parskip}{6pt}\sloppy\begin{document}'
tex+=r'{\large '+esc(paras[0][2:])+r'}\par '+esc(paras[1])+r'\par\includegraphics[width=\linewidth]{diagram.pdf}\par '+'\n\n'.join(esc(x) for x in paras[2:])+r'\end{document}'
(p/'report.tex').write_text(tex)
with (p/'render.log').open('w') as out: subprocess.run(['pdflatex','-interaction=nonstopmode','-halt-on-error','report.tex'],cwd=p,stdout=out,check=True)
