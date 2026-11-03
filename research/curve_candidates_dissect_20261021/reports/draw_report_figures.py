"""Render only the record-availability counts in the frozen study summaries.

This is document layout; it performs no curve arithmetic or new experiment.
"""
import json
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt

ROOT=Path(__file__).resolve().parent
OUT=ROOT/'report-figures'
OUT.mkdir(exist_ok=True)
s=json.loads((ROOT/'continuation/continuation-summary.json').read_text())
source={'source':'continuation/continuation-summary.json','unit':'parameter records',
        'population_per_view':s['original_default_calls'],
        'series':[{'label':'Original default output','available':s['original_default_calls']-s['original_incomplete_calls'],'incomplete':s['original_incomplete_calls']},
                  {'label':'Assessed view','available':s['assessed_records_with_all_fields'],'incomplete':s['remaining_incomplete_calls']}],
        'note':'Field availability is distinct from correctness. The original output contains 10 subsequently corrected records. Deterministic accounting, not a statistical estimate.'}
(OUT/'graph-source.json').write_text(json.dumps(source,indent=2)+'\n')
plt.rcParams.update({'font.family':'DejaVu Sans','font.size':10})
fig,ax=plt.subplots(figsize=(8.0,3.0),layout='constrained')
for i,r in enumerate(source['series']):
 ax.barh(i,r['available'],color='#17334B',height=.45,label='All fields returned / available' if i==0 else None)
 ax.barh(i,r['incomplete'],left=r['available'],color='#D18A27',height=.45,label='Incomplete factorization' if i==0 else None)
 ax.text(8,i,str(r['available']),va='center',color='white',weight='bold')
 ax.text(296,i,str(r['incomplete'])+' incomplete',va='center',color='#855010',fontsize=9)
ax.set_yticks([0,1],[r['label'] for r in source['series']]);ax.invert_yaxis()
ax.set_xlim(0,345);ax.set_xticks([0,50,100,150,200,250,292]);ax.set_xlabel('Parameter records (292 per view)')
ax.spines[['top','right','left']].set_visible(False);ax.tick_params(axis='y',length=0)
ax.legend(loc='lower left',bbox_to_anchor=(0,1.02),frameon=False,ncol=2,fontsize=8)
fig.savefig(OUT/'availability.svg',format='svg');fig.savefig(OUT/'availability.png',dpi=150)
svg_path=OUT/'availability.svg'
svg_path.write_text('\n'.join(line.rstrip() for line in svg_path.read_text().splitlines())+'\n')
plt.close(fig)
