"""Layout only: plot recorded native values; no mathematical computation."""
from pathlib import Path
import json
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch
import typst
r=Path(__file__).resolve().parent
cm=json.loads((r/'runs/cm17-explicit-level77-incidence.json').read_text())
g=json.loads((r/'runs/g16-diagonal-cover-section-bases.json').read_text())
fig=plt.figure(figsize=(11,6.8),facecolor='#f8fafc');ax=fig.add_axes([0,0,1,1]);ax.set_axis_off()
ax.text(.045,.94,'Round 16 · explicit equations and the next descent gates',fontsize=17,weight='bold',color='#14364d')
ax.text(.045,.895,'Exact native data + partial Lean proofs · no IC cost assessment',fontsize=11,color='#486173')
b=fig.add_axes([.085,.43,.43,.34]);rows=cm['finite_field_records'];xs=[row['field_degree'] for row in rows]
b.bar([x-.17 for x in xs],[len(row['untwisted_incidence_points']) for row in rows],width=.32,color='#2784a5',label='Original')
b.bar([x+.17 for x in xs],[row['twisted_incidence_count'] for row in rows],width=.32,color='#c27b31',label='Diamond twist')
b.set_xticks(xs);b.set_xlabel('Field extension degree k');b.set_ylabel('Affine incidence points (exact count)');b.set_ylim(0,13);b.legend(frameon=False,fontsize=9);b.spines[['top','right']].set_visible(False);b.set_title('CM17 · direct torsion set checks through F32',loc='left',fontsize=11)
def box(x,y,w,h,title,body,col):
 ax.add_patch(FancyBboxPatch((x,y),w,h,boxstyle='round,pad=0.01',facecolor=col,edgecolor='#869fae'))
 ax.text(x+.012,y+h-.026,title,fontsize=11,weight='bold',va='top',color='#14364d');ax.text(x+.012,y+h-.077,body,fontsize=10,va='top',linespacing=1.5,color='#324b5a')
box(.60,.60,.34,.19,'CM16 · integral obstruction','Primitive Bezout denominator: 704\nDivisible by subgroup prime 11\nOther correspondences remain open','#ffeeda')
box(.60,.35,.34,.19,'G16 · actual diagonal covers','Degrees: 99,225 and 50,176\nFirst cover separable / odd\n243 target sections still undescended','#dfedf5')
box(.065,.09,.415,.19,'Q21 · four old points excluded','Actual source-bound quartic separator\nh^6 membership in original ideal\nLean residual + actual gate certificates','#dff0e6')
box(.535,.09,.405,.19,'Q22 · four new survivors','Every original and prior repair retained\nFour search visits per graph\nFull saturation / proper models open','#fff0da')
ax.text(.07,.325,'Population: all five specified fields; no sampling uncertainty. F32 supplies positive controls.',fontsize=10,color='#486173')
fig.savefig(r/'routes.svg',bbox_inches='tight');fig.savefig(r/'routes.png',dpi=150,bbox_inches='tight');plt.close(fig)
p=r/'routes.svg';p.write_text('\n'.join(z.rstrip() for z in p.read_text().splitlines())+'\n')
def txt(s):return json.dumps(s,ensure_ascii=False)
lines=(r/'REPORT.md').read_text().splitlines();out=['#set page(paper: "a4", margin: (x: 16mm, y: 14mm), numbering: "1")','#set text(font: "DejaVu Sans", size: 8.6pt)','#set par(leading: 0.35em)','#set heading(numbering: none)'];i=0;insert=False
while i<len(lines):
 line=lines[i]
 if not line:i+=1;continue
 if line.startswith('#'):
  n=len(line)-len(line.lstrip('#'));title=line[n:].strip()
  if title in ['Integral sign separation: a concrete negative result','Saturation: four old models excluded, four new survivors','Validation, retained failures and next experiments']:out.append('#pagebreak()')
  out.append(f'#heading(level: {n})[#text({txt(title)})]');i+=1;continue
 if line.startswith('|'):
  rows=[]
  while i<len(lines) and lines[i].startswith('|'):
   row=[x.strip() for x in lines[i].strip('|').split('|')]
   if not all(set(x)<=set('-: ') for x in row):rows.append(row)
   i+=1
  out.append('#text(size:7.2pt)[#table(columns:'+str(len(rows[0]))+',inset:3pt,stroke:0.35pt + rgb("cbd5e1"),'+','.join('[#text('+txt(x)+')]' for row in rows for x in row)+')]');continue
 para=[]
 while i<len(lines) and lines[i] and not lines[i].startswith(('#','|')):para.append(lines[i]);i+=1
 p=' '.join(para);out.append('#text('+txt(p)+')\n')
 if not insert and p.startswith('The main new construction'):
  out.append('#image("routes.svg",width:100%)');insert=True
out.append('#heading(level:2)[Evidence links]\n#text("Local files beside this PDF: CM.md, THETA.md, SATURATION.md, PROOF_MAP.md, ledger.json, runs.sha256, runs.tar.xz and REPLAY.sh. Global modular bridges follow Round15 CM.md; the present round supplies explicit affine circuits and partial kernel proofs, not a proper factor-map evaluator.")')
(r/'report.typ').write_text('\n\n'.join(out)+'\n');typst.compile(str(r/'report.typ'),output=str(r/'report.pdf'),root=str(r))
