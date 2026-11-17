"""Layout only: display the native Round21 root traces and source-route records."""
from pathlib import Path
import json
import re
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch
import typst
r=Path(__file__).resolve().parent
s=json.loads((r/'runs/round21-native-summary.json').read_text())
q=s['Q35']
fig=plt.figure(figsize=(11,6.6),facecolor='#f8fafc')
ax=fig.add_axes([.075,.52,.86,.30])
values=[int(x,16) for x in q['trace18_values']]
ax.bar(list(range(19)),values,color=['#d98732']+['#217a92']*18,width=.7)
ax.set_xticks(list(range(19)));ax.set_ylim(0,1.35);ax.set_yticks([0,1])
ax.set_xlabel('exponent i in the recorded primitive nineteenth-root powers',fontsize=10)
ax.set_ylabel('Tr18(zeta^i) (F2 value)',fontsize=10)
ax.spines[['top','right']].set_visible(False)
fig.text(.055,.955,'Round 21 · complete descent and equivariant recovery',fontsize=16,weight='bold',color='#14364d')
fig.text(.055,.903,f"{q['four_prefixes_exhausted']:,} ordered root prefixes · zero compatible fifth roots · exact trace values",fontsize=10.5,color='#486173')
canvas=fig.add_axes([0,0,1,1],frameon=False);canvas.set_axis_off()
def box(x,w,title,body,color):
 canvas.add_patch(FancyBboxPatch((x,.16),w,.225,boxstyle='round,pad=0.012',facecolor=color,edgecolor='#b3c3cf'))
 canvas.text(x+.012,.354,title,fontsize=11,weight='bold',color='#14364d',va='top')
 canvas.text(x+.012,.302,body,fontsize=10,color='#294656',va='top',linespacing=1.55)
box(.06,.265,'G31 · source recovery','A Phi = 5 I, equivariant\nDenominator 5 is sharp\nFinal quotient still open','#dff0e6')
box(.365,.265,'Q35 · complete descent','Fivefold trace-zero identity\nFive roots cannot sum to zero\nGeometric bridge written','#fff0da')
box(.67,.265,'Q37–40 · retained frontier',f"{s['ordinary_coupled_relations']} coupled norms\n{s['pure_source_relations']:,} pure-source relations\n4 models remain on node 187",'#e9e3f3')
fig.text(.06,.088,'Evidence: Q35–40 / G31 · 19 exact root traces · algebraic counts only · 206-statement proof map',fontsize=9.3,color='#486173')
fig.text(.06,.045,'No uncertainty applies to these exact values. Full saturation and the integral level77 map remain open.',fontsize=9,color='#486173')
fig.savefig(r/'routes.svg',bbox_inches='tight');fig.savefig(r/'routes.png',dpi=150,bbox_inches='tight');plt.close(fig)
svg=r/'routes.svg'
svg.write_text('\n'.join(line.rstrip() for line in svg.read_text().splitlines())+'\n')
source=(r/'REPORT.md').read_text()
parts=re.split(r'\n\s*\n',source.strip())
out=['#set page(paper: "a4", margin: (x: 16mm, y: 14mm), numbering: "1")','#set text(font: "DejaVu Sans", size: 10.5pt)','#set par(leading: 0.38em)','#set heading(numbering: none)']
for part in parts:
 text=' '.join(part.splitlines())
 if text.startswith('!['):
  out.append('#image("routes.svg",width:100%)')
 elif text.startswith('#'):
  level=len(text)-len(text.lstrip('#'))
  label=text[level:].strip()
  if label=='All factors matter in the polynomial repairs':out.append('#pagebreak()')
  if label=='Verification and remaining requirements':out.append('#pagebreak()')
  out.append(f'#heading(level:{level})[#text({json.dumps(label,ensure_ascii=False)})]')
 else:
  text=re.sub(r'\[([^]]+)\]\([^)]+\)',r'\1',text).replace('`','')
  out.append(f'#text({json.dumps(text,ensure_ascii=False)})')
out.append('#link("https://github.com/aburan28/crypto/pull/2215")[PR2215: source artifacts, proof map and replay]')
(r/'report.typ').write_text('\n\n'.join(out).rstrip()+'\n')
typst.compile(str(r/'report.typ'),output=str(r/'report.pdf'),root=str(r))
