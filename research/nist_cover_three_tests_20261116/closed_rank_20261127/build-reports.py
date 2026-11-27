#!/usr/bin/env python3
"""Render recorded values only; no curve, group, polynomial or rank arithmetic."""
import json
from pathlib import Path
import subprocess

import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt

ROOT = Path(__file__).resolve().parent
plt.rcParams.update({'svg.fonttype': 'none', 'font.size': 10})

def plot(destination, labels, values, title, ylabel):
    fig, axis = plt.subplots(figsize=(6.6, 2.6), layout='constrained')
    bars = axis.bar(labels, values, color=['#315a8c', '#759ab5', '#54856d'][:len(labels)])
    if len(labels) <= 6:
        axis.bar_label(bars, padding=3)
    else:
        axis.tick_params(axis='x', labelrotation=60, labelsize=8)
    axis.set_ylabel(ylabel)
    axis.set_title(title, fontsize=11)
    axis.set_ylim(0, max(values, default=1)*1.22 or 1)
    for suffix in ('svg', 'pdf'):
        fig.savefig(destination.with_suffix('.'+suffix))
    plt.close(fig)

def pdf(source):
    subprocess.run(['pandoc', source.name, '--from', 'markdown+tex_math_dollars',
                    '--pdf-engine=pdflatex', '-V', 'colorlinks=true',
                    '-V', 'geometry:margin=0.65in', '-V', 'fontsize=10pt',
                    '--resource-path=.', '-o', source.with_suffix('.pdf').name],
                   cwd=source.parent, check=True)

attempts = {
    'pilot-v1': 'Fibre pilot recovered2 rational points over O and5 over signed old column0; original witnesses retained.',
    'full-v1': 'All226 signed target fibres completed in15 batches;652 rational curve points including all original226.',
    'sections-old-v1': 'Four native blocks recovered seven complete line sections on226 points; every block retained rank6.',
    'sections-lifted-v1': 'Eleven blocks covered212878 pair/tangent attempts on652 points; only the same seven sections, rank6.',
    'audit-old-v1': 'Strict build failed: missing iostream header. No mathematical worker ran. Original diagnostics retained.',
    'audit-old-v2': 'Corrected strict build and independent audit passed. Full rank108, auxiliary102, closed6, gain0.',
    'audit-lifted-v1': 'Independent audit of the lifted pool passed. Full rank108, auxiliary102, closed6, gain0.',
    'norms-v1': 'Strict build failed: audit::SSL/OpenSSL type ambiguity and misleading indentation. No mathematical worker ran.',
    'norms-v2': 'Both signed-norm domains passed. Old101926 pairs:7rows, rank6. Augmented369370 pairs:109rows including zero, full108, auxiliary102, closed6, gain0.',
}

for name, outcome in attempts.items():
    raw = ROOT/'evidence'/name
    destination = ROOT/'reports'/name
    destination.mkdir(parents=True, exist_ok=True)
    calls = sorted((raw/'calls').iterdir())
    entries = []
    for call in calls:
        entries.append({'call': call.name,
                        'exit': int((call/'exit-code.txt').read_text()),
                        **json.loads((call/'cost.json').read_text())})
    records = {'attempt': name, 'calls': entries,
               'wrapper': json.loads((raw/'wrapper-stderr-and-cost.txt').read_text()),
               'source_commit': (raw/'source-commit.txt').read_text().strip()}
    (destination/'recorded-values.json').write_text(json.dumps(records, indent=2)+'\n')
    # Times are resource receipts; labels distinguish builds from mathematical workers.
    if len(entries) >= 2:
        plot(destination/'calls', [r['call'].replace('batch-', 'b') for r in entries],
             [r['wall_seconds'] for r in entries], f'{name}: one execution per call',
             'Recorded wall seconds')
    else:
        dot = 'digraph G {rankdir=LR;node[shape=box];a[label="Frozen sources and inputs"];b[label="Strict native build"];c[label="Build failure; no worker"];a->b->c;}\n'
        (destination/'calls.dot').write_text(dot)
        for suffix in ('svg', 'pdf'):
            subprocess.run(['dot', '-T'+suffix, 'calls.dot', '-o', 'calls.'+suffix], cwd=destination, check=True)
    table = '\n'.join(f"|{r['call']}|{r['exit']}|{r['wall_seconds']}|" for r in entries)
    text = f'''# Native attempt {name}

{outcome} [pcc:unverified]

![Recorded calls for {name}. One execution each; no repeated-sample interval. Failed builds use a workflow diagram because no comparable mathematical executions exist.](calls.pdf)

| Call | Exit | Recorded wall seconds |
|---|---|---|
{table}

Complete wrapper: {records['wrapper']['wall_seconds']} wall seconds. These are
resource receipts, not performance comparisons. Host: Linux6.18.57 x86_64;
exact allowed CPU is in the attempt's resource-envelope.txt. One CPU,
1GiB address-space cap,300s strict build and600s per native worker. No
random seeds or statistical sampling: fixed inputs and bounded deterministic
enumerations. Sample size is one execution per listed call. No timing spread
was measured.

Exact command/configuration: frozen source/run script and protocol under
../../evidence/{name}/source/. Source ancestor: `{records['source_commit']}`.
The manifest identifies the actual frozen source, including uncommitted
changes at execution time. Inputs, stdout, stderr, terminal exits, cost
receipts and immutable subject hashes are under ../../evidence/{name}/.
See ../../REPORT.md for controls, domain limits, primary references,
interpretation and follow-up; no positive additional rank is established.
'''
    (destination/'REPORT.md').write_text(text)
    pdf(destination/'REPORT.md')

old = json.loads((ROOT/'evidence/norms-v2/calls/old/stdout.json').read_text())
aug = json.loads((ROOT/'evidence/norms-v2/calls/augmented/stdout.json').read_text())
data = {'old_norm': {k:old[k] for k in ('full_rank','auxiliary_rank','closed_rank')},
        'augmented_norm': {k:aug[k] for k in ('full_rank','auxiliary_rank','closed_rank')}}
(ROOT/'rank-values.json').write_text(json.dumps(data, indent=2)+'\n')
fig, axis = plt.subplots(figsize=(6.8, 3), layout='constrained')
for position, key in enumerate(('full_rank','auxiliary_rank','closed_rank')):
    x = [position*0.25, 1+position*0.25]
    bars = axis.bar(x, [old[key],aug[key]], width=0.22, label=key.replace('_',' '))
    axis.bar_label(bars, padding=2)
axis.set_xticks([0.25,1.25], ['Original225-column domain','Existing429-column domain'])
axis.set_ylabel('Recorded rank over subgroup field')
axis.set_ylim(0,132)
axis.legend(loc='upper left')
for suffix in ('svg','pdf'):
    fig.savefig(ROOT/('ranks.'+suffix))
plt.close(fig)
pdf(ROOT/'REPORT.md')
for svg in [ROOT/'ranks.svg', *(ROOT/'reports').glob('*/calls.svg')]:
    svg.write_text('\n'.join(line.rstrip() for line in svg.read_text().splitlines())+'\n')
