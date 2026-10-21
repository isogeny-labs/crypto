"""Layout only; mathematical values are read from native CM15/G15/M16 exports."""
from pathlib import Path
import json
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch
import typst
root=Path(__file__).resolve().parent
raw=root/'runs'
cm=json.loads((raw/'cm15-one-prime-carriers.json').read_text())
g=json.loads((raw/'g15-theta-class-descent.json').read_text())
s=json.loads((raw/'m16-new-pushed-supports.json').read_text())
fig=plt.figure(figsize=(11,7.3),facecolor='#f8fafc')
ax=fig.add_axes([0,0,1,1]);ax.set_axis_off()
ax.text(.045,.95,'Round 15 · new carrier and construction leads',fontsize=17,weight='bold',color='#12304a')
ax.text(.045,.905,'Exact finite checks + written geometric bridges · explicit maps / equations remain open',fontsize=10,color='#425466')
bar=fig.add_axes([.09,.43,.43,.36]);rows=cm['winners']
bars=bar.bar([str(r['n']) for r in rows],[r['carrier_genus'] for r in rows],color='#3183a6',width=.58)
bar.set_yscale('log');bar.set_ylim(10,220000);bar.bar_label(bars,labels=[str(r['carrier_genus']) for r in rows],padding=4,fontsize=10)
bar.set_ylabel('Character-cover genus (log scale)');bar.set_xlabel('Extension degree n')
bar.spines[['top','right']].set_visible(False);bar.grid(axis='y',alpha=.15);bar.set_axisbelow(True)
bar.set_title('One-prime CM level 7p',loc='left',fontsize=12,weight='bold',pad=16)
def box(x,y,w,h,title,body,color='#e2eef8'):
 ax.add_patch(FancyBboxPatch((x,y),w,h,boxstyle='round,pad=0.01',facecolor=color,edgecolor='#7792a9'))
 ax.text(x+.014,y+h-.025,title,fontsize=11,weight='bold',color='#12304a',va='top')
 ax.text(x+.014,y+h-.075,body,fontsize=10,color='#273d50',va='top',linespacing=1.6)
box(.61,.625,.33,.19,'Actual theta-class correction','3lambda target: P'+str(g['ambient_projective_dimension'])+'\nSections: '+str(g['sections_3lambda'])+'\nSection bases / proper ideals open')
box(.61,.37,.33,.19,'New coupled norms','Ordinary pushed degrees: '+', '.join(str(r['pushed_cusp_SS_ordinary_degrees'][2]) for r in s['records'][1:])+'\nExact factors and branch checks\nSource fibers / images open')
box(.06,.075,.41,.19,'Independent CM controls','Full Euler products and a2 orbits\nComplete S,T coset genus checks\nLMFDB level77 / 539 local factors','#e4f2e9')
box(.54,.075,.40,.19,'Saturation boundary','All four models: no h=t first jet\nRamified h=t² lifts remain possible\nFull quotient saturation open','#fff0d9')
ax.text(.06,.307,'n=131 requires a geometric diamond quadratic twist to obtain E0. No IC cost verdict.',fontsize=10,color='#425466')
fig.savefig(root/'routes.svg',bbox_inches='tight');fig.savefig(root/'routes.png',dpi=150,bbox_inches='tight');plt.close(fig)
svg=root/'routes.svg';svg.write_text('\n'.join(line.rstrip() for line in svg.read_text().splitlines())+'\n')
def txt(s):return json.dumps(s,ensure_ascii=False)
lines=(root/'REPORT.md').read_text().splitlines();out=['#set page(paper: "a4", margin: (x: 17mm, y: 15mm), numbering: "1")','#set text(font: "DejaVu Sans", size: 8.7pt)','#set par(leading: 0.38em)','#set heading(numbering: none)'];i=0;inserted=False
while i<len(lines):
 line=lines[i]
 if not line:i+=1;continue
 if line.startswith('#'):
  n=len(line)-len(line.lstrip('#'))
  if line[n:].strip() in ('Proper polarized target','Full quotient saturation'):out.append('#pagebreak()')
  out.append(f'#heading(level: {n})[#text({txt(line[n:].strip())})]');i+=1;continue
 if line.startswith('|'):
  rows=[]
  while i<len(lines) and lines[i].startswith('|'):
   row=[x.strip() for x in lines[i].strip('|').split('|')]
   if not all(set(x)<=set('-: ') for x in row):rows.append(row)
   i+=1
  out.append('#text(size: 7.2pt)[#table(columns: '+str(len(rows[0]))+', inset: 3pt, stroke: 0.35pt + rgb("cbd5e1"),'+','.join('[#text('+txt(x)+')]' for row in rows for x in row)+')]');continue
 para=[]
 while i<len(lines) and lines[i] and not lines[i].startswith(('#','|')):para.append(lines[i]);i+=1
 p=' '.join(para)
 if p.startswith('[Protocol]'):continue
 out.append('#text('+txt(p)+')\n')
 if not inserted and p.startswith('The strongest new lead'):
  out.append('#image("routes.svg",width:100%)\n');inserted=True
out.append('#heading(level:1)[Source links]\n#link("https://antsmath.org/ANTSXV/papers/ANTS-XV_molin-page.pdf")[Molin–Page, Theorem37] · #link("https://www.math.ru.nl/~bmoonen/BookAV/ThetaGr.pdf")[Moonen, Theta groups] · #link("https://www.math.uchicago.edu/~ngo/Shimura.pdf")[Genestier–Ngo, section2.2] · #link("https://www.lmfdb.org/ModularForm/GL2/Q/holomorphic/77/2/l/a/")[LMFDB77.2.l.a]\n#text("Local evidence: CM.md, THETA.md, PROOF_MAP.md, ledger.json, runs.tar.xz, runs.sha256 and REPLAY.sh beside this PDF. Frozen failures and written global bridges remain explicit.")')
(root/'report.typ').write_text('\n\n'.join(out)+'\n')
typst.compile(str(root/'report.typ'),output=str(root/'report.pdf'),root=str(root))
