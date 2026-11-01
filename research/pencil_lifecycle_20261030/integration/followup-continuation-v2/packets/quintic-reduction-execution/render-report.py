from pathlib import Path
import subprocess,hashlib,json,html
p=Path(__file__).parent
labels=[['Fallback composition','Exact word quotients','Incoming degree ≤ 4'],['Reduction entry','Monic U / initial prem','Stored invariant'],['Reduction branch','Exact Q / negated prem','Constructed trace'],['Final stored pair','Canonical and valid','9 axiom readbacks']]
svg='<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="120"><rect width="1000" height="120" fill="white"/>'
for i,ls in enumerate(labels):
 x=8+250*i;svg+=f'<rect x="{x}" y="8" width="234" height="100" rx="4" fill="#eef5f8" stroke="#24485a"/>'
 for j,line in enumerate(ls):svg+=f'<text x="{x+117}" y="{34+27*j}" text-anchor="middle" font-family="sans-serif" font-size="16">{html.escape(line)}</text>'
svg+='</svg>';(p/'flow.svg').write_text(svg)
subprocess.run(['rsvg-convert','-f','pdf','-o',str(p/'diagram.pdf'),str(p/'flow.svg')],check=True)
def esc(s):
 return s.replace('&',r'\&').replace('%',r'\%').replace('_',r'\_').replace('#',r'\#').replace('^',r'\textasciicircum{}').replace('<',r'$<$').replace('>',r'$>$')
paras=(p/'REPORT.md').read_text().split('\n\n')
tex=r'\documentclass[10pt]{article}\usepackage[margin=.65in]{geometry}\usepackage[T1]{fontenc}\usepackage{graphicx}\setlength{\parindent}{0pt}\setlength{\parskip}{7pt}\sloppy\begin{document}'
tex+=r'{\large '+esc(paras[0][2:])+r'}\par '+esc(paras[1])+r'\par\includegraphics[width=\linewidth]{diagram.pdf}\par '+'\n\n'.join(esc(s) for s in paras[2:])+r'\end{document}'
(p/'report.tex').write_text(tex)
with (p/'render.log').open('w') as f:subprocess.run(['pdflatex','-interaction=nonstopmode','-halt-on-error','report.tex'],cwd=p,stdout=f,check=True)
