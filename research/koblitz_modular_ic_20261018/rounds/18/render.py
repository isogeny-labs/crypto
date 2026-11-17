"""Layout only: display exact counts already recorded by the native experiments."""
from pathlib import Path
import json
import re
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch
import typst
r=Path(__file__).resolve().parent
maps=json.loads((r/'runs/cm21-affine-linear-elliptic-map-search.json').read_text())
hold=json.loads((r/'runs/cm22-independent-elliptic-map-holdouts.json').read_text())
fig=plt.figure(figsize=(11,7),facecolor='#f8fafc');ax=fig.add_axes([0,0,1,1]);ax.set_axis_off()
ax.text(.04,.95,'Round 18 · canonical characters and explicit map tests',fontsize=17,weight='bold',color='#14364d')
ax.text(.04,.904,'Exact native evidence + partial Lean proofs · no IC cost or rank assessment',fontsize=11,color='#486173')
def box(x,y,w,h,title,body,col):
 ax.add_patch(FancyBboxPatch((x,y),w,h,boxstyle='round,pad=0.01',facecolor=col,edgecolor='#869fae'))
 ax.text(x+.012,y+h-.025,title,fontsize=11,weight='bold',va='top',color='#14364d')
 ax.text(x+.012,y+h-.074,body,fontsize=10.5,va='top',linespacing=1.45,color='#324b5a')
b=fig.add_axes([.09,.60,.37,.22]);before=[v['nonconstant_fits'] for v in maps['records']];after=[len(v['nonconstant_survivors']) for v in hold['records']]
b.bar([-.16,.84],before,.30,color='#467da7',label='Ten-point finite fits');b.bar([.16,1.16],after,.30,color='#d0873a',label='After independent holdouts')
for i in range(2):b.text(i-.16,before[i]+2,str(before[i]),ha='center',fontsize=10);b.text(i+.16,2,str(after[i]),ha='center',fontsize=10,color='#956025')
b.set_xticks([0,1],['a=0','a=1']);b.set_ylabel('Nonconstant candidates',fontsize=10);b.set_ylim(0,125);b.legend(frameon=False,fontsize=8.3,loc='upper right');b.spines[['top','right']].set_visible(False)
box(.55,.60,.39,.22,'CM19–22 · actual bounded search','Diamond-2 correspondence evaluated\nAll 102 finite false fits refuted\nIntegral factor inclusion still open','#dfedf5')
box(.055,.32,.265,.19,'G19 · canonical characters','Exponents: 35, 0, 50, 23\nUnique symmetric normalization\nWritten bundle-class bridge','#dff0e6')
box(.365,.32,.265,.19,'G20 · selected-cover no-lift','B⁻¹ R B denominator: 105\nExact modulo-81 obstruction\nQuotient descent not excluded','#fff0da')
box(.675,.32,.265,.19,'G21 · stable refinement','B C = [105], commuting with R\nExplicit integral CM matrix C\nMixed invariant basis open','#e9e3f3')
box(.055,.08,.885,.16,'Q25–28 · source-bound saturation steps','Four Q24 points excluded at depth 6 → four Q26 points excluded at depth 5\nFour Q28 models on nodes 95, 97, 123 remain unclassified; full saturation is open.','#fff0da')
ax.text(.065,.265,'Counts cover the complete finite-fit populations; eight new incidences test every candidate.',fontsize=9.5,color='#486173')
ax.text(.06,.025,'Evidence: CM19–22, G19–21 and Q25–28 JSON in runs.tar.xz · 94 new named Lean results',fontsize=9.4,color='#486173')
fig.savefig(r/'routes.svg',bbox_inches='tight');fig.savefig(r/'routes.png',dpi=150,bbox_inches='tight');plt.close(fig)
p=r/'routes.svg';p.write_text('\n'.join(s.rstrip() for s in p.read_text().splitlines())+'\n')
def txt(s):return json.dumps(s,ensure_ascii=False)
lines=(r/'REPORT.md').read_text().splitlines();out=['#set page(paper: "a4", margin: (x: 16mm, y: 14mm), numbering: "1")','#set text(font: "DejaVu Sans", size: 9pt)','#set par(leading: 0.38em)','#set heading(numbering: none)'];i=0;insert=False
while i<len(lines):
 line=lines[i]
 if not line:i+=1;continue
 if line.startswith('#'):
  n=len(line)-len(line.lstrip('#'));title=line[n:].strip()
  if n==2:out.append('#pagebreak()')
  out.append(f'#heading(level:{n})[#text({txt(title)})]');i+=1;continue
 if line.startswith('|'):
  rows=[]
  while i<len(lines) and lines[i].startswith('|'):
   row=[x.strip() for x in lines[i].strip('|').split('|')]
   if not all(set(x)<=set('-: ') for x in row):rows.append(row)
   i+=1
  out.append('#text(size:7.5pt)[#table(columns:'+str(len(rows[0]))+',inset:3pt,stroke:0.35pt + rgb("cbd5e1"),'+','.join('[#text('+txt(x)+')]' for row in rows for x in row)+')]');continue
 para=[]
 while i<len(lines) and lines[i] and not lines[i].startswith(('#','|')):para.append(lines[i]);i+=1
 p=re.sub(r'\[([^\]]+)\]\(([^)]+)\)',r'\1 (\2)',' '.join(para));out.append('#text('+txt(p)+')\n')
 if not insert and p.startswith('The canonical theta characters'):
  out.append('#image("routes.svg",width:100%)');insert=True
out.append('#heading(level:2)[Evidence and sources]\n#text("Companion files: CM.md, THETA.md, SATURATION.md, PROOF_MAP.md, SOURCES.md, ledger.json and REPLAY.sh. Primary theta references: Damien Robert, The arithmetic of theta groups and biextensions of abelian varieties, Theorem 1.1 proof (2023); Ben Moonen, The theta group of a line bundle, 8.10–8.15. The full modular/divisor geometry remains written; source-code hashes, failures and exact replay receipts are retained beside this report.")')
(r/'report.typ').write_text('\n\n'.join(out)+'\n');typst.compile(str(r/'report.typ'),output=str(r/'report.pdf'),root=str(r))
