# Render only the saved records; no cryptographic computation or new experiments.
from pathlib import Path
import json,shutil
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import typst,fitz
s=Path('research/n83_candidate_pdp_20261028');out=s/'reports/run-20261030T1714Z-prime-lefschetz-native';out.mkdir()
rows=json.loads((s/'evidence/prime-lefschetz-native/summary.json').read_text())['rows']
points=[]
for m in range(3,10):
 group=[r for r in rows if r['m']==m]
 assert len(group)==6 and len({r['first_fall_degree'] for r in group})==1
 points.append({'m':m,'source_records':len(group),'conditional_first_fall_degree':group[0]['first_fall_degree']})
(out/'plot-data.json').write_text(json.dumps(points,indent=2)+'\n')
fig,ax=plt.subplots(figsize=(8,2.15),layout='constrained')
ax.plot([p['m'] for p in points],[p['conditional_first_fall_degree'] for p in points],marker='o',color='#116a61')
ax.set(xlabel='m (six source records per point)',ylabel='Conditional highest-form FFD',xticks=range(3,10),ylim=(16,18),yticks=[16,17,18])
ax.grid(alpha=.2);ax.set_title('r3: saved conditional certificates; independent review pending',fontsize=10)
fig.savefig(out/'degrees.svg');plt.close(fig)
table='\n'.join(f"[{p['m']}],[{p['source_records']}],[{p['conditional_first_fall_degree']}]," for p in points)
text='''#set page(paper:"a4",margin:16mm)
#set text(font:"DejaVu Serif",size:9pt)
= Native prime Lefschetz certificates: r2 and r3
Observations only. The native r3 source audit and arithmetic certificates completed for all42 retained prime m3–9 cases, followed by fresh and S3-restored native replays. The recorded highest-form FFD is17 conditional on the written Lefschetz/Koszul argument and external block SLP theorem. Independent mathematical review and formalization remain pending.

#image("degrees.svg",width:100%)
#table(columns:3,inset:3pt,[m],[Source records],[Conditional FFD],TABLE)

*Meaning and scope.* Each m has original,baseline,spike40 arms and planted/natural targets. All42 original input/source/base hashes match. The structure and8192term upper witness are shared; seven leading gates are shared across cases. These are not42independent Gröbner computations or a formal proof. No new LFD, solving-degree, degree-of-regularity, arbitrary-factor-base or elite-base improvement is established. The claim concerns the exact saved polynomial presentations. All prior datasets remain retained; the full three-family m2–9 objective remains open.

*Execution outcomes.* EKS job n83-native-lefschetz-20261030-r2, UIDe157574b-2d6a-4450-9e13-fbc596db96fd, reached DeadlineExceeded17:29:56Z. Last predeadline Pod was Pending/node=null; worker start was not observed. Terminal retained no Pod; logs empty; final raw S3 inventory0objects. One submitted infrastructure job covered42planned slots, not42native process failures. Export submission counts are explicitly separated from execution/publication counts. The read-only node-pool probe was denied AssumeRole; error retained.

*Separate v3 continuation.* Native existing-host run n83-prime-lefschetz-native-20261030-r3 completed17:37:59–17:38:13Z. All four phases exited0: source audit, certificate production, fresh source replay, fresh certificate replay. Both native replays passed again after S3 restoration. Four120s phase caps,2GiB address space,720s outer deadline,zero retries. Batch times are diagnostics only; wall_valid=false, no performance comparison. Source revisionc080c9a021405096cc9597d5dbd715da1894db6f and both binaries match the tested image/source.14native tests (6new,8inherited), image tests and signal controls are retained.

*Custody and reproducibility.* Raw archive779736bytes,208payloads, SHA25634255f1d3aba276141a286952570f0ced90fbdd8a04a46d9faaf922afeed4be4. All payload hashes pass before/after restored replay. File definitions: PRIME_LEFSCHETZ_NATIVE_RUN_FORMAT.json. Evidence and commands: evidence/prime-lefschetz-native/README.md. Failedr2: evidence/prime-lefschetz-native-r2. Both unified exports pass validation with file hashes; a first r2 export draft with incorrect instance labels is retained with its findings. No failed attempt was discarded.

#text(size:7pt)[Frozen v2:a74055d42a694d1ee8d54e2bed1084a6350668fcb86a6db8c5975f020ae53c59 #linebreak() Frozen v3:355e8228ed4ad5c85cc210e18c35ec932338a3785fdb9b42024746dbb6671133 #linebreak() Written argument:PRIME_LEFSCHETZ_ARGUMENT.md. Mathematical review and full formalization are open.]
'''.replace('TABLE',table)
(out/'report.typ').write_text(text);shutil.copy2(__file__,out/'render.py')
typst.compile(str(out/'report.typ'),output=str(out/'report.pdf'))
d=fitz.open(out/'report.pdf')
for i,p in enumerate(d):p.get_pixmap().save('/home/ec2-user/n83-candidate-recovery/prime-lefschetz-native-local-support/report-page-'+str(i+1)+'.png')
print(json.dumps({'pages':len(d),'directory':str(out)}))
