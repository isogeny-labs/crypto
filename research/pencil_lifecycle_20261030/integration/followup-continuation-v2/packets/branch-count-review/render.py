from pathlib import Path
import json,subprocess,html
p=Path(__file__).parent
d=json.loads((p/'summary.json').read_text())
labels=[('Source fidelity reviewed',d['source_reviewed_main_declarations'],'#287956'),('Main declarations outside this source review',d['source_unreviewed_main_declarations'],'#9a7225')]
svg='<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="170"><rect width="1000" height="170" fill="white"/><text x="20" y="25" font-family="sans-serif" font-size="18">Source-fidelity coverage: 11 main declarations; T-1358</text>'
for i,(label,n,color) in enumerate(labels):
 y=50+i*55
 svg+=f'<text x="20" y="{y+20}" font-family="sans-serif" font-size="16">{html.escape(label)}</text><rect x="450" y="{y}" width="{n*65}" height="30" fill="{color}"/><text x="{460+n*65}" y="{y+21}" font-family="sans-serif" font-size="18">{n}</text>'
svg+='</svg>'
(p/'coverage.svg').write_text(svg)
subprocess.run(['rsvg-convert','-f','pdf','-o',str(p/'diagram.pdf'),str(p/'coverage.svg')],check=True)
def esc(s):
 for a,b in [('\\',r'\textbackslash{}'),('&',r'\&'),('%',r'\%'),('_',r'\_'),('#',r'\#')]:s=s.replace(a,b)
 return s
paras=(p/'REPORT.md').read_text().split('\n\n')
tex=r'\documentclass[10pt]{article}\usepackage[margin=.7in]{geometry}\usepackage[T1]{fontenc}\usepackage{graphicx}\setlength{\parindent}{0pt}\setlength{\parskip}{7pt}\sloppy\begin{document}'
tex+=r'{\large '+esc(paras[0].lstrip('# '))+r'}\par\includegraphics[width=\linewidth]{diagram.pdf}\par '+'\n\n'.join(esc(x) for x in paras[1:])+r'\end{document}'
(p/'report.tex').write_text(tex)
with (p/'render.stdout').open('w') as f:subprocess.run(['pdflatex','-interaction=nonstopmode','-halt-on-error','report.tex'],cwd=p,stdout=f,check=True)
