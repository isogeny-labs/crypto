"""Plot recorded native summary values and lay out the report; no solver work."""
from pathlib import Path
import json
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
root=Path(__file__).resolve().parent
s=json.loads((root/'summary.json').read_text())
a=json.loads((root/'grid-audit.json').read_text())
q=json.loads((root/'protocol.json').read_text())
ids=q['extensions']['primary_cases']
labels=[f"n{n} · {k}" for n in [83,131] for k in ['planted','uniform','cancellation']]
fig,ax=plt.subplots(figsize=(9.2,4.4),layout='constrained')
colors={'baseline':'#4b78a5','baseline-aa':'#91adc9','candidate':'#258671'}
for j,arm in enumerate(['baseline','baseline-aa','candidate']):
 cells=[s['cells'][f'{i}/{arm}']['instructions'] for i in ids]
 y=[i+(j-1)*.24 for i in range(len(ids))]
 x=[c['median']/1e6 for c in cells]
 errors=[[ (c['median']-c['min'])/1e6 for c in cells],[(c['max']-c['median'])/1e6 for c in cells]]
 ax.barh(y,x,height=.21,label=arm,color=colors[arm],xerr=errors,error_kw={'capsize':2,'elinewidth':1})
ax.set_yticks(range(6),labels);ax.invert_yaxis();ax.set_xlabel('Million simulated instructions per cold process (Callgrind Ir)')
ax.spines[['top','right']].set_visible(False);ax.legend(loc='lower right');fig.savefig(root/'instructions.svg');plt.close(fig)
(root/'plot-data.json').write_text(json.dumps({'source':'summary.json','protocol':q['freeze']['protocol_sha256'],'cells':s['cells'],'comparisons':s['comparisons']},indent=2)+'\n')
rows=[]
for r,v in zip(s['comparisons'],a['cell_verdicts']):
 n='83' if 'n83-' in r['instance'] else '131'
 kind='planted' if 'distinct' in r['instance'] else 'uniform' if 'uniform' in r['instance'] else 'cancel'
 rows.append(f'[{n}/{kind}], [{r["baseline_median_ir"]:,}], [{r["candidate_median_ir"]:,}], [{100*r["instructions_removed_fraction"]:.2f}%], [{"pass" if v["instruction_reduction_criterion"] else "inconclusive"}],')
fieldrows=[]
for i in ids:
 b=s['cells'][i+'/baseline']['field'];c=s['cells'][i+'/candidate']['field']
 label=('83' if 'n83-' in i else '131')+'/'+('planted' if 'distinct' in i else 'uniform' if 'uniform' in i else 'cancel')
 fieldrows.append(f'[{label}], [{b["field_mul"]["median"]:,} / {c["field_mul"]["median"]:,}], [{b["field_square"]["median"]:,} / {c["field_square"]["median"]:,}], [{b["inverse_informational"]["median"]:,} / {c["inverse_informational"]["median"]:,}],')
body='''#set page(paper: "a4", margin: 15mm)
#set text(font: "DejaVu Sans", size: 9pt)
= Trace-zero solver: validated work reduction
2026-11-15 · PR3744 · TZF-20261115-cold-instruction-work · unsigned producer evidence

#image("instructions.svg", width: 100%)

#table(columns: (auto,auto,auto,auto,auto), inset: 4pt,
 [Field/case], [Baseline median Ir], [Candidate median Ir], [Removed], [Criterion],
ROWS
)

*Observed:* all 90 planned query executions complete, each with a fresh native field-tally process and a fresh Callgrind process. All decisions, point indices, model diagnostics and witness order match. Six fixed d12 controls ×three arms ×five repetitions; these are paired disclosed anchors, not independent population samples. Error bars show the recorded five-repeat min/max; full A/A medians and ranges are in summary.json.

*Decision rule:* a cell passes when candidate median instruction count is at least 5% lower and baseline A/A median difference is at most 1%. Every inconclusive cell is retained. This is whole-process simulated instruction work, including parsing, setup, Gaussian elimination, allocation and replay. It is not native runtime, cycles, an external-best comparison or an asymptotic claim [pcc:unverified].

*Controls and resources:* every planted/cancellation query Found with original-group replay; each uniform query Refuted against separate arithmetic evidence. The 12-call largest-class pilot passed before freeze. CPU0, 2GiB, 600second combined pilot/freeze/primary cap, no retry or paid resources. Planning estimate ESTIMATEseconds is resource planning, not timing evidence. No rows excluded or interrupted.

#pagebreak()
= What changed and how it was checked

*Implementation:* hoist squared V/W basis elements once; eliminate coefficients using three packed words, retaining arbitrary-width fallback; share one Montgomery inversion across at most 32 consecutive residuals; specialize monic quadratic roots. The first residual remains scalar to avoid charging unused batch work for an immediate answer. Search order, full nullspace enumeration, conservative caps, actual supplied signs and final original-group replay remain.

#table(columns: (auto,auto,auto,auto), inset: 4pt,
 [Case], [Field mul: old / new], [Field square: old / new], [Inv: old / new],
FIELDROWS
)

The field ledger includes cold fixture/variety construction through final replay; parsing is outside this field ledger and inside the instruction count. Inversions are informational: their internal multiplications/squarings are already counted. Unused residual-batch tail work is charged. Never add unlike field operations into a group-only ratio.

*Validation:* all 65 prior scale cases give exactly identical old/new decisions and witness order. New focused checks compare packed/generic solution kernels, batch/scalar sums across all small-curve points (including infinity, doubling and opposites), and every monic quadratic over F32 against exhaustive roots and the old routine. Prior linearized, terminal and 20-test suites pass; library type-check passes [pcc:unverified].

A separate Rust checker imports no production field/group/solver modules. Schoolbook polynomial arithmetic plus Euclid inversion rechecks all 65 known answers, including all 19 negatives; wide negatives require independent inconsistency of every necessary residual system. It validates all base coordinates/kernel admissions and replays the 416 saved tuples from 650 earlier rows, plus new saved tuples. Corrupted-point and false-refutation fixtures are both rejected. This producer-written check is stronger cross-arithmetic evidence, not independently signed verification [pcc:unverified].

*Scope:* E0 y²+xy=x³+1 over n83 and ECC2K-130 n131; existing exact curve/modulus/G identities remain in workload and prior study. Full-query old/new instruction comparisons do not establish best external performance against M4RI/FLINT/Magma, native-time gain, factor-base logarithms or a complete relation-collection pipeline. Canonical scoreboard/timeline/performance-gains values remain historical; no admitted runtime/full-cost record is changed. Existing ledger/main, whole-root and benchrec merge dependencies remain.

*Reproduction:* source SOURCE; protocol PROTOCOL; native binary BINARY. README.md links commands, source pins, rows, summary, canonical export, grid audit, separate arithmetic checks, original inputs and the archived raw process tree. Every PDF page was visually inspected. Sources checked: existing S3/half-trace/Itoh–Tsujii modules and Montgomery-batching references in repository code; KB unavailable, wrapper127. No mathematical novelty assertion.
'''
body=body.replace('FIELDROWS','\n'.join(fieldrows)).replace('ROWS','\n'.join(rows)).replace('ESTIMATE',f'{q["sample_size"]["preflight_runtime"]["estimated_total_seconds"]:.3f}').replace('SOURCE',q['freeze']['commit']).replace('PROTOCOL',q['freeze']['protocol_sha256']).replace('BINARY',q['arms'][0]['instrument']['binary_sha256'])
(root/'report.typ').write_text(body)
print('Rendered recorded-value graph and report source')
