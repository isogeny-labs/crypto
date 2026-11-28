"""Layout recorded native results; no curve/solver research in Python."""
import json
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
root=Path(__file__).resolve().parents[2];t=root/'research/trace_zero_terminal_20261115';l=root/'research/trace_zero_linearized_20261115'
tr=t/'reports/run-20261115T0542Z-terminal';lr=l/'reports/run-20261115T0613Z-linearized'
for p in [tr,lr]:p.mkdir(parents=True,exist_ok=True)
sets=[(t,tr,['constants','identity-first','tail-domain','tail-pairs','sliced','sliced-aa','pair-table'],98),(l,lr,['sliced-original','prepared-sliced','linearized','linearized-aa','pair-table'],130)]
for data,out,names,total in sets:
 s=json.loads((data/'summary.json').read_text());counts={n:s['groups'][n] for n in names}
 (out/'plot-data.json').write_text(json.dumps({'summary':str(data.relative_to(root))+'/summary.json','scope':'Exact recorded finite-panel counts; paired repeats, not independent targets','groups':counts},indent=2)+'\n')
 fig,ax=plt.subplots(figsize=(9,3.2 if total==98 else 2.7),layout='constrained');left=[0]*len(names)
 for key,label,color in [('found','Found','#258671'),('refuted','Refuted','#3677ac'),('inconclusive','UNKNOWN','#c4cbd2')]:
  values=[counts[n][key] for n in names];bars=ax.barh(names,values,left=left,color=color,height=.67,label=label)
  for bar,v in zip(bars,values):
   if v:ax.text(bar.get_x()+v/2,bar.get_y()+bar.get_height()/2,str(v),ha='center',va='center',fontsize=10,color='white' if key!='inconclusive' else '#1a3342')
  left=[a+b for a,b in zip(left,values)]
 ax.invert_yaxis();ax.set_xlim(0,total);ax.set_xlabel(f'Query executions (count): {total//2} fixed instances ×2 repeats per arm')
 ax.spines[['top','right']].set_visible(False);ax.legend(loc='upper center',bbox_to_anchor=(.5,1.19),ncols=3,frameon=False)
 fig.savefig(out/'outcomes.svg');plt.close(fig)
q1=json.loads((t/'protocol.json').read_text());q2=json.loads((l/'protocol.json').read_text())
terminal='''= Terminal restrictions and constant-residual S3
2026-11-15 · PR3744 · TZT-20261115-terminal-domains · local unsigned evidence

#image("outcomes.svg", width: 100%)

*Observed:* all 686 query executions complete with zero invalid decisions. Per 98 rows, constants decides 50, identity-first 64, marginal tail 78, correlated tail 74, sliced 98 and fresh pair-table 98. Sliced A/A has zero outcome or conflict discrepancies. Each repetition uses the same 49 instances; it is not an independent population sample.

*Construction:* for P+Q+R=T, enumerate the actual final point R and solve P+Q=T−R with a fixed-target two-point S3 system. Every residual shares the same total conflict, model and deadline budget. Incomplete residual work stays UNKNOWN. Actual supplied signs and identity branches remain; every accepted sum is replayed in the original group.

Marginal tail restricts the finite intermediate to x(T−B); correlated tail additionally retains its coupling with the last x. Both use established native domain machinery. Identity-first searches the same complete masks in a different order. The classical reference rebuilds its exact pair table for every query; its setup is charged, with no free cached truth.

*Nontrivial controls:* retain all 33 earlier cases and add 16 holdouts at n83/131,d6/8. Planted targets come from three distinct x values and are nonidentity and outside the base. Wide bases have 26/134 points at n83 and 26/112 at n131. Sliced finds every distinct-x planted holdout, including n131,d8. These are controlled targets on the actual curve, not the public challenge Q.

*Accounting:* per 98 executions, recorded additions+doublings are 3342 sliced and 132124 fresh pair-table; maximum sliced SAT allocation is 981 variables. Those units exclude field/ANF construction and SAT work, and setup could be amortized in another workload. No runtime ratio or full-cost gain is claimed. The extra coupled constraint leaves more UNKNOWN here than the marginal constraint; both outcomes are retained.

*Flow and controls:* planned 686, attempted 686, completed 686, excluded 0, interrupted 0, retries 0. A separate 35-call engineering pilot and native analysis smoke pass their fixed gates. One CPU, 2GiB, 600-second combined pilot/freeze/run cap; each query 2000 conflicts and 750ms safety fuse. Shared host, no L2 clock. The user's local EKS/S3 exception applies; no paid compute launched.

*Validation:* native audit rechecks all 49 truths and replays all 398 saved original point tuples; no mismatch. Canonical export/source/file check passes. Two focused exact small-group/cap checks and final production regressions are retained in the scale study. No independent signed review exists. Completed earlier panels remain historical; the new strategy changes their scoped non-singleton UNKNOWN outcome.

*Reproduction and identity:* README.md links the exact manifest, raw rows, exits, pilot, summary, export and native sources. Source SOURCE; protocol PROTOCOL; binary BINARY. This reuses the existing S3/group/meet-in-the-middle algebra; no mathematical originality or challenge-recovery claim.
'''.replace('SOURCE',q1['freeze']['commit']).replace('PROTOCOL',q1['freeze']['protocol_sha256']).replace('BINARY',q1['arms'][0]['instrument']['binary_sha256'])
example=json.loads((l/'example-relation.json').read_text())
linear='''= Symmetric-residual linearization on larger bases
2026-11-15 · PR3744 · TZL-20261115-symmetric-residual · local unsigned evidence

#image("outcomes.svg", width: 100%)

*Observed:* all 650 primary executions complete with zero invalid decisions. Per 130 rows, linearized and linear-AA each give 92 Found, 38 Refuted, 0 UNKNOWN. Prepared sliced SAT gives 80/34/16, original sliced SAT 78/32/20 and fresh pair-table 74/32/24. Linear A/A outcomes and counted work agree. Each arm repeats the same 65 instances twice; these are not independent targets.

#table(columns: (auto,auto,auto,auto), inset: 4pt,
 [New scale cases only], [Found], [Refuted], [UNKNOWN],
 [Original sliced SAT], [10], [2], [20],
 [Prepared sliced SAT], [12], [4], [16],
 [Linearized], [24], [8], [0],
 [Fresh pair-table], [6], [2], [24],
)

*New heldouts:* all 49 anchors are unchanged. Sixteen new cases cover n83/131,d10/12 with two distinct-x planted targets, a uniform full-width scalar target and a cancellation control per cell. Bases contain 512/2008 points at n83 and 442/1994 at n131. Every distinct-x scale control is found by the linear method. The public challenge Q is not used.

*Work units:* the linear solver allocates no SAT variables and uses at most 35 binary coefficient columns, a separate unit. Across 130 rows it records 17892 group additions+doublings; prepared SAT 43086, original SAT 43070 and fresh pair-table 3776671. These counts include setup/lifts/replay and capped attempts but exclude Gaussian, field/root and SAT work. They are not runtime or complete successful-work ratios. Batch-amortized reference costs are not measured.

*Flow:* planned/attempted/completed 650/650/650, no exclusions, interruptions or retries. A 45-call largest-class engineering pilot passes; stratified planning estimate 150.689seconds, only a resource estimate. Each panel is capped at CPU0, 2GiB, 600seconds including pilot/freeze; each oracle uses a 750ms safety fuse and 2000 SAT conflicts where applicable. No isolated timing, population, scaling or full-DLP inference.

*Checks:* native post-run audit rebuilds complete pair tables per distinct base, rechecks all 65 truths, asserts a complete unique query grid, and replays 416 saved tuples without mismatch. Three focused coefficient-cap/small-group checks pass; prior 20 and terminal 2 regressions and root library check pass. All canonical export/source/file checks pass. Every UNKNOWN remains visible. No independent signed review is attached.

*Identity:* source SOURCE; protocol PROTOCOL; binary BINARY. Raw rows and model diagnostics, the accepted export, fixed seeds, curve/subgroup identities and exact commands are linked in README.md. The actual curve is E0 over F2^131, modulus z^131+z^13+z²+z+1, with its pinned original Certicom subgroup fixture.

#pagebreak()
= Why the residual problem becomes linear
#image("mechanism.svg", width: 100%)

For fixed z=x(T−R), set u=x(P)+x(Q) and v=x(P)x(Q). The existing binary S3 identity gives v²+zv+z²u²+b=0. Here u is a *field sum*, not x(P+Q). If V is the span of actual admitted x codes and W=span(V·V), then u∈V and v∈W. Frobenius squaring is F2-linear, so their coefficient bits satisfy a binary linear system. No group partial sum is forced into V.

A particular coefficient solution plus its full nullspace describes every necessary candidate. Factor X²+uX+v with the existing field solver, check actual finite base membership and all supplied signs, then re-add in the original group. A deficient space exceeding the shared model cap returns UNKNOWN; no uniqueness assumption is made. Prepared SAT admission validates immutable inputs once and uses only a private admitted-residual path.

The standard integral expansion (xy+xz+yz)²=(xy)²+z²(x+y)²+2xyz(x+y) has evaluated certificate IDC1h420f65ecd4500a54, degree4, 32 issued+32 local replay points over2^61−1. Reducing the even correction gives the known characteristic-two S3 form. The certificate evaluates the integral expansion only, not a formal solver/completeness proof. Expanded coefficients have magnitude at most2. No novelty claim.

*Concrete saved relation:* scale-n131-d12-distinct-planted-0, base size1994. The following original points sum to the target in the native audit. The solver uses source span dimension12, product span23, 35 columns, consistent rank35/nullity0; it examines400 residual systems before finding this tuple.

#table(columns: (auto,auto,auto), inset: 4pt,
 [Point], [x (hex)], [y (hex)],
 POINTROWS
)
Target x: TARGETX

Target y: TARGETY

The full saved record is example-relation.json, tied to the frozen workload and integrity-audit.json. These are planted point-decomposition controls on ECC2K-130's curve, not recovery of its public discrete logarithm. Earlier UNKNOWN observations remain valid for their earlier source/encodings. The new source-bound result extends the solved finite-base controls [pcc:unverified].
'''.replace('SOURCE',q2['freeze']['commit']).replace('PROTOCOL',q2['freeze']['protocol_sha256']).replace('BINARY',q2['arms'][0]['instrument']['binary_sha256'])
linear=linear.replace('POINTROWS','\n'.join(f'[P{i+1}], [{p[0]}], [{p[1]}],' for i,p in enumerate(example['points']))).replace('TARGETX',example['target'][0]).replace('TARGETY',example['target'][1])
prefix='#set page(paper: "a4", margin: 15mm)\n#set text(font: "DejaVu Sans", size: 9pt)\n'
for out,body in [(tr,terminal),(lr,linear)]:
 (out/'body.typ').write_text(body);(out/'report.typ').write_text(prefix+body)
(l/'report.typ').write_text(prefix+terminal.replace('"outcomes.svg"','"../trace_zero_terminal_20261115/reports/run-20261115T0542Z-terminal/outcomes.svg"')+'\n#pagebreak()\n'+linear.replace('"outcomes.svg"','"reports/run-20261115T0613Z-linearized/outcomes.svg"').replace('"mechanism.svg"','"reports/run-20261115T0613Z-linearized/mechanism.svg"'))
print('wrote recorded-result plots and report sources')
