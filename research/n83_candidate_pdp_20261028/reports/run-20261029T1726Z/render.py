from pathlib import Path
import json,hashlib,subprocess,shutil
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import typst,fitz
s=Path('research/n83_candidate_pdp_20261028');support=Path('/home/ec2-user/n83-candidate-recovery/degree4-witness-support')
read=lambda p:json.loads(p.read_text());sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
summary=read(s/'evidence/degree4-witness/summary.json');pub=read(s/'evidence/degree4-witness-publication.json');protocol=read(s/'protocol-degree4-witness.json');storage=read(s/'evidence/degree4-witness-storage.json');rows=summary['rows']
head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()
stamp=pub['batch_started_at'].replace('-','').replace(':','')[:13]+'Z'
r=s/'reports'/('run-'+stamp);r.mkdir(parents=True,exist_ok=False)
(r/'plot-data.json').write_text(json.dumps({'schema':'n83.degree-four-witness-plot/v1','data_commit':head,'summary_sha256':sha(s/'evidence/degree4-witness/summary.json'),'rows':rows},indent=2)+'\n')
plt.rcParams.update({'font.family':'DejaVu Sans','font.size':9,'svg.fonttype':'none'})
fig,ax=plt.subplots(figsize=(10,3.4),layout='constrained');labels=[]
colors={'original':'#116d8a','baseline':'#667f93','spike40':'#7952a3'}
for i,row in enumerate(rows):
    labels.append(row['key'].replace('-m3','').replace('original','orig').replace('baseline','base').replace('koblitz','K').replace('binary','B').replace('planted','P').replace('natural','N'))
    if row['verified']:
        ax.bar(i,row['strict_candidates'],color=colors[row['dataset']]);ax.text(i,row['strict_candidates']+4,str(row['strict_candidates']),ha='center',fontsize=8)
    else:ax.text(i,15,'unverified',rotation=90,ha='center')
ax.set_xticks(range(len(rows)),labels,rotation=32,ha='right');ax.set_ylim(0,278);ax.axhline(249,color='#999999',linewidth=.7,linestyle='--')
ax.set_ylabel('Strict candidates out of 249');ax.set_title('Degree-four consequences outside complete V3 · exact retained inputs',fontsize=10)
ax.grid(axis='y',alpha=.15);ax.spines[['top','right']].set_visible(False)
fig.savefig(r/'strict-candidate-counts.svg');fig.savefig(r/'strict-candidate-counts.png',dpi=180);plt.close(fig)
svg=r/'strict-candidate-counts.svg';svg.write_text('\n'.join(x.rstrip() for x in svg.read_text().splitlines())+'\n')
def val(row,key):return str(row.get(key)) if row['verified'] and row.get(key) is not None else 'unknown'
mdtable='\n'.join(f"| {x['key']} | {val(x,'candidates')} | {val(x,'strict_candidates')} | {val(x,'v3_rank')} | >= {val(x,'last_fall_lower_bound')} |" for x in rows)
md=f'''# Strict degree-four falls on retained n83 m3 systems

Run `n83-degree4-witness-20261029-r1`. **{pub['fresh_replay_cases']}/12 producer/fresh-process pairs and {pub['s3_restored_replay_cases']}/12 S3-restored replays passed.** Native results record {pub['strict_candidates']} strict candidates among {pub['total_verified_candidates']} verified coordinate candidates. {pub['new_lfd_lower_four_cases']} exact saved systems acquire the new lower bound **LFD >= 4**; {pub['finite_family_null_cases']} have no strict witness in this family. Exact LFD and global upper bounds remain unknown. Independent scientific review is pending.

| Retained case | Candidates | Strict | Complete V3 rank | LFD bound |
|---|---:|---:|---:|---|
{mdtable}

![Strict candidate counts](strict-candidate-counts.svg)

K/B denote Koblitz/general binary; P/N denote planted/natural. Counts are deterministic, correlated coordinate witnesses, not independent samples or a ranking of factor-base quality. The old lower bound 3 is inherited; the strict degree4 witnesses are new.

All 12 original/baseline/spike40 Koblitz/binary, planted/natural m3 identities retain n83, ell27, 164 Boolean variables and their exact saved point manifests. Each base remains fixed across its m2--9 panel. No point, target, equation or random draw changed. Original and paired planted targets differ; paired planted targets share low26 support; natural inputs repeat across original/baseline and are not independent draws.

## Certificate mechanism

Assemble the complete first 83 saved equations into the field-valued F=S3(X0,X1,U), checking every coefficient including lower terms. For L=X0,X1,U, construct the ordinary product L*F, whose coordinate equations have degree<=4 derivations. Reduce with the original Boolean field equations, retaining ordinary polynomials and multilinear normal forms, and verify degree<=3. All83 coordinates per multiplier are tested.

Replay complete V2, the immutable V3 prefix and its completed suffix. Reduce each candidate against this complete V3, recording every reducer ID and the full remainder. A nonzero remainder proves a degree<=3 polynomial in V4 outside V3; hence LFD>=4. The finite-family test does not construct complete V4 or a global upper certificate. These data cannot distinguish LFD=4 from LFD>4.

The construction is the known multiplier identity in [Semaev section4.5](https://arxiv.org/html/1504.01175). The strictness criterion follows [Caminata--Gorla Definitions1.5/2.6 and Theorem2.8](https://arxiv.org/html/2112.05579). This phase checks nonmembership for the full retained chained systems; it does not assume a first-fall heuristic.

## Preservation and verification

Fourteen native tests passed, including independent tiny complete V3/V4 controls, field multiplication checked against polynomial long division, ordinary-degree restrictions, parent/trace completion and altered serialized witness rejection. Runtime xyz+1 yields3 strict candidates and xyz yields0. The initial control setup failure and every earlier dataset remain preserved.

All {pub['manifest_entries_before_and_after']['raw']} new raw files, {pub['manifest_entries_before_and_after']['prefix']} immutable-prefix files and {pub['manifest_entries_before_and_after']['suffix']} completed-suffix files passed manifest checks before and after restored replay. The dataset retains {pub['immutable_cumulative_summaries']} cumulative case summaries and {pub['immutable_block_summaries']} block summaries, all candidate/remainder rows and all ordinary/normal polynomial files. Restored parent trees retain their earlier S3 download provenance and were reverified here.

The diagnostic launcher interval is {pub['batch_started_at']} to {pub['batch_finished_at']}; it includes setup and fresh replay. One native process, 8 GiB virtual memory,1800 s per producer/replay plus 5 s grace. No clock was qualified for performance. The EKS inventory recovered after one timeout; the earlier n83 storage probe remains failed with unconfirmed cause. The recorded existing-host fallback changes no IAM or capacity settings.

Source commit: `{protocol['freeze']['commit']}`. Protocol identity: `{protocol['freeze']['protocol_sha256']}`. Binary SHA256: `{protocol['arms'][0]['instrument']['binary_sha256']}`. Data commit before report: `{head}`.

Raw object: `{storage['object']}`. Both parent archives listed in `evidence/degree4-witness-storage.json` are required for replay. `DEGREE4_WITNESS_FORMAT.json` documents header offsets, coefficient encodings, graded colex columns, reducer IDs, partial-file semantics and nulls. The native summary, per-case results, unified export and publication receipt retain exact hashes and outcomes.

The original objective remains open: all three n83 families, m2--9, exact FFD/LFD characterization and elite bases lowering PDP regularity. This batch adds strict m3 LFD lower certificates; it establishes no regularity reduction or elite-base advantage.
'''
(r/'report.md').write_text(md)
table='\n'.join(f"[{x['key'].replace('-m3','')}], [{val(x,'candidates')}], [{val(x,'strict_candidates')}], [{val(x,'v3_rank')}], [>= {val(x,'last_fall_lower_bound')}]," for x in rows)
typ=f'''#set page(paper:"a4",margin:(x:15mm,y:14mm),footer:context align(right,text(size:8pt,fill:rgb("64748b"))[N83 / strict degree-four witnesses · #counter(page).display()]))
#set text(font:"DejaVu Sans",size:10pt)
#set par(leading:.45em)
#text(size:21pt,weight:"bold")[Strict degree-four falls]

#text(size:11pt,fill:rgb("116d8a"))[Retained n83 m3 systems · original / baseline / spike40]

*{pub['fresh_replay_cases']}/12 producer and fresh-replay pairs passed; {pub['s3_restored_replay_cases']}/12 S3-restored replays passed.*
{pub['strict_candidates']} strict coordinate witnesses among {pub['total_verified_candidates']} verified candidates.
{pub['new_lfd_lower_four_cases']} systems acquire *LFD >= 4*. Exact LFD and global upper bounds remain unknown.
Independent scientific review is pending.

#text(size:8pt)[#table(columns:(2.5fr,.6fr,.6fr,.8fr,.7fr),inset:4pt,stroke:.35pt+rgb("d3dae4"),
[Retained case], [Tested], [Strict], [V3 rank], [LFD],
{table}
)]

#image("strict-candidate-counts.svg",width:100%)
#text(size:8pt,fill:rgb("475569"))[K/B: Koblitz/general binary; P/N: planted/natural.
Coordinate witnesses are correlated. Counts do not rank base quality.]

*Unchanged inputs.* Every case retains n83, m3, ell27, 164 Boolean variables and
its exact saved point-base manifest. No point, target or equation changed.
Original and paired planted targets differ; natural original/baseline sources repeat.
The former lower bound 3 is inherited; the degree4 witnesses are new.

#pagebreak()
#text(size:18pt,weight:"bold")[How to check the evidence]

*1. Check the source.* Assemble all 83 coordinates of the first saved cubic into
F=S3(X0,X1,U), including every lower term. Match the complete field polynomial
to the unchanged equation source.

*2. Check membership in V4.* Multiply F by X0,X1,U in the ordinary polynomial
ring. Every coordinate has a degree<=4 derivation. Reduce repeated variables
with the original Boolean equations; retain the correction provenance and
verify that each normal form has degree<=3. Test all 249 coordinates.

*3. Check nonmembership in V3.* Replay complete V2, the immutable V3 prefix and
completed suffix. Reduce each candidate against that complete basis. The files
retain candidate bytes, ordered reducer IDs and full remainders. A nonzero
remainder proves a strict degree4 fall and LFD>=4.

*What remains unknown.* The test does not construct complete V4. It cannot
distinguish LFD=4 from LFD>4. An allzero finite family would give no global upper
bound. No degree-of-regularity reduction or elite-base advantage follows.

*Validation.* Fourteen native tests pass, including independent tiny V3/V4
closures, polynomial long-division arithmetic checks, degree guards and altered
trace/witness rejection. Runtime controls: xyz+1 gives 3 strict witnesses; xyz
gives 0. The initial fixture setup failure remains archived.

*Restored data.* All {pub['manifest_entries_before_and_after']['raw']} new raw files,
{pub['manifest_entries_before_and_after']['prefix']} prefix files and
{pub['manifest_entries_before_and_after']['suffix']} suffix files match their manifests
before and after replay. All candidate, remainder and ordinary/normal polynomial
files are retained, with {pub['immutable_cumulative_summaries']} case summaries and
{pub['immutable_block_summaries']} block summaries. Both parent archives are required.

*Resources and limits.* One native process, 8 GiB, 1800 s per producer/replay plus 5 s
grace. Existing-host L0 fallback after the retained failed EKS storage probe;
its application cause remains unconfirmed. No performance clock or speed claim.
Batch boundaries: {pub['batch_started_at']} to {pub['batch_finished_at']}.

*Reproduce.* Read `DEGREE4_WITNESS_FORMAT.json`, the native summary in
`evidence/degree4-witness/`, and `evidence/degree4-witness-storage.json`.
The unified export binds outcomes to the frozen protocol and raw hashes.
#text(size:8pt)[Source: `{protocol['freeze']['commit']}` #linebreak()
Protocol: `{protocol['freeze']['protocol_sha256']}` #linebreak()
Data before report: `{head}`]

*Literature.* #link("https://arxiv.org/html/1504.01175")[Semaev, section4.5]
supplies the multiplier identity. #link("https://arxiv.org/html/2112.05579")[Caminata--Gorla, Theorem2.8]
supplies the strict-fall criterion. The full n83 three-family, m2--9 FFD/LFD and
elite-base objective remains open.
'''
(r/'report.typ').write_text(typ);shutil.copy2(__file__,r/'render.py')
typst.compile(str(r/'report.typ'),output=str(r/'report.pdf'))
doc=fitz.open(r/'report.pdf')
for i,page in enumerate(doc):page.get_pixmap(matrix=fitz.Matrix(1.2,1.2)).save(support/f'report-page-{i+1}.png')
print(json.dumps({'directory':str(r),'pages':len(doc),'pdf_sha256':sha(r/'report.pdf'),'data_commit':head}))
