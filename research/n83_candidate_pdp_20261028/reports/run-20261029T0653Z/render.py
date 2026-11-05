from pathlib import Path
import json,hashlib,subprocess,shutil
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from matplotlib.patches import Patch
import typst,fitz
s=Path('research/n83_candidate_pdp_20261028');support=Path('/home/ec2-user/n83-candidate-recovery/continue-support');read=lambda p:json.loads(p.read_text());sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
summary=read(s/'evidence/degree3-continuation/summary.json');pub=read(s/'evidence/degree3-continuation-publication.json');p=read(s/'protocol-degree3-continuation.json');storage=read(s/'evidence/degree3-continuation-storage.json');term=read(s/'evidence/degree3-continuation/terminal.json');rows=summary['rows'];n=len(rows);verified=sum(r['verified'] for r in rows)
stamp=pub['first_trial_utc'].replace('-','').replace(':','')[:13]+'Z';r=s/'reports'/('run-'+stamp);r.mkdir(parents=True,exist_ok=False);head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()
(r/'plot-data.json').write_text(json.dumps({'schema':'n83.m3-continuation-plot/v1','summary_sha256':sha(s/'evidence/degree3-continuation/summary.json'),'data_commit':head,'rows':rows},indent=2)+'\n')
plt.rcParams.update({'font.family':'DejaVu Sans','font.size':9,'svg.fonttype':'none'});fig,ax=plt.subplots(figsize=(10,3.4),layout='constrained');labels=[]
for i,row in enumerate(rows):
 labels.append(row['key'].replace('-m3','').replace('original','orig').replace('baseline','base').replace('koblitz','K').replace('binary','B').replace('planted','P').replace('natural','N'))
 if row['verified']:
  base=row['continuation']['prefix_rows'];ax.bar(i,base,color='#94a3b8');ax.bar(i,row['added_rows'],bottom=base,color='#087e8b' if row['closure_complete'] else '#d97706')
 else:ax.text(i,5000,'unverified',rotation=90,ha='center')
ax.set_ylabel('Independent rows');ax.set_xticks(range(n),labels,rotation=35,ha='right');ax.set_title('n83-degree3-continuation-20261029-r1 · 12 retained input identities',fontsize=10)
ax.legend(handles=[Patch(facecolor='#94a3b8',label='Immutable parent'),Patch(facecolor='#087e8b',label='New rows; complete V3'),Patch(facecolor='#d97706',label='New rows; capped')],fontsize=8,ncols=3,loc='upper left');ax.set_ylim(0,max([r['closure_rank'] or 0 for r in rows]+[1])*1.24);ax.grid(axis='y',alpha=.15);ax.spines[['top','right']].set_visible(False);fig.savefig(r/'continuation-ranks.svg');plt.close(fig);svg=r/'continuation-ranks.svg';svg.write_text('\n'.join(x.rstrip() for x in svg.read_text().splitlines())+'\n')
def value(x):return 'unknown' if x is None else str(x)
def done(x):return 'yes' if x is True else 'no' if x is False else 'unknown'
mdrows='\n'.join(f"| {x['key']} | {value(x['added_rows'])} | {value(x['closure_rank'])} | {done(x['closure_complete'])} | {value(x['last_fall_upper_bound'])} |" for x in rows)
md=f'''# Retained n83 m3 closure continuation

Run `n83-degree3-continuation-20261029-r1`. Producer observations only; independent scientific review is pending. {verified}/12 cases passed production and fresh replay; {pub['downloaded_replay_cases']} S3-restored replays passed. {pub['full_v3_closures']} queues exhausted, {pub['capped_cases']} remained capped. Exact LFD is certified for {pub['exact_lfd_cases']} cases. Full V3 alone is not a global LFD upper bound.

| Retained case | New rows | Total rank | Complete V3 | LFD upper |
|---|---:|---:|---|---|
{mdrows}

![Immutable parent and new suffix rows](continuation-ranks.svg)

There are {pub['new_admitted_rows']} newly certified rows and {pub['new_processed_operations']} new processed queue sources, derived by summing those native fields over verified cases. The parent contributes 32768 rows per case. {pub['inherited_fall_witness_cases']} strict-fall witnesses are inherited from the parent; {pub['new_suffix_fall_witness_cases']} reported witnesses first occur in the suffix. Inherited lower bounds are not newly discovered falls. No rank is interpreted as degree of regularity or solving degree, and differences between these retained inputs do not rank factor-base quality.

All twelve original/baseline/spike40 Koblitz/binary, planted/natural m3 identities are preserved. Each system has 164 Boolean variables and retains its fixed 2^27-candidate per-curve base used across m2..9. No point, target, equation or random draw changed. Original and paired planted targets differ; paired planted targets share low26 support. Repeated natural sources are not independent samples.

The native instrument first replays complete V2 and the original V3 prefix, validates the parent's exact verdict and four file hashes, and reconstructs its queue. Only new rows and operations are written. Global IDs continue across the boundary. Fresh replay recomputes each suffix operation canonically, verifies all bytes and the exact first stopping boundary. Ten tests pass, including all735295 column indices, ordinary-degree guards, byte-identical concatenation versus uninterrupted toy runs, both cap kinds and rejection of wrong parent/offset/operations/trailing bytes/false completion. All runtime controls pass. Frozen parent code and data remain unchanged.

The frozen limits are65536 total rows and262144 total operations including the parent,8GiB,one process,1200seconds per producer/replay plus5seconds termination grace,30000seconds batch,no retries. The command was `bash research/n83_candidate_pdp_20261028/run-degree3-continuation.sh /home/ec2-user/n83-candidate-recovery/target/release/degree3_continue /home/ec2-user/n83-candidate-recovery/degree3-continuation-r1`. The original output is sealed; replay from restored archives instead of repeating that invocation. Batch{term['started_at']}–{term['finished_at']}; timestamps are diagnostic, CPU and qualified wall timing were not measured.

The EKS storage probe ended exit1; MCP log retrieval failed and local kubectl could not assume the launcher role. Its application failure cause is unconfirmed. Existing-host fallback changed no IAM or capacity. This is algebraic trace continuation under a new protocol, not process-memory/CRIU restoration; physical process-restoration count is0.

All{pub['raw_manifest_entries_verified']} new raw manifest entries and{pub['parent_manifest_entries_before_and_after']} parent entries match before and after restored replay. Every suffix, capped/partial output, operation, control, immutable progress summary and error is retained. Parent archive `{p['extensions']['parent_archive']['sha256']}` remains independently stored. New archive `{storage['object']}`; SHA256 `{storage['archive']['sha256']}`. Raw new bytes:{pub['raw_bytes']}; suffix dense-row bytes:{pub['suffix_basis_bytes']}. See [format](../../DEGREE3_CONTINUATION_FORMAT.json), [design](../../DEGREE3_CONTINUATION_DESIGN.md), [native summary](../../evidence/degree3-continuation/summary.json), [export](../../experiment-degree3-continuation.export.json), [storage](../../evidence/degree3-continuation-storage.json) and [publication checks](../../evidence/degree3-continuation-publication.json).

Source `{p['freeze']['commit']}`; protocol `{p['freeze']['protocol_sha256']}`; data `{head}`. Source/protocol and S3 checkpoint preceded production. Data were committed and pushed before this report. The prior fixed-base solver attempts, exact m2 regularities K28/B28/P42 and larger-m binary intervals remain unchanged. Higher-degree fall witnesses require nonmembership in complete lower spaces; global LFD bounds, exact larger-m regularity and held-out elite-base selection remain open. Literature basis: [Caminata–Gorla, definitions1.3–1.5 and Example2.4](https://arxiv.org/html/2112.05579).
'''
# Preserve the native wording in records; improve only spacing in report prose.
for a,b in {'all735295':'all 735295','are65536':'are 65536','and262144':'and 262144',',8GiB,one process,1200seconds':', 8 GiB, one process, 1200 seconds','plus5seconds':'plus 5 seconds',',30000seconds batch,no':', 30000 seconds batch, no','Batch{':'Batch {','Batch2026':'Batch 2026','ended exit1':'ended exit 1','is0':'is 0','All{':'All {','All'+str(pub['raw_manifest_entries_verified']):'All '+str(pub['raw_manifest_entries_verified']),'and'+str(pub['parent_manifest_entries_before_and_after']):'and '+str(pub['parent_manifest_entries_before_and_after']),'bytes:':'bytes: ','definitions1.3':'definitions 1.3','Example2.4':'Example 2.4'}.items():md=md.replace(a,b)
view=read(s/'FIXED_BASE_REGULARITY.json'); grouped={}
for row in view['rows']:
 key=(row['family'],row['m']); state=(row['status'],row['dreg_exact'],row['dreg_lower_bound'],row['dreg_upper_bound'])
 assert key not in grouped or grouped[key]==state
 grouped[key]=state
def degree_cell(family,m):
 state,exact,lo,hi=grouped[(family,m)]
 return str(exact)+' (exact)' if exact is not None else 'No finite top vanishing degree' if state=='no_finite_top_vanishing_degree' else str(lo)+'–'+str(hi)+' (bounds)'
fixed_md='\n'.join('| '+str(m)+' | '+degree_cell('koblitz',m)+' | '+degree_cell('binary',m)+' | '+degree_cell('prime',m)+' |' for m in range(2,10))
md+='\n## Original fixed-base regularity table\n\nThis is a view of previously published certificates on the original fixed bases, not a new solver run. All 48 source hashes and base identities match. Each family/m row below represents both saved target kinds; their values agree. The unchanged original point counts are Koblitz 134220785, general binary 134227133 and prime 134213656, from 134217728 candidate x-values each.\n\n| m | Koblitz | General binary | Prime |\n|---|---|---|---|\n'+fixed_md+'\n\nThe degree convention is the first vanishing degree of the quotient by the original highest homogeneous forms. Positive-dimensional quotients have no finite such degree in this presentation; this says nothing about infinite Castelnuovo–Mumford regularity. Bounds remain bounds. These values are distinct from FFD, LFD and solving degree. The original bounded solver sweep attempted all 48 inputs in 96 runs: 4 affine completions, 63 timeouts and 29 memory limits. Those limits are not degree certificates. Read [the 48-row CSV](../../FIXED_BASE_REGULARITY.csv) and [JSON field semantics and source hashes](../../FIXED_BASE_REGULARITY.json).\n'
shutil.copy2(s/'FIXED_BASE_REGULARITY.json',r/'fixed-base-regularity-view.json')
(r/'report.md').write_text(md)
tr='\n'.join(f"[{x['key'].replace('-m3','')}], [{value(x['added_rows'])}], [{value(x['closure_rank'])}], [{done(x['closure_complete'])}], [{value(x['last_fall_upper_bound'])}]," for x in rows)
typ=f'''#set page(paper:"a4",margin:(x:15mm,y:14mm),footer:context align(right,text(size:8pt,fill:rgb("64748b"))[N83 / m3 continuation · #counter(page).display()]))
#set text(font:"DejaVu Sans",size:10pt)
#set par(leading:.5em)
#text(size:21pt,weight:"bold")[Retained n83 m3 closure continuation]

#text(fill:rgb("087e8b"))[{verified}/12 verified · {pub['full_v3_closures']} complete V3 spaces · {pub['exact_lfd_cases']} exact LFD values]

Run `n83-degree3-continuation-20261029-r1`. Producer observations;
independent scientific review pending. Full V3 alone is not a global LFD upper
certificate. All original parent data are preserved unchanged.

#table(columns:(2.9fr,.8fr,.8fr,.8fr,.8fr),inset:4pt,stroke:.4pt+rgb("d3dae4"),
[Retained case], [New rows], [Total], [V3 full], [LFD upper],
{tr}
)

#image("continuation-ranks.svg",width:100%)

*What changed.* {pub['new_admitted_rows']} new independent rows and
{pub['new_processed_operations']} new queue operations, summed from verified
native records. Each parent has 32768 rows. {pub['inherited_fall_witness_cases']}
reported strict-fall witnesses are inherited; {pub['new_suffix_fall_witness_cases']}
first occur in the suffix. A complete V3 space can support later nonmembership
checks, but does not by itself establish LFD stabilization. No rank is called a
regularity value or base-quality improvement.

#pagebreak()
#text(size:18pt,weight:"bold")[Continuation, verification and preservation]

*Twelve retained identities.* Original, baseline and spike40; Koblitz/general
binary; planted/natural. Each m3 system has 164 Boolean variables and the fixed
2^27-candidate base reused across m2..9. No point, target, equation or random
sample changed. Original and paired planted targets differ; repeated natural
inputs are not independent draws. Candidate x-coordinate decompositions still
require curve-lift/sign verification.

*Exact continuation.* Replay complete V2 and the full parent prefix. Check
its source, verdict, four file hashes, row/operation counts and pending queue.
Only new rows and operations are written. IDs remain global. Fresh replay
recomputes the exact reduction sequence and checks every suffix row byte and
the first stopping boundary. The suffix header has 44 bytes; each production
row has 11489 little-endian words, or 91912 bytes. The format map defines all
735295 columns, offsets, references and null conventions.

*Limits and controls.* Stop at queue exhaustion, 65536 cumulative rows or
262144 cumulative operations. One process, 8 GiB, 1200 seconds per production
or replay plus 5 seconds grace, 30000 seconds batch; no retries. Ten native
tests pass, including byte-identical concatenation with uninterrupted toy runs,
both cap types and parent/offset/operation/trailing-byte/completion tampering.
Runtime controls pass. A quadratic outside complete V2 gives lower3; a global
unit or affine-completion argument is separately required for upper3.

*Preservation.* {pub['downloaded_replay_cases']} S3-restored replays passed.
All {pub['raw_manifest_entries_verified']} suffix-run manifest entries and
{pub['parent_manifest_entries_before_and_after']} parent entries match before
and after replay. New raw bytes: {pub['raw_bytes']}. Every row, operation,
partial/cap state, original source, control and immutable progress snapshot
remains. Restore parent and suffix into separate directories and check both
manifests around replay. No earlier run is replaced.

*Execution and claim limits.* EKS storage probe exit1; log retrieval failed,
so its application cause is unconfirmed. Existing-host fallback, no IAM or
capacity change. No timing comparison; CPU and qualified wall times unmeasured.
Batch `{term['started_at']}` to `{term['finished_at']}`. This is application-level
trace continuation, not CRIU recovery. Independent scientific review remains
pending. Earlier regularity results are unchanged; exact higher-m LFD/regularity,
prime LFD and held-out elite-base selection remain open.

#text(size:8pt)[Source: `{p['freeze']['commit']}`]#linebreak()
#text(size:8pt)[Protocol: `{p['freeze']['protocol_sha256']}`]#linebreak()
#text(size:8pt)[Suffix archive: `{storage['archive']['sha256']}`]#linebreak()
#text(size:8pt)[Data commit: `{head}`]#linebreak()

Read `DEGREE3_CONTINUATION_FORMAT.json`, `DEGREE3_CONTINUATION_DESIGN.md`,
`experiment-degree3-continuation.export.json` and the storage/publication records.
Definitions and plateau limitation: Caminata–Gorla, arXiv:2112.05579,
Definitions 1.3–1.5 and Example 2.4.
'''
fixed_typ='\n'.join('['+str(m)+'], ['+degree_cell('koblitz',m)+'], ['+degree_cell('binary',m)+'], ['+degree_cell('prime',m)+'],' for m in range(2,10))
typ+=f'''#pagebreak()
#text(size:18pt,weight:"bold")[Original fixed-base regularity sweep]

A consolidated view of existing source-bound certificates. These are the
original 2^27-candidate bases reused across m=2–9. Each row covers both the
saved planted and natural target. All 48 system hashes and base identities
were checked when joining the source summaries. No new algebra was performed
for this table; the new continuation is described on the preceding pages.

#table(columns:(.35fr,1.1fr,1.1fr,1.6fr),inset:6pt,stroke:.4pt+rgb("d3dae4"),
[m], [Koblitz], [General binary], [Prime],
{fixed_typ}
)

*What the degree means.* First vanishing degree of the graded quotient by the
original highest homogeneous forms. The binary/Koblitz intervals remain
bounds; their exact degrees are unresolved. Positive-dimensional prime top
quotients have no finite first graded vanishing degree in this formulation.
This does not mean infinite Castelnuovo–Mumford regularity. FFD, LFD and solving
degree are separate invariants.

*Actual shared point bases.* From 134217728 candidate x-values per curve:
Koblitz 134220785 points, general binary 134227133 points and prime 134213656
points. Both affine signs are retained. The per-family point manifest stays
unchanged across m=2–9. The point binary layout, shard hashes and restoration
instructions are in `research/n83_fixed_factor_base_20261028/`.

*Execution coverage.* The original fixed-base solver sweep attempted all 48
systems in 96 solver runs: 4 affine completions, 63 timeouts and 29 memory
limits. Caps and the maximum degree of a final affine basis are not regularity
certificates. Subsequent certificates supply the exact/interval/structural
results above. Candidate-base differences still require held-out matched
sampling and do not establish an elite base.

*Direct data files.* `FIXED_BASE_REGULARITY.csv` has 48 rows, one per saved
original system. `FIXED_BASE_REGULARITY.json` defines fields, nulls, source
paths/hashes and the degree convention. This report retains a byte-identical
`fixed-base-regularity-view.json`. `DATA_INDEX.json` maps all older datasets;
`BASE_BINDINGS.json` records original and paired point-manifest identities.

*Scope remaining.* Higher-m exact binary regularity, prime LFD and a base
selection result remain open. The present continuation completes V3 spaces
but certifies no new exact LFD or regularity reduction.
'''
(r/'report.typ').write_text(typ);shutil.copy2(__file__,r/'render.py');typst.compile(str(r/'report.typ'),output=str(r/'report.pdf'));doc=fitz.open(r/'report.pdf')
for i,page in enumerate(doc):page.get_pixmap(matrix=fitz.Matrix(1.2,1.2)).save(support/('report-page-'+str(i+1)+'.png'))
print(json.dumps({'directory':str(r),'pages':len(doc),'pdf_sha256':sha(r/'report.pdf'),'data_commit':head}))
