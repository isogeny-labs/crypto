from pathlib import Path
import json,hashlib,datetime
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import typst,fitz
R=Path('/var/tmp/sat-cli-custody-acceptance-20261103-v1');O=Path(__file__).parent
load=lambda n:json.loads((R/n).read_text())
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
a=load('analysis.json');start=load('attempt-start.json');end=load('attempt-result.json');p=load('protocol.json');e=load('experiment.export.json');v=load('experiment.export.validation.json')
commands=[{'name':f.parent.name,**json.loads(f.read_text())} for f in sorted((R/'commands').glob('*/receipt.json'))]
paths=['analysis.json','attempt-start.json','attempt-result.json','protocol.json','ready.json','experiment.export.json','experiment.export.validation.json','source-before.json','source-after.json','schedule.json','limits.json','commands.json','run/cells/000000000000/manifest.json','run/cells/000000000001/manifest.json','run/cells/000000000000/host.json']+[str(f.relative_to(R)) for f in sorted((R/'commands').glob('*/receipt.json'))]
data={'source_root':str(R),'source_files':{n:{'sha256':sha(R/n),'bytes':(R/n).stat().st_size} for n in paths},'cells':a['cells'],'commands':commands,'attempt_start':start,'attempt_result':end,'canonical_validation':v,'source':e['source'],'protocol_id':p['protocol_id'],'protocol_version':p['version'],'protocol_canonical_hash':p['freeze']['protocol_sha256'],'host':load('run/cells/000000000000/host.json')}
(O/'data.json').write_text(json.dumps(data,indent=2)+'\n')
fig,ax=plt.subplots(figsize=(7,2.8));labels=['Cell 0: plain','Cell 1: twist'];values=[x['actual_outcome']['patterns_checked'] for x in a['cells']];totals=[x['actual_outcome']['patterns_total'] for x in a['cells']]
ax.barh(labels,totals,color='#dfe5ec',label='Recorded total patterns');ax.barh(labels,values,color='#16758c',label='Recorded checked patterns');ax.set_xlim(0,2.4);ax.set_xticks([0,1,2]);ax.set_xlabel('Patterns (count)');ax.invert_yaxis();ax.spines[['top','right']].set_visible(False)
for i,(value,total) in enumerate(zip(values,totals)):ax.text(total+0.05,i,f'{value}/{total}',va='center')
ax.legend(loc='lower center',bbox_to_anchor=(0.5,1.01),ncol=2,frameon=False,fontsize=9);fig.tight_layout();fig.savefig(O/'patterns.svg');plt.close(fig)
duration=(datetime.datetime.fromisoformat(end['finished_at'].replace('Z','+00:00'))-datetime.datetime.fromisoformat(start['started_at'].replace('Z','+00:00'))).total_seconds()
text='''#set page(paper: "a4", margin: 18mm)
#set text(font: "DejaVu Sans", size: 10pt)
#set par(leading: 0.6em)
= SAT CLI acceptance: retained initial evidence
Descriptive run report · 5 November 2026

The recorded acceptance completed all eleven CLI commands with exit code zero. Both fixed ordinary-binary cells retained a checked initial pattern through verification, report/export, local custody creation, restoration and restored verification. The analyzer records PASS; the canonical export validation records zero schema errors and zero semantic findings. This report describes those receipts and does not assign a research verdict.

#image("patterns.svg", width: 100%)
#text(size: 8pt)[Recorded counts from analysis.json and the two cell manifests. Sample: two cells, one per twist; no repeats or uncertainty estimates. Run: sat-cli-custody-acceptance-20261103-v1. The gray bar is each cell's recorded total, not another trial.]

== Recorded outcomes
#table(columns: (1fr,1fr,1fr), inset: 5pt,
[Field], [Cell 0 / plain], [Cell 1 / twist],
[Initial patterns checked], [1 of 2], [1 of 2],
[Native outcome], [partial], [partial],
[Solver completed], [false], [false],
[Scalar recovered without witness], [false], [false],
[Worker exit code], [0], [0],
[Diagnostic elapsed milliseconds], [203.069508], [102.878048])

All values above are recorded. The elapsed values are supervisor diagnostics with clock_valid_for_comparison=false; no speed comparison is made. Whole-worker retired instructions are null. CPU time and a valid whole-worker instruction count are not supplied by these outcome records.

Two cells were requested and attempted; both worker processes completed, with zero recorded failed, timed-out, interrupted or resumed cells. Their algebraic outcomes remain partial. One Generate invocation is recorded. SAT causal training admission remains false. No independent negative CLI generation arm was part of this acceptance; the protocol points to prior exact-source test gates for corruption and refusal controls.

#pagebreak()
= Instrument, inputs and receipt custody
The executed code commit was #raw("COMMIT"). Source before/after checks and the export report dirty=false. The retained binary SHA-256 was:
#raw("BINARY", block: true)

Protocol #raw("sat-cli-initial-custody-acceptance-20261103-v1"), version 2, was frozen before data. Canonical protocol hash:
#raw("PROTOCOL", block: true)
The full protocol file hash is distinct:
#raw("FILEHASH", block: true)

Fixed inputs: ordinary binary family, n=23, m=3, both twists, legacy factor base size 4, seed 42, one curve seed, generator and key per twist, degree bound 3, work limit 16384. Producer caps were 60 seconds per cell, 256 MiB memory, 128 MiB trace; the driver cap was 1800 seconds. Replay uses its separate registered allowance. Exact command arrays and all receipt fields are retained in data.json; no command was rerun to produce this report.

The eleven commands were plan, preflight, generate, verify, status, report, export, custody create, custody verify, custody restore, and restored verify. Local restore assertions checked original run bytes. The host record reports x86_64, Intel Xeon 6975P-C, 32 available logical CPUs, Linux 6.18.54-121.166.amzn2023.x86_64 and L0 isolation. These are descriptors, not an exclusive allocation claim.

Recorded start: START. Recorded finish: END. Derived whole-attempt elapsed interval: DURATION seconds, calculated as finish minus start. This interval includes driver and verification activity and is not a solver timing measurement. No new cloud launch or remote request is recorded by this local acceptance.

== Evidence and next step
Source root: #raw("/var/tmp/sat-cli-custody-acceptance-20261103-v1"). data.json lists the exact source hashes and embeds the command receipts. The source experiment.export.json SHA-256 is:
#raw("EXPORT", block: true)

Full SAT search, UNSAT certification and SAT causal training projection are not supplied by this initial-prefix acceptance. Next, integrate and validate the original-owner row and propagation extensions against their separately frozen source gates. Publish these derived report artifacts after the immutable run records.
'''
for k,value in {'COMMIT':start['source_commit'],'BINARY':start['binary_sha256'],'PROTOCOL':p['freeze']['protocol_sha256'],'FILEHASH':start['protocol_sha256'],'START':start['started_at'],'END':end['finished_at'],'DURATION':str(duration),'EXPORT':sha(R/'experiment.export.json')}.items():text=text.replace(k,value)
(O/'report.typ').write_text(text)
typst.compile(str(O/'report.typ'),output=str(O/'report.pdf'))
doc=fitz.open(O/'report.pdf');(O/'pdf-text.txt').write_text('\n'.join(page.get_text() for page in doc))
for i,page in enumerate(doc):page.get_pixmap(matrix=fitz.Matrix(1.2,1.2)).save(O/f'page-{i+1}.png')
print('pages',len(doc));print('elapsed',duration)
