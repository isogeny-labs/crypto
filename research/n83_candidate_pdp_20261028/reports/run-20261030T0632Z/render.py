from pathlib import Path
import hashlib,json,subprocess,shutil
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import typst,fitz
s=Path('research/n83_candidate_pdp_20261028');q=Path('/home/ec2-user/n83-candidate-recovery/prime-terminal-support');read=lambda p:json.loads(p.read_text());sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()
for n in ['evidence/prime-terminal/summary.json','evidence/prime-terminal-publication.json']:assert subprocess.check_output(['git','show',head+':'+str(s/n)])==(s/n).read_bytes()
rows=read(s/'evidence/prime-terminal/summary.json')['rows'];pub=read(s/'evidence/prime-terminal-publication.json');p=read(s/'protocol-prime-terminal-r3.json');stamp=pub['batch_started_at'].replace('-','').replace(':','')[:13]+'Z';r=s/'reports'/('run-'+stamp);r.mkdir()
plt.rcParams.update({'font.family':'DejaVu Sans','font.size':9,'svg.fonttype':'none'})
fig,ax=plt.subplots(figsize=(9.4,3.2),layout='constrained');colors=['#116d8a','#bd641d','#6d4d8f']
for ai,arm in enumerate(['original','baseline','spike40']):
 for ti,target in enumerate(['planted','natural']):
  rr=[x for x in rows if x['arm']==arm and x['target_kind']==target]
  ax.scatter([x['m']+(ai-1)*.18+(ti-.5)*.06 for x in rr],[x['first_fall_upper_bound'] for x in rr],color=colors[ai],marker='v' if ti==0 else 'o',s=35,label=arm+' / '+target)
ax.set_xticks(range(3,10));ax.set_xlabel('Number of factor-base points m');ax.set_ylabel('Certified FFD upper bound');ax.set_ylim(0,21);ax.set_yticks([0,5,10,15,17,20]);ax.grid(axis='y',alpha=.2);ax.legend(ncols=3,fontsize=8,loc='lower center');ax.set_title('All 42 retained prime systems: upper certificate at degree 17',fontsize=11);ax.spines[['top','right']].set_visible(False)
fig.savefig(r/'degrees.svg');fig.savefig(r/'degrees.png',dpi=180);plt.close(fig);svg=r/'degrees.svg';svg.write_text('\n'.join(x.rstrip() for x in svg.read_text().splitlines())+'\n')
groups=[]
for m in range(3,10):
 rr=[x for x in rows if x['m']==m];assert len(rr)==6;upper=sorted({x['first_fall_upper_bound'] for x in rr});degree=sorted({x['affine_consequence_degree'] for x in rr});groups.append((m,len(rr),','.join(map(str,upper)),','.join(map(str,degree))))
table='\n'.join(f'| {m} | {n} | ≤{u} | {d} | unknown |' for m,n,u,d in groups)
md=f'''# Higher-m prime terminal annihilator certificates

Observations only; independent scientific review and full algebraic formalization are pending. Run `n83-prime-terminal-20261030-r3`: **42/42 producers, 42/42 fresh replays and 42/42 S3-restored replays passed**. No exact FFD or new LFD certificate is claimed.

| m | Retained cases | Highest-form FFD upper | Affine consequence degree | Exact FFD / new LFD |
|---:|---:|---:|---:|---|
{table}

![Recorded upper certificates](degrees.svg)

Each group contains original/baseline/spike40 × planted/natural. Values are source-bound certificate outputs, not independent samples or a ranking. Every affine consequence has a derivation of degree at most17 and actual degree16. Other chain equations may derive it earlier: nonmembership inV16 was not tested. All strict-witness and new LFD fields are false/null. Previous prime m2 FFD17/LFD≥17 results remain separate.

## What is certified

The terminal highest form is `F=U^2*X^2`. Normalized last-block selectors satisfy square-zero top relations. The degree13 polynomial `H=product(z_2j-z_(2j+1))` has8192 distinct terms and `H*X^2=0` modulo selector squares. Project all variables outside the last selector block tozero. Every original top generator maps tozero while H survives. Hence H cannot be the terminal coordinate of a Koszul syzygy; the written argument gives a nontrivial degree17 syzygy and FFD≤17. No exclusion of earlier syzygies is claimed.

The affine consequence is `g=A0*B0(U)+A1*B1(U)+A2*B2(U)`, where A0=H,A1=NF(HX),A2=NF(HX²). All126 sparse factors are stored exactly with canonical coefficients. The three B-polynomials are explicit degree≤2 coefficient lists. No U field equation or new preprocessing is introduced. PRIME_TERMINAL_FORMAT.json specifies header bytes, masks, variables and reconstruction.

## Validation, failures and custody

- 18 native tests pass. Runtime controls include18 independent ordinary-exponent expansions and18 non-annihilator controls. Synthetic source tests cover21 arm/m layouts;source,base,witness,projection andcorrupt-factor failures are checked.
- All269 raw manifest payloads match before/after S3-restored replay. Unified export verifies with zero findings. Index19datasets/70hashes;original48regularityrows andbasebindings unchanged.
- R1 EKS JobUIDd91a1d5b-be44-4f06-ad23-2afca910bbf1 failed exit2 before native computation: worker file permissions. R2 JobUID553a48de-f5f6-46ed-9a62-9234b0045951 failed exit1 before native computation: source HeadObject403. Both frozen protocols,source bundles,images andlogs retained. R3 used establishedhostfallback,no IAM/capacitychanges.
- R3 limits:oneprocess,2GiBvirtual,60scontrols,300sproducer,300sfreshreplay,+5sgrace,720souter,no retries. Diagnostic interval:{pub['batch_started_at']}–{pub['batch_finished_at']};not timing evidence.
- Root library/Cargo inputs unchanged from prior completed delivery. Earlier full-library run and exact fixture rechecks remain the applicable record;this phase adds native certificate tests.

Hydrate `prime-terminal-archives/runs.tar.gz`,extract into a fresh ROOT,verify manifest,run `instrument replay ROOT ROOT/results`,thenverify manifest again. Source bundles and all failures are retained separately. See storage/publication JSON and DATA_INDEX.json.

The full n83 three-family m2–9 FFD/LFD and elite-base goal remains open. Prime exact/higher-m FFD,strict LFD/globalupper,binaryquarticfrontier andheld-out selection remain obligations. No elite-base or speed gain is established.

Source: [Caminata–Gorla Definition1.3 and Section4](https://arxiv.org/html/2112.05579);dataset-specific argument in PRIME_TERMINAL_DESIGN.md. Sourcecommit:`{p['arms'][0]['instrument']['commit']}`. Protocol:`{p['freeze']['protocol_sha256']}`. Data before report:`{head}`.
''';(r/'report.md').write_text(md)
cells='\n'.join(f'[{m}],[{n}],[≤{u}],[{d}],[unknown],' for m,n,u,d in groups)
t=f'''#set page(paper:"a4",margin:(x:17mm,y:16mm),footer:context align(right,text(size:8pt,fill:rgb("667085"))[N83 / prime terminal · #counter(page).display()]))
#set text(font:"DejaVu Serif",size:9pt)
#set par(justify:false,leading:0.45em)
#show heading:set text(fill:rgb("173c50"))
= Higher-m prime terminal certificates
#text(fill:rgb("116d8a"))[n83-prime-terminal-20261030-r3 · all 42 retained prime m3–9 systems]

Observations only. *42/42 producer cases,42/42 fresh replays and42/42 S3-restored replays passed.* Independent scientific review and full algebraic formalization remain pending.

#table(columns:(0.5fr,0.8fr,1.2fr,1.4fr,1.5fr),inset:4pt,stroke:0.3pt+rgb("d8e0e8"),[m],[Cases],[FFD upper],[Affine degree],[Exact FFD / new LFD],{cells})
#image("degrees.svg",width:100%)

*Boundaries.* Each group contains original/baseline/spike40 × planted/natural. Every stored affine consequence has derivation degree at most17 and actual degree16. Other chain equations may derive it earlier. Nonmembership inV16 is not tested;no new LFD bound follows. Previous prime m2 FFD17 andLFD≥17 are separate results.

*Unchanged workload.* n83,ell27,actual immutable point bases reused across m2–9. Original targets and equations remain fixed. These are correlated retained cases,not independent samples or a factor-base ranking. No elite-base or performance gain is established.

#pagebreak()
= Certificate argument and reproducibility

*Highest-form upper.* The terminal top is `F=U^2*X^2`. The degree13 product `H=product(z_2j-z_(2j+1))` has8192 distinct terms and annihilates `X^2` modulo selector squares. Projection onto the last selector block kills every original top generator while preserving H. Thus H is outside the other-generator ideal and cannot be a terminal Koszul coordinate. The written projection argument gives FFD≤17;earlier-syzygy exclusion is not supplied.

*Exact affine data.* `A0=H,A1=NF(HX),A2=NF(HX^2)`. Store all126 sparse A factors plus explicit degree≤2 B(U) coefficients. The circuit `g=sum Ai*Bi(U)` reconstructs every coefficient. U remains an unbounded intermediate;no field equation or preprocessing is added. PRIME_TERMINAL_FORMAT.json defines every byte and variable mapping. Membership at derivation degree17 alone is not a strict filtered-fall certificate.

*Validation.* 18 native tests;18 independent expanded-polynomial comparisons;18 negative annihilator controls;21 synthetic full-source layouts. Source mutations,wrong base,invalid planted coordinates,wrong projection andcorrupt/truncated factors are rejected. All269 raw payloads match before/after S3 replay. Export verification:zero findings. Data index:19datasets/70hashes. Original48regularityrows andbasebindings unchanged.

*Retained infrastructure failures.* R1 EKS failed exit2 before native computation becauseUID1000 could not read worker.py. R2 fixed permissions but failed exit1 on source HeadObject403. Both versions,protocols,images andlogs remain preserved. R3 used the established host fallback without IAM/capacity changes.

*Execution limits.* One process,2GiBvirtual,60scontrols,300sproducer,300sfreshreplay,+5sgrace,720souter,no retries. Diagnostic batch:{pub['batch_started_at']} to {pub['batch_finished_at']}. No timing or CRIU claim.

*Reproduce.* Hydrate/extract raw archive;verify manifest;run `instrument replay ROOT ROOT/results`;verify manifest again. Raw archive is sufficient for replay. All failed/development attempts and later delivery receipts are retained separately.

*Remaining scope.* Exact higher-m FFD,strict LFD andglobalupper,binaryquarticfrontier andheld-out elite-base selection remain open. Native replay does not substitute for independent scientific review or full algebraic proof formalization.

*Source.* #link("https://arxiv.org/html/2112.05579")[Caminata–Gorla Definition1.3 and Section4];dataset argument:PRIME_TERMINAL_DESIGN.md.
#text(size:8pt)[Source:`{p['arms'][0]['instrument']['commit']}` #linebreak() Protocol:`{p['freeze']['protocol_sha256']}` #linebreak() Data before report:`{head}`]
''';(r/'report.typ').write_text(t);shutil.copy2(__file__,r/'render.py');typst.compile(str(r/'report.typ'),output=str(r/'report.pdf'));doc=fitz.open(r/'report.pdf')
for i,page in enumerate(doc):page.get_pixmap(matrix=fitz.Matrix(1.2,1.2)).save(q/f'report-page-{i+1}.png')
print(json.dumps({'directory':str(r),'pages':len(doc),'pdf_sha256':sha(r/'report.pdf'),'data_commit':head}))
