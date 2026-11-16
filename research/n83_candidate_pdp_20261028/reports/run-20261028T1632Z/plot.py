from pathlib import Path
import json
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
p=Path(__file__).resolve().parent;d=json.loads((p/'plot-data.json').read_text());h=d['hilbert_function']
f,ax=plt.subplots(figsize=(8,3.4));ax.plot(range(len(h)),h,'o-',markersize=3,color='#176583')
ax.axvline(d['dreg_exact'],color='#a74830',linestyle='--',label='First zero: '+str(d['dreg_exact']))
ax.set_yscale('symlog',linthresh=1);ax.set_ylim(bottom=0);ax.set_xlabel('Homogeneous degree');ax.set_ylabel('Exact quotient dimension (symlog)')
ax.set_title('Same original-top Hilbert function for six retained prime m=2 inputs')
ax.legend(frameon=False);ax.spines[['top','right']].set_visible(False);f.tight_layout();f.savefig(p/'hilbert.svg');plt.close(f)
q=p/'hilbert.svg';q.write_text('\n'.join(x.rstrip() for x in q.read_text().splitlines())+'\n')
