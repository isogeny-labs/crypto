"""Render recorded native results only. No curve arithmetic or solver runs."""
import json
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
root=Path(__file__).resolve().parents[2]
s=root/'research/trace_zero_comparisons_20261115'
audit=json.loads((s/'integrity-audit.json').read_text())
specs=[
 ('original',root/'research/trace_zero_experiments_20261109',root/'research/trace_zero_experiments_20261109/reports/run-20261115T0450Z',['explicit','certified','partial-trace','certified-aa'],8,'export-final.json','1. Certified domain and partial traces'),
 ('encoding',s/'encoding',s/'reports/run-20261115T0458Z-encoding',['certified','constants','implicit','full-point','constants-aa'],13,'export.json','2. Matched relation encodings'),
 ('tau',s/'tau',s/'reports/run-20261115T0502Z-tau',['direct','tau-image','tau-preimage','direct-aa'],14,'export.json','3. Reversible tau presentations')]
all_bodies=[]
for panel,data,out,names,aa,exportname,title in specs:
 out.mkdir(parents=True,exist_ok=True)
 summary=json.loads((data/'summary.json').read_text());protocol=json.loads((data/'protocol.json').read_text());export=json.loads((data/exportname).read_text())
 counts=summary['arms'] if panel=='original' else {n:summary['groups'][n] for n in names}
 (out/'plot-data.json').write_text(json.dumps({'scope':'Recorded exact finite-panel counts from native summary; no population interval','summary':str((data/'summary.json').relative_to(root)),'arms':counts},indent=2)+'\n')
 fig,ax=plt.subplots(figsize=(9,2.55),layout='constrained')
 left=[0]*len(names)
 for key,label,color in [('found','Found','#258671'),('refuted','Refuted','#3677ac'),('inconclusive','UNKNOWN','#c4cbd2')]:
  values=[counts[n][key] for n in names]
  bars=ax.barh(names,values,left=left,color=color,label=label,height=.64)
  for bar,value in zip(bars,values):
   if value:ax.text(bar.get_x()+value/2,bar.get_y()+bar.get_height()/2,str(value),ha='center',va='center',fontsize=10,color='white' if key!='inconclusive' else '#1a3342')
  left=[x+y for x,y in zip(left,values)]
 ax.invert_yaxis();ax.set_xlim(0,66);ax.set_xlabel('Queries (count): 33 fixed instances ×2 repeats per arm')
 ax.spines[['top','right']].set_visible(False);ax.legend(loc='upper center',bbox_to_anchor=(.5,1.24),ncols=3,frameon=False)
 fig.savefig(out/'outcomes.svg');plt.close(fig)
 native=next(p for p in audit['panels'] if str((data/'rows.jsonl').relative_to(root))==p['path'])
 repeat='; '.join(n+': '+str(native['repeat_counts'][n+'/rep0']['decided'])+'/'+str(native['repeat_counts'][n+'/rep1']['decided'])+' decisions' for n in names)
 table=[]
 for n in names:
  c=counts[n]
  table.append(f'[{n}], [{c["found"]}], [{c["refuted"]}], [{c["inconclusive"]}], [{c["conflicts_total"]}],')
 details={
 'original': '''Certified and partial-trace arms each decide50/66 queries; explicit decides38/66. The A/A arms have identical outcomes and eight conflict discrepancies. Partial traces do not add decided queries here. These are fixed-panel completion differences, not an isolated runtime or population result.

The frozen instrument internally replays every Found answer and checks Refuted against complete pair-table truth. Its raw format retains replay booleans, not successful point tuples: the post-run audit rechecks truth, but cannot independently replay those omitted tuples. The two later panels save actual points.

The accepted canonical artifact is export-final.json. Earlier export.json records an initial adapter metadata/schema attempt and remains pinned as historical evidence; raw rows/outcomes were unchanged. Automation stored the two raw JSONL files under the content hashes in .artifacts.json; identical local bytes were restored from committed source for validation.''',
 'encoding': '''Certified and constants each decide50/66; implicit48/66; full-point34/66. All five arms have zero invalid decisions. The constants A/A arms have identical outcomes and thirteen conflict discrepancies. Allocation shrinks without a completion gain on this panel.

At n131, certified/constants/implicit each give4Found,4Refuted,8UNKNOWN out of16; full-point gives16UNKNOWN. Maximum allocated variables at n131 are104669 /4217 /6572 /106110 respectively. The d4 base has only two signed points. All n131 d5 cases, with eight base points, remain UNKNOWN. Thus the successful wide relations here are singleton-x controls.

The implicit arm receives a linear basis, not an enumerated point table. Full-point chains retain complete generic/double/opposite/identity branches.172 saved original point tuples replay in the post-run integrity audit. Native total group additions+doublings are374 /390 /410 /168 for the four distinct methods; failed/capped queries remain included.''',
 'tau': '''Direct and tau-preimage each decide50/66; tau-image48/66. The A/A arms agree on outcomes with fourteen conflict discrepancies. All successful mapped answers are transported back;142 saved original point tuples replay in the post-run audit. No completion advantage is observed for these transported finite-domain presentations.

At n131 all three distinct presentations give4Found,4Refuted,8UNKNOWN. Direct maximum allocation is4217 variables; both transported presentations reach61728. Across all66 rows, native group additions+doublings are390 direct,4888 image and31238 preimage, including setup and replay. These counts exclude field/ANF construction and SAT work and are not runtime ratios.

Exploratory post-run profile: for the eight-point n131 d5 base,126 x-coordinate bits are common constants directly, versus17 after either transport. This is a structural observation, not a preregistered causal conclusion. No unenumerated lifted-delta map or assertion that the transported x set is linear is supplied.'''}[panel]
 body=f'''= {title}
2026-11-15 · crypto PR3744 · {protocol['protocol_id']} · unsigned local evidence

#image("outcomes.svg", width: 100%)

#table(columns: (auto,auto,auto,auto,auto), inset: 3pt,
 [Arm], [Found], [Refuted], [UNKNOWN], [SAT conflicts],
 {chr(10).join(table)}
)

{details}

*Flow:* planned {export['flow']['planned']}, attempted {export['flow']['attempted']}, completed {export['flow']['completed']}, not-completed 0, excluded 0, interrupted 0. Every UNKNOWN remains in the denominator. Zero retries and zero paid resources. 33 common known-answer instances: n7/9/83/131,d4/5, planted/cancellation/two full-width uniform-scalar targets, plus the n5 signed-base UNSAT control.

*Controls and scope:* the preregistered small-field positive pilot gate passes; two paired repeats are the same targets, not independent population samples. Per 33-query repeat counts: {repeat}. A/A outcome discrepancy is zero. Summed conflicts include censored queries and are not comparable successful-work costs. No clock, scaling or challenge-recovery claim.

*Execution:* one CPU, 2GiB address-space ceiling, 600second total cap per panel including its pilot/freeze. Each oracle uses 2000 cumulative SAT conflicts and a 750ms safety fuse. Shared local host; no L2 isolation. The user's explicit local EKS/S3 exception applies. All source, input, caps and analysis were frozen before primary rows.

*Identity:* source {protocol['freeze']['commit']}; protocol {protocol['freeze']['protocol_sha256']}. Binary {protocol['arms'][0]['instrument']['binary_sha256']}. Raw rows, native summary, pilot, exit status, accepted export, file checks and host receipt are linked by README.md. Canonical export/file checks pass; no independent signed review-run record exists. Known inverse/halving and tau norm algebra are prior work, not new theorems.
'''
 (out/'body.typ').write_text(body)
 prefix='#set page(paper: "a4", margin: 15mm)\n#set text(font: "DejaVu Sans", size: 8.5pt)\n'
 (out/'report.typ').write_text(prefix+body)
 # Preserve exact native table/caption data as the quantitative source.
 links=f'''# {title}\n\n{details}\n\nFlow: planned{export['flow']['planned']}, attempted{export['flow']['attempted']}, completed{export['flow']['completed']}; zero interruptions, exclusions or retries. No isolated clock/population inference. All claims are unsigned local evidence [pcc:unverified].\n\n[PDF](report.pdf) · [recorded graph](outcomes.svg) · [plot data](plot-data.json)\n\nSource data: {str(data.relative_to(root))}/rows.jsonl, summary.json, protocol.json and {exportname}. Source{protocol['freeze']['commit']}; frozen protocol{protocol['freeze']['protocol_sha256']}. Common host receipt: research/trace_zero_comparisons_20261115/checks/host.txt. No independent signed review-run record. See the main study README for all reproduction commands and scope.\n'''
 (out/'README.md').write_text(links)
 all_bodies.append(body.replace('"outcomes.svg"','"'+str((out/'outcomes.svg').relative_to(s))+'"') if out.is_relative_to(s) else body.replace('"outcomes.svg"','"../trace_zero_experiments_20261109/reports/run-20261115T0450Z/outcomes.svg"'))
(s/'report.typ').write_text('#set page(paper: "a4", margin: 15mm)\n#set text(font: "DejaVu Sans", size: 8.5pt)\n'+'\n#pagebreak()\n'.join(all_bodies))
print('Rendered3 recorded-value SVGs and report sources')
