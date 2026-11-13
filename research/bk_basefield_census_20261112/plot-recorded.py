"""Plot frozen native integer counts only; no statistical recomputation."""
from pathlib import Path
import json,datetime
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt

s=Path(__file__).resolve().parent
for mode in ['preflight','main']:
 stamp=datetime.datetime.fromisoformat((s/f'runs/{mode}/reference.start').read_text().strip().replace('Z','+00:00')).strftime('%Y%m%dT%H%MZ')
 out=s/'reports'/('run-'+stamp);out.mkdir(parents=True,exist_ok=True)
 rows=[json.loads(x)for x in(s/f'analysis/{mode}/analysis.jsonl').read_text().splitlines()]
 data=[x for x in rows if x['kind']=='full_by_prime']
 (out/'plot-data.json').write_text(json.dumps({'source':f'analysis/{mode}/analysis.jsonl','rows':data,'scope':'Exact weighted coefficient-model cases with r² base-field points killedbyr. Distinct eligibility sets by r; no iid/probability interpretation.'},indent=2)+'\n')
 fig,ax=plt.subplots(figsize=(7.3,2.65))
 bars=ax.bar([str(x['r'])for x in data],[x['models']for x in data],color='#28638b',width=.62)
 ax.set_yscale('log');ax.set_ylim(bottom=1,top=max(x['models']for x in data)*10)
 ax.bar_label(bars,labels=[format(x['models'],',')for x in data],padding=4,fontsize=8)
 ax.margins(y=.23);ax.set_xlabel('Prime r');ax.set_ylabel('Coefficient models (log scale)');ax.set_axisbelow(True);ax.grid(axis='y',alpha=.22)
 ax.set_title('Exactly r² rational r-torsion points, including identity',fontsize=10,pad=12)
 fig.tight_layout();fig.savefig(out/'full-torsion.svg');fig.savefig(out/'full-torsion.pdf');plt.close(fig)
 p=out/'full-torsion.svg';p.write_text('\n'.join(x.rstrip()for x in p.read_text().splitlines())+'\n')
 print(json.dumps({'mode':mode,'report_dir':str(out),'recorded_bars':len(data)}))
