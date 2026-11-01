from pathlib import Path
import json
p=Path(__file__).parent
def esc(s):
 return ''.join({'\\':r'\textbackslash{}','_':r'\_','%':r'\%','&':r'\&','#':r'\#','{':r'\{','}':r'\}','^':r'\textasciicircum{}'}.get(c,c) for c in s)
parts=(p/'REPORT.md').read_text().split('\n\n')
t=[r'\documentclass[11pt]{article}',r'\usepackage[margin=0.85in]{geometry}',r'\usepackage[T1]{fontenc}',r'\usepackage{hyperref}',r'\begin{document}',r'\sloppy',r'\section*{Arbitrary-weight C-zero output bridge}']
for x in parts[1:]:t += [esc(x.strip())+r'\par\medskip']
t += [r'\paragraph{Artifact references} WeightedCZero.lean; PayloadScale.lean; WeightedPoint.lean; WeightedGuards.lean; status.json. The editable diagram is flow.svg. Source assumptions and failures are recorded in REPORT.md.',r'\end{document}'];(p/'report.tex').write_text('\n'.join(t)+'\n')
