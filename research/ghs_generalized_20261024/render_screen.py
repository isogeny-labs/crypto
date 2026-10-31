"""Render recorded generalized-census counts; no cryptanalytic computation here."""
from pathlib import Path
import json
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

root=Path(__file__).resolve().parent
fig,axes=plt.subplots(4,1,figsize=(10,12),constrained_layout=True)
for ax,(degree,a,b) in zip(axes,((6,0,"20"),(8,0,"20"),(10,0,"20"),(12,1,"200"))):
    receipt=json.loads((root/'results'/f'screen-n{degree}-a{a}-b{b}.json').read_text())
    for row in receipt['base_rows']:
        q=row['base_degree']; hist=sorted((int(g),num) for g,num in row['genus_histogram'].items())
        ax.scatter([q]*len(hist),[g for g,_ in hist],s=[max(12,min(400,n*2)) for _,n in hist],label=f'base GF(2^{q})')
    ax.set(xlabel='Base-field degree',ylabel='Recorded descent genus',title=f'Absolute degree {degree}: all {receipt["c_fraction_numerator"]} nonzero c values')
    ax.set_xticks([x['base_degree'] for x in receipt['base_rows']]);ax.grid(alpha=.2)
fig.suptitle('Generalized GHS screen: genus profiles (marker area capped at 400 for legibility)')
fig.savefig(root/'genus-profiles.svg')
fig.savefig(root/'genus-profiles.png',dpi=150)
