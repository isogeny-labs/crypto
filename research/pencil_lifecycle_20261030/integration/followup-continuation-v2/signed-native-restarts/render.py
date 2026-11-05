from pathlib import Path
import json,subprocess
p=Path(__file__).parent;s=json.loads((p/'summary.json').read_text())
svg='<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="230"><rect width="1000" height="230" fill="white"/>'
for i,(label,n,color) in enumerate([('Walk-duplicate restarts',s['restart_counts']['walk_duplicate'],'#327b69'),('Chosen-duplicate restarts',s['restart_counts']['chosen_duplicate'],'#327b69'),('Verified returned scalars',s['verified_source_scalars'],'#4278a5'),('Candidate cap reached',s['capped_controls'],'#a87535')]):
 y=12+i*45
 svg+=f'<text x="15" y="{y+18}" font-family="sans-serif" font-size="16">{label}</text><rect x="280" y="{y}" width="{n*15}" height="26" fill="{color}"/><text x="{290+n*15}" y="{y+18}" font-family="sans-serif" font-size="16">{n}</text>'
svg+='<text x="15" y="215" font-family="sans-serif" font-size="14">64 fixed software controls on one p739 instance; counts are not population frequencies.</text></svg>'
(p/'flow.svg').write_text(svg)
subprocess.run(['rsvg-convert','-f','pdf','-o',str(p/'diagram.pdf'),str(p/'flow.svg')],check=True)
def esc(x):return x.replace('&',r'\&').replace('_',r'\_').replace('%',r'\%').replace('#',r'\#').replace('<=',r'$\leq$')
paras=(p/'REPORT.md').read_text().split('\n\n')
tex=r'\documentclass[10pt]{article}\usepackage[margin=.55in]{geometry}\usepackage[T1]{fontenc}\usepackage{graphicx}\setlength{\parindent}{0pt}\setlength{\parskip}{4pt}\sloppy\begin{document}'
tex+=r'{\large '+esc(paras[0][2:])+r'}\par '+esc(paras[1])+r'\par\includegraphics[width=.9\linewidth]{diagram.pdf}\par '+'\n\n'.join(esc(x) for x in paras[2:])+r'\end{document}'
(p/'report.tex').write_text(tex)
with (p/'render.log').open('w') as f:subprocess.run(['pdflatex','-interaction=nonstopmode','-halt-on-error','report.tex'],cwd=p,stdout=f,check=True)
