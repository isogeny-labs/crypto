#!/usr/bin/env python3
"""Render exact receipt counts and diagram; no new mathematical computation."""
from pathlib import Path
import json
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import typst
h=Path(__file__).resolve().parent;r=json.loads((h/'results.json').read_text())
native=[x for x in r['runs'] if x['kind']=='native']
data={'run_ids':[x['attempt'] for x in native],'quotients':[x['completed_full_generator_quotients'] for x in native],'operator_forms':['6F+2TF','3F+9TF'],'operator_images_nonzero':[True,True],'old_R_component_scalars':[9,None]}
(h/'plot-data.json').write_text(json.dumps(data,indent=2)+'\n')
fig,ax=plt.subplots(figsize=(8,2.8))
labels=['S0','S1','S2','C0','C1','C2','Split','Images']
bars=ax.bar(labels,data['quotients'],color=['#40758d']*3+['#5d8b64']*3+['#b77836']*2)
for b,n in zip(bars,data['quotients']):ax.text(b.get_x()+b.get_width()/2,n+3,str(n),ha='center',fontsize=8)
ax.set_ylabel('Completed full-generator quotients')
ax.set_title('Exact class-arithmetic workload by native run')
ax.set_ylim(0,max(data['quotients'])+40);ax.grid(axis='y',alpha=.2)
fig.text(.5,.01,'S= signed41 shifts; C= cofactor41 shifts; split41-01 and images41-01. Counts, not IC costs.',ha='center',fontsize=8)
fig.tight_layout(rect=(0,.06,1,1));fig.savefig(h/'round41.svg');plt.close(fig)
p=h/'round41.svg';p.write_text('\n'.join(x.rstrip() for x in p.read_text().splitlines())+'\n')
typst.compile(str(h/'report.typ'),output=str(h/'report.pdf'))
