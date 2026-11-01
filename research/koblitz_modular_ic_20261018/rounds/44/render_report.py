#!/usr/bin/env python3
"""Render exact source trace/control counters, not a timing or attack graph."""
from pathlib import Path
import json
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import typst
h=Path(__file__).resolve().parent;r=json.loads((h/'results.json').read_text())
controls=r['retained_source_filter_controls'];labels=['83 original','83 twist','284 original','284 twist'];keys=[('83','0'),('83','1'),('284','0'),('284','1')]
inert=[sum(x['base_degree']==d and x['twist']==t and x['orientation_trace']=='1'for x in controls)for d,t in keys];eligible=[sum(x['base_degree']==d and x['twist']==t and x['orientation_trace']=='0'for x in controls)for d,t in keys]
data={'labels':labels,'trace_one_controls':inert,'trace_zero_controls':eligible,'source_cases':r['source_blocks'],'note':'Fixed retained controls; trace0is eligibility only, not a nonzero class or probability','runs':['filter83-01','filter284-01','point12-02','point436-03']}
(h/'plot-data.json').write_text(json.dumps(data,indent=2)+'\n')
fig,ax=plt.subplots(figsize=(8.8,2.8));x=range(4)
ax.bar(x,inert,label='Trace 1: sign obstruction',color='#a36e48');ax.bar(x,eligible,bottom=inert,label='Trace 0: not excluded by this screen',color='#3e7b8b');ax.set_xticks(list(x),labels,fontsize=9);ax.set_ylabel('Fixed retained source blocks');ax.set_ylim(0,6.6);ax.legend(fontsize=8,loc='upper center',ncol=2);ax.grid(axis='y',alpha=.2)
fig.text(.5,.01,'filter83-01/filter284-01: five generator blocks per form; no sampled success rate or IC rank.',ha='center',fontsize=8)
fig.tight_layout(rect=(0,.07,1,1));fig.savefig(h/'round44.svg');plt.close(fig)
p=h/'round44.svg';p.write_text('\n'.join(x.rstrip()for x in p.read_text().splitlines())+'\n')
typst.compile(str(h/'report.typ'),output=str(h/'report.pdf'))
