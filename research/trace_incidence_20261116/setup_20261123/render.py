"""Layout only. Statistics, confidence intervals and checks are native C++ outputs."""
import json
import shutil
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.lib.styles import getSampleStyleSheet
from reportlab.platypus import SimpleDocTemplate, Paragraph, Spacer, Image, Table, TableStyle, PageBreak
from reportlab.lib import colors
D=Path(__file__).resolve().parent
O=D/'reports';O.mkdir(exist_ok=True)
B=D.parents[2]/'benchmarks/20261123-trace-complete-cost';B.mkdir(parents=True,exist_ok=True)
pdfmetrics.registerFont(TTFont('Readable','/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf'))
styles=getSampleStyleSheet()
for s in styles.byName.values():s.fontName='Readable'
styles['Normal'].fontSize=9;styles['Normal'].leading=13
styles['Title'].fontSize=19;styles['Title'].leading=23
stages=[('summary-a1.json','Public setup/input representation'),('summary-target-a1.json','Complete target and asymmetric pools'),('summary-ap-a1.json','Labelled progressions and older rho grid'),('summary-holdout-a1.json','Fresh targets, exact kernels, current-module rho'),('summary-flat-a1.json','Fresh480 targets, exact flat containers')]
def para(s):return Paragraph(s,styles['Normal'])
def table(rows,widths=None):
 t=Table(rows,colWidths=widths or [160,75,90,65,65]);t.setStyle(TableStyle([('FONTNAME',(0,0),(-1,-1),'Readable'),('FONTSIZE',(0,0),(-1,-1),7),('BACKGROUND',(0,0),(-1,0),colors.HexColor('#e6edf2')),('BOTTOMPADDING',(0,0),(-1,-1),5)]));return t
notes=[
 'Same60 original public targets. Seven arms isolate checked-context reuse and public-input representation. Minimal input is an explicitly untimed representation ablation; full-to-minimal conversion is not credited as free end-to-end work.',
 'Same60 original targets. All attempts are inside each complete-target call. Reuse, third-pool expansion and retained-pool growth charge actual label construction, every preceding miss and renewed index work. The grow arm has censors.',
 'Three independent mixed-label starts/steps are actually constructed, with six charged scalar combinations, projective additions and batch affine conversion. Progressions are correlated; no iid density assumption. Single-walker older rho controls have severe censoring.',
 '60 fresh public targets; known scalars are separate and never supplied to solvers. Three pair kernels operate on exactly equal progression streams. Current rho source module is intact but compiled against an older compatible ECC adapter crate. The broader mixed-version suite has20 passes,1 unresolved root-oracle mismatch and1 ignored test; all rho/group tests pass.',
 '480 further fresh public targets:96 perB,32 percurve. Exact flat hashing changes containers only. The generator-only arm isolates rejection membership. An identical flat AP4 repeat exposes execution-order noise. Native target-cluster bootstrap uses10000 replicates and resamples within the three fixed curves; uncertainty does not cover other curves or hosts.'
]
method='Each independent target pays given-input parsing, curve/subgroup checks, all actual mixed labels/steps/conversions/rejections, all pool attempts and misses, index construction, witness/scalar recovery and public affine replay. Process launch is outside the complete adapter interval; process peak RSS and wrapper time are retained separately. No reusable setup is amortized across targets. One warmup and five measured rounds rotate arms; a shared local CPU is pinned, without L2 isolation. Counts in tables are target-round units, not independent targets.'
standing='Unsigned producer measurements [pcc:unverified]. Every recovered scalar is replayed with retained affine arithmetic; the two fresh cohorts also match private known answers. No signed independent validation, canonical counted-operation admission, global frontier proof or cryptographic-size conclusion follows from these wall times.'
md=['# Complete-cost trace-incidence optimization — 23 November 2026','',
'Five separately frozen native experiments preserve the previous implementations and charge complete single-target recovery. Exact flat containers, arithmetic kernels and explicitly charged labelled progressions are tested against retained controls. [pcc:unverified]','',
'[Combined PDF](reports/RESULTS.pdf) · [Final native summary](results/summary-flat-a1.json) · [Protocols](PROTOCOL_FLAT.json) · [Raw custody](results/LOCAL_CUSTODY.json)','',
'## Accounting and scope','',method,'',standing,'',
'This resumes the independent labelled-pool incidence path. The earlier paper trace/root acceleration remains a separate component: this fastest pair engine does not call it. Structured PKM/isogeny bases do not supply these mixed labels. No free combination of their advantages is used.','',
'The control panel has B8–128 and subgroup size approximately8B³ (the largest field is about24 bits). These observations do not establish the requested prime-field breakthrough, binary-field improvement, global Pareto optimality or rho parity at cryptographic sizes.','',
'## Native experiments','']
all_data=[]
for i,(filename,title) in enumerate(stages,1):
 d=json.loads((D/'results'/filename).read_text());all_data.append(d)
 rows=[(k.rsplit('/',1)[0],v) for k,v in d['groups'].items() if k.endswith('/128')]
 # Sorting here is presentation by fixed source key, never selection by outcome.
 fig,ax=plt.subplots(figsize=(8,max(3.3,len(rows)*.28+1.2)))
 vals=[v['complete_wall_s']['median']*1000 for _,v in rows]
 ax.barh([a for a,_ in rows],vals,color=['#677f9c' if 'rho' in a or a.startswith('current-') else '#258679' for a,_ in rows])
 ax.set_xlabel('Complete adapter wall time (ms), median; bounded costs include censors')
 ax.set_title(f'Run {i}: B128 · {title}',fontsize=11)
 for j,(_,v) in enumerate(rows):ax.text(vals[j],j,f"  {v['solved']}/{v['complete_wall_s']['n']}",va='center',fontsize=7)
 ax.set_xlim(0,max(vals)*1.2);fig.tight_layout();stem=f'run{i}-b128';fig.savefig(O/(stem+'.svg'));fig.savefig(O/(stem+'.png'),dpi=150);plt.close(fig)
 rows_table=[['Arm','Wall ms','Range ms','Solved','RSS KiB']]
 md += [f'### Run{i}: {title}','',notes[i-1],'',f'[Run{i} PDF](reports/RUN{i}.pdf)','', '| Arm | Median complete ms | Observed min–max ms | Solved / units | Median RSS KiB |','|---|---:|---:|---:|---:|']
 for arm,v in rows:
  w=v['complete_wall_s'];rs=v.get('peak_rss_kib',{});rss=rs.get('median')
  cells=[arm,f"{w['median']*1000:.4f}",f"{w['min']*1000:.3f}–{w['max']*1000:.3f}",f"{v['solved']}/{w['n']}",f'{rss:.0f}' if rss is not None else 'not emitted']
  rows_table.append(cells);md.append('| '+' | '.join(cells)+' |')
 md += ['',f'![Run{i} B128](reports/{stem}.svg)','', 'All displayed observations are provisional. Complete bounded costs retain censored runs; a censor is never a speed win. [pcc:unverified]','']
 height=max(3.3,len(rows)*.28+1.2)
 story=[Paragraph(f'Complete-cost optimization: run {i}',styles['Title']),para(title+' · 23 November 2026 · provisional local data'),Spacer(1,10),Image(str(O/(stem+'.png')),width=470,height=470*height/8),Spacer(1,8),table(rows_table),Spacer(1,10),para(notes[i-1]),Spacer(1,8),para(method),Spacer(1,8),para(standing)]
 SimpleDocTemplate(str(O/f'RUN{i}.pdf'),topMargin=28,bottomMargin=28).build(story)
final=all_data[-1]
md += ['## Fresh480-target confirmation: uncertainty and completion','',
'The primary statistic is candidate/reference ratio of empirical mean complete costs, averaging five rounds within each target. Native stratified target-cluster bootstrap resamples32 targets independently within each of the three curves perB. The frozen scoped timing criterion requires no censors and one-sided95% upper ratio≤1. The table reports B128 against the current-module tuned8 policy; all three tested rho policies and AP sizes are retained in JSON. [pcc:unverified]','',
'| Candidate / tuned8 rho | Mean-cost ratio | Bootstrap 95% interval | One-sided upper95% | Incomplete pairs | Criterion |','|---|---:|---:|---:|---:|---|']
cirows=[['Candidate / rho8','Mean ratio','95% interval','Incomplete','Criterion']]
for arm in ['tree-ap2','tree-ap4','tree-ap8','flat-gen-ap4','flat-ap2','flat-ap4','flat-ap8','flat-ap4-aa']:
 v=final['target_cluster_mean_ratios'][arm+'_over_current-tuned8/128']
 cells=[arm,f"{v['ratio']:.4f}",f"{v['bootstrap_q025']:.4f}–{v['bootstrap_q975']:.4f}",str(v['incomplete_target_round_pairs']),str(v['scoped_wall_parity_criterion'])]
 cirows.append(cells);md.append(f"| {arm} | {v['ratio']:.4f} | {v['bootstrap_q025']:.4f}–{v['bootstrap_q975']:.4f} | {v['bootstrap_q95']:.4f} | {v['incomplete_target_round_pairs']} | {v['scoped_wall_parity_criterion']} |")
md += ['', 'Ratios above1 indicate greater candidate cost. Intervals condition on the fixed curves and observed target population. With any censor, the finite-budget ratio is not an unconditional expected solve-time estimate. Median ratios, ratio of means and means of ratios answer different questions and are kept separate.','',
'## Prior art and retained limitations','',
'Sources checked (KB unavailable): repository wrapper exits127 and no KB MCP is exposed. The bounded fallback is the pinned [crypto/cryptanalysis assessment](../historical/repo_controls/repository_assessment.json), supplied Spaenlehauer paper arXiv:2610.32508v1 section8.1, retained quotient/GCD and pair controls, FLINT modular arithmetic, and current `src/cryptanalysis/prime_fast.rs`. Existing stride-start construction appears at lines1370 and3993. Progressions, Barrett reduction, batch inversion and flat hashing are established techniques; no new complexity bound or priority claim is made.','',
'The current rho module and all configuration fields are retained with source/compatibility hashes. It is not a whole-current-root-crate build or proof that every external implementation was beaten. The unresolved broad-suite root-oracle mismatch is recorded in `results/current-rho-suite.json` and raw logs. No failing test is discarded or counted as passing.','',
'## Reproduce and custody','',
'Build `build.sh`, `build-target.sh`, progression/kernel siblings and `build-flat.sh` using the sealed FFI/static dependency cache specified in source. `build-current-rho.sh` uses the exact current module plus sealed older compatibility rlibs. Retained executables permit native replay; a source rebuild needs those external caches, whose hashes and limitations accompany the raw archive. New-output guards prevent overwriting runs.','',
'Run `qualify-flat.sh BUILD PUBLIC NEW_QUAL_DIR`, then `run-flat.sh BUILD PUBLIC CURRENT_RHO NEW_RUN_DIR`. Compile `analyze-flat.cpp` with the native affine translation object and FLINT; invoke `analyze-flat ROWS PUBLIC PRIVATE_VALIDATION SUMMARY`. Solvers never receive the private file. `render.py` performs layout only. Immutable per-stage source manifests and pre/post executable/dependency hashes accompany each run.','',
'All rows, warmups, misses, build errors, broad-test failures, input fixtures, qualifier binaries, source checkpoints and native analysis are retained under the authorized temporary local raw-custody exception. No S3 custody or signed result promotion is claimed. Reports and graphs are copied byte-for-byte into `benchmarks/20261123-trace-complete-cost`; canonical benchmark index and leaderboard rows remain unchanged pending admission.','',
'## Requirements remaining','',
'| Requested item | Status and evidence |','|---|---|',
'| Run and optimize complete labelled-pool recovery | Five native matrices; complete cold adapters and retained controls; producer checks unsigned |',
'| Independently check fast trace, collisions and exceptional points | Prior retained [recurrence study](../recurrence_20261117/README.md); no change to its proof subjects |',
'| Improve implicit incidence GCD over existing controls | No improved asymptotic bound established; pair control remains separate from root speedup |',
'| Auxiliary curve/isogeny/twist end-to-end advantage | Prior toy paper routes retained; no new charged mixed-label transfer construction |',
'| Establish rho parity and global Pareto frontier | Scoped local ratios above; strongest whole-system reference/counting admission and general proof remain open |',
'| Prime-field breakthrough / binary-field speedup | Not established by this small prime-field panel; no new binary-field run |','']
(D/'README.md').write_text('\n'.join(md))
story=[Paragraph('Trace-incidence complete-cost results',styles['Title']),para('Five successive native experiments · 23 November 2026 · unsigned producer data'),Spacer(1,10),para(method),Spacer(1,10),Image(str(O/'run5-b128.png'),width=470,height=470*max(3.3,11*.28+1.2)/8),Spacer(1,8),table(cirows,[150,65,105,70,65]),Spacer(1,10),para('Native target-cluster intervals resample within the three fixed curves. Criterion: no censors and one-sided95% upper mean-cost ratio at most1. Raw bounded costs include misses/censors. Other scales and all rho policies remain in the native summary. [pcc:unverified]'),PageBreak(),Paragraph('Construction, checks and limits',styles['Title']),para(notes[2]),Spacer(1,10),para(notes[3]),Spacer(1,10),para(notes[4]),Spacer(1,10),para(standing),Spacer(1,10),para('B8–128; largest field about24 bits. APs are correlated and their actual labels and relation coverage are charged. Known structured PKM/isogeny factor bases do not supply these labels for free. Root-kernel improvements do not enter this pair-engine runtime.'),Spacer(1,10),para('Frozen protocols, exact sources, all five native summaries and RUN1–RUN5 PDFs are linked from README. Failed builds, broad test mismatch, all warmups and censors remain in the raw archive. Current-module source rebuild needs the sealed older compatible crate cache. No S3 custody, canonical benchmark admission, global Pareto optimality or general ECDLP speedup is claimed.'),Spacer(1,10),para('Sources checked: supplied paper section8.1; pinned crypto/cryptanalysis quotient/GCD and pair controls; current prime_fast.rs stride starts/tuned rho; FLINT arithmetic. KB unavailable; standard engineering methods, no novelty claim.')]
SimpleDocTemplate(str(O/'RESULTS.pdf'),topMargin=28,bottomMargin=28).build(story)
for p in O.iterdir():
 if p.suffix in ('.svg','.png','.pdf'):shutil.copyfile(p,B/p.name)
(B/'README.md').write_text('# Local complete-cost trace-incidence measurements\n\n[Full study](../../research/trace_incidence_20261116/setup_20261123/README.md) · [Combined PDF](RESULTS.pdf).\n\nFive separately frozen native panels test checked-context reuse, asymmetric pools, charged labelled progressions, exact arithmetic and flat-container kernels against retained rho controls. Fresh480-target confirmation includes same-code repeats and native target-cluster intervals. These are unsigned local observations; no canonical timing admission or global Pareto claim. [pcc:unverified]\n\nByte-identical report and graph copies; raw custody and source-module compatibility limitations are explicit in the study. Canonical index remains unchanged pending normal admission.\n')
print(O)
