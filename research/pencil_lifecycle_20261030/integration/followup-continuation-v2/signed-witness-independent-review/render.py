from pathlib import Path
import subprocess
P=Path(__file__).resolve().parent
review=P.name.endswith('independent-review')
rows=[('Certificate implications','Source reviewed','Direct sums checked'),('Full 64-stream coverage','Gap in old checker','Enforced in wrapper'),('Terminal and summary joins','Gap in old checker','Enforced in wrapper'),('Wrapper authorship','Original non-author review','New code by reviewer')]
svg='<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="240"><rect width="1000" height="240" fill="white"/>'
for i,row in enumerate([('Obligation','Original source review','Checker v2')]+rows):
 y=15+i*44
 svg+=f'<rect x="5" y="{y}" width="990" height="40" fill="'+('#dce7ed' if i==0 else '#f1f5f7')+'"/>'
 for x,text in zip([15,335,665],row):svg+=f'<text x="{x}" y="{y+26}" font-family="sans-serif" font-size="16">{text}</text>'
svg+='</svg>'
(P/'review-matrix.svg').write_text(svg)
subprocess.run(['rsvg-convert','-f','pdf','-o',str(P/'diagram.pdf'),str(P/'review-matrix.svg')],check=True)
source=(P/('REVIEW.md' if review else 'README.md')).read_text()
def esc(s):
 for a,b in [('\\',r'\textbackslash{}'),('&',r'\&'),('%',r'\%'),('_',r'\_'),('#',r'\#')]:s=s.replace(a,b)
 return s.replace('`','')
paras=source.split('\n\n')
tex=r'\documentclass[10pt]{article}\usepackage[margin=.55in]{geometry}\usepackage[T1]{fontenc}\usepackage{graphicx}\setlength{\parindent}{0pt}\setlength{\parskip}{5pt}\sloppy\begin{document}'
tex+=r'{\large '+esc(paras[0][2:])+r'}\par Task T-1408. 2 November 2026.\par\includegraphics[width=\linewidth]{diagram.pdf}\par '
tex+='\n\n'.join(esc(x) for x in paras[1:])+r'\end{document}'
(P/'report.tex').write_text(tex)
result=subprocess.run(['pdflatex','-interaction=nonstopmode','-halt-on-error','report.tex'],cwd=P,capture_output=True)
(P/'render-output.txt').write_bytes(result.stdout+result.stderr)
result.check_returncode()
