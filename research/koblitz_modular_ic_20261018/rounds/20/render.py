"""Layout only: display exact values recorded by the Round20 native experiments."""
from pathlib import Path
import json
import re
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch
import typst
r=Path(__file__).resolve().parent
maps=[json.loads((r/f'runs/cm25-quadratic-controlled-poles-q{i}.json').read_text()) for i in range(8)]
fig=plt.figure(figsize=(11,6.7),facecolor='#f8fafc')
ax=fig.add_axes([.075,.52,.86,.30])
xs=list(range(8))
for sign,color,offset in [(0,'#217a92',-.17),(1,'#d98732',.17)]:
 vals=[d['records'][sign]['pointwise_valid_x'] for d in maps]
 ax.bar([x+offset for x in xs],vals,width=.31,color=color,label=f'a = {sign}')
 for x,v in zip(xs,vals):ax.text(x+offset,v+.7,str(v),ha='center',va='bottom',fontsize=9)
ax.set_ylim(0,56);ax.set_xticks(xs,['1','b','c','1+c','K','lambda','nu','1+lambda'])
ax.set_ylabel('pointwise valid x forms (count)',fontsize=10)
ax.set_xlabel('denominator q in x=N/q², y=M/q³ · 32,768 N per sign/family',fontsize=10)
ax.legend(frameon=False,loc='upper right');ax.spines[['top','right']].set_visible(False)
fig.text(.055,.955,'Round 20 · integral recovery and proper source geometry',fontsize=16,weight='bold',color='#14364d')
fig.text(.055,.905,'524,288 quadratic numerator/sign cases · 10 exact constants · no nonconstant fit',fontsize=10.5,color='#486173')
canvas=fig.add_axes([0,0,1,1],frameon=False);canvas.set_axis_off()
def box(x,w,title,body,color):
 canvas.add_patch(FancyBboxPatch((x,.145),w,.225,boxstyle='round,pad=0.012',facecolor=color,edgecolor='#b3c3cf'))
 canvas.text(x+.012,.338,title,fontsize=11,weight='bold',color='#14364d',va='top')
 canvas.text(x+.012,.29,body,fontsize=10,color='#294656',va='top',linespacing=1.55)
box(.06,.265,'G28 / G30 · source image','L Phi = I over Z[w]\n20 global fixed equations\n100 Galois-stable equations','#dff0e6')
box(.365,.265,'G26 / G29 · source sections','Canonical Pic2 match\n2,952 ordinary F2 sections\n2,688 evaluation rank / sign','#fff0da')
box(.67,.265,'Q33 / Q34 · retained null','Top relation (U+V)^19\n4 survivors remain\nFinal quotient / saturation open','#e9e3f3')
fig.text(.06,.075,'Evidence: G26–30, CM25/26, Q33/34 · algebraic counts only · 76 named Lean results with proof limits',fontsize=9.3,color='#486173')
fig.text(.06,.035,'K = b+(1+c)c. Evaluation rank measures section independence; the final quotient remains open.',fontsize=9,color='#486173')
fig.savefig(r/'routes.svg',bbox_inches='tight');fig.savefig(r/'routes.png',dpi=150,bbox_inches='tight');plt.close(fig)
p=r/'routes.svg';p.write_text('\n'.join(s.rstrip() for s in p.read_text().splitlines())+'\n')
def txt(s):return json.dumps(s,ensure_ascii=False)
lines=(r/'REPORT.md').read_text().splitlines();out=['#set page(paper: "a4", margin: (x: 16mm, y: 14mm), numbering: "1")','#set text(font: "DejaVu Sans", size: 10.5pt)','#set par(leading: 0.38em)','#set heading(numbering: none)'];i=0;insert=False
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
  out.append('#text(size:8.5pt)[#table(columns:'+str(len(rows[0]))+',inset:3pt,stroke:0.35pt + rgb("cbd5e1"),'+','.join('[#text('+txt(x)+')]' for row in rows for x in row)+')]');continue
 para=[]
 while i<len(lines) and lines[i] and not lines[i].startswith(('#','|')):para.append(lines[i]);i+=1
 p=re.sub(r'\[([^\]]+)\]\(([^)]+)\)',r'\1 (\2)',' '.join(para));out.append('#text('+txt(p)+')\n')
 if not insert and p.startswith('The new evidence'):
  out.append('#image("routes.svg",width:100%)');insert=True
(r/'report.typ').write_text('\n\n'.join(out).rstrip()+'\n');typst.compile(str(r/'report.typ'),output=str(r/'report.pdf'),root=str(r))
