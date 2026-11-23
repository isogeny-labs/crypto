from pathlib import Path
import json,hashlib,subprocess,shutil
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import typst,fitz
s=Path('research/n83_candidate_pdp_20261028');q=Path('/home/ec2-user/n83-candidate-recovery/prime-linear-development')
read=lambda p:json.loads(p.read_text());sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()
for n in ['evidence/prime-linear/summary.json','evidence/prime-linear-publication.json']:
 assert subprocess.check_output(['git','show',head+':'+str(s/n)])==(s/n).read_bytes()
pub=read(s/'evidence/prime-linear-publication.json');p=read(s/'protocol-prime-linear.json');rows=read(s/'evidence/prime-linear/summary.json')['rows']
stamp=pub['batch_started_at'].replace('-','').replace(':','')[:13]+'Z';out=s/'reports'/('run-'+stamp+'-prime-linear');out.mkdir()
groups=[]
for m in range(3,10):
 rr=[x for x in rows if x['m']==m];assert len(rr)==6
 assert len({(x['projected_rank'],x['projected_columns'],x['first_fall_lower_bound']) for x in rr})==1
 x=rr[0];groups.append((m,6,x['projected_rank'],x['projected_columns'],x['first_fall_lower_bound']))
fig,ax=plt.subplots(figsize=(8.8,2.6),layout='constrained');plt.rcParams['svg.fonttype']='none'
ax.plot([g[0] for g in groups],[g[3] for g in groups],color='#9ca3af',linewidth=3,label='Full linear multiplier domain')
ax.scatter([g[0] for g in groups],[g[2] for g in groups],color='#116d8a',s=35,label='Verified projected rank',zorder=3)
ax.set(xlabel='m',ylabel='Dimension',xticks=range(3,10),title='Seven shared degree-five coefficient maps');ax.legend();ax.grid(alpha=.2)
fig.savefig(out/'ranks.svg');fig.savefig(out/'ranks.png',dpi=150);plt.close(fig)
table='\n'.join(f'| {m} | {n} | {rank} / {cols} | {"≥"+str(lower) if lower is not None else "inconclusive"} |' for m,n,rank,cols,lower in groups)
text=f'''# Prime degree-five syzygy exclusion

Run n83-prime-linear-20261030-r1. All42 retained prime m3–9 cases passed producer, fresh-process and S3-restored replay. Seven template computations are shared evidence, not42 independent rank experiments. Independent scientific review and full proof formalization remain pending.

| m | Cases | Projected rank / columns | FFD lower bound |
|---:|---:|---:|---|
{table}

![Projected ranks](ranks.svg)

Each case keeps its original immutable2^27-candidate factor base, target and polynomial system. Six overlapping five-selector coefficient restrictions cover all27 selectors in each block while retaining every intermediate variable. Injectivity of the projected degree-five map forces every original linear syzygy coefficient to vanish. The regular sequence of selector squares accounts for the remaining Koszul syzygies. This is a lower bound on highest-form FFD, not an LFD or solving-degree result. Deficiency would be inconclusive for the original system.

The earlier terminal dataset supplies upperFFD17. Combined bounds are stored only after matching exact source and base-manifest hashes for all42 cases. ExactFFD remains unknown; no newLFD bound or elite-base improvement is established.

Validation:21 test invocations (five new tests plus two copies of eight inherited donor tests). Independent six-term polynomial expansion checks30 quartics; dense elimination compares six small-prime maps. Three-selector annihilators and incomplete coverage are detected. All{pub['manifest_payloads_before_and_after']} raw payloads verify before and after restored replay. Frozen protocol {p['freeze']['protocol_sha256']}; source {p['arms'][0]['instrument']['commit']}; data commit {head}.

One native process,2GiB,300s producer and300s fresh replay,720s outer,zero retries. Existing-host fallback follows retained EKS shared-bucket403; no IAM/capacity change or performance claim. Diagnostic interval {pub['batch_started_at']} to {pub['batch_finished_at']}. The initial recovery note lacked required headings; its rejected version and logs are retained, and the corrected gate passed before production.

PRIME_LINEAR_FORMAT.json defines JSON fields, variable numbering, sparse matrix rows and pivot certificates. Hydrate prime-linear-archives/runs.tar.gz, extract into ROOT, verify manifest, run `instrument replay ROOT ROOT/protocol-prime-linear.json ROOT/results`, and verify manifest again.

Primary definitions: [Caminata–Gorla](https://arxiv.org/html/2112.05579), Definition1.3 and Section4. Study-specific covering argument: PRIME_LINEAR_DESIGN.md. The broader three-family m2–9 FFD/LFD and elite-base objective remains open.
'''
(out/'report.md').write_text(text)
cells='\n'.join(f'[{m}],[{n}],[{rank} / {cols}],[{"≥"+str(lower) if lower is not None else "inconclusive"}],' for m,n,rank,cols,lower in groups)
typ=f'''#set page(paper:"a4",margin:16mm)
#set text(font:"DejaVu Serif",size:9pt)
#set par(leading:0.45em)
= Prime degree-five syzygy exclusion
All42 retained prime m3–9 cases passed producer,fresh andS3-restored replay. Seven shared templates; independent scientific review pending.
#table(columns:4,inset:4pt,[m],[Cases],[Rank / columns],[FFD lower],{cells})
#image("ranks.svg",width:100%)
*Meaning.* Six overlapping five-selector restrictions cover all27 selectors in each block and retain all intermediate variables. Full projected rank excludes original linear syzygies. Selector squares form a regular sequence; the written quotient argument gives the lower bound. A deficient projection is inconclusive. Actual fixed factor bases and original equations are unchanged.

*Combined evidence.* Earlier upperFFD17 certificates are joined only after source and base-manifest hashes match for all42 cases. ExactFFD remains unknown. No newLFD,solving-degree,elite-base or performance claim.

*Checks.* 21 test invocations comprise five new tests and duplicated eight donor tests. Six independent dense rank comparisons and30 independently expanded quartics; annihilator and coverage controls. All{pub['manifest_payloads_before_and_after']} raw payloads verify before/after restored replay.

*Execution and retention.* One process,2GiB,300s producer/300s replay,720s outer,no retries. Existing-host fallback after retained EKS source403. Rejected recovery-note headings and correction retained; gate passed before production. Interval:{pub['batch_started_at']} to{pub['batch_finished_at']},diagnostic only.

*Replay.* PRIME_LINEAR_FORMAT.json documents every field. Extract archive,check manifest,run `instrument replay ROOT ROOT/protocol-prime-linear.json ROOT/results`,check manifest again. All original and new evidence remains retained. Full three-family m2–9 and elite-base objective open.

#text(size:7pt)[Source:{p['arms'][0]['instrument']['commit']} #linebreak() Protocol:{p['freeze']['protocol_sha256']} #linebreak() Data:{head}]
#link("https://arxiv.org/html/2112.05579")[Caminata–Gorla Definition1.3 and Section4];study argument:PRIME_LINEAR_DESIGN.md.
'''
(out/'report.typ').write_text(typ);shutil.copy2(__file__,out/'render.py');typst.compile(str(out/'report.typ'),output=str(out/'report.pdf'))
doc=fitz.open(out/'report.pdf')
for i,page in enumerate(doc):page.get_pixmap(matrix=fitz.Matrix(1.3,1.3)).save(q/f'report-page-{i+1}.png')
(q/'report-identity.json').write_text(json.dumps({'directory':str(out),'pages':len(doc),'pdf_sha256':sha(out/'report.pdf'),'data_commit':head},indent=2)+'\n');print((q/'report-identity.json').read_text())
