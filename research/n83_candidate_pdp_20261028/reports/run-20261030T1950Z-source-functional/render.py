import pathlib,json,datetime
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import typst,fitz
s=pathlib.Path('research/n83_candidate_pdp_20261028');e=s/'evidence/source-functional-r2';j=json.loads((e/'summary.json').read_text());rows=j['rows']
out=s/'reports/run-20261030T1950Z-source-functional';out.mkdir(exist_ok=False)
fig,ax=plt.subplots(figsize=(8.4,2.8))
for arm,offset,col in [('original',-.23,'#247ba0'),('baseline',0,'#70c1b3'),('spike40',.23,'#e07a5f')]:
 group=[r for r in rows if r['arm']==arm];counts=[sum(r['pairing_nonzero'] for r in group if r['m']==m) for m in range(3,10)]
 ax.bar([m+offset for m in range(3,10)],counts,width=.22,label=arm,color=col)
ax.set(xticks=list(range(3,10)),yticks=[0,1,2],ylim=(0,2.5),xlabel='m (two targets per arm and m)',ylabel='Nonzero exact pairings',title='Recorded source-bound pairing status; no LFD or regularity inference')
ax.legend(loc='upper center',ncol=3);fig.tight_layout();fig.savefig(out/'pairings.svg');plt.close(fig)
(out/'plot-data.json').write_text(json.dumps(rows,indent=2)+'\n')
nonzero=sum(r['pairing_nonzero'] for r in rows);degree=sorted(set(r['coefficient_degree'] for r in rows));term=sorted(set(r['coefficient_terms'] for r in rows));cust=json.loads((e/'custody.json').read_text())
text=f'''#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
#set heading(numbering: none)
= n83 source-bound terminal functional panel
Observations only; this report does not promote a mathematical claim.

The retained prime m3–9 panel contains 42 cases: original, baseline and spike40, each with planted and natural targets. It contributes to the wider three-family m2–9 fixed-base study. Inputs and factor bases were unchanged.

== Execution and retained failures
EKS run n83-source-functional-20261030-r1 reached its deadline after unschedulable Pod observations. No worker start was observed and no raw object was published. This is an infrastructure outcome, not a refutation. Its request, Job/Pod identities, statuses and empty raw listing are preserved.

A separately frozen version 2 ran n83-source-functional-20261030-r2 on the existing host with the same native binary and 42 input/source/base identities. Producer and fresh replay completed; restored S3 bytes passed a further native replay. Each phase had a 120-second limit and 2 GiB virtual-memory limit. No retries; no timing claim. Per-case wall/CPU times were not measured.

== Recorded results
All 42 source and retained-factor checks passed. Nonzero pairings: {nonzero}/42. Recorded coefficient degrees: {degree}. Recorded nonzero term counts: {term}. These are degrees/counts of C, not degrees of regularity.
#image("pairings.svg", width: 100%)
The bars count recorded nonzero pairings for two targets per arm/m in run r2. Deterministic exact arithmetic; no statistical interval is asserted.

== What was checked
The native instrument reconstructs each complete source, verifies input/base/certificate hashes, recomputes A0/A1/A2 and C, and compares the direct complement pairing to the exact scalar formula. The pinned image passed 22 development test invocations: six new tests plus eight unique donor tests imported twice. Mutation and controller success/failure/termination controls passed. Production source commit: d0f98fafe1df2b06ad5df6ca6f033178cfa7dc02.

== Claim limits and custody
Strict filtered fall remains unverified. All LFD and regularity fields remain null. V16 was not computed. The separate syzygy and filtered-space argument still needs independent verification. Candidate x-domain equations do not enforce the expanded liftable-point lookup ideal. No elite-base advantage or speedup is established.

Raw archive: {cust['archive_bytes']} bytes, {cust['manifest_payloads']} verified payloads. SHA256:
#text(size: 7pt)[{cust['archive_sha256']}]
See evidence/source-functional-r1 and -r2, both experiment-source-functional exports, protocol-source-functional(-local).json, and SOURCE_FUNCTIONAL_RUN_FORMAT.json. Full coefficient polynomials and exact executable are retained in S3; Git holds case records, hashes, formats and this report.
'''
(out/'report.typ').write_text(text)
typst.compile(str(out/'report.typ'),output=str(out/'report.pdf'))
doc=fitz.open(out/'report.pdf');print('pages',len(doc))
for i,page in enumerate(doc):page.get_pixmap(matrix=fitz.Matrix(1.2,1.2)).save('/home/ec2-user/n83-candidate-recovery/source-functional-report-page-'+str(i+1)+'.png')
