from pathlib import Path
import json,subprocess
p=Path(__file__).parent
svg='<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="323"><rect width="1000" height="323" fill="white"/><g font-family="sans-serif"><text x="15" y="24" font-size="19">Private runtime supervisor: software controls, no runtime execution</text>'
for i,(label,passed,errors) in enumerate([('Initial controls',18,0),('Mode-fixture attempt',19,1),('Corrected fixture',20,0),('Final stop-poll controls',21,0)]):
 y=50+i*38;svg+=f'<text x="15" y="{y+17}" font-size="15">{label}</text><rect x="220" y="{y}" width="{passed*28}" height="25" fill="#397d69"/>'
 if errors:svg+=f'<rect x="{220+passed*28}" y="{y}" width="{errors*28}" height="25" fill="#c57445"/>'
 svg+=f'<text x="{230+(passed+errors)*28}" y="{y+17}" font-size="14">{passed} pass; {errors} error</text>'
svg+='<text x="15" y="217" font-size="13">Recorded software counts. Intermediate failure preserved; independent final replay: 21 passed.</text><rect x="15" y="236" width="280" height="48" fill="#e6edf4"/><text x="25" y="256" font-size="14">Proposed assembly service: 600s</text><text x="25" y="276" font-size="12">Private 4 GiB disk; one load/build</text><path d="M300 261H350" stroke="#333"/><rect x="355" y="236" width="270" height="48" fill="#e6edf4"/><text x="365" y="256" font-size="14">Stopped/empty + disk custody</text><text x="365" y="276" font-size="12">Uncertainty blocks continuation</text><path d="M630 261H680" stroke="#333"/><rect x="685" y="236" width="300" height="48" fill="#e6edf4"/><text x="695" y="256" font-size="14">Proposed qualification service: 300s</text><text x="695" y="276" font-size="12">New owner/endpoint; separate admission</text><text x="15" y="310" font-size="13">Diagram shows source design only. Live systemd/cgroup/mount/Docker enforcement remains unverified.</text></g></svg>'
(p/'flow.svg').write_text(svg);subprocess.run(['rsvg-convert','-f','pdf','-o',str(p/'diagram.pdf'),str(p/'flow.svg')],check=True)
def esc(x):return x.replace('&',r'\&').replace('_',r'\_').replace('%',r'\%').replace('#',r'\#')
paras=(p/'REPORT.md').read_text().split('\n\n');tex=r'\documentclass[10pt]{article}\usepackage[margin=.55in]{geometry}\usepackage[T1]{fontenc}\usepackage{graphicx}\setlength{\parindent}{0pt}\setlength{\parskip}{4pt}\sloppy\begin{document}'
tex+=r'{\large '+esc(paras[0][2:])+r'}\par '+esc(paras[1])+r'\par\includegraphics[width=.96\linewidth]{diagram.pdf}\par '+'\n\n'.join(esc(x) for x in paras[2:])+r'\end{document}'
(p/'report.tex').write_text(tex)
with (p/'render.log').open('w') as f:subprocess.run(['pdflatex','-interaction=nonstopmode','-halt-on-error','report.tex'],cwd=p,stdout=f,check=True)
