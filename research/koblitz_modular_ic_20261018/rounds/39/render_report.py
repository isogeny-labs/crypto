#!/usr/bin/env python3
"""Render recorded receipt counts only, without scientific computation."""
from pathlib import Path
import json
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import typst
h=Path(__file__).resolve().parent
data={'run':'joint39-01','characters':['D=7','D=8'],'Pi7':[9,0],'Pi8':[0,9],
'collector_run':'collect39-01','chart_points':420,'closed_points':60,'deck_orbits':6,
'local_run':'local39-01','certified_local_presentations':60}
(h/'plot-data.json').write_text(json.dumps(data,indent=2)+'\n')
fig,ax=plt.subplots(figsize=(8,3.3))
ax.bar([-.18,.82],data['Pi7'],width=.36,label='Pi7',color='#23698e')
ax.bar([.18,1.18],data['Pi8'],width=.36,label='Pi8',color='#c47729')
ax.set_xticks([0,1],data['characters']);ax.set_ylim(0,10)
ax.set_ylabel('Scalar action mod 11 (representative)')
ax.set_title('Exact rational-line separation after diamond twist')
ax.legend(loc='upper center',ncols=2);ax.grid(axis='y',alpha=.2)
fig.text(.5,.01,'joint39-01: complete finite check; no statistical uncertainty or timing measurement',ha='center',fontsize=8)
fig.tight_layout(rect=(0,.06,1,1));fig.savefig(h/'round39.svg');plt.close(fig)
typst.compile(str(h/'report.typ'),output=str(h/'report.pdf'))
