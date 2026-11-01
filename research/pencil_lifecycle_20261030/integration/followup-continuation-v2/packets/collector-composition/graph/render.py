from pathlib import Path
import json
p=Path(__file__).parent
def esc(s):
 return ''.join({'\\':r'\textbackslash{}','_':r'\_','%':r'\%','&':r'\&','#':r'\#','{':r'\{','}':r'\}','^':r'\textasciicircum{}'}.get(c,c) for c in s)
parts=(p/'REPORT.md').read_text().split('\n\n')
t=[r'\documentclass[11pt]{article}',r'\usepackage[margin=0.85in]{geometry}',r'\usepackage[T1]{fontenc}',r'\usepackage{hyperref}',r'\begin{document}',r'\sloppy',r'\section*{Signed graph potential algebra}']
for x in parts[1:]:t += [esc(x.strip())+r'\par\medskip']
t += [r'\paragraph{Artifact references} GraphAlgebra.lean; GraphControls.lean; GraphReadback.lean; status.json. The editable diagram is flow.svg. Exact source snapshot is graph-native.rs.txt.',r'\end{document}'];(p/'report.tex').write_text('\n'.join(t)+'\n')
nodes=[(40,70,'Node to parent potential','node = s*parent + a + b*k'),(390,70,'Parent to root potential','parent = t*root + c + d*k'),(215,190,'Path compression','(s*t, a+s*c, b+s*d)'),(40,330,'Reduced edge','cu*ru + cv*rv = A+B*k'),(390,330,'Forward union','ru = (-cu*cv)*rv + cu*A + cu*B*k'),(40,470,'Opposite-size union','Reverse offsets: (-s*a, -s*b)'),(390,470,'Fixed-child transport','parent = s*(x-a) + s*(y-b)*k')]
a=['<svg xmlns="http://www.w3.org/2000/svg" width="760" height="610">','<rect width="760" height="610" fill="white"/>','<style>text{font-family:Arial,sans-serif;fill:#18334b} .h{font-size:17px;font-weight:bold}.b{font-size:12px}</style>','<text x="30" y="30" class="h">Signed graph affine potential identities</text>','<defs><marker id="a" markerWidth="8" markerHeight="8" refX="7" refY="3" orient="auto"><path d="M0,0 L0,6 L7,3z" fill="#476884"/></marker></defs>']
for x,y,u,v in [(190,150,300,190),(540,150,430,190),(365,270,190,330),(340,370,390,370),(540,410,190,470),(540,410,540,470)]:a += [f'<path d="M{x},{y} L{u},{v}" stroke="#476884" fill="none" marker-end="url(#a)"/>']
for x,y,h,b in nodes:a += [f'<rect x="{x}" y="{y}" width="300" height="80" rx="7" fill="#eef4f8" stroke="#7890a3"/>',f'<text x="{x+10}" y="{y+28}" class="h">{h}</text>',f'<text x="{x+10}" y="{y+53}" class="b">{b}</text>']
a += ['<text x="30" y="586" class="b">CommRing identities; union/reversal require involutive signs. Native graph interpretation remains separate.</text>','</svg>'];(p/'flow.svg').write_text('\n'.join(a)+'\n')
