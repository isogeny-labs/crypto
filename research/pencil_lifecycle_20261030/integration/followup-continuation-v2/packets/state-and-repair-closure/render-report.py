from pathlib import Path
import json,re,hashlib,subprocess
p=Path(__file__).parent
body=(p/'REPORT.md').read_text()
(p/'REPORT.md').write_text(body)
labels=[['Mutable find','state preservation'],['Kummer repair','conditional nonzero'],['Cantor reduction','class derivation'],['Saved accounting','phase conservation']]
svg='<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="140"><rect width="1000" height="140" fill="white"/>'
for i,lines in enumerate(labels):
 x=10+250*i
 svg+=f'<rect x="{x}" y="30" width="225" height="70" fill="#edf5f7" stroke="#17364a"/>'
 for j,t in enumerate(lines):svg+=f'<text x="{x+112}" y="{58+24*j}" text-anchor="middle" font-family="sans-serif" font-size="16">{t.replace(">","&gt;")}</text>'
 if i<3:svg+=f'<path d="M{x+225} 65h25l-6 -4m6 4l-6 4" stroke="#17364a" fill="none"/>'
svg+='</svg>'
(p/'flow.svg').write_text(svg)
def esc(s):
 for a,b in [('\\',r'\textbackslash{}'),('&',r'\&'),('%',r'\%'),('_',r'\_'),('#',r'\#'),('^',r'\textasciicircum{}'),('<',r'$<$'),('>',r'$>$')]:s=s.replace(a,b)
 return s
paragraphs=body.split('\n\n')
tex=r'\documentclass[10pt]{article}\usepackage[margin=.65in]{geometry}\usepackage[T1]{fontenc}\usepackage{graphicx}\usepackage{url}\setlength{\parindent}{0pt}\setlength{\parskip}{7pt}\sloppy\begin{document}'
tex+=esc(paragraphs[0][2:])+ '\n\n'+esc(paragraphs[1])+r'\begin{center}\small\fbox{\shortstack{Source-derived value proofs\\scoped assumptions and controls}} $\longrightarrow$ \fbox{\shortstack{Recorded provenance\\required fields and hash chain}}\\[8pt] $\Downarrow$\\[8pt] \fbox{\shortstack{Inspected review artifacts\\remaining assumptions explicit}}\end{center}'+'\n\n'+'\n\n'.join(esc(x) for x in paragraphs[2:])+r'\end{document}'
(p/'report.tex').write_text(tex)
subprocess.run(['pdflatex','-interaction=nonstopmode','-halt-on-error','report.tex'],cwd=p,stdout=(p/'render.log').open('w'),check=True)
(p/'manifest.json').write_text(json.dumps({str(q.relative_to(p)):hashlib.sha256(q.read_bytes()).hexdigest() for q in sorted(p.rglob('*')) if q.is_file() and q.name!='manifest.json' and q.suffix not in ['.olean','.png','.aux']},indent=2)+'\n')
