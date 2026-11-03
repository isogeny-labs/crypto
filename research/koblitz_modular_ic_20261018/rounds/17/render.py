"""Layout only: display recorded native values; no research arithmetic."""
from pathlib import Path
import json
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch
import typst
r=Path(__file__).resolve().parent
cm=json.loads((r/'runs/cm18-integral-Frobenius-lattice-controls.json').read_text())
g=json.loads((r/'runs/g18-factored-odd-kernel-theta-lifts.json').read_text())
fig=plt.figure(figsize=(11,7.0),facecolor='#f8fafc')
ax=fig.add_axes([0,0,1,1]);ax.set_axis_off()
ax.text(.04,.95,'Round 17 · integral maps, theta lifts and boundary tests',fontsize=17,weight='bold',color='#14364d')
ax.text(.04,.905,'Native certificates + partial Lean proofs · global geometry remains written',fontsize=11,color='#486173')
def box(x,y,w,h,title,body,col):
 ax.add_patch(FancyBboxPatch((x,y),w,h,boxstyle='round,pad=0.01',facecolor=col,edgecolor='#869fae'))
 ax.text(x+.012,y+h-.023,title,fontsize=11,weight='bold',va='top',color='#14364d')
 ax.text(x+.012,y+h-.067,body,fontsize=9.7,va='top',linespacing=1.45,color='#324b5a')
box(.055,.64,.39,.19,'CM18 · explicit lattice controls','Forward: fixed 11-line survives\nNatural reverse: fixed 11-line cancels\nSplit / cyclic fixed dimensions: '+str(cm['split_fixed11_dimension'])+' / '+str(cm['cyclic_fixed11_dimension']),'#dfedf5')
box(.55,.64,.39,.19,'Actual level-77 map · open','Written prime-to-11 existence argument\nConditional on compact modular bridge\nNo evaluated integral correspondence','#fff0da')
ax.annotate('',xy=(.54,.735),xytext=(.46,.735),arrowprops=dict(arrowstyle='->',color='#9a794c',linestyle='dashed',lw=1.6))
ax.text(.46,.77,'gap',fontsize=9,color='#86643a')
b=fig.add_axes([.10,.22,.37,.28]);dims=g['two_stage_section_dimensions']
b.bar(['Cover','Kernel quotient','Target'],dims,color=['#678fbd','#3e8f92','#8a67a6'],width=.55)
b.set_yscale('log');b.set_ylim(1,1e11);b.set_ylabel('Section count (log scale)',fontsize=9)
b.spines[['top','right']].set_visible(False);b.tick_params(labelsize=9)
for i,value in enumerate(dims):b.annotate(f'{value:,}',(i,value),xytext=(0,5),textcoords='offset points',ha='center',fontsize=9)
b.set_title('G18 · derived dimensions under written bridge',loc='left',fontsize=10)
box(.55,.365,.39,.18,'Odd kernel and factored lifts','E[3] × E[105] · order 99,225\nFour normalized generators per graph\nCanonical character / F₂ descent open','#dff0e6')
box(.55,.13,.39,.18,'Q23 / Q24 · exact boundary tests','All four Q22 points excluded at depth 6\nFour new companion survivors verified\nWhole saturation / genuine arcs open','#fff0da')
ax.text(.085,.105,'Dimensions are theoretical counts, not measured matrix ranks.',fontsize=9.6,color='#486173')
ax.text(.055,.045,'Evidence: CM18, G17/G18, Q23/Q24 JSON records in runs.tar.xz · no IC cost or rank promotion',fontsize=9.3,color='#486173')
fig.savefig(r/'routes.svg',bbox_inches='tight');fig.savefig(r/'routes.png',dpi=150,bbox_inches='tight');plt.close(fig)
p=r/'routes.svg';p.write_text('\n'.join(z.rstrip() for z in p.read_text().splitlines())+'\n')
def txt(s):return json.dumps(s,ensure_ascii=False)
lines=(r/'REPORT.md').read_text().splitlines();out=['#set page(paper: "a4", margin: (x: 16mm, y: 14mm), numbering: "1")','#set text(font: "DejaVu Sans", size: 9pt)','#set par(leading: 0.38em)','#set heading(numbering: none)'];i=0;insert=False
while i<len(lines):
 line=lines[i]
 if not line:i+=1;continue
 if line.startswith('#'):
  n=len(line)-len(line.lstrip('#'));title=line[n:].strip()
  if n==2:out.append('#pagebreak()')
  out.append(f'#heading(level: {n})[#text({txt(title)})]');i+=1;continue
 if line.startswith('|'):
  rows=[]
  while i<len(lines) and lines[i].startswith('|'):
   row=[x.strip() for x in lines[i].strip('|').split('|')]
   if not all(set(x)<=set('-: ') for x in row):rows.append(row)
   i+=1
  out.append('#text(size:7.5pt)[#table(columns:'+str(len(rows[0]))+',inset:3pt,stroke:0.35pt + rgb("cbd5e1"),'+','.join('[#text('+txt(x)+')]' for row in rows for x in row)+')]');continue
 para=[]
 while i<len(lines) and lines[i] and not lines[i].startswith(('#','|')):para.append(lines[i]);i+=1
 p=' '.join(para);out.append('#text('+txt(p)+')\n')
 if not insert and p.startswith('The main new lead'):
  out.append('#image("routes.svg",width:100%)');insert=True
out.append('#heading(level:2)[Evidence links]\n#text("Companion files: CM.md, THETA.md, SATURATION.md, PROOF_MAP.md, SOURCES.md, ledger.json, runs.sha256, runs.tar.xz and REPLAY.sh. Primary integral Tate theorem source: Sam Lichtenstein, The Tate conjecture, Theorem 1.1 and Section 3, math.stanford.edu/~conrad/mordellsem/Notes/L03.pdf. The existence deduction and all modular/divisor bridges remain written; they are not full Lean geometry proofs.")')
(r/'report.typ').write_text('\n\n'.join(out)+'\n');typst.compile(str(r/'report.typ'),output=str(r/'report.pdf'),root=str(r))
