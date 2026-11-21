#!/usr/bin/env python3
"""Render recorded native sizes; no algebra or measured timing."""
from pathlib import Path
import json
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import typst
h=Path(__file__).resolve().parent;r=json.loads((h/'results.json').read_text())
a=r['integer_lattices'];b=[c for c in r['cases'] if c['global_recovery_status']=='certified']
data={'lattice_n':[c['n'] for c in a],'integral_dimensions':[c['dimension'] for c in a],'inverse_n':[c['n'] for c in b],'inverse_degrees':[c['inverse_polynomial_degree'] for c in b],'runs':['integer42-01','global42-01'],'measurement':'exact native size counters; no performance measurements'}
(h/'plot-data.json').write_text(json.dumps(data,indent=2)+'\n')
fig,axes=plt.subplots(1,2,figsize=(9,3))
for ax,labels,values,title in [(axes[0],data['lattice_n'],data['integral_dimensions'],'Integral denominator matrix dimension'),(axes[1],data['inverse_n'],data['inverse_degrees'],'Explicit modular inverse degree')]:
 bars=ax.bar([str(x) for x in labels],values,color='#36758a')
 for bar,v in zip(bars,values):ax.text(bar.get_x()+bar.get_width()/2,v+2,str(v),ha='center')
 ax.set_xlabel('n');ax.set_ylim(0,150);ax.set_title(title,fontsize=10);ax.grid(axis='y',alpha=.2)
fig.text(.5,.015,'integer42-01 and global42-01: complete exact certificates; sizes are not IC costs.',ha='center',fontsize=8)
fig.tight_layout(rect=(0,.06,1,1));fig.savefig(h/'round42.svg');plt.close(fig)
p=h/'round42.svg';p.write_text('\n'.join(x.rstrip() for x in p.read_text().splitlines())+'\n')
typst.compile(str(h/'report.typ'),output=str(h/'report.pdf'))
