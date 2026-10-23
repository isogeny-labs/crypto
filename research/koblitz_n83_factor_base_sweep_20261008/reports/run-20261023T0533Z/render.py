"""Lay out native recorded values; no experiment execution or result calculation."""
import json
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import typst
root = Path(__file__).resolve().parent
records = json.loads((root/'data/reclamation.json').read_text())['snapshots']
fig, axes = plt.subplots(2, 1, figsize=(9, 7), constrained_layout=True)
for field, label in [('owned_capacity_bytes','Owned Vec capacities'), ('arena_capacity_bytes','Arena capacity'), ('arena_used_bytes','Arena used'), ('worker_rss_bytes','Worker RSS')]:
    axes[0].plot([r['conflicts'] for r in records], [r[field] for r in records], label=label)
axes[0].set(xlabel='Conflicts', ylabel='Recorded bytes', title='K2048-c3: storage and worker residency (152 snapshots)')
axes[0].legend(fontsize=9)
axes[0].grid(alpha=.2)
axes[1].step([r['conflicts'] for r in records], [r['arena_reclaimed_words'] for r in records], where='post')
axes[1].set(xlabel='Conflicts', ylabel='Cumulative words removed', title='Recorded arena reclamation; n=1 native query')
axes[1].grid(alpha=.2)
fig.savefig(root/'storage.svg')
fig.savefig(root/'storage.png', dpi=150)
plt.close(fig)
typst.compile(str(root/'report.typ'), output=str(root/'report.pdf'))
