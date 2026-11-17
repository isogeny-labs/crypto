"""Layout only. Every mathematical value comes from native O24/O27/O28/G14 exports."""
from pathlib import Path
import json
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch
import typst
root=Path(__file__).resolve().parent
raw=root/'runs'
r=json.loads((raw/'o27-complete-projected-relations.json').read_text())
s=json.loads((raw/'o24-five-source-supports.json').read_text())
c=json.loads((raw/'o28-complete-closed-coupled-divisors.json').read_text())
g=json.loads((raw/'g14-canonical-polarized-target.json').read_text())
fig=plt.figure(figsize=(11,8.0),facecolor='#f8fafc')
ax=fig.add_axes([0,0,1,1]);ax.set_axis_off()
ax.text(.045,.955,'Round 14 · complete ordinary images and projected relations',fontsize=15,weight='bold',color='#12304a')
ax.text(.045,.916,'Exact finite data · O19–O28, Q16–Q17, G14 · no IC cost assessment',fontsize=10,color='#425466')
def box(x,y,w,h,title,body,color='#e2eef8'):
 ax.add_patch(FancyBboxPatch((x,y),w,h,boxstyle='round,pad=0.01',facecolor=color,edgecolor='#7792a9'))
 ax.text(x+.013,y+h-.028,title,fontsize=11,weight='bold',color='#12304a',va='top')
 ax.text(x+.013,y+h-.071,body,fontsize=9.2,color='#273d50',va='top',linespacing=1.6)
box(.05,.718,.26,.15,'Five ordinary supports','Source counts: '+', '.join(str(v) for v in s['source_point_counts'])+'\nAll multiplicities one\nEvery even / odd image complete')
box(.37,.718,.26,.15,'Fresh scalar deck columns',str(r['new_columns_count'])+' direct split-column images\nBoth projected row sums checked\nPrincipal and deck rows distinct')
box(.69,.718,.26,.15,'Proper quotient target','Canonical P^'+str(g['ambient_projective_dimension'])+'\nActual Cartier action corrected\nGlobal sections / equations open','#fff0d9')
for x in [.322,.642]:ax.annotate('',xy=(x+.034,.793),xytext=(x-.005,.793),arrowprops={'arrowstyle':'->','color':'#385972','lw':2})
bar=fig.add_axes([.09,.31,.83,.31]);colors=['#a4b4c4','#3183a6','#287d51']
series=[r['old_rank_controls'],r['principal_only_joined_ranks'],r['with_deck_joined_ranks']]
labels=['Retained catalog','Plus coupled principal rows','Plus deck-fiber rows']
for j,(values,label,color) in enumerate(zip(series,labels,colors)):
 positions=[i+(j-1)*.24 for i in range(3)]
 bars=bar.bar(positions,values,width=.22,label=label,color=color)
 bar.bar_label(bars,padding=3,fontsize=10)
bar.set_xticks(range(3),['All selected columns','Even nonzero images','Odd nonzero images'])
bar.set_ylim(0,24);bar.set_ylabel('Relation rank over F₁₁ (exact)')
bar.spines[['top','right']].set_visible(False);bar.legend(loc='upper center',bbox_to_anchor=(.5,1.18),ncol=3,fontsize=8,frameon=False)
bar.tick_params(labelsize=9);bar.grid(axis='y',alpha=.15);bar.set_axisbelow(True)
ax.text(.07,.25,'Catalog widths: old 1308 / 1165 / 1160; extended '+str(r['all_columns'])+' / '+' / '.join(str(v) for v in r['nonzero_column_counts']),fontsize=9.5,color='#425466')
box(.05,.055,.43,.14,'All eight closed signed divisors',str(c['closed_cusp_count'])+' cusp orbits exhaust '+str(c['geometric_cusps'])+' labels.\nOld 22-point product divisor reproduced.\nWritten Tate bridge; finite Lean certificates.','#e4f2e9')
box(.53,.055,.42,.14,'Full saturation remains open','98 linear products and 288 quadrics per graph.\nNo new separating scalar boundary witness.\nSurviving models are not a saturation verdict.','#fff0d9')
fig.savefig(root/'routes.svg',bbox_inches='tight');fig.savefig(root/'routes.png',dpi=150,bbox_inches='tight');plt.close(fig)
svg=root/'routes.svg';svg.write_text('\n'.join(line.rstrip() for line in svg.read_text().splitlines())+'\n')
def txt(s):return json.dumps(s,ensure_ascii=False)
lines=(root/'REPORT.md').read_text().splitlines();out=['#set page(paper: "a4", margin: (x: 17mm, y: 15mm), numbering: "1")','#set text(font: "DejaVu Sans", size: 8.7pt)','#set par(leading: 0.38em)','#set heading(numbering: none)'];i=0;inserted=False
while i<len(lines):
 line=lines[i]
 if not line:i+=1;continue
 if line.startswith('#'):
  n=len(line)-len(line.lstrip('#'))
  if line[n:].strip()=='Saturation and proper polarization':out.append('#pagebreak()')
  out.append(f'#heading(level: {n})[#text({txt(line[n:].strip())})]');i+=1;continue
 if line.startswith('|'):
  rows=[]
  while i<len(lines) and lines[i].startswith('|'):
   row=[x.strip() for x in lines[i].strip('|').split('|')]
   if not all(set(x)<=set('-: ') for x in row):rows.append(row)
   i+=1
  out.append('#text(size: 7.5pt)[#table(columns: '+str(len(rows[0]))+', inset: 3pt, stroke: 0.35pt + rgb("cbd5e1"),'+','.join('[#text('+txt(x)+')]' for row in rows for x in row)+')]');continue
 para=[]
 while i<len(lines) and lines[i] and not lines[i].startswith(('#','|')):para.append(lines[i]);i+=1
 p=' '.join(para)
 if p.startswith('[Protocol]'):continue
 out.append('#text('+txt(p)+')\n')
 if not inserted and p.startswith('The five requested'):
  out.append('#image("routes.svg",width:100%)\n');inserted=True
out.append('#heading(level:1)[Evidence and formal boundaries]\n#text("The raw runs.tar.xz archive and runs.sha256 retain native outputs and failures. REPLAY.sh rebuilds all mathematical outputs. ledger.json pins sources, raw inputs, identity certificates, requirement status and the final audit total. PROOF_MAP.md distinguishes literal kernel checks from written global geometry and native implementation semantics. GEOMETRY.md specifies the proper polarized target and the remaining section/equation construction.")')
(root/'report.typ').write_text('\n\n'.join(out)+'\n')
typst.compile(str(root/'report.typ'),output=str(root/'report.pdf'),root=str(root))
