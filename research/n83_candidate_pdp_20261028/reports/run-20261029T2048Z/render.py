"""Render committed native observations; no polynomial computation or new experiment."""
from pathlib import Path
import json,hashlib,subprocess,shutil
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import typst,fitz
s=Path('research/n83_candidate_pdp_20261028');support=Path('/home/ec2-user/n83-candidate-recovery/degree4-quartic-support')
read=lambda p:json.loads(p.read_text())
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
summary=read(s/'evidence/degree4-quartic/summary.json')
pub=read(s/'evidence/degree4-quartic-publication.json')
p=read(s/'protocol-degree4-quartic.json');storage=read(s/'evidence/degree4-quartic-storage.json')
head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()
for name in ['evidence/degree4-quartic/summary.json','evidence/degree4-quartic-publication.json']:
 assert subprocess.check_output(['git','show',head+':'+str(s/name)])==(s/name).read_bytes()
rows=summary['rows'];stamp=pub['batch_started_at'].replace('-','').replace(':','')[:13]+'Z'
r=s/'reports'/('run-'+stamp);r.mkdir(parents=True,exist_ok=False)
plt.rcParams.update({'font.family':'DejaVu Sans','font.size':9,'svg.fonttype':'none'})
fig,ax=plt.subplots(figsize=(10,3.15),layout='constrained');labels=[];largest=1
for i,row in enumerate(rows):
 labels.append(row['key'].replace('-m3','').replace('original','orig').replace('baseline','base').replace('spike40','sp40').replace('koblitz','K').replace('binary','B').replace('planted','P').replace('natural','N'))
 if row['verified']:
  counts=row['state']['added_rows_by_degree'];low=sum(counts[:4]);quartic=counts[4]
  largest=max(largest,low+quartic)
  ax.bar(i,low,color='#c56e24',label='Degree 0–3' if i==0 else None)
  ax.bar(i,quartic,bottom=low,color='#116d8a',label='Degree 4' if i==0 else None)
  ax.text(i,low+quartic+35,str(low+quartic),ha='center',fontsize=8)
 else:ax.text(i,1,'unverified',rotation=90,ha='center')
ax.set_xticks(range(len(rows)),labels,rotation=32,ha='right');ax.set_ylim(0,largest*1.18)
ax.set_ylabel('New independent rows');ax.set_title('Rows retained under the 2 GiB basis-payload cap',fontsize=11)
ax.grid(axis='y',alpha=.15);ax.spines[['top','right']].set_visible(False);ax.legend(loc='upper right',fontsize=8)
fig.savefig(r/'added-rows.svg');fig.savefig(r/'added-rows.png',dpi=180);plt.close(fig)
svg=r/'added-rows.svg';svg.write_text('\n'.join(x.rstrip() for x in svg.read_text().splitlines())+'\n')
def value(row,field):
 if not row['verified']:return 'unverified'
 v=row.get(field,row.get('state',{}).get(field));return 'unknown' if v is None else str(v)
def bound(row):
 if not row['verified']:return 'unverified'
 return '='+str(row['last_fall_degree']) if row['last_fall_degree'] is not None else '>='+str(row['last_fall_lower_bound'])
table='\n'.join(f"| {x['key']} | {value(x,'added_rows')} | {value(x,'attempted_products')} | {value(x,'pending_products')} | {bound(x)} |" for x in rows)
stops=', '.join(f'{k}: {v}' for k,v in sorted(pub['stop_reasons'].items()))
checks=', '.join(f'{k}: {v}' for k,v in pub['manifest_entries_before_and_after'].items())
interrupted=pub.get('restored_verification_interrupted_attempts',0)
low=sum(pub['added_rows_by_degree'][:4])
md=f'''# Hybrid quartic closure on retained n83 m3 systems

Observation report for `n83-degree4-quartic-20261029-r1`. **{pub['fresh_replay_cases']}/12 producer/fresh-process pairs and {pub['s3_restored_replay_cases']}/12 S3-restored replays passed.** Independent scientific review remains pending. Native totals are **{pub['added_rows']} new independent rows**, including **{low} of degree at most three**. Full V4 normal-form closures: **{pub['complete_v4_cases']}**. Global LFD upper certificates: **{pub['global_lfd_upper_bound_cases']}**; exact LFD certificates: **{pub['exact_lfd_cases']}**. The lower bound4 is inherited from the verified strict-witness dataset.

| Retained case | New rows | Product attempts | Products pending | LFD |
|---|---:|---:|---:|---:|
{table}

![Admitted rows by polynomial degree](added-rows.svg)

The plot shows exact native counts under the frozen storage cap, not a ranking of factor bases. K/B denote Koblitz/general binary; P/N denote planted/natural targets. Repeated natural original/baseline inputs are not independent draws. There is no sampling error bar or performance comparison.

## What was computed

The inputs retain n83, m3, ell27 and 164 Boolean variables. Actual point manifests, targets and original equations are unchanged; each point base remains fixed across m2–9 within its dataset/family. The original 48-row fixed-base degree-of-regularity view is a separate, unchanged result. This batch does not measure prime-field or higher-m LFD.

The native program first replays complete V2 and four immutable parent layers: the V3 prefix and completed suffix, strict degree-four coordinate witnesses, and their restricted propagation. It then queues variable products of all added parent rows of degree at most3, followed by all complete-V3 cubics. Variables are outermost in the initial queue; every new degree<=3 row receives an urgent all-variable task. Quartic rows are not multiplied. Every allowed ordinary product has degree at most4 before reduction by the original Boolean field equations.

Each row contains a dense degree<=3 block plus sorted quartic column indices. The trace records every ordered reducer and insertion. A candidate that crosses the payload limit is saved separately and its source remains pending. A weight cap would preserve the partial reduction and reducer prefix. Neither blocked candidate is counted as an admitted basis row. Full queue exhaustion would certify the Boolean normal-form image of V4, but would not by itself prove a global LFD upper bound.

Recorded totals: {pub['attempted_products']} attempted products, {pub['completed_products']} completed products, {pub['pending_products']} pending products and {pub['new_basis_payload_bytes']} admitted row-record bytes. Stops: {stops}. These sums are labelled arithmetic on the per-case native counters. The smaller capped row count for a case cannot establish lower PDP regularity.

## Certificate and validation boundaries

Only an admitted unit or exhaustive affine ideal equality can provide the implemented global upper4 certificate. The affine check enumerates all assignments only when at most 12 variables remain free. In this batch no new lower-degree rows were found in the recorded prefixes; the native per-case affine results describe whether any assignment check was possible. Unknown upper and exact degrees stay null. A cap supplies no lower5, no full V4 result and no universal FFD/LFD theorem.

All 28 native tests pass. Independent tiny dense V4 calculations agree with completed hybrid closures. Controls cover a genuine quartic-to-linear cancellation, exact4 for xyz+1, a negative xyz case, every cap, fill-in interrupted during reduction, complete queue coverage and priority, unit/affine gates, and altered/truncated/tail records. Software replay is distinct from independent mathematical review and full algebraic formalization.

The retained multiplier identity is discussed in [Semaev section4.5](https://arxiv.org/html/1504.01175). Degree-bounded closure and global fall definitions are given by [Caminata–Gorla Definition1.5 and section2](https://arxiv.org/html/2112.05579). The exact algorithm and upper-certificate scope are documented in [the frozen design](../../DEGREE4_QUARTIC_DESIGN.md).

## Reproduction and storage

Limits: 16,384 new independent rows; 2 GiB serialized new basis payload; 131,072 product attempts; 1,048,576 active quartic nonzeros; 8 GiB virtual memory; one native process; 1800s producer/fresh replay plus 5s grace; 48000s outer. Every case was attempted once. Batch boundaries in UTC were {pub['batch_started_at']} to {pub['batch_finished_at']}; these include preparation and replay and are not performance measurements. The existing-host L0 fallback retains the failed EKS storage-probe evidence without claiming a diagnosed application cause, IAM changes or CRIU recovery.

Before/after restored-replay manifest counts: {checks}. The complete raw archive, all {pub['immutable_cumulative_summaries']} cumulative summaries, blocked candidates and compact pending queues remain retained. Restored verification interrupted attempts retained: {interrupted}; completed statuses were verified and skipped during resume, and the interrupted log was preserved. Production was not rerun. Native replay writes no dataset files.

Source `{p['freeze']['commit']}`; protocol `{p['freeze']['protocol_sha256']}`; binary `{p['arms'][0]['instrument']['binary_sha256']}`; committed data before this report `{head}`. Raw object: `{storage['object']}`. The [format map](../../DEGREE4_QUARTIC_FORMAT.json) specifies headers, offsets, row IDs, hashes, queue cursors and null semantics. The [archive README](../../degree4-quartic-archives/README.md) gives the replay command; all four immutable parent archives are required.

The broad goal remains active: all three n83 families, m2–9, FFD/LFD characterization and held-out factor bases that reduce PDP regularity. These capped membership prefixes do not establish an elite-base advantage.
'''
(r/'report.md').write_text(md)
typ_table='\n'.join(f"[{x['key'].replace('-m3','')}], [{value(x,'added_rows')}], [{value(x,'attempted_products')}], [{value(x,'pending_products')}], [{bound(x)}]," for x in rows)
typ=f'''#set page(paper:"a4",margin:(x:15mm,y:14mm),footer:context align(right,text(size:8pt,fill:rgb("64748b"))[N83 / quartic closure · #counter(page).display()]))
#set text(font:"DejaVu Sans",size:10pt)
#set par(leading:.45em)
#text(size:21pt,weight:"bold")[Hybrid quartic closure]

#text(size:10pt,fill:rgb("116d8a"))[n83-degree4-quartic-20261029-r1 · retained m3 systems]

*{pub['fresh_replay_cases']}/12 producer/fresh pairs and {pub['s3_restored_replay_cases']}/12 S3-restored replays passed.*
{pub['added_rows']} new independent rows; {low} of degree at most three.
Full V4 closures: *{pub['complete_v4_cases']}*. Global upper certificates: *{pub['global_lfd_upper_bound_cases']}*.
Exact LFD certificates: *{pub['exact_lfd_cases']}*. Lower4 is inherited; review pending.

#text(size:8pt)[#table(columns:(2.5fr,.7fr,.8fr,1fr,.5fr),inset:4pt,stroke:.35pt+rgb("d3dae4"),
[Retained case], [New rows], [Attempts], [Pending], [LFD],
{typ_table}
)]

#image("added-rows.svg",width:100%)
#text(size:8pt,fill:rgb("475569"))[Exact capped counts, not factor-base quality. K/B: Koblitz/general binary;
P/N: planted/natural. Repeated inputs are not independent draws.]

*Unchanged inputs.* n83, m3, ell27,164 Boolean variables. Actual point manifests
remain fixed across m2–9; targets and equations are unchanged. The 48-row
fixed-base regularity view is separate. No prime/higher-m LFD result is added.

*Stops.* {stops.replace('_',' ')}. Blocked candidates and pending work remain saved.

#pagebreak()
#text(size:18pt,weight:"bold")[Claim boundaries and reproducibility]

*The calculation.* Replay complete V2 and four immutable parent layers. Queue
products of added parent rows of degree at most3 and all complete-V3 cubics.
New degree-at-most-three consequences receive urgent variable-product tasks.
Each ordinary product has degree at most4 before Boolean field reduction.
Dense lower blocks and sorted quartic indices retain every row and reducer.
A blocked candidate remains separate and does not consume its queued source.

*Completion is separate from a global bound.* Exhausting the full product queue
would certify Boolean normal-form V4. A global upper4 still requires an admitted
unit or exhaustive affine ideal equality (at most 12 free variables). With the
inherited strict lower4, either certificate gives exact4. Caps supply no lower5
or universal FFD/LFD conclusion. Unknown upper/exact degrees remain null.

*Validation.* All 28 native tests pass: independent dense V4 equality, genuine
quartic cancellation, positive/negative controls, every cap, fill-in interruption,
queue coverage, global upper gates and altered/truncated records. Independent
scientific review and full algebraic formalization remain pending.

*Restoration.* Manifest counts verified before and after replay: {checks}.
All {pub['immutable_cumulative_summaries']} cumulative summaries are retained.
Interrupted restored-verification attempts preserved: {interrupted}; successful
statuses were checked and skipped on resume. Production was not repeated.

*Bounds.* 16384 new rows; 2 GiB new basis payload; 131,072 attempts; 1,048,576 active
quartic nonzeros; 8 GiB virtual memory; one native process; 1800s per producer/replay
plus 5s grace. Existing-host L0 fallback; no performance or CRIU claim.
Diagnostic UTC batch boundaries: {pub['batch_started_at']} to {pub['batch_finished_at']}.

*Reproduce.* Read DEGREE4_QUARTIC_FORMAT.json and the archive README.
Four immutable parent archives are required. The storage JSON and unified export
bind raw objects and every verification record.
#text(size:8pt)[Source: `{p['freeze']['commit']}` #linebreak()
Protocol: `{p['freeze']['protocol_sha256']}` #linebreak()
Data before report: `{head}`]

*Sources.* #link("https://arxiv.org/html/1504.01175")[Semaev section4.5];
#link("https://arxiv.org/html/2112.05579")[Caminata–Gorla Definition1.5 and section2].
The full n83 three-family m2–9 FFD/LFD and held-out elite-base objective remains open.
'''
(r/'report.typ').write_text(typ);shutil.copy2(__file__,r/'render.py')
typst.compile(str(r/'report.typ'),output=str(r/'report.pdf'))
doc=fitz.open(r/'report.pdf')
for i,page in enumerate(doc):page.get_pixmap(matrix=fitz.Matrix(1.2,1.2)).save(support/f'report-page-{i+1}.png')
print(json.dumps({'directory':str(r),'pages':len(doc),'pdf_sha256':sha(r/'report.pdf'),'data_commit':head}))
