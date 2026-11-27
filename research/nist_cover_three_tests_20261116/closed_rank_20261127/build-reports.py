#!/usr/bin/env python3
"""Render recorded values only; no curve, group, polynomial or rank arithmetic."""
import json
from pathlib import Path
import subprocess
import sys

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
    'trace-v1': 'Strict build failed: integer multiplication in a boolean condition under -Werror. No mathematical worker ran.',
    'trace-v2': 'Both domains passed at targets0,kappa,2kappa. Only the formal zero row at0; no quartet at2kappa. Closed rank6 in both modes; gain0.',
    'conics-v1': 'All64pencils completed on652points:41632candidate occurrences;21supported conics, exactly known line products; full108, auxiliary102, closed6, gain0. Repeated-line multiplicity control passed.',
    'fibre-conics-v1': 'Input-copy wrapper failed before any build or mathematical worker: a stray mode argument was passed to cp. Partial snapshot and full stderr retained.',
}

selected = set(sys.argv[1:])
six_result = ROOT/'evidence/six-v1/calls/full/stdout.json'
six_exit = six_result.parent/'exit-code.txt'
if six_exit.exists() and six_exit.read_text().strip() == '0':
    six = json.loads(six_result.read_text())
    attempts['six-v1'] = (
        f"Pilot and full length-six workers passed. Full original225-column domain: "
        f"{six['unordered_triples']} triples;{six['distinct_nonzero_spanning_rows']} "
        f"nonzero spanning rows;closed rank{six['closed_rank']};gain{six['candidate_gain']}. "
        "Separate arithmetic and nonempty known-answer controls passed.")
fibre_result = ROOT/'evidence/fibre-conics-v2/calls/collect/stdout.json'
fibre_exit = fibre_result.parent/'exit-code.txt'
if fibre_exit.exists() and fibre_exit.read_text().strip() == '0':
    fibre = json.loads(fibre_result.read_text())
    attempts['fibre-conics-v2'] = (
        f"Fibre extension passed:{fibre['four_point_image_groups']} signed image groups;"
        f"{fibre['degenerate_image_groups_excluded']} degenerate groups excluded;"
        f"{fibre['pencil_count']} pencils;{fibre['candidate_occurrences']} candidate occurrences;"
        f"{fibre['supported_conics']} supported conics;closed rank{fibre['closed_rank']};"
        f"gain{fibre['candidate_gain']}. All declared controls passed.")
if selected - attempts.keys():
    raise SystemExit('Unknown attempt selected')
for name, outcome in attempts.items():
    if selected and name not in selected:
        continue
    raw = ROOT/'evidence'/name
    destination = ROOT/'reports'/name
    destination.mkdir(parents=True, exist_ok=True)
    calls = sorted((raw/'calls').iterdir())
    entries = []
    for call in calls:
        entries.append({'call': call.name,
                        'exit': int((call/'exit-code.txt').read_text()),
                        **json.loads((call/'cost.json').read_text())})
    wrapper_lines = (raw/'wrapper-stderr-and-cost.txt').read_text().splitlines()
    cost_line = next(line for line in reversed(wrapper_lines) if line.startswith('{"wall_seconds":'))
    commit_file = raw/'source-commit.txt'
    records = {'attempt': name, 'calls': entries,
               'wrapper': json.loads(cost_line),
               'source_commit': commit_file.read_text().strip() if commit_file.exists()
               else 'not captured: wrapper failed before this stage'}
    (destination/'recorded-values.json').write_text(json.dumps(records, indent=2)+'\n')
    # Times are resource receipts; labels distinguish builds from mathematical workers.
    if len(entries) >= 2:
        plot(destination/'calls', [r['call'].replace('batch-', 'b') for r in entries],
             [r['wall_seconds'] for r in entries], f'{name}: one execution per call',
             'Recorded wall seconds')
    else:
        middle = 'Strict native build' if entries else 'Input copy'
        last = 'Build failure; no worker' if entries else 'Wrapper failure; no build or worker'
        dot = f'digraph G {{rankdir=LR;node[shape=box];a[label="Source snapshot"];b[label="{middle}"];c[label="{last}"];a->b->c;}}\n'
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
allowed CPU is in resource-envelope.txt when that stage was reached. Declared
limits:one CPU,1GiB address space,300s build and600s per native worker. No
random seeds or statistical sampling: fixed inputs and bounded deterministic
enumerations. Sample size is one execution per listed call. No timing spread
was measured.

Exact command/configuration: frozen source/run script and protocol under
../../evidence/{name}/source/. Source ancestor,when captured:

```
{records['source_commit']}
```

The manifest,when reached,identifies the frozen source, including uncommitted
changes at execution time. Available inputs,stdout,stderr,terminal exits,cost
receipts and subject hashes are under ../../evidence/{name}/. A pre-build
wrapper failure has no compiler,worker or later-stage sealing receipt.
See ../../REPORT.md for controls, domain limits, primary references,
interpretation and follow-up; no positive additional rank is established.
'''
    (destination/'REPORT.md').write_text(text)
    pdf(destination/'REPORT.md')

old = json.loads((ROOT/'evidence/norms-v2/calls/old/stdout.json').read_text())
aug = json.loads((ROOT/'evidence/norms-v2/calls/augmented/stdout.json').read_text())
metrics = ('full_rank','auxiliary_rank','closed_rank')
cases = [('Norm225',old),('Norm429',aug)]
for label, relative in [('Trace429','trace-v2/calls/augmented'),
                        ('Conic64','conics-v1/calls/collect'),
                        ('Fibre conics','fibre-conics-v2/calls/collect'),
                        ('Six225','six-v1/calls/full')]:
    source = ROOT/'evidence'/relative/'stdout.json'
    terminal = source.parent/'exit-code.txt'
    if terminal.exists() and terminal.read_text().strip() == '0':
        cases.append((label,json.loads(source.read_text())))
data = {label:{k:record[k] for k in metrics} for label,record in cases}
(ROOT/'rank-values.json').write_text(json.dumps(data, indent=2)+'\n')
fig, axis = plt.subplots(figsize=(7.2, 3), layout='constrained')
for position, key in enumerate(metrics):
    x = [i+position*0.25 for i in range(len(cases))]
    bars = axis.bar(x, [record[key] for _,record in cases], width=0.22, label=key.replace('_',' '))
    axis.bar_label(bars, padding=2, fontsize=7 if len(cases)>4 else 10)
axis.set_xticks([i+0.25 for i in range(len(cases))], [label for label,_ in cases])
axis.tick_params(axis='x', labelsize=9)
axis.set_ylabel('Recorded rank over subgroup field')
axis.set_ylim(0,132)
axis.legend(loc='upper left',bbox_to_anchor=(0,1.18),ncol=3,fontsize=9)
for suffix in ('svg','pdf'):
    fig.savefig(ROOT/('ranks.'+suffix))
plt.close(fig)
pdf(ROOT/'REPORT.md')
for svg in [ROOT/'ranks.svg', *(ROOT/'reports').glob('*/calls.svg')]:
    svg.write_text('\n'.join(line.rstrip() for line in svg.read_text().splitlines())+'\n')
