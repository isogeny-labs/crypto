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
    'partial-v1': 'All212843distinct pair/tangent lines checked:212836at old support degree2,seven at degree4,none at degree3. No old flex. Hidden-support extraction and corruption controls pass;closed rank6.',
    'partial-audit-v1': 'Separate GMP prime-field arithmetic reproduces every ordered pair-line key,tangent contact and retained support. Geometry controls pass;closed rank6. This stage does not re-prove the parent elliptic map.',
    'graph-v1': 'Complete rational two-residual graph:106421new partial edges;106319rows peeled;102two-row core components. Full/auxiliary/closed ranks106427/106421/6,gain0. Eight graph fixtures match dense ranks;two invalid singleton witnesses reject.',
    'mixed-v1': 'All101926aggregate pairs and212842individual residual-image slots checked:101701distinct signed aggregate labels,zero cross-aliases. Disjoint-block ranks208128/208122/6are derived from the saved graph and private aggregate pivots;no giant dense matrix was formed. Cached/full map,membership and matrix controls pass.',
    'divided-v1': 'Strict build failed: unqualified helper SSL collides with the OpenSSL global typedef. No mathematical worker ran. Frozen source and diagnostics retained.',
    'divided-v2': 'All three denominators1,2,3pass:101926pairs and226checked divisions each;305103distinct xlabels,zero unequal-expression aliases and zero candidate rows. Actual augmented ranks108/102/6,gain0. Nonempty G,2Gfixture finds primitive(2,-1),rank1;three identity branches and all corruption controls pass.',
    'mixed-divided-v1': 'Both new divided families pass:203852pairs,203402distinct signed labels,212842individual residual-image slots,and zero cross-aliases. The full individual image stream matches mixed-v1. Derived disjoint-block ranks309829/309823/6,gain0;no giant dense matrix formed. All declared controls pass.',
    'expanded-conics-v1': 'First16new signed-fibre conic pencils pass on1434actual points:22944candidate occurrences,zero supported conics,computed ranks236/230/6,gain0. All31qualifying signed groups admit general-position anchors. Separate same-collector pencil finds one nonzero conic;21direct products span6;all controls pass.',
    'all-expanded-conics-v1': 'Wrapper failed during input copy before compiler or mathematical worker. A broad path-suffix replacement added an unintended third cp argument. Frozen partial snapshot and exact stderr retained;no scientific scan ran.',
}

figure_only = '--figure-only' in sys.argv[1:]
selected = set(sys.argv[1:]) - {'--figure-only'}
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
for anchor_attempt in ('anchors-v1','anchors-expanded-v1'):
    source=ROOT/'evidence'/anchor_attempt/'calls/scan/stdout.json'
    terminal=source.parent/'exit-code.txt'
    if terminal.exists() and terminal.read_text().strip()=='0':
        anchor=json.loads(source.read_text())
        attempts[anchor_attempt]=(
            f"Actual fibre expansion passed:{anchor['selected_sections']}selected sections,"
            f"{anchor['new_auxiliary_columns']}new auxiliary image columns,"
            f"{anchor['new_curve_points']}new rational curve points,"
            f"{anchor['expanded_curve_pool']}total curve points. "
            f"{anchor['signed_norm_pairs']}signed pairs yield"
            f"{anchor['distinct_norm_rows_including_zero']}necessary image rows;"
            f"computed full/auxiliary/closed ranks{anchor['full_rank']}/"
            f"{anchor['auxiliary_rank']}/{anchor['closed_rank']},gain{anchor['candidate_gain']}. "
            f"Reused{anchor.get('reused_fibres',0)}frozen complete fibres;"
            f"solved{anchor.get('freshly_solved_fibres',anchor['new_auxiliary_columns'])}fresh fibres. "
            "Both elliptic streams/joins,actual witness recovery and corruption controls pass.")
all_conics_source=ROOT/'evidence/all-expanded-conics-v2/calls/scan/stdout.json'
all_conics_exit=all_conics_source.parent/'exit-code.txt'
if all_conics_exit.exists() and all_conics_exit.read_text().strip()=='0':
    all_conics=json.loads(all_conics_source.read_text())
    attempts['all-expanded-conics-v2']=(
        f"Complete same-image four-subset family passes:"
        f"{all_conics['qualifying_groups']}signed groups,"
        f"{all_conics['combined_pencil_count']}total general-position pencils;"
        f"{all_conics['reused_pencils']}reused empty pencils and"
        f"{all_conics['pencil_count']}fresh scans. "
        f"{all_conics['combined_candidate_occurrences']}combined candidate occurrences,"
        f"{all_conics['supported_conics']}new supported conics;computed ranks"
        f"{all_conics['full_rank']}/{all_conics['auxiliary_rank']}/"
        f"{all_conics['closed_rank']},gain{all_conics['candidate_gain']}. All controls pass.")
mixed_sign_source=ROOT/'evidence/mixed-sign-conics-v1/calls/combine/stdout.json'
mixed_sign_exit=mixed_sign_source.parent/'exit-code.txt'
if mixed_sign_exit.exists() and mixed_sign_exit.read_text().strip()=='0':
    mixed_sign=json.loads(mixed_sign_source.read_text())
    attempts['mixed-sign-conics-v1']=(
        f"Native plan/batch/combine passes:{mixed_sign['mixed_sign_pencils']}new mixed-sign pencils,"
        f"{mixed_sign['batch_count']}complete batches,"
        f"{mixed_sign['same_sign_pencils_skipped']}completed same-sign cases skipped;"
        f"{mixed_sign['candidate_occurrences']}candidate occurrences,"
        f"{mixed_sign['supported_conics']}supported conics. Computed core ranks"
        f"{mixed_sign['core_full_rank']}/{mixed_sign['core_auxiliary_rank']}/"
        f"{mixed_sign['closed_rank']};restored full/auxiliary ranks"
        f"{mixed_sign['full_rank']}/{mixed_sign['auxiliary_rank']}derive by adding102private ranks. "
        "Coverage,replay,nonempty and corruption controls pass.")
balanced_source=ROOT/'evidence/expanded-balanced-v1/calls/scan/stdout.json'
balanced_exit=balanced_source.parent/'exit-code.txt'
if balanced_exit.exists() and balanced_exit.read_text().strip()=='0':
    balanced=json.loads(balanced_source.read_text())
    attempts['expanded-balanced-v1']=(
        "Expanded481-image bound passes:464166pairs at each new target0and2kappa;"
        f"{balanced['domains'][0]['distinct_rows_including_zero']}row at0and"
        f"{balanced['domains'][1]['distinct_rows_including_zero']}rows at2kappa. "
        "Zero patterns are formal;balanced conic pattern excluded at this domain. "
        f"Imported{balanced['imported_kappa_rows']}kappa rows without reenumeration. Computed core"
        f"{balanced['core_full_rank']}/{balanced['core_auxiliary_rank']}/{balanced['closed_rank']};"
        f"restored{balanced['full_rank']}/{balanced['auxiliary_rank']}counts derive by adding102private ranks. All controls pass.")
mixed_image_source=ROOT/'evidence/mixed-image-conics-v1/calls/combine/stdout.json'
if (mixed_image_source.parent/'exit-code.txt').exists() and (mixed_image_source.parent/'exit-code.txt').read_text().strip()=='0':
    mixed_image=json.loads(mixed_image_source.read_text())
    attempts['mixed-image-conics-v1']=(
        f"All{mixed_image['mixed_image_pencils']}mixed-image pencils pass:"
        f"{mixed_image['candidate_occurrences']}candidate occurrences,"
        f"{mixed_image['known_product_conics']}supported known products,"
        f"{mixed_image['additional_conics']}additional conics and"
        f"{mixed_image['independent_additional_rows']}independent added rows. "
        "Exact polynomial/parent-row certificates,coverage and replay controls pass. "
        f"Computed core{mixed_image['core_full_rank']}/{mixed_image['core_auxiliary_rank']}/"
        f"{mixed_image['closed_rank']};restored236/230counts derive by adding102private ranks. "
        "Separate numerical rank7control is excluded from actual data.")
full_image_source=ROOT/'evidence/full-image-bound-v1/calls/scan/stdout.json'
if (full_image_source.parent/'exit-code.txt').exists() and (full_image_source.parent/'exit-code.txt').read_text().strip()=='0':
    full_image=json.loads(full_image_source.read_text())
    attempts['full-image-bound-v1']=(
        "Complete685-image union passes with all460auxiliaries active. "
        +';'.join(f"target{d['target_multiple']}: {d['pairs']}pairs,"
                 f"{d['distinct_rows_including_zero']}rows" for d in full_image['domains'])+'. '
        f"Imported{full_image['imported_kappa_union_rows']}distinct kappa rows from both inventories;"
        f"{full_image['new_independent_rows']}new independent rows. Directly computed ranks"
        f"{full_image['full_rank']}/{full_image['auxiliary_rank']}/{full_image['closed_rank']};"
        "no private-block restoration formula used. Both group engines,joins,basis "
        "reconstruction and corruption controls pass. Separate numerical rank7control "
        "is excluded from actual data.")
extension_source=ROOT/'evidence/mixed-image-extension-v1/calls/combine/stdout.json'
if (extension_source.parent/'exit-code.txt').exists() and (extension_source.parent/'exit-code.txt').read_text().strip()=='0':
    extension=json.loads(extension_source.read_text())
    attempts['mixed-image-extension-v1']=(
        f"Complete old-pair strata for seeds 0,32,64,96 pass: {extension['raw_choices']} raw choices, "
        f"{extension['degenerate_choices']} degenerate, {extension['completed_choices_skipped']} completed "
        f"tuples skipped, {extension['excluded_image_choices']} image-pattern exclusions, "
        f"{extension['mixed_image_pencils']} new pencils. "
        f"{extension['candidate_occurrences']} candidate occurrences recover "
        f"{extension['known_product_conics']} distinct known products and "
        f"{extension['additional_conics']} additional conics. "
        "Native prior-plan binding, exact domain partition, geometry replay and parent-row "
        "certificates pass. Computed core "
        f"{extension['core_full_rank']}/{extension['core_auxiliary_rank']}/{extension['closed_rank']}; "
        f"restored full/auxiliary {extension['full_rank']}/{extension['auxiliary_rank']} are derived. "
        "Separate numerical rank-seven control is excluded from actual data.")
four_line_source=ROOT/'evidence/four-line-conics-v1/calls/combine/stdout.json'
if (four_line_source.parent/'exit-code.txt').exists() and (four_line_source.parent/'exit-code.txt').read_text().strip()=='0':
    four_line=json.loads(four_line_source.read_text())
    attempts['four-line-conics-v1']=(
        f"Product-free four-line selection completes {four_line['raw_choices']} raw choices: "
        f"{four_line['mixed_image_pencils']} admitted pencils, "
        f"{four_line['degenerate_choices']} degenerate and "
        f"{four_line['excluded_image_choices']} image-pattern exclusions. "
        f"{four_line['candidate_occurrences']} candidate occurrences yield "
        f"{four_line['supported_conics']} supported conics and "
        f"{four_line['independent_additional_rows']} independent added rows. "
        "Native incidence checks exclude every known line product; both completed plans "
        "bind 356 prior tuples and actual pencils repeat none. Full geometry collector, "
        "nonempty separate fixtures and final coverage/replay controls pass. Computed core "
        f"{four_line['core_full_rank']}/{four_line['core_auxiliary_rank']}/{four_line['closed_rank']}; "
        f"restored {four_line['full_rank']}/{four_line['auxiliary_rank']} counts are derived.")
tangent_source=ROOT/'evidence/tangent-conics-v1/calls/combine/stdout.json'
if (tangent_source.parent/'exit-code.txt').exists() and (tangent_source.parent/'exit-code.txt').read_text().strip()=='0':
    tangent=json.loads(tangent_source.read_text())
    attempts['tangent-conics-v1']=(
        f"Three-point/forced-tangent selection completes {tangent['raw_choices']} raw choices: "
        f"{tangent['tangent_pencils']} admitted pencils in {tangent['batch_count']} batches. "
        f"{tangent['candidate_occurrences']} candidate occurrences yield "
        f"{tangent['supported_conics']} supported conics and "
        f"{tangent['independent_additional_rows']} independent added rows. "
        f"All {tangent['forced_contact_candidates_verified']} forced second-coefficient parameters "
        "are checked. Separate same-collector product fixture is nonempty; its higher-contact "
        f"candidate has order {tangent['control_higher_contact_order']}. "
        "Four-constraint basis, local multiplicity, corrupted selector/support/row/origin and "
        "coverage controls pass. Computed core "
        f"{tangent['core_full_rank']}/{tangent['core_auxiliary_rank']}/{tangent['closed_rank']}; "
        f"restored {tangent['full_rank']}/{tangent['auxiliary_rank']} counts are derived. "
        "Separate numerical rank-seven control enters no actual matrix.")
bitangent_source=ROOT/'evidence/bitangent-conics-v1/calls/combine/stdout.json'
if (bitangent_source.parent/'exit-code.txt').exists() and (bitangent_source.parent/'exit-code.txt').read_text().strip()=='0':
    bitangent=json.loads(bitangent_source.read_text())
    attempts['bitangent-conics-v1']=(
        f"Two-point/bitangent selection completes {bitangent['raw_choices']} raw choices: "
        f"{bitangent['bitangent_pencils']} admitted pencils. "
        f"{bitangent['candidate_occurrences']} candidate occurrences yield "
        f"{bitangent['supported_conics']} supported conics and "
        f"{bitangent['independent_additional_rows']} independent added rows. "
        f"All {bitangent['forced_contact_candidates_verified']} higher-contact branches are checked. "
        "The same-collector known-line-square fixture recovers a nonzero row; its "
        f"higher-contact orders are {bitangent['control_higher_contact_orders']}. "
        "Both contact constraints, exact product exclusion, local multiplicities, "
        "selector/profile/origin/support/row and coverage controls pass. Computed core "
        f"{bitangent['core_full_rank']}/{bitangent['core_auxiliary_rank']}/{bitangent['closed_rank']}; "
        f"restored {bitangent['full_rank']}/{bitangent['auxiliary_rank']} counts are derived. "
        "Separate numerical rank-seven control enters no actual matrix.")
if selected - attempts.keys():
    raise SystemExit('Unknown attempt selected')
for name, outcome in attempts.items():
    if figure_only:
        continue
    if selected and name not in selected:
        continue
    raw = ROOT/'evidence'/name
    destination = ROOT/'reports'/name
    destination.mkdir(parents=True, exist_ok=True)
    calls = sorted((raw/'calls').iterdir())
    if name in ('mixed-sign-conics-v1','mixed-image-conics-v1','mixed-image-extension-v1','four-line-conics-v1','tangent-conics-v1','bitangent-conics-v1'):
        def call_order(call):
            if call.name=='build': return (0,0)
            if call.name=='plan': return (1,0)
            if call.name.startswith('batch-'): return (2,int(call.name.removeprefix('batch-')))
            return (3,0)
        calls.sort(key=call_order)
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
                        ('Six225','six-v1/calls/full'),
                        ('Partial','partial-v1/calls/collect'),
                        ('Residual graph','graph-v1/calls/collect'),
                        ('Mixed (derived)','mixed-v1/calls/scan'),
                        ('Divided','divided-v2/calls/scan'),
                        ('Mixed /2,/3','mixed-divided-v1/calls/scan'),
                        ('Anchor16','anchors-v1/calls/scan'),
                        ('Anchor256','anchors-expanded-v1/calls/scan'),
                        ('Conic16','expanded-conics-v1/calls/scan'),
                        ('Fibre-all','all-expanded-conics-v2/calls/scan'),
                        ('Mixed sign*','mixed-sign-conics-v1/calls/combine'),
                        ('Balanced*','expanded-balanced-v1/calls/scan'),
                        ('Mixed image*','mixed-image-conics-v1/calls/combine'),
                        ('Union685','full-image-bound-v1/calls/scan'),
                        ('Old-pair*','mixed-image-extension-v1/calls/combine'),
                        ('Four-line*','four-line-conics-v1/calls/combine'),
                        ('Tangent*','tangent-conics-v1/calls/combine'),
                        ('Bitangent*','bitangent-conics-v1/calls/combine')]:
    source = ROOT/'evidence'/relative/'stdout.json'
    terminal = source.parent/'exit-code.txt'
    if terminal.exists() and terminal.read_text().strip() == '0':
        record=json.loads(source.read_text())
        if label in ('Mixed (derived)','Mixed /2,/3'):
            record={key:record['combined_'+key] for key in metrics}
        cases.append((label,record))
data = {label:{k:record[k] for k in metrics} for label,record in cases}
for label,record in cases:
    if record.get('restored_ranks_are_derived'):
        data[label].update({key:record[key] for key in
                            ('restored_ranks_are_derived','core_full_rank','core_auxiliary_rank')})
(ROOT/'rank-values.json').write_text(json.dumps(data, indent=2)+'\n')
fig, (axis,closed_axis) = plt.subplots(2,1,figsize=(9,4.6),sharex=True,
                                     gridspec_kw={'height_ratios':[2,1]},layout='constrained')
for position, key in enumerate(metrics[:2]):
    x = [i+(position-.5)*.36 for i in range(len(cases))]
    values=[record[key] for _,record in cases]
    bars=axis.bar(x,values,width=.33,label=key.replace('_',' '))
    axis.bar_label(bars,labels=[str(v) if 0<v<1000 else '' for v in values],padding=2+8*position,fontsize=6.5)
    for at,value in zip(x,values):
        if not value:
            axis.text(at,.9,'0',ha='center',va='bottom',fontsize=7)
axis.set_yscale('log')
axis.set_ylim(.7,max(record['full_rank'] for _,record in cases)*8)
axis.set_ylabel('Full / auxiliary rank (log scale)')
axis.legend(loc='upper left',bbox_to_anchor=(0,1.25),ncol=2,fontsize=9)
closed_values=[record['closed_rank'] for _,record in cases]
bars=closed_axis.bar(range(len(cases)),closed_values,width=.55,color='#54856d')
closed_axis.bar_label(bars,padding=2,fontsize=8)
closed_axis.set_ylim(0,max(closed_values)*1.5)
closed_axis.set_ylabel('Closed rank')
closed_axis.set_xticks(range(len(cases)),[label for label,_ in cases])
closed_axis.tick_params(axis='x',labelsize=7,labelrotation=28)
for suffix in ('svg','pdf'):
    fig.savefig(ROOT/('ranks.'+suffix))
plt.close(fig)
pdf(ROOT/'REPORT.md')
for svg in [ROOT/'ranks.svg', *(ROOT/'reports').glob('*/calls.svg')]:
    svg.write_text('\n'.join(line.rstrip() for line in svg.read_text().splitlines())+'\n')
