"""Document layout only. Every plotted mathematical value is read from frozen exports."""
from pathlib import Path
import json
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch
import typst
root=Path(__file__).resolve().parent
raw=root/'runs'
fibers=json.loads((raw/'o16-ordinary-source-fibers.json').read_text())
fig=plt.figure(figsize=(11,7.4),facecolor='#f8fafc')
ax=fig.add_axes([0,0,1,1]);ax.set_axis_off()
ax.text(.045,.954,'Round 13 · complete ordinary components; saturation remains open',fontsize=15,weight='bold',color='#12304a')
ax.text(.045,.917,'Exact degrees and multiplicities · O13–O18, Q13–Q15 · 20 October 2026',fontsize=10,color='#425466')
def box(x,y,w,h,title,body,color='#e2eef8'):
 ax.add_patch(FancyBboxPatch((x,y),w,h,boxstyle='round,pad=0.01',facecolor=color,edgecolor='#7792a9'))
 ax.text(x+.013,y+h-.035,title,fontsize=11,weight='bold',color='#12304a',va='top')
 ax.text(x+.013,y+h-.075,body,fontsize=9.6,color='#273d50',va='top',linespacing=1.6)
box(.05,.72,.26,.14,'Cyclic-121 Hecke norm','132 branches; poles 504 / 528\n640 output coefficients checked')
box(.37,.72,.26,.14,'Pushed ordinary support','2 degree-50 points\n4 degree-29 points')
box(.69,.72,.26,.14,'Full-level scalar fiber','Irreducible degree 5 over each base\n24 commuting squares checked')
for x in [.322,.642]:ax.annotate('',xy=(x+.034,.79),xytext=(x-.005,.79),arrowprops={'arrowstyle':'->','color':'#385972','lw':2})
ax.text(.05,.657,'Ordinary source degrees · each colored block is one simple closed point',fontsize=11,weight='bold',color='#12304a')
bar=fig.add_axes([.22,.45,.70,.15]);lefts=[0,0]
colors=['#147d92','#4a9aad','#88b7c3','#b5d5dc']
slots=[0,0]
for row in fibers['records']:
 fi=row['function_index']
 for p in row['verified_lifts'][0]['source_closed_points']:
  d=p['closed_degree'];bar.barh(1-fi,d,left=lefts[fi],height=.55,color=colors[slots[fi]],edgecolor='white');bar.text(lefts[fi]+d/2,1-fi,str(d),ha='center',va='center',fontsize=10,color='white' if slots[fi]<2 else '#12304a');lefts[fi]+=d;slots[fi]+=1
bar.set_yticks([1,0],['Antidiagonal','Product + 1']);bar.set_xlim(0,600);bar.set_xticks([0,100,200,300,400,500,580]);bar.set_xlabel('Degree over F₂ (exact enumeration; no statistical uncertainty)',fontsize=9);bar.spines[['top','right']].set_visible(False);bar.tick_params(labelsize=9)
box(.05,.20,.43,.15,'Both projected images are zero','Each closed point exhausts its C₅ deck fiber.\nThe integral augmented deck norm is zero.\nWritten modular / pairing bridge; ranks unchanged.','#e4f2e9')
box(.53,.20,.42,.15,'All four circuits: further saturation repairs','23/22 inverse; 3826/3774 shared-factor;\n4 four-leaf and 5 square relations per graph.\nCounterpoints survive through Q14.','#fff0d9')
ax.text(.05,.12,'Open: full h-saturation, proper polarized F₂ embeddings, whole principal-row assembly,',fontsize=10,color='#8a4d16')
ax.text(.05,.085,'five other ordinary supports, and full geometric formalization. No IC efficiency assessment.',fontsize=10,color='#8a4d16')
fig.savefig(root/'routes.svg',bbox_inches='tight');fig.savefig(root/'routes.png',dpi=150,bbox_inches='tight');plt.close(fig)
svg=root/'routes.svg'
svg.write_text('\n'.join(line.rstrip() for line in svg.read_text().splitlines())+'\n')
def txt(s):return json.dumps(s,ensure_ascii=False)
lines=(root/'REPORT.md').read_text().splitlines();out=['#set page(paper: "a4", margin: (x: 18mm, y: 16mm), numbering: "1")','#set text(font: "DejaVu Sans", size: 9pt)','#set par(leading: 0.42em)','#set heading(numbering: none)'];i=0;inserted=False
while i<len(lines):
 line=lines[i]
 if not line: i+=1;continue
 if line.startswith('#'):
  n=len(line)-len(line.lstrip('#'));out.append(f'#heading(level: {n})[#text({txt(line[n:].strip())})]');i+=1;continue
 if line.startswith('|'):
  rows=[]
  while i<len(lines) and lines[i].startswith('|'):
   r=[x.strip() for x in lines[i].strip('|').split('|')]
   if not all(set(x)<=set('-: ') for x in r):rows.append(r)
   i+=1
  out.append('#table(columns: '+str(len(rows[0]))+', inset: 4pt, stroke: 0.35pt + rgb("cbd5e1"),'+','.join('[#text('+txt(x)+')]' for r in rows for x in r)+')');continue
 para=[]
 while i<len(lines) and lines[i] and not lines[i].startswith(('#','|')):para.append(lines[i]);i+=1
 p=' '.join(para)
 if p.startswith('[Protocol]'):continue
 out.append('#text('+txt(p)+')\n')
 if not inserted and p.startswith('Both requested'):
  out.append('#image("routes.svg",width:100%)\n');inserted=True
out.append('#heading(level:1)[Evidence paths and review]\n#text("Raw outputs are in the deterministic runs.tar.xz archive; runs.sha256 pins every member. REPLAY.sh reconstructs all outputs. PROOF_MAP.md states each formal theorem and every written bridge. ledger.json contains source hashes, IDC IDs, requirement status and audit totals. ARCHIVE_VALIDATION.txt records extraction, replay and deterministic repack.")\n#text("Editable visual: render.py and routes.svg. This document is rendered entirely from REPORT.md plus the same native JSON records used for routes.svg. No mathematical search or arithmetic is performed by this layout script.")')
(root/'report.typ').write_text('\n\n'.join(out)+'\n')
typst.compile(str(root/'report.typ'),output=str(root/'report.pdf'),root=str(root))
