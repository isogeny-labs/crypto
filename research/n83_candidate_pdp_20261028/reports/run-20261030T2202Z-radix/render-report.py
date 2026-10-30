import pathlib,json,matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import typst
p=pathlib.Path(__file__).resolve().parent;s=p.parents[1];v=json.loads((s/'evidence/radix-census-r2/summary.json').read_text());rows=v['rows'];data=[{'socle_degree':r['socle_degree'],'degree':r['first_vanishing_degree'] if r['first_vanishing_degree'] is not None else r['lower_bound'],'status':'exact' if r['first_vanishing_degree'] is not None else 'lower bound'} for r in rows];(p/'plot-data.json').write_text(json.dumps(data,indent=2)+'\n')
fig,ax=plt.subplots(figsize=(7.2,2.5))
for status,marker,color in [('exact','o','#177a57'),('lower bound','^','#b86c24')]:
 a=[x for x in data if x['status']==status];ax.scatter([x['socle_degree'] for x in a],[x['degree'] for x in a],s=18,marker=marker,color=color,label=f'{status} ({len(a)} records)',alpha=.7)
ax.axhline(42,color='#444',linestyle='--',linewidth=.8,label='Original all-binary degree 42');ax.set_xscale('log');ax.set_yscale('log');ax.set_xlabel('Selector-block socle degree D');ax.set_ylabel('Degree / lower bound');ax.legend(fontsize=8);fig.tight_layout();fig.savefig(p/'radix-degrees.svg');plt.close(fig)
f=p/'radix-degrees.svg';f.write_text('\n'.join(x.rstrip() for x in f.read_text().splitlines())+'\n')
typst.compile(str(p/'report.typ'),output=str(p/'report.pdf'))
