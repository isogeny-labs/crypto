from pathlib import Path
import json,html,re,subprocess
p=Path(__file__).parent;m=json.loads((p/'matrix.json').read_text());rows=m['rows']
labels=[('Factor-base sweep','5 arms x 1,260 recorded','Matched counts retained'),('Published law','2 arms x 1,260 recorded','Universal joins separate'),('Pencil collector','C1: 0 paired completions','Full cohort incomplete'),('Kummer collector','N19: 1,260 recorded','Exceptional closure separate')]
svg='<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="420"><rect width="1000" height="420" fill="white"/>'
for i,(title,a,b) in enumerate(labels):
 x=10+250*i;svg+=f'<rect x="{x}" y="12" width="230" height="110" fill="#eef5f8" stroke="#24485a"/>'
 for j,t in enumerate([title,a,b]):svg+=f'<text x="{x+115}" y="{42+j*28}" text-anchor="middle" font-family="sans-serif" font-size="15">{html.escape(t)}</text>'
svg+='<text x="500" y="163" text-anchor="middle" font-family="sans-serif" font-size="19">Five follow-up lanes: source joins and review coverage</text>'
for i,t in enumerate(['Machine correspondence','Collector invariants','Exceptional branches','Kummer closure / signs','Target information / cost']):
 x=10+i*198;svg+=f'<rect x="{x}" y="192" width="187" height="80" fill="#fff7e8" stroke="#8b6834"/><text x="{x+93}" y="222" text-anchor="middle" font-family="sans-serif" font-size="13">{html.escape(t)}</text><text x="{x+93}" y="249" text-anchor="middle" font-family="sans-serif" font-size="13">Component evidence retained</text>'
svg+='<rect x="120" y="310" width="760" height="80" fill="#f5eeee" stroke="#855"/><text x="500" y="340" text-anchor="middle" font-family="sans-serif" font-size="17">New scientific runs: renewed limits + bindings + qualification + review</text><text x="500" y="368" text-anchor="middle" font-family="sans-serif" font-size="15">Coverage audit only; no raw recomputation or new performance comparison</text></svg>'
(p/'coverage.svg').write_text(svg);subprocess.run(['rsvg-convert','-f','pdf','-o',str(p/'diagram.pdf'),str(p/'coverage.svg')],check=True)
def esc(s):
 out=''
 for c in s:
  out+= {'&':r'\&','%':r'\%','_':r'\_','#':r'\#','^':r'\textasciicircum{}','<':r'$<$','>':r'$>$','≥':r'$\geq$','≤':r'$\leq$','–':'--','`':'','~':r'\textasciitilde{}'}.get(c,c)
 return out
tex=r'\documentclass[10pt]{article}\usepackage[margin=.65in]{geometry}\usepackage[T1]{fontenc}\usepackage{graphicx}\usepackage{url}\setlength{\parindent}{0pt}\setlength{\parskip}{7pt}\sloppy\begin{document}'
tex+=r'\section*{Genus-2 goal coverage audit}'+esc('Snapshot '+m['commit'])+r'\par '
tex+=esc('Read-only source and artifact inspection of four original leads and five follow-up lanes. This report distinguishes recorded completed cohorts from remaining source joins, exceptional coverage and new scientific admission. No experiment or proof was rerun and no raw cohort was recomputed. Existing original scope, controls and failures are preserved.')
tex+=r'\par\includegraphics[width=\linewidth]{diagram.pdf}\par '
tex+=esc('Diagram sources: the nine records in matrix.json; the exact 32 inspected evidence records and SHA256 digests are listed in evidence-bindings.json. This diagram is categorical coverage, not a measurement plot. Cohort numbers are retained-record observations, not new statistical claims.')
tex+=r'\par '+esc('P denotes research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets; F denotes the sibling integration/five-followups; R denotes research/projective_law_followup_20261022. Full paths and citations also remain in REPORT.md.')
for group,title in [(rows[:2],'Original leads: factor base and complete law'),(rows[2:4],'Original leads: pencil and Kummer'),(rows[4:7],'Follow-ups: correspondence, invariants and branches'),(rows[7:],'Follow-ups: Kummer and target information')]:
 tex+=r'\clearpage\section*{'+esc(title)+'}'
 for row in group:
  tex+=r'\subsection*{'+esc(row['lane'])+'}'+r'\textbf{Recorded scope.} '+esc(row['achieved'])+r'\par\textbf{Remaining requirement.} '+esc(row['remaining'])+r'\par\textbf{Evidence.} '
  for path in row['evidence'].split('; '):tex+=r'\path|'+path+r'|\par '
tex+=r'\clearpage\section*{Admission, costs and independent next tasks}'
tail=(p/'REPORT.md').read_text().split('## Admission and cost joins that are actually missing\n',1)[1]
for para in tail.split('\n\n'):
 if para.startswith('## '):tex+=r'\subsection*{'+esc(para[3:])+'}'
 else:tex+=esc(para)+r'\par '
tex+=r'\end{document}';(p/'report.tex').write_text(tex)
with (p/'render.log').open('w') as f:subprocess.run(['pdflatex','-interaction=nonstopmode','-halt-on-error','report.tex'],cwd=p,stdout=f,check=True)
