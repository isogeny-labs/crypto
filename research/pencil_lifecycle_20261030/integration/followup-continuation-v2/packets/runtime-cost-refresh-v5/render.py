from pathlib import Path
import csv,subprocess
p=Path(__file__).parent
labels=['USD 200 shared daily cap','USD 131.13 reported spend','USD 58 prior reserve','USD 1 proposed addition','USD 9.87 estimated rest']
svg='<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="130"><rect width="1000" height="130" fill="white"/><g font-family="sans-serif"><text x="15" y="22" font-size="18">Qualification cost refresh</text>'
for i,label in enumerate(labels):
 x=10+i*198;svg+=f'<rect x="{x}" y="43" width="180" height="45" fill="#e2eee9"/><text x="{x+8}" y="69" font-size="11">{label}</text>'
 if i<4:svg+=f'<path d="M{x+180} 65h17" stroke="#333"/>'
svg+='<text x="15" y="115" font-size="13">Prior worker debit 2400 + proposed 600 = 3000; remaining 22200 seconds. No admission.</text></g></svg>'
(p/'graph.svg').write_text(svg);subprocess.run(['rsvg-convert','-f','pdf','-o',str(p/'diagram.pdf'),str(p/'graph.svg')],check=True)
def esc(x):return x.replace('&',r'\&').replace('_',r'\_').replace('%',r'\%').replace('#',r'\#').replace('^',r'\textasciicircum{}').replace('<',r'\textless{}').replace('>',r'\textgreater{}')
paras=(p/'REPORT.md').read_text().split('\n\n');tex=r'\documentclass[10pt]{article}\usepackage[margin=.55in]{geometry}\usepackage[T1]{fontenc}\usepackage{graphicx}\setlength{\parindent}{0pt}\setlength{\parskip}{4pt}\sloppy\begin{document}'
tex+=r'{\large '+esc(paras[0][2:])+r'}\par '+esc(paras[1])+r'\par\includegraphics[width=.96\linewidth]{diagram.pdf}\par '+'\n\n'.join(esc(x) for x in paras[2:])+r'\end{document}'
(p/'report.tex').write_text(tex)
with (p/'render.log').open('w') as f:subprocess.run(['pdflatex','-interaction=nonstopmode','-halt-on-error','report.tex'],cwd=p,stdout=f,check=True)
