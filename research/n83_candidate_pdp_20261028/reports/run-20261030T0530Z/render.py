from pathlib import Path
import hashlib,json,subprocess,shutil
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import typst,fitz
s=Path('research/n83_candidate_pdp_20261028');q=Path('/home/ec2-user/n83-candidate-recovery/prime-first-fall-support')
read=lambda p:json.loads(p.read_text());sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()
for name in ['evidence/prime-first-fall/summary.json','evidence/prime-first-fall-publication.json']:
 assert subprocess.check_output(['git','show',head+':'+str(s/name)])==(s/name).read_bytes()
summary=read(s/'evidence/prime-first-fall/summary.json');pub=read(s/'evidence/prime-first-fall-publication.json');p=read(s/'protocol-prime-first-fall.json');rows=summary['rows']
stamp=pub['batch_started_at'].replace('-','').replace(':','')[:13]+'Z';r=s/'reports'/('run-'+stamp);r.mkdir(parents=True,exist_ok=False)
plt.rcParams.update({'font.family':'DejaVu Sans','font.size':9,'svg.fonttype':'none'})
fig,ax=plt.subplots(figsize=(9.4,3.15),layout='constrained')
labels=[]
for i,row in enumerate(rows):
 labels.append(row['key'].replace('-prime-m2','').replace('original','orig').replace('baseline','base').replace('planted','P').replace('natural','N'))
 ffd=row['first_fall_degree'];lower=row['last_fall_lower_bound']
 ax.bar(i-.12,ffd,width=.24,color='#116d8a',label='Highest-form FFD (exact)' if i==0 else None)
 ax.text(i-.12,ffd+.35,str(ffd),ha='center',fontsize=9)
 if lower is not None:
  ax.scatter([i+.19],[lower],marker='^',s=65,color='#b95d1c',label='LFD lower bound; upper unknown' if i==0 else None,zorder=3)
  ax.text(i+.19,lower+1.15,'≥'+str(lower),ha='center',fontsize=9,color='#88430f')
 else:ax.text(i+.19,1,'unknown',rotation=90,ha='center')
ax.set_xticks(range(len(rows)),labels);ax.set_ylim(0,23);ax.set_ylabel('Degree')
ax.set_title('Certificates on the six retained prime m2 systems',fontsize=11)
ax.legend(loc='upper center',fontsize=8,ncols=2);ax.grid(axis='y',alpha=.15);ax.spines[['top','right']].set_visible(False)
fig.savefig(r/'degrees.svg');fig.savefig(r/'degrees.png',dpi=180);plt.close(fig)
svg=r/'degrees.svg';svg.write_text('\n'.join(line.rstrip() for line in svg.read_text().splitlines())+'\n')
def val(row,key):return 'unknown' if row[key] is None else str(row[key])
table='\n'.join(f"| {row['key']} | {val(row,'first_fall_degree')} | {val(row,'first_strict_filtered_fall_degree')} | ≥{val(row,'last_fall_lower_bound')} | unknown |" for row in rows)
md=f'''# Prime m2 first-fall and strict filtered-witness certificates

Run `n83-prime-first-fall-20261030-r1`, n83, m2, ell27. **{pub['producer_cases']}/6 producers, {pub['fresh_replay_cases']}/6 fresh replays and {pub['s3_restored_replay_cases']}/6 S3-restored replays passed.** Independent scientific review and full algebraic formalization remain pending.

| Retained input | Highest-form FFD | First strict filtered fall | LFD lower bound | Exact/global upper LFD |
|---|---:|---:|---:|---|
{table}

![Exact FFD and separate LFD lower bounds](degrees.svg)

The six strict witnesses have degree-at-most17 derivations and degree16 normal forms outside V16 under the written filtered-space argument. All six production coefficient functionals are nonzero. The global LFD upper bound and exact LFD remain unknown. The unchanged original-top regularity is42. No factor-base quality ranking, performance gain or universal FFD/LFD conclusion follows. Original/baseline natural inputs repeat and are not independent samples.

## Exact polynomial data and proof boundary

The normalized source has selector equations `z_i^2=w_i*z_i`, `u_i^2=w_i*u_i`, and `f=S3(X,Y,t)`. Its highest form is `X^2Y^2`. Certified multiplication ranks are injective through multiplier degree12. The nonzero degree13 polynomial `H=product(z_2j-z_(2j+1))` annihilates `X^2` in the square-zero associated graded algebra. The written Koszul/annihilator argument gives highest-form FFD17.

The strict witness is `g=NF(H*f)`. It is stored exactly as `A0 tensor B0+A1 tensor B1+A2 tensor B2`, with all six sparse factors and the reconstruction circuit. No expanded coefficient is sampled or discarded. A direct complementary-mask pairing on those factors equals `2*(-2)^13*(sum(weights)-2*target_x)`. It is nonzero in all six retained cases and annihilates the lower-space leading image. The written injectivity/closure argument characterizes V16; this is not a materialized dense V16 basis or a full ideal upper certificate.

## Validation and preservation

- {pub['native_tests_passed']} native tests pass, including complete source/witness suffix checks and corrupt/truncated data rejection.
- {pub['expanded_control_cases']} independent expanded-polynomial comparisons, {pub['dense_control_cases']} small dense filtered-closure comparisons, and {pub['exceptional_target_controls']} exceptional-target controls passed. Zero pairing is recorded as inconclusive for the affine witness.
- All {pub['manifest_payloads_before_and_after']} raw manifest payloads match before and after S3-restored replay. All previous datasets remain unchanged.
- Fixed limits:2GiB virtual memory,60s runtime controls,300s producer,300s fresh replay,+5s grace each,720s outer,no retries. Diagnostic batch boundaries:{pub['batch_started_at']}–{pub['batch_finished_at']}. No timing or CRIU claim.
- EKS CLI role assumption was denied and MCP inventory timed out. No new EKS job was submitted; the established host fallback was retained without IAM/capacity changes.

Read `PRIME_FIRST_FALL_FORMAT.json`, `PRIME_FIRST_FALL_DESIGN.md`, `prime-first-fall-archives/README.md`, `experiment-prime-first-fall.export.json` and the storage/publication JSON records. The full three-family m2–9 goal, global LFD upper certificates, higher-m work and held-out elite-base search remain open.

Sources: [Caminata–Gorla Definitions1.3/1.5 and Section4](https://arxiv.org/html/2112.05579); [Phuong–Tran Theorem2.7 and proof](https://arxiv.org/pdf/2211.13548). The dataset-specific argument is written in PRIME_FIRST_FALL_DESIGN.md.

Source commit:`{p['freeze']['commit']}`. Protocol:`{p['freeze']['protocol_sha256']}`. Data committed before this report:`{head}`.
'''
(r/'report.md').write_text(md)
cells='\n'.join(f"[{row['key'].replace('-prime-m2','')}], [{val(row,'first_fall_degree')}], [{val(row,'first_strict_filtered_fall_degree')}], [≥{val(row,'last_fall_lower_bound')}], [unknown]," for row in rows)
typ=f'''#set page(paper:"a4",margin:(x:17mm,y:16mm),footer:context align(right,text(size:8pt,fill:rgb("667085"))[N83 / prime first-fall · #counter(page).display()]))
#set text(font:"DejaVu Serif",size:9pt)
#set par(justify:false,leading:0.45em)
#show heading:set text(fill:rgb("173c50"))
= Prime m2 first-fall certificates
#text(fill:rgb("116d8a"))[n83-prime-first-fall-20261030-r1 · six retained n83 systems]

*{pub['producer_cases']}/6 producers, {pub['fresh_replay_cases']}/6 fresh replays and {pub['s3_restored_replay_cases']}/6 S3-restored replays passed.* Independent scientific review and full algebraic formalization remain pending.

#table(columns:(2.4fr,0.8fr,1fr,0.9fr,1fr),inset:4pt,stroke:0.3pt+rgb("d8e0e8"),
 [Retained input],[FFD],[Strict fall],[LFD lower],[Upper/exact],
 {cells})
#image("degrees.svg",width:100%)

*Witnesses.* Each has a derivation of degree at most17 and a degree16 normal form outside V16 under the written filtered-space argument. All six production functionals are nonzero. Global LFD upper and exact values remain unknown.

*Unchanged data.* n83,m2,ell27,54 selectors. Actual point manifests,targets and original equations remain fixed; each point base is reused across m2–9. Original-top regularity remains42. Original/baseline natural inputs repeat. No elite-base or performance gain is established.

#pagebreak()
= Algebraic certificates and reproducibility

*Highest-form FFD.* Normalize with `z_i=w_i*v_i`. Selector squares give the square-zero associated graded algebra. The original quartic top is `X^2Y^2`. Multiplication ranks are injective through multiplier degree12. The degree13 product `H=product(z_2j-z_(2j+1))` is nonzero and annihilates `X^2`. The written Koszul/annihilator argument gives first-fall degree17; rank arithmetic and the sparse witness are checked natively.

*Strict affine fall.* Store `g=NF(H*f)` exactly as three tensor products of six complete sparse factors. The degree16 coefficient functional,computed directly from complementary masks,equals `2*(-2)^13*(sum(weights)-2*target_x)`. Nonzero implies a strict degree17 fall using the written characterization of V16. V16 is characterized algebraically;no dense V16 basis or global ideal upper certificate is claimed. A zero functional is inconclusive for this test.

*Validation.* {pub['native_tests_passed']} native tests; {pub['expanded_control_cases']} expanded-polynomial comparisons; {pub['dense_control_cases']} small dense-closure comparisons; {pub['exceptional_target_controls']} exceptional-target controls. Inherited rank controls include small-characteristic deficiencies. Source checks require the entire equation template and exact planted-witness suffix. All {pub['manifest_payloads_before_and_after']} raw payloads match before and after S3 replay.

*Bounds and execution.* 2GiB virtual memory;60s controls;300s producer and300s fresh replay;+5s termination grace;720s outer;no retries. Batch UTC:{pub['batch_started_at']} to {pub['batch_finished_at']}. These are diagnostic boundaries,not a performance measurement. EKS CLI role assumption was denied and MCP inventory timed out;existing-host fallback,no new EKS job or IAM/capacity change.

*Reproduce.* Hydrate/extract the raw archive,verify its manifest,run `instrument replay ROOT ROOT/results`,then verify the manifest again. PRIME_FIRST_FALL_FORMAT.json defines every factor byte and tensor reconstruction. The raw archive is sufficient for replay;source and base identities remain bound to earlier datasets.

*Scope.* Global prime LFD upper/exact,higher-m work,the binary quartic frontier and held-out elite-base selection remain open. Native replay is distinct from independent scientific review and a full algebraic formal proof.

*Sources.* #link("https://arxiv.org/html/2112.05579")[Caminata–Gorla Definitions1.3/1.5 and Section4]; #link("https://arxiv.org/pdf/2211.13548")[Phuong–Tran Theorem2.7]. Dataset-specific proof:PRIME_FIRST_FALL_DESIGN.md.
#text(size:8pt)[Source:`{p['freeze']['commit']}` #linebreak()
Protocol:`{p['freeze']['protocol_sha256']}` #linebreak()
Data before report:`{head}`]
'''
(r/'report.typ').write_text(typ);shutil.copy2(__file__,r/'render.py');typst.compile(str(r/'report.typ'),output=str(r/'report.pdf'))
doc=fitz.open(r/'report.pdf')
for i,page in enumerate(doc):page.get_pixmap(matrix=fitz.Matrix(1.2,1.2)).save(q/f'report-page-{i+1}.png')
print(json.dumps({'directory':str(r),'pages':len(doc),'pdf_sha256':sha(r/'report.pdf'),'data_commit':head}))
