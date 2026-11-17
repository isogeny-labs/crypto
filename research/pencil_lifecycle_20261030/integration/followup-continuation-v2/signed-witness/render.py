from pathlib import Path
import json,subprocess
p=Path(__file__).parent;s=json.loads((p/'summary.json').read_text())
svg='<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="190"><rect width="1000" height="190" fill="white"/>'
for i,(label,n,color) in enumerate([('Scalar row-combination certificates',s['counts']['determined'],'#327b69'),('Target-varying affine certificates',s['counts']['undetermined'],'#4278a5')]):
 y=25+i*60
 svg+=f'<text x="15" y="{y+20}" font-family="sans-serif" font-size="16">{label}</text><rect x="340" y="{y}" width="{n*14}" height="30" fill="{color}"/><text x="{350+n*14}" y="{y+20}" font-family="sans-serif" font-size="16">{n}</text>'
svg+='<text x="15" y="165" font-family="sans-serif" font-size="14">64 retained software streams; all match native terminal outcomes; no collector rerun.</text></svg>'
(p/'flow.svg').write_text(svg)
subprocess.run(['rsvg-convert','-f','pdf','-o',str(p/'diagram.pdf'),str(p/'flow.svg')],check=True)
def esc(x):return x.replace('&',r'\&').replace('_',r'\_').replace('%',r'\%').replace('#',r'\#').replace('<=',r'$\leq$')
paras=(p/'REPORT.md').read_text().split('\n\n')
tex=r'\documentclass[10pt]{article}\usepackage[margin=.55in]{geometry}\usepackage[T1]{fontenc}\usepackage{graphicx}\setlength{\parindent}{0pt}\setlength{\parskip}{4pt}\sloppy\begin{document}'
tex+=r'{\large '+esc(paras[0][2:])+r'}\par '+esc(paras[1])+r'\par\includegraphics[width=.9\linewidth]{diagram.pdf}\par '+'\n\n'.join(esc(x) for x in paras[2:])+r'\end{document}'
(p/'report.tex').write_text(tex)
with (p/'render.log').open('w') as f:subprocess.run(['pdflatex','-interaction=nonstopmode','-halt-on-error','report.tex'],cwd=p,stdout=f,check=True)
