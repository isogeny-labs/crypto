from pathlib import Path
import base64,json,subprocess
p=Path(__file__).parent
svg='<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="220"><rect width="1000" height="220" fill="white"/>'
for i,(label,color) in enumerate([('New source: context verification returns', '#4278a5'), ('Check monotonic / UTC deadline and pending signal', '#327b69'), ('Reject expiry or interruption before accepting success', '#327b69'), ('18 mocked tests pass; separate independent v2 review pending', '#a87535')]):
 y=8+i*45
 svg+=f'<rect x="10" y="{y}" width="980" height="35" fill="{color}"/><text x="25" y="{y+24}" font-family="sans-serif" font-size="17" fill="white">{label}</text>'
svg+='<text x="15" y="211" font-family="sans-serif" font-size="14">T-1407: source/software checks only; whole-process and daemon containment remain unresolved.</text></svg>'
(p/'flow.svg').write_text(svg)
subprocess.run(['rsvg-convert','-f','pdf','-o',str(p/'diagram.pdf'),str(p/'flow.svg')],check=True)
def esc(x):return x.replace('&',r'\&').replace('_',r'\_').replace('%',r'\%').replace('#',r'\#').replace('<=',r'$\leq$')
paras=(p/'REPORT.md').read_text().split('\n\n')
tex=r'\documentclass[10pt]{article}\usepackage[margin=.55in]{geometry}\usepackage[T1]{fontenc}\usepackage{graphicx}\setlength{\parindent}{0pt}\setlength{\parskip}{4pt}\sloppy\begin{document}'
tex+=r'{\large '+esc(paras[0][2:])+r'}\par '+esc(paras[1])+r'\par\includegraphics[width=.9\linewidth]{diagram.pdf}\par '+'\n\n'.join(esc(x) for x in paras[2:])+r'\end{document}'
(p/'report.tex').write_text(tex)
with (p/'render.log').open('w') as f:subprocess.run(['pdflatex','-interaction=nonstopmode','-halt-on-error','report.tex'],cwd=p,stdout=f,check=True)

for name in ['render.log','report.log']:
 f=p/name;(p/(name+'.b64')).write_text(base64.b64encode(f.read_bytes()).decode()+'\n');f.unlink()
for name in ['report.aux']:
 (p/name).unlink(missing_ok=True)
