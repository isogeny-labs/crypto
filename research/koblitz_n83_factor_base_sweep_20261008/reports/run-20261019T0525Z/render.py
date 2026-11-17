"""Lay out recorded native observations only. No solver or research computation."""
import json
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch
import typst

p = Path(__file__).resolve().parent
d = json.loads((p / 'figure-data.json').read_text())
peer = json.loads((p / 'peer-figure-data.json').read_text())

fig, axes = plt.subplots(len(d['controls']), 1, figsize=(9, 7.5))
for ax, arm in zip(axes, d['controls']):
    events = [v for v in arm['sat_trace']['events'] if v['elapsed_ms'] > 0]
    ax.barh([v['phase'].replace('_', ' ') for v in events],
            [v['elapsed_ms'] for v in events], color=arm['color'])
    ax.invert_yaxis()
    ax.set_xscale('log')
    ax.set_xlabel('Elapsed ms from this solve entry (log)')
    ax.set_title(arm['label'] + '\n' + arm['sat_trace']['status'], fontsize=10)
fig.suptitle('Native solver checkpoints | distinct controls, n=1 each, L0')
fig.tight_layout(rect=(0, .04, 1, .92))
fig.text(.5, .01, d['run_id'] + ' | checkpoints are not exclusive phase durations', ha='center', fontsize=8)
for ext in ['svg', 'png']:
    fig.savefig(p / f'phases.{ext}', dpi=150)
plt.close(fig)

fig, ax = plt.subplots(figsize=(9, 3.2))
for i, arm in enumerate(d['controls']):
    ax.barh(i, arm['outer_wall_ms'], color=arm['color'],
            hatch='//' if arm['wall_censored'] else None)
ax.set_yticks(range(len(d['controls'])), [v['label'] for v in d['controls']])
ax.invert_yaxis()
ax.set_xlabel('Supervised outer wall milliseconds, L0; n=1 per distinct workload')
ax.set_title('Observed control walls | three uncensored observations')
fig.tight_layout()
for ext in ['svg', 'png']:
    fig.savefig(p / f'outer-wall.{ext}', dpi=150)
plt.close(fig)

fig, axes = plt.subplots(1, 2, figsize=(10, 3.4))
for ax, metric, title in zip(axes, ['sat_elapsed_ms', 'container_wall_ms'], ['Reported SAT interval', 'Reported container wall']):
    ax.bar([a['label'].upper() for a in peer['arms']], [a[metric] for a in peer['arms']],
           color=['#477ea6', '#9771a9', '#9771a9', '#477ea6'])
    ax.set_ylabel('Milliseconds')
    ax.set_title(title)
    ax.tick_params(axis='x', labelrotation=20)
fig.suptitle('Separate peer ABBA receipt | c7i.xlarge; two repeats per K')
fig.tight_layout(rect=(0, .07, 1, .89))
fig.text(.5, .01, peer['job_id'] + ' | native worker; original Bash/Python audit provenance', ha='center', fontsize=8)
for ext in ['svg', 'png']:
    fig.savefig(p / f'peer-paired.{ext}', dpi=150)
plt.close(fig)

fig, ax = plt.subplots(figsize=(10, 2.3))
ax.set_xlim(0, 10); ax.set_ylim(0, 3); ax.axis('off')
for x, text in [(.15, 'Approved 49,012,484 B bundle\nS3 fresh download verified\n308 payload checks pass'),
                (3.6, 'New native boundary trace\nPinned and free selectors\n5000 conflicts / 180 s each'),
                (7.05, 'Actual selectors replayed\n787 conflicts, verified SAT\nNo cold winner selected')]:
    ax.add_patch(FancyBboxPatch((x, .8), 2.8, 1.25, boxstyle='round,pad=.04', fc='#edf3f7', ec='#477ea6'))
    ax.text(x+1.4, 1.425, text, ha='center', va='center', fontsize=10)
for x in [3, 6.45]:
    ax.annotate('', xy=(x+.5, 1.425), xytext=(x, 1.425), arrowprops={'arrowstyle':'->'})
ax.set_title('Publication and continuation | separate preserved evidence rounds')
fig.tight_layout()
for ext in ['svg', 'png']:
    fig.savefig(p / f'execution.{ext}', dpi=150)
plt.close(fig)
typst.compile(str(p/'report.typ'), output=str(p/'report.pdf'),
              font_paths=[str(Path(matplotlib.__file__).parent/'mpl-data/fonts/ttf')])
