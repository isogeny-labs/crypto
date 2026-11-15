from pathlib import Path
import json,hashlib,subprocess,shutil
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import typst,fitz
s=Path('research/n83_candidate_pdp_20261028');support=Path('/home/ec2-user/n83-candidate-recovery/prime-quadratic-bounded-development')
pub=json.loads((s/'evidence/prime-quadratic-publication.json').read_text())
rows=json.loads((s/'evidence/prime-quadratic/summary.json').read_text())['rows']
head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()
assert subprocess.check_output(['git','show',head+':'+str(s/'evidence/prime-quadratic/summary.json')])==(s/'evidence/prime-quadratic/summary.json').read_bytes()
out=s/'reports/run-20261030T1541Z-prime-quadratic';out.mkdir()
groups=[]
for m in range(3,10):
    rr=[x for x in rows if x['m']==m];assert len(rr)==6
    assert len({(x['projected_rank'],x['projected_columns'],x['first_fall_lower_bound']) for x in rr})==1
    groups.append(rr[0])
fig,ax=plt.subplots(figsize=(8,2.5),layout='constrained')
ax.plot([x['m'] for x in groups],[x['projected_columns'] for x in groups],label='Full quadratic multiplier domain',color='#999999',linewidth=3)
ax.scatter([x['m'] for x in groups],[x['projected_rank'] for x in groups],label='Replayed rank',color='#146b8a',zorder=3)
ax.set(xlabel='m',ylabel='Dimension',xticks=range(3,10));ax.legend();ax.grid(alpha=.2)
fig.savefig(out/'ranks.svg');plt.close(fig)
cells='\n'.join(f"[{x['m']}],[6],[{x['projected_rank']} / {x['projected_columns']}],[7–17]," for x in groups)
text=f'''#set page(paper:"a4",margin:16mm)
#set text(font:"DejaVu Serif",size:9pt)
= Bounded prime quadratic coefficient maps
*Observations only; independent scientific review and formalization pending.* Run n83-prime-quadratic-20261030-r1. Seven templates and all42 source cases passed production, fresh replay and S3-restored replay. Zero failed, capped or timed-out templates.
#table(columns:4,inset:4pt,[m],[Source cases],[Rank / columns],[FFD interval],{cells})
#image("ranks.svg",width:100%)
*Meaning.* The recorded degree-six maps are injective. The pair-covering and selector-square regular-sequence argument in PRIME_QUADRATIC_DESIGN.md gives conditional highest-form FFD ≥7. Matching exact source and factor-base hashes to the earlier upper17 dataset gives7–17 for all42 cases. Seven shared maps are not42 independent rank experiments. No exactFFD,newLFD,solving-degree,degree-of-regularity or elite-base improvement is established.

*Inputs.* Original,baseline,spike40 arms; planted and natural targets;m3–9. Each arm retains its immutable factor base and source system. The full three-family m2–9 objective remains open; this continuation measures prime cases only.

*Execution.* One template process at a time;8GiB address space;300s producer and300s replay,5s termination grace;120s assembly passes;4800s outer limit;zero automatic retries. Caps:100000columns,5000000coefficients,50000000reduction updates,2000000basis entries. Exact commands and identities are in the frozen protocol and archived runner. Existing-host fallback; fresh EKS source-access probe timed out with no retained Pod/log and gives no access verdict. Interval {pub['batch_started_at']}–{pub['batch_finished_at']} is diagnostic only,with no isolation or speed claim.

*Validation and retention.*26 native test invocations comprise10 new tests and16 duplicated donor invocations;two runner controls retain failure/timeout journals. All198 raw payloads matched their hashes before and after S3 replay. Every coefficient column and elimination row is retained. Archive86,270,953bytes; format and replay instructions:PRIME_QUADRATIC_FORMAT.json and evidence/prime-quadratic/README.md. Metadata publication required a directory-path correction; no experiment rerun.

#text(size:7pt)[Data commit:{head} #linebreak() Raw SHA256:1db8c8bec12f40ac3ef2a239eba5654541c7b01097b4befd80d12676d7275da9]
Primary definitions:#link("https://arxiv.org/html/2112.05579")[Caminata–Gorla,Definition1.3 and Section4].
'''
(out/'report.typ').write_text(text);shutil.copy2(__file__,out/'render.py')
(out/'plot-data.json').write_text(json.dumps(groups,indent=2)+'\n')
typst.compile(str(out/'report.typ'),output=str(out/'report.pdf'))
doc=fitz.open(out/'report.pdf')
for i,page in enumerate(doc):page.get_pixmap(matrix=fitz.Matrix(1.4,1.4)).save(support/f'report-page-{i+1}.png')
identity={'directory':str(out),'pages':len(doc),'pdf_sha256':hashlib.sha256((out/'report.pdf').read_bytes()).hexdigest(),'data_commit':head}
(support/'report-identity.json').write_text(json.dumps(identity,indent=2)+'\n');print(json.dumps(identity))
