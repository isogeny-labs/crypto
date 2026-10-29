"""Render only committed native data and verified publication metadata."""
from pathlib import Path
import json,hashlib,subprocess,shutil
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import typst,fitz
s=Path('research/n83_candidate_pdp_20261028');support=Path('/home/ec2-user/n83-candidate-recovery/degree4-propagation-support')
read=lambda p:json.loads(p.read_text());sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
summary=read(s/'evidence/degree4-propagation/summary.json');pub=read(s/'evidence/degree4-propagation-publication.json');protocol=read(s/'protocol-degree4-propagation.json');storage=read(s/'evidence/degree4-propagation-storage.json');rows=summary['rows']
head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()
for path in [s/'evidence/degree4-propagation/summary.json',s/'evidence/degree4-propagation-publication.json']:
    assert subprocess.check_output(['git','show',head+':'+str(path)])==path.read_bytes(),str(path)
stamp=pub['batch_started_at'].replace('-','').replace(':','')[:13]+'Z'
r=s/'reports'/('run-'+stamp);r.mkdir(parents=True,exist_ok=False)
(r/'plot-data.json').write_text(json.dumps({'schema':'n83.degree-four-propagation-plot/v1','data_commit':head,'summary_sha256':sha(s/'evidence/degree4-propagation/summary.json'),'rows':rows},indent=2)+'\n')
plt.rcParams.update({'font.family':'DejaVu Sans','font.size':9,'svg.fonttype':'none'})
fig,ax=plt.subplots(figsize=(10,3.2),layout='constrained');labels=[];largest=1
for i,row in enumerate(rows):
    labels.append(row['key'].replace('-m3','').replace('original','orig').replace('baseline','base').replace('koblitz','K').replace('binary','B').replace('planted','P').replace('natural','N'))
    if row['verified']:
        counts=row['state']['added_rows_by_degree'];low=sum(counts[:3]);cubic=counts[3];largest=max(largest,low+cubic)
        ax.bar(i,low,color='#c56e24',label='Degree 0–2' if i==0 else None)
        ax.bar(i,cubic,bottom=low,color='#116d8a',label='Degree 3' if i==0 else None)
        ax.text(i,low+cubic+3,str(low+cubic),ha='center',fontsize=8)
    else:ax.text(i,1,'unverified',rotation=90,ha='center')
ax.set_xticks(range(len(rows)),labels,rotation=32,ha='right');ax.set_ylim(0,largest*1.15+5)
ax.set_ylabel('New independent rows (count)');ax.set_title('Verified V4 consequences admitted after complete V3',fontsize=10)
ax.grid(axis='y',alpha=.15);ax.spines[['top','right']].set_visible(False);ax.legend(loc='upper right',fontsize=8)
fig.savefig(r/'added-rows.svg');fig.savefig(r/'added-rows.png',dpi=180);plt.close(fig)
svg=r/'added-rows.svg';svg.write_text('\n'.join(x.rstrip() for x in svg.read_text().splitlines())+'\n')
def value(row,field):
    if not row['verified']:return 'unverified'
    x=row.get(field,row.get('state',{}).get(field));return 'unknown' if x is None else str(x)
def bound(row):
    if not row['verified']:return 'unverified'
    return '=' + str(row['last_fall_degree']) if row['last_fall_degree'] is not None else '>=' + str(row['last_fall_lower_bound'])
table='\n'.join(f"| {x['key']} | {value(x,'added_rows')} | {value(x,'processed_operations')} | {value(x,'linear_rank')} | {bound(x)} |" for x in rows)
stops=', '.join(f'{k}: {v}' for k,v in sorted(pub['stop_reasons'].items()))
checks=', '.join(f'{k}: {v}' for k,v in pub['manifest_entries_before_and_after'].items())
md=f'''# Propagating degree-four consequences on retained n83 m3 systems

Observation report for `n83-degree4-propagation-20261029-r1`; independent scientific review is pending. **{pub['fresh_replay_cases']}/12 producer/fresh-process pairs and {pub['s3_restored_replay_cases']}/12 S3-restored replays passed.** Counts below come from native results; totals sum verified cases. Global upper certificates: **{pub['global_lfd_upper_bound_cases']}**; exact LFD certificates: **{pub['exact_lfd_cases']}**. Lower4 is inherited from the prior strict-witness dataset. No unrestricted V4 computation or elite-base advantage is claimed.

| Retained case | New rows | New operations | Linear rank | LFD |
|---|---:|---:|---:|---|
{table}

![New independent rows by polynomial degree](added-rows.svg)

Graph: exact counts for the 12 retained identities in this run; K/B denote Koblitz/general binary and P/N planted/natural. Deterministic counts have no sampling error bar. Natural original/baseline inputs repeat; the identities are not independent draws and row counts do not rank base quality.

Native totals: {pub['added_rows']} new independent rows, {pub['independent_seed_rows']} independent seed insertions and {pub['processed_operations']} new operations. Stops: {stops}. Unit certificates: {pub['unit_certificate_cases']}; affine-completion certificates: {pub['affine_certificate_cases']}. Missing or failed cases retain their exact status and partial data; unknown upper and exact degrees remain null.

## Procedure and claim boundary

All cases retain n83, m3, ell27 and 164 Boolean variables, with unchanged saved points, targets and equations. Each actual point-base manifest remains fixed across m2–9 within its dataset/family. Original and paired planted targets differ; the paired targets share low26 support. The original 48-row fixed-base degree-of-regularity view remains separate and unchanged.

Replay complete V2, the immutable V3 prefix, its completed suffix and all249 strict degree-four coordinate candidates. Insert the candidates against the evolving complete-V3 basis. Queue variable multiples of newly inserted rows only when their degree is at most2; each such ordinary product has degree at most3 and remains in V4. Save every reducer, inserted row and remaining queue. Cubic multiplication is outside this procedure.

Only two gates can certify a global upper4. An admitted unit proves1 belongs to V4. Alternatively, extract linear constraints and, when at most12 variables remain free, enumerate every affine Boolean assignment and check every original equation. If all satisfy the equations, the admitted linear-plus-Boolean ideal equals the original ideal; degree-compatible reductions give the global upper. Together with the inherited strict lower4, either gate gives exact LFD4 for that saved system.

A restricted queue exhaustion or resource cap supplies neither a global upper bound nor a lower5. It is not a complete V4 computation. The lower bound is inherited and explicitly labelled. Runtime positive control xyz+1 reaches exact4 with independent tiny V3/V4 checks; negative control xyz produces no upper certificate. All19 native tests pass, covering both caps, unit/dependent seeds, affine certificates with pending queues, and tampered/truncated serialization.

The underlying multiplier identity is established in [Semaev section4.5](https://arxiv.org/html/1504.01175). The global closure and fall definitions are in [Caminata--Gorla Definition1.5 and Theorems2.8–2.9](https://arxiv.org/html/2112.05579). The complete upper argument is written in DEGREE4_PROPAGATION_DESIGN.md; tests and deterministic replay are distinct from independent scientific review.

## Reproduction and storage

Frozen limits:65536 total rows including complete V3,262144 new operations,12 free variables for exhaustive affine checks,8GiB virtual memory,one native process,1800s per producer/replay plus5s grace,48000s outer; no automatic retry. Diagnostic batch boundaries: {pub['batch_started_at']} to {pub['batch_finished_at']}. These include setup and replay and are not a performance measurement. Existing-host L0 fallback follows the retained failed EKS storage probe, whose application cause is unconfirmed; no IAM or capacity changes.

Manifest payload counts checked before and after restored replay: {checks}. All {pub['immutable_cumulative_summaries']} cumulative summaries, operations, row bytes, pending queues and affine checks are retained. Every successful producer was replayed using S3-restored raw data and three previously restored immutable parents. Native replay writes no dataset files.

Source `{protocol['freeze']['commit']}`; protocol `{protocol['freeze']['protocol_sha256']}`; binary `{protocol['arms'][0]['instrument']['binary_sha256']}`; committed data before report `{head}`. Raw object: `{storage['object']}`. Read DEGREE4_PROPAGATION_FORMAT.json for header offsets, graded colex column order, row IDs, JSON fields, partial records and null semantics. The archive README gives the replay command. All three parent archives are required.

The full objective remains active: all three n83 families, m2–9, FFD/LFD characterization and held-out factor-base selection that lowers PDP regularity. No new regularity reduction follows from this propagation dataset.
'''
(r/'report.md').write_text(md)
typ_table='\n'.join(f"[{x['key'].replace('-m3','')}], [{value(x,'added_rows')}], [{value(x,'processed_operations')}], [{value(x,'linear_rank')}], [{bound(x)}]," for x in rows)
typ=f'''#set page(paper:"a4",margin:(x:15mm,y:14mm),footer:context align(right,text(size:8pt,fill:rgb("64748b"))[N83 / degree-four propagation · #counter(page).display()]))
#set text(font:"DejaVu Sans",size:10pt)
#set par(leading:.45em)
#text(size:21pt,weight:"bold")[Degree-four consequence propagation]

#text(size:10pt,fill:rgb("116d8a"))[n83-degree4-propagation-20261029-r1 · retained m3 systems]

Observation report; independent scientific review is pending.
*{pub['fresh_replay_cases']}/12 producer/fresh pairs and {pub['s3_restored_replay_cases']}/12 S3-restored replays passed.*
Global upper certificates: *{pub['global_lfd_upper_bound_cases']}*. Exact LFD certificates: *{pub['exact_lfd_cases']}*.
Lower4 is inherited. No unrestricted V4 or elite-base advantage is claimed.

#text(size:8pt)[#table(columns:(2.5fr,.7fr,.8fr,.7fr,.6fr),inset:4pt,stroke:.35pt+rgb("d3dae4"),
[Retained case], [New rows], [Operations], [Linear rank], [LFD],
{typ_table}
)]

#image("added-rows.svg",width:100%)
#text(size:8pt,fill:rgb("475569"))[12 retained identities; exact counts, no sampling error bars. K/B: Koblitz/general binary;
P/N: planted/natural. Repeated natural inputs are not independent draws.]

*Unchanged inputs.* n83, m3, ell27,164 Boolean variables and exact saved point
manifests. Bases remain fixed across m2–9. Points, targets and equations are
unchanged. The original fixed-base regularity view is a separate result.

*Recorded totals.* {pub['added_rows']} new rows; {pub['processed_operations']} operations.
Stops: {stops.replace('_',' ')}.

#pagebreak()
#text(size:18pt,weight:"bold")[Certificate gates and reproducibility]

*Replay the parents.* Verify complete V2, both complete V3 layers, and the
saved249 strict degree-four coordinate candidates. Insert seeds against the
evolving basis. Multiply newly admitted degree0–2 rows by each variable; these
products remain in V4. Retain every row, reducer, pending source and assignment.
Cubic multiplication is outside this procedure.

*Global upper gate.* An admitted unit gives1 in V4. Alternatively, solve all
admitted linear constraints and exhaustively check every affine Boolean
assignment against every original equation (at most12 free variables). If all
pass, the linear-plus-field ideal equals the original ideal. Degree-compatible
reductions give global upper4. Combined with inherited lower4, this gives exact4.

*Unknown means unknown.* A restricted queue stall or cap supplies no upper,
no lower5 and no complete V4. Unit certificates: {pub['unit_certificate_cases']};
affine certificates: {pub['affine_certificate_cases']}. Counts do not rank base quality.

*Validation.* All19 native tests pass. The positive xyz+1 control reaches exact4;
the xyz control has no upper certificate. Independent tiny V3/V4 controls,
both caps, unit/dependent seeds, pending-queue certificates and altered or
truncated traces are checked. Software replay is not independent scientific review.

*Restoration.* Manifest checks before and after replay cover
{checks}. {pub['immutable_cumulative_summaries']} cumulative summaries and every
partial payload are retained. Native replay writes no dataset files.

*Limits.* 65536 total rows,262144 operations,8GiB,one process,1800s per
producer/replay plus5s grace. Existing-host L0 fallback after retained failed
EKS storage probe; no performance claim. Batch boundaries:
{pub['batch_started_at']} to {pub['batch_finished_at']}.

*Reproduce.* The archive README gives the exact command; all three parent
archives are required. DEGREE4_PROPAGATION_FORMAT.json documents bytes and
nulls. See evidence/degree4-propagation-storage.json and the unified export.
#text(size:8pt)[Source: `{protocol['freeze']['commit']}` #linebreak()
Protocol: `{protocol['freeze']['protocol_sha256']}` #linebreak()
Data before report: `{head}`]

*Literature.* #link("https://arxiv.org/html/1504.01175")[Semaev section4.5];
#link("https://arxiv.org/html/2112.05579")[Caminata–Gorla Definition1.5 and Theorems2.8–2.9].
The full n83 three-family m2–9 FFD/LFD and held-out elite-base goal remains open.
'''
(r/'report.typ').write_text(typ);shutil.copy2(__file__,r/'render.py')
typst.compile(str(r/'report.typ'),output=str(r/'report.pdf'))
doc=fitz.open(r/'report.pdf')
for i,page in enumerate(doc):page.get_pixmap(matrix=fitz.Matrix(1.2,1.2)).save(support/f'report-page-{i+1}.png')
print(json.dumps({'directory':str(r),'pages':len(doc),'pdf_sha256':sha(r/'report.pdf'),'data_commit':head}))
