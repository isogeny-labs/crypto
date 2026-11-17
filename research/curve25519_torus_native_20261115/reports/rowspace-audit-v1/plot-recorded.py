# Layout only: copy Rust-recorded fields; no cryptanalytic computation.
from pathlib import Path
import json
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
root=Path(__file__).resolve().parent
native=json.loads((root/'../../rowspace-audit-v1/native-v1/controls.json').resolve().read_text())
recorded=[r for r in native['cases'] if r['id'].startswith('original-')]
(root/'native-counts.json').write_text(json.dumps(recorded,indent=2)+'\n')
plt.rcParams['svg.fonttype']='none'
fig,axes=plt.subplots(1,2,figsize=(7.2,2.5),layout='constrained')
for ax,row in zip(axes,recorded):
    s=row['summary'];values=[s['original_input_rank'],s['total_input_rank']]
    bars=ax.bar(['Original rows','Full row space'],values,color=['#627b96','#21918c'])
    ax.bar_label(bars,padding=3);ax.set_ylim(0,max(values)*1.25)
    ax.set_title(row['id']);ax.set_ylabel('Space dimension');ax.tick_params(axis='x',labelsize=8)
fig.suptitle('Original six-member circuit: no added input-space dimension',fontsize=11)
fig.savefig(root/'ranks.svg');fig.savefig(root/'ranks.pdf')
p=root/'ranks.svg';p.write_text('\n'.join(line.rstrip() for line in p.read_text().splitlines())+'\n')
