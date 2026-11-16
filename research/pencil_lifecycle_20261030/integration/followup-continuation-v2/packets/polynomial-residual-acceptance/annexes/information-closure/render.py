from pathlib import Path
import json,html
b=Path(__file__).parent;d=json.loads((b/'contracts.json').read_text())
def esc(s):
 for a,z in [('\\',r'\textbackslash{}'),('&',r'\&'),('%',r'\%'),('$',r'\$'),('#',r'\#'),('_',r'\_'),('{',r'\{'),('}',r'\}')]:
  if a!='\\':s=s.replace(a,z)
 return s.replace('^',r'\textasciicircum{}').replace('->',r'$\to$').replace('<=',r'$\le$')
labels=[('C4','Adjacent-pair transition','Label propagation and None cases'),('C5','Signed recovery','Chart domain and sign uniqueness'),('C3','Duplicate-label state','First labels and coefficient difference'),('C6','Graph equations','Group projection and terminal invariant'),('C2','Prefix information','Rank criterion and counter isolation'),('C1','Physical cost ledger','Once-only setup and all dispositions')]
svg=['<svg xmlns="http://www.w3.org/2000/svg" width="1100" height="650" viewBox="0 0 1100 650"><rect width="1100" height="650" fill="white"/><defs><marker id="a" markerWidth="10" markerHeight="8" refX="9" refY="4" orient="auto"><path d="M0 0L10 4L0 8" fill="#52677d"/></marker></defs><g font-family="sans-serif"><text x="40" y="42" font-size="24" fill="#16324f">Source contracts: information and charged work</text><text x="40" y="72" font-size="16">Dependency map from saved evidence; no new experiments or cost estimates</text>']
pos=[(40,115),(400,115),(760,115),(760,340),(400,340),(40,340)]
for (id,title,sub),(x,y) in zip(labels,pos):
 svg += [f'<rect x="{x}" y="{y}" width="300" height="135" rx="8" fill="#edf3f8" stroke="#52677d"/><text x="{x+16}" y="{y+30}" font-size="20" fill="#16324f">{id}: {html.escape(title)}</text><text x="{x+16}" y="{y+65}" font-size="14">{html.escape(sub)}</text><text x="{x+16}" y="{y+108}" font-size="14">Composition obligation remains explicit</text>']
for x1,y1,x2,y2 in [(340,182,400,182),(700,182,760,182),(910,250,910,340),(760,407,700,407),(400,407,340,407)]:svg += [f'<line x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}" stroke="#52677d" stroke-width="2" marker-end="url(#a)"/>']
svg += ['<text x="40" y="535" font-size="17">Full cohort scope: 1,260 targets. Complete cost ratio: null. Natural yield: null.</text><text x="40" y="565" font-size="15">Existing lemmas, adapters and independent reviews are reused; boxes do not indicate proof completion.</text><text x="40" y="595" font-size="15">Source identities, hashes, boundaries and contracts: contracts.json and REPORT.md.</text></g></svg>']
(b/'dependency.svg').write_text('\n'.join(svg)+'\n')
tex=[r'\documentclass[10pt]{article}',r'\usepackage[margin=0.8in]{geometry}',r'\usepackage[T1]{fontenc}',r'\usepackage{lmodern,hyperref,tikz}',r'\hypersetup{colorlinks=true,urlcolor=blue}',r'\setlength{\parindent}{0pt}\setlength{\parskip}{7pt}\sloppy',r'\begin{document}',r'\begin{center}{\LARGE Relation information and Kummer source contracts}\\[6pt]Source-only audit, 1 November 2026\end{center}',r'The audit identifies six composition contracts joining physical execution, signed labelled divisors, graph relations, target information and complete charged work. Existing implementations and review artifacts are reused. No experiments, new target trajectories, cloud actions, timing measurements or theorem certifications were performed.',r'\textbf{Scope preserved:} 1,260 targets. Complete cost ratio and natural yield remain null. This packet is a source-contract map, not a completion certificate.',r'\begin{center}\begin{tikzpicture}[every node/.style={draw,rounded corners,text width=4.3cm,align=center,minimum height=1.1cm,font=\small},>=stealth]']
for idx,(id,title,sub) in enumerate(labels):
 x=[0,5.1,10.2,10.2,5.1,0][idx];y=[0,0,0,-2.1,-2.1,-2.1][idx]
 tex += [r'\node (n'+str(idx)+') at ('+str(x)+','+str(y)+') {'+esc(id+': '+title)+r'\\'+esc(sub)+'};']
for i in range(5):tex += [rf'\draw[->] (n{i}) -- (n{i+1});']
tex += [r'\end{tikzpicture}\end{center}',r'Arrows indicate required composition; a node does not assert that its obligation is proved. The editable SVG contains the same dependency sequence.',r'\textbf{Historical saved observations.} The prior portfolio inventory contained 140 files, all matching their recorded hashes during the preceding read-only audit. Its standardized table records five factor-base arms with 1,260 rows each, two law arms with 1,260 each and a Kummer differential arm with 1,260. Historical pencil coverage records 543 verified candidate keys, one capped-only key and 716 without indexed candidate rows. The later C1 gate records 7/18 observations, 4/15 measured, no complete pairs and two timeouts. These are saved-artifact counts, not new runs or scalar replays.',r'The retained duplicate archive analysis reports 420 dispositions: 218 ineligible and 202 compared, comprising 174 first observations and 28 redundant observations. The actual source uses retained coordinate witnesses, so these counts do not estimate naturally generated duplicate yield.',r'\newpage']
for i,c in enumerate(d['contracts']):
 tex += [r'\section*{'+esc(c['id'])+'}',r'\textbf{Existing evidence.} '+esc(c['existing']),r'\textbf{Unproved or unbound transition.} '+esc(c['gap']),r'\textbf{Concrete contract.} '+esc(c['contract']),r'\textbf{Next source/proof work.} '+esc(c['next']),r'\textbf{Source references:} '+esc(', '.join(c['source_refs']))+'.']
 if i%2==1:tex += [r'\newpage']
tex += [r'\section*{Accounting partition}',esc(d['accounting_ledger']['scope']),r'\textbf{Physical identity:} '+esc(d['accounting_ledger']['identity'])]
for k in ['setup_once','per_case_disjoint','nonadditive_or_external']:
 tex += [r'\textbf{'+esc(k)+r'}\begin{itemize}']+[r'\item '+esc(x) for x in d['accounting_ledger'][k]]+[r'\end{itemize}']
tex += [r'\section*{Source custody}',r'Exact absolute paths and SHA256 hashes are preserved in contracts.json. The following source keys bind the report statements.']
for k,v in d['sources'].items():
 rel=v['path'].split('/integration/')[-1]
 tex += [r'\textbf{'+esc(k)+r'}: \nolinkurl{'+rel+r'}\\{\scriptsize\texttt{'+v['sha256']+r'}}\par']
tex += [r'\end{document}']
(b/'report.tex').write_text('\n'.join(tex)+'\n')
