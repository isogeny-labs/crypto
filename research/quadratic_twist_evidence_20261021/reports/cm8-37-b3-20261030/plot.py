"""Presentation of native-emitted counts only; no research arithmetic."""
from pathlib import Path
import json,os
os.environ.setdefault('MPLCONFIGDIR','/tmp/qt-b3-mpl')
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
p=Path(__file__).resolve().parent
d=json.loads((p/'plot-data.json').read_text())['panels']
fig,axes=plt.subplots(1,2,figsize=(10,3.5),sharex=True)
for ax,key,label in zip(axes,['accepted_queries','verified_targets'],['Certified diagnostic queries','Verified source-target runs']):
 for role,style in zip(['main','twist','matched','pullback'],['o-','s--','^:','x-.']):
  ax.plot([v['cap']for v in d],[v[role][key]for v in d],style,label=role,markersize=3)
 ax.set_xlabel('Expanded-witness cap');ax.set_ylabel(label);ax.set_xticks(range(9));ax.grid(alpha=.2)
fig.suptitle('F37 / r19: all size-three policies, one original encoding')
fig.legend(*axes[0].get_legend_handles_labels(),loc='lower center',ncol=4,frameon=False)
fig.tight_layout(rect=[0,.08,1,.96]);fig.savefig(p/'frontier.svg');fig.savefig(p/'frontier.pdf')
