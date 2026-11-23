from pathlib import Path
import html, json, hashlib, re
p=Path(__file__).parent
boxes=[('Native post-find union branch','Choose orientation; read fixed pairs; attach child; transport or compare anchor'),('Checked whole-state invariant','Every vertex equation and every stored anchor survive both orientations'),('Independent controls','Target-offset reversal; signed grandchild; retained anchors; both-fixed answer'),('Zero-constraint boundary','Valid state proves constant0; invalid-state model none does not model panic'),('Composition still separate','Preceding finds; forest/rank/size; native words/arrays; compiled execution')]
s=['<svg xmlns="http://www.w3.org/2000/svg" width="960" height="560" viewBox="0 0 960 560"><rect width="960" height="560" fill="white"/><text x="30" y="32" font-family="sans-serif" font-size="22">Signed-graph union: independent source review</text>']
for i,(title,body) in enumerate(boxes):
 y=55+i*96; color='#e0f2ed' if i<2 else '#fff1d6' if i<4 else '#eeeef4'
 s += [f'<rect x="30" y="{y}" width="900" height="74" rx="8" fill="{color}" stroke="#687385"/><text x="50" y="{y+27}" font-family="sans-serif" font-size="19" font-weight="bold">{html.escape(title)}</text><text x="50" y="{y+54}" font-family="sans-serif" font-size="17">{html.escape(body)}</text>']
 if i<4:s +=[f'<path d="M480 {y+75} v18" stroke="#687385" stroke-width="2"/><path d="M475 {y+87} l5 6 l5 -6" fill="none" stroke="#687385"/>']
s+=['<text x="30" y="550" font-family="sans-serif" font-size="13">pencil-graph-union-review-20261101 | 5 main + 8 producer + 12 reviewer | no science</text></svg>']
(p/'flow.svg').write_text(''.join(s))
def esc(s):
 for a,b in {'²':'^2','≤':'<=','≠':'!=','−':'-','–':'--','∞':'infinity','ℚ':'Q'}.items():s=s.replace(a,b)
 return ''.join({'\\':r'\textbackslash{}','_':r'\_','%':r'\%','&':r'\&','#':r'\#','{':r'\{','}':r'\}','^':r'\textasciicircum{}'}.get(c,c) for c in s)
t=[r'\documentclass[10pt]{article}',r'\usepackage[margin=0.7in]{geometry}',r'\usepackage[T1]{fontenc}',r'\usepackage{graphicx,hyperref}',r'\begin{document}',r'\sloppy',r'\section*{Independent review of signed-graph union mutation}',r'\includegraphics[width=\linewidth]{flow.pdf}']
for x in (p/'REPORT.md').read_text().split('\n\n')[1:]:t += [esc(x.strip())+r'\par\medskip']
t +=[r'\end{document}'];(p/'report.tex').write_text('\n'.join(t))
