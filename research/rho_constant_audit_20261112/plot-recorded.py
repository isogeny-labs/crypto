"""Layout recorded native means/intervals only; no statistical computation."""
from pathlib import Path
import json,re,datetime
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
r=Path(__file__).resolve().parent
fields={c['slug']:c['field_bits'] for c in json.loads((r/'evidence/build-provenance.json').read_text())['curves']}
for name in ['preflight','main']:
 session=json.loads((r/'sessions'/name/'session.json').read_text());stamp=datetime.datetime.fromtimestamp(session['started_unix_ms']/1000,datetime.timezone.utc).strftime('%Y%m%dT%H%MZ');out=r/'reports'/('run-'+stamp);out.mkdir(parents=True,exist_ok=True)
 rows=[]
 for line in(r/'analysis'/name/'table.stdout').read_text().splitlines()[2:]:
  if not line.startswith('|'):continue
  c=[x.strip()for x in line.split('|')[1:-1]]
  if len(c)<8:continue
  interval=[float(v)for v in re.findall(r'\d+\.\d+',c[7])]
  rows.append(dict(arm=c[1],curve=c[3],field_bits=fields[c[3]],verified=c[5],mean_s=float(c[6]),interval=interval))
 d={'session_id':session['session_id'],'source':'native ecbench table.stdout; values and intervals copied as printed, no recomputed estimates','unit':'counted GAE / sqrt(r)','rows':rows,'ideal_reference':0.8862269254527579,'scope':'Finite fixed inputs. Reference is standard-negation average model, not universal lower bound. Scientific clocks invalid.'}
 (out/'plot-data.json').write_text(json.dumps(d,indent=2)+'\n')
 fig,ax=plt.subplots(figsize=(7.5,3.65));styles=[('rho-neg',-.18,'#22669c','negation'),('rho-plain',0,'#bf6d20','plain'),('rho-neg-aa',.18,'#397a54','same-seed control')]
 for arm,offset,color,label in styles:
  rs=[x for x in rows if x['arm']==arm];xs=[x['field_bits']+offset for x in rs];ys=[x['mean_s']for x in rs]
  if all(len(x['interval'])==2 for x in rs):
   # Coordinate distances to the already recorded interval endpoints.
   err=[[x['mean_s']-x['interval'][0]for x in rs],[x['interval'][1]-x['mean_s']for x in rs]]
   ax.errorbar(xs,ys,yerr=err,fmt='o',capsize=3,color=color,label=label,markersize=4)
  else:ax.plot(xs,ys,'o',color=color,label=label,markersize=4)
 ax.axhline(d['ideal_reference'],color='#555555',linestyle='--',linewidth=1,label='standard-negation reference 0.886')
 ax.set_ylim(bottom=0);ax.set_xticks([14,16,18,20]);ax.set_xlabel('Registered prime-roster field size (bits)');ax.set_ylabel('Total counted GAE / sqrt(r)');ax.grid(axis='y',alpha=.22);ax.legend(fontsize=8,loc='upper right');ax.set_title('Frozen main: 48 measured runs per curve and arm'if name=='main'else'Preflight: one fixed target per curve and arm',fontsize=11)
 fig.tight_layout();fig.savefig(out/'means.svg');fig.savefig(out/'means.pdf');plt.close(fig)
 svg=out/'means.svg';svg.write_text('\n'.join(line.rstrip()for line in svg.read_text().splitlines())+'\n')
 print(json.dumps({'name':name,'report_dir':str(out),'session':session['session_id'],'points':len(rows)}))
