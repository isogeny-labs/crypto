#!/usr/bin/env python3
"""Render exact recorded construction counts only."""
from pathlib import Path
import json
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import typst
h=Path(__file__).resolve().parent
r=json.loads((h/'results.json').read_text())
data={'jet_run':'jets40-01','section_ranks':r['source_jets']['section_ranks'],'coupled_run':'couple40-01','complete_divisors':59,'small_dimensions':[68]*59,'large_dimensions':[196]*59,'pair_run':'pair40-01','difference_principal':False}
(h/'plot-data.json').write_text(json.dumps(data,indent=2)+'\n')
fig,ax=plt.subplots(figsize=(8,2.7))
ax.bar(['L(D0)','L(2D0)','L(3D0)'],data['section_ranks'],color=['#33718e','#588b6b','#c78237'])
for i,n in enumerate(data['section_ranks']):ax.text(i,n+7,str(n),ha='center')
ax.set_ylabel('F2 dimension');ax.set_ylim(0,370)
ax.set_title('Fixed section spaces on 60 degree-seven supports, precision 17')
ax.grid(axis='y',alpha=.2)
fig.text(.5,.01,'jets40-01: exact ranks and all pivot recombinations; no statistical or timing claim',ha='center',fontsize=8)
fig.tight_layout(rect=(0,.05,1,1));fig.savefig(h/'round40.svg');plt.close(fig)
svg=h/'round40.svg';svg.write_text('\n'.join(line.rstrip() for line in svg.read_text().splitlines())+'\n')
typst.compile(str(h/'report.typ'),output=str(h/'report.pdf'))
