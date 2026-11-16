import json,pathlib,collections
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import typst
p=pathlib.Path(__file__).resolve().parent
rows=json.loads((p.parents[1]/'validation-radix-development/controls.json').read_text())['rows']
counts={q:{'maximal':sum(r['p']==q and r['explicit_rank']==r['maximal_rank'] for r in rows),'nonmaximal':sum(r['p']==q and r['explicit_rank']!=r['maximal_rank'] for r in rows)} for q in (2,3,101)}
(p/'plot-data.json').write_text(json.dumps(counts,indent=2)+'\n')
fig,ax=plt.subplots(figsize=(7,2.5))
x=list(range(3));a=[counts[q]['maximal'] for q in counts];b=[counts[q]['nonmaximal'] for q in counts]
ax.bar(x,a,label='Explicit rank equals maximal rank',color='#237a57');ax.bar(x,b,bottom=a,label='Nonmaximal rank',color='#c66438');ax.set_xticks(x,['F2','F3','F101']);ax.set_ylabel('Recorded matrix checks');ax.legend(fontsize=8,loc='lower right');ax.set_title('29 small radix partitions; all target degrees')
fig.tight_layout();fig.savefig(p/'rank-controls.svg');plt.close(fig)
svg=p/'rank-controls.svg'
svg.write_text('\n'.join(line.rstrip() for line in svg.read_text().splitlines())+'\n')
typst.compile(str(p/'report.typ'),output=str(p/'report.pdf'))
