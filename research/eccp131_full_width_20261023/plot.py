#!/usr/bin/env python3
"""Quantitative graph from the retained auxiliary-order output."""
from collections import Counter
from pathlib import Path
import re
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt

HERE = Path(__file__).parent
lines = (HERE/'runs/search.stdout').read_text().splitlines()
orders = [(int(m[1]),int(m[2])) for line in lines
          if (m := re.fullmatch(r'ORDER (\d+) (\d+)',line))]
assert len(orders) == 301 and [b for b,_ in orders] == list(range(1,302))
counts = Counter((n & -n).bit_length()-1 for _,n in orders)
fig,ax = plt.subplots(figsize=(7,3.5))
ax.bar(sorted(counts),[counts[k] for k in sorted(counts)],color='#4477aa')
ax.axvline(8,color='#bb4422',linestyle='--',label='order-256 divisibility')
ax.set(xlabel='v₂(#E′) among searched curves',ylabel='Candidate curves (count)',
       title='Exact ECCp-131 field: first accepted witness at candidate 301')
ax.legend(loc='upper right')
fig.tight_layout()
fig.savefig(HERE/'order_valuations.svg')
print(dict(sorted(counts.items())))
