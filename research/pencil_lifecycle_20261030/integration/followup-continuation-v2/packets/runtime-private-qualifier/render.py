from pathlib import Path
import subprocess
P=Path(__file__).resolve().parent
svg='<svg xmlns="http://www.w3.org/2000/svg" width="1100" height="220"><rect width="1100" height="220" fill="white"/>'
rows=[('Fresh qualification service','Private daemon / owned socket / cgroupfs'),('Explicit parent below service','Container PID + start identity + cgroup inode'),('Held PID1 gate (15s)','Exact nonce release only after ancestry checks'),('Preserved Q0-Q4','Strict empty diff / 1 PID / 512 MiB / 1 CPU')]
for i,(left,right) in enumerate(rows):
 y=10+48*i
 svg+=f'<rect x="10" y="{y}" width="1080" height="42" fill="#e8f0f4"/><text x="22" y="{y+28}" font-family="sans-serif" font-size="19">{left}</text><text x="440" y="{y+28}" font-family="sans-serif" font-size="18">{right}</text>'
svg+='<text x="15" y="214" font-family="sans-serif" font-size="14">Source + mocked checks only; native gate and runtime admission remain pending. T-1417.</text></svg>'
(P/'flow.svg').write_text(svg)
subprocess.run(['rsvg-convert','-f','pdf','-o',str(P/'diagram.pdf'),str(P/'flow.svg')],check=True)
paras=(P/'REPORT.md').read_text().split('\n\n')
def esc(s):
 for a,b in [('&',r'\&'),('_',r'\_'),('%',r'\%'),('#',r'\#')]:s=s.replace(a,b)
 return s
tex=r'\documentclass[10pt]{article}\usepackage[margin=.55in]{geometry}\usepackage[T1]{fontenc}\usepackage{graphicx}\setlength{\parindent}{0pt}\setlength{\parskip}{5pt}\sloppy\begin{document}'
tex+=r'{\large '+esc(paras[0][2:])+r'}\par Task T-1417. 3 November 2026.\par\includegraphics[width=\linewidth]{diagram.pdf}\par '
tex+='\n\n'.join(esc(x) for x in paras[1:])+r'\end{document}'
(P/'report.tex').write_text(tex)
result=subprocess.run(['pdflatex','-interaction=nonstopmode','-halt-on-error','report.tex'],cwd=P,capture_output=True)
(P/'render-output.txt').write_bytes(result.stdout+result.stderr);result.check_returncode()
