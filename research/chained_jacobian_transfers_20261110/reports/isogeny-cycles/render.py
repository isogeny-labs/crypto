from pathlib import Path
import json,subprocess
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import typst
p=Path(__file__).resolve().parent
for n in ['factors','closure']:subprocess.run(['dot','-Tsvg',str(p/(n+'.dot')),'-o',str(p/(n+'.svg'))],check=True)
rows=json.loads((p/'catalog-data.json').read_text())['rows']
plt.rcParams.update({'svg.fonttype':'none','font.family':'DejaVu Sans','font.size':9})
fig,ax=plt.subplots(figsize=(8,1.65),layout='constrained');x=list(range(len(rows)))
ax.barh(x,[r['principal'] for r in rows],color='#4f8b6b',label='Principal polarization')
ax.barh(x,[r['not_principal'] for r in rows],left=[r['principal'] for r in rows],color='#b77957',label='No principal polarization')
for i,r in enumerate(rows):
 ax.text(r['principal']/2,i,str(r['principal']),ha='center',va='center',color='white',weight='bold')
 if r['not_principal']:ax.text(r['principal']+r['not_principal']/2,i,str(r['not_principal']),ha='center',va='center',color='white',weight='bold')
ax.set_yticks(x,[r['layer'] for r in rows]);ax.invert_yaxis();ax.set_xlabel('Unpolarized AV classes · written classification f48205');ax.set_xlim(0,80);ax.set_xticks([0,12,36,72]);ax.spines[['top','right']].set_visible(False);ax.legend(loc='upper right',ncol=2,bbox_to_anchor=(1,1.35),frameon=False)
fig.savefig(p/'catalog.svg');plt.close(fig)
for f in p.glob('*.svg'):f.write_text('\n'.join(x.rstrip() for x in f.read_text().splitlines()).rstrip()+'\n')
typst.compile(str(p/'report.typ'),output=str(p/'report.pdf'))
