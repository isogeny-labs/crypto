#!/usr/bin/env python3
"""Render recorded native ranks and product counts; no new math experiment."""
from pathlib import Path
import json
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import typst
h=Path(__file__).resolve().parent;r=json.loads((h/'results.json').read_text());st=r['product_stages']
data={'labels':[x['label'] for x in st],'ranks':[x['output_rank'] for x in st],'products':[x['attempted_products'] for x in st],'run':'multiply43-01','precision_run':'jets43-02','source_precision':r['source_precision'],'reference_precision':r['reference_precision'],'reference_rows':r['reference_rows'],'note':'Exact construction counters; no timing or IC rank'}
(h/'plot-data.json').write_text(json.dumps(data,indent=2)+'\n')
fig,axes=plt.subplots(1,2,figsize=(9,3))
for ax,values,title in [(axes[0],data['ranks'],'Verified independent section rank'),(axes[1],data['products'],'Attempted products (both methods checked)')]:
 bars=ax.bar(data['labels'],values,color='#397e83');ax.set_title(title,fontsize=10);ax.tick_params(axis='x',labelsize=8);ax.grid(axis='y',alpha=.2)
 for b,v in zip(bars,values):ax.text(b.get_x()+b.get_width()/2,v+max(values)*.025,str(v),ha='center',fontsize=8)
 ax.set_ylim(0,max(values)*1.17)
axes[1].ticklabel_format(axis='y',style='plain')
fig.text(.5,.015,'multiply43-01; rank stopping rule fixed before data. Source expansions: jets43-02.',ha='center',fontsize=8)
fig.tight_layout(rect=(0,.06,1,1));fig.savefig(h/'round43.svg');plt.close(fig)
p=h/'round43.svg';p.write_text('\n'.join(x.rstrip() for x in p.read_text().splitlines())+'\n')
typst.compile(str(h/'report.typ'),output=str(h/'report.pdf'))
