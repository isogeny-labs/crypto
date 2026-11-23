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
stages=[('summary-a1.json','Public setup/input representation'),('summary-target-a1.json','Complete target and asymmetric pools'),('summary-ap-a1.json','Labelled progressions and older rho grid'),('summary-holdout-a1.json','Fresh targets, exact kernels, current-module rho'),('summary-flat-a1.json','Fresh 480 targets, exact flat containers'),('summary-field-a1.json','Fresh 480 targets, arithmetic and Bloom ablation'),('summary-scaling-a1.json','Growing fields and density-matched pool shapes'),('summary-hot-a1.json','Fresh p31 targets, blocked filter, allocation and CPU tuning')]
def para(s):return Paragraph(s,styles['Normal'])
def table(rows,widths=None):
 t=Table(rows,colWidths=widths or [145,60,60,90,55,50],repeatRows=1);t.setStyle(TableStyle([('FONTNAME',(0,0),(-1,-1),'Readable'),('FONTSIZE',(0,0),(-1,-1),7),('BACKGROUND',(0,0),(-1,0),colors.HexColor('#e6edf2')),('BOTTOMPADDING',(0,0),(-1,-1),5)]));return t
notes=[
 'Same 60 original public targets. Seven arms isolate checked-context reuse and public-input representation. Minimal input is an explicitly untimed representation ablation; full-to-minimal conversion is not credited as free end-to-end work.',
 'Same 60 original targets. All attempts are inside each complete-target call. Reuse, third-pool expansion and retained-pool growth charge actual label construction, every preceding miss and renewed index work. The grow arm has censors.',
 'Three independent mixed-label starts/steps are actually constructed, with six charged scalar combinations, projective additions and batch affine conversion. Progressions are correlated; no iid density assumption. Single-walker older rho controls have severe censoring.',
 '60 fresh public targets; known scalars are separate and never supplied to solvers. Three pair kernels operate on exactly equal progression streams. Current rho source module is intact but compiled against an older compatible ECC adapter crate. The broader mixed-version suite has20 passes,1 unresolved root-oracle mismatch and1 ignored test; all rho/group tests pass.',
 '480 further fresh public targets:96 perB,32 per curve. Exact flat hashing changes containers only. The generator-only arm isolates rejection membership. An identical flat AP4 repeat exposes execution-order noise. Native target-cluster bootstrap uses10000 replicates and resamples within the three fixed curves; uncertainty does not cover other curves or hosts.'
 ,'480 further fresh targets. Barrett point generation/validation retains all checks; the separately predeclared no-Bloom candidate regresses atB128 and is not replaced retrospectively by its winning ablation. DirectAP8 passes the strict local-wall criterion only atB64 on this fixed panel.'
 ,'192 further fresh targets over12 ordinary prime-order curves,16 targets per curve, B128/256/512/640. Three curves use p=2147483647. Fixed pool shapes hold nominal first*second*third/r near1. Field/Bloom is retained; no-Bloom balanced is a losing control. Six tuned rho walker policies and a quarter-shape repeat remain visible. Given-input curve construction is common excluded input, not free algorithmic auxiliary-curve work.'
 ,'192 further fresh p31 targets,64 per curve. Exact blocked Bloom, buffer reservation/reuse and generic/CPU-native compiler builds preserve each point and label stream. Twelve generic/native rho policies are matched. Primary combined-native-balanced and its same-code repeat are predeclared; all allocation/filter/compiler ablations remain visible. Partial selector counters are not complete primitive-operation counts.'
]
method='Each independent target pays given-input parsing, curve/subgroup checks, all actual mixed labels/steps/conversions/rejections, all pool attempts and misses, index construction, witness/scalar recovery and public scalar check. Process launch is outside the complete adapter interval; process peak RSS and wrapper time are retained separately. No reusable setup is amortized across targets. One warmup and five measured rounds rotate arms; a shared local CPU is pinned, without L2 isolation. Counts in tables are target-round units, not independent targets.'
check_scope='IC includes its retained affine public scalar check; rho includes its native group check. Independent affine/private-answer post-analysis and exhaustive no-timer label/oracle qualification are outside both performance intervals. All actual label construction and public checks required by the solver remain inside. Cold refers to one-target setup charging; OS caches and host are not isolated.'
root_gate=json.loads((D/'results'/'root-cargo-gate-final.json').read_text())['summary']
standing='Unsigned producer measurements [pcc:unverified]. Every recovered scalar is replayed with retained affine arithmetic; all fresh cohorts also match private known answers. No signed independent validation, canonical counted-operation admission, global frontier proof or cryptographic-size conclusion follows from these wall times.'
md=['# Complete-cost trace-incidence optimization — 23 November 2026','',
'Eight separately frozen native experiments preserve the previous implementations and charge complete single-target recovery. Exact flat containers, arithmetic kernels and explicitly charged labelled progressions are tested against retained controls. [pcc:unverified]','',
'[Combined PDF](reports/RESULTS.pdf) · [Final native summary](results/summary-hot-a1.json) · [Protocols](PROTOCOL_HOT.json) · [Raw custody](results/LOCAL_CUSTODY.json)','',
'## Accounting and scope','',method,'',check_scope,'',standing,'',root_gate+' [pcc:unverified]','',
'This resumes the independent labelled-pool incidence path. The earlier paper trace/root acceleration remains a separate component: this fastest pair engine does not call it. Structured PKM/isogeny bases do not supply these mixed labels. No free combination of their advantages is used.','',
'The control panel has B8–640 and subgroup size approximately8B³ (the largest field has31 bits). These observations do not establish the requested prime-field breakthrough, binary-field improvement, global Pareto optimality or rho parity at cryptographic sizes. [pcc:unverified]','',
'## Native experiments','']
headline_data=json.loads((D/'results'/'observed-frontier.json').read_text())
h=headline_data['comparisons']['hot']['640']['baseline-balanced'];r=headline_data['comparisons']['hot']['640']['native-tuned32']
headline=f"The predeclared final primary averages {h['candidate_mean_complete_wall_s']*1000:.4f}ms versus its matched baseline {h['reference_mean_complete_wall_s']*1000:.4f}ms: {h['speedup_ratio_of_means']:.4f}× speedup, bootstrap 95% interval {h['bootstrap_speedup_q025']:.4f}–{h['bootstrap_speedup_q975']:.4f}×. The fastest observed mean among the twelve tested rho controls is native tuned32 at {r['reference_mean_complete_wall_s']*1000:.4f}ms. Primary and that reference use mean process-peak RSS {r['candidate_mean_process_rss_kib']:.0f} and {r['reference_mean_process_rss_kib']:.0f}KiB. The primary parity criterion fails. [pcc:unverified]"
md[2:2]=[headline,'']
all_data=[]
for i,(filename,title) in enumerate(stages,1):
 d=json.loads((D/'results'/filename).read_text());all_data.append(d)
 size=640 if i>=7 else 128
 rows=[(k.rsplit('/',1)[0],v) for k,v in d['groups'].items() if k.endswith('/'+str(size))]
 # Sorting here is presentation by fixed source key, never selection by outcome.
 fig,ax=plt.subplots(figsize=(8,max(3.3,len(rows)*.28+1.2)))
 vals=[v['complete_wall_s']['mean']*1000 for _,v in rows]
 ax.barh([a for a,_ in rows],vals,color=['#677f9c' if 'rho' in a or a.startswith(('current-','generic-','native-')) else '#258679' for a,_ in rows])
 ax.set_xlabel('Complete adapter wall time (ms), mean; bounded costs include censors')
 ax.set_title(f'Run {i}: B{size} · {title}',fontsize=11)
 for j,(_,v) in enumerate(rows):ax.text(vals[j],j,f"  {v['solved']}/{v['complete_wall_s']['n']}",va='center',fontsize=7)
 ax.set_xlim(0,max(vals)*1.2);fig.tight_layout();stem=f'run{i}-b{size}';fig.savefig(O/(stem+'.svg'));fig.savefig(O/(stem+'.png'),dpi=150);plt.close(fig)
 rows_table=[['Arm','Mean ms','Median ms','Range ms','Solved','RSS KiB']]
 md += [f'### Run{i}: {title}','',notes[i-1],'',f'[Run{i} PDF](reports/RUN{i}.pdf)','', '| Arm | Mean complete ms | Median complete ms | Observed min–max ms | Solved / units | Median RSS KiB |','|---|---:|---:|---:|---:|---:|']
 for arm,v in rows:
  w=v['complete_wall_s'];rs=v.get('peak_rss_kib',{});rss=rs.get('median')
  cells=[arm,f"{w['mean']*1000:.4f}",f"{w['median']*1000:.4f}",f"{w['min']*1000:.3f}–{w['max']*1000:.3f}",f"{v['solved']}/{w['n']}",f'{rss:.0f}' if rss is not None else 'not emitted']
  rows_table.append(cells);md.append('| '+' | '.join(cells)+' |')
 md += ['',f'![Run{i} B{size}](reports/{stem}.svg)','', 'All displayed observations are provisional. Complete bounded costs retain censored runs; a censor is never a speed win. [pcc:unverified]','']
 height=max(3.3,len(rows)*.28+1.2)
 story=[Paragraph(f'Complete-cost optimization: run {i}',styles['Title']),para(title+' · 23 November 2026 · provisional local data'),Spacer(1,10),Image(str(O/(stem+'.png')),width=470,height=470*height/8),Spacer(1,8),table(rows_table),Spacer(1,10),para(notes[i-1]),Spacer(1,8),para(method),Spacer(1,8),para(check_scope),Spacer(1,8),para(standing)]
 SimpleDocTemplate(str(O/f'RUN{i}.pdf'),title=f'Complete-cost optimization run {i}',author='Provisional native benchmark study',topMargin=28,bottomMargin=28).build(story)
final=all_data[-1]
primary_result=final['primary_combined_native_all_twelve_reference_criterion']['640']
md += ['## Predeclared final result','',f'The primary combined-native-balanced arm passes the frozen all-twelve-reference local-wall criterion: **{primary_result}**. This is the native recorded criterion, not a global Pareto or cryptographic-size assertion. [pcc:unverified]','']
md += ['## Growing-field confirmation: uncertainty and completion','',
'The primary statistic is candidate/reference ratio of empirical mean complete costs, averaging five rounds within each target. Native stratified target-cluster bootstrap resamples64 targets independently within each of the three curves perB. The frozen scoped timing criterion requires no censors and one-sided95% upper ratio≤1. The table reports B640 against the CPU-native current-module tuned64 policy; all twelve generic/native rho policies and all fixed ablations are retained in JSON. [pcc:unverified]','',
'| Candidate / native tuned64 rho | Mean-cost ratio | Bootstrap 95% interval | One-sided upper95% | Incomplete pairs | Criterion |','|---|---:|---:|---:|---:|---|']
cirows=[['Candidate / native rho64','Mean ratio','95% interval','Incomplete','Criterion']]
for arm in ['baseline-balanced','reserve-balanced','blocked-balanced','combined-balanced','baseline-native-balanced','combined-native-balanced','baseline-quarter','combined-native-quarter','combined-native-balanced-aa']:
 v=final['target_cluster_mean_ratios'][arm+'_over_native-tuned64/640']
 cells=[arm,f"{v['ratio']:.4f}",f"{v['bootstrap_q025']:.4f}–{v['bootstrap_q975']:.4f}",str(v['incomplete_target_round_pairs']),str(v['scoped_wall_parity_criterion'])]
 cirows.append(cells);md.append(f"| {arm} | {v['ratio']:.4f} | {v['bootstrap_q025']:.4f}–{v['bootstrap_q975']:.4f} | {v['bootstrap_q95']:.4f} | {v['incomplete_target_round_pairs']} | {v['scoped_wall_parity_criterion']} |")
frontier=json.loads((D/'results'/'observed-frontier.json').read_text())
md += ['', '### Primary arm against every rho build/policy','', '| Reference | Mean reference ms | Primary/reference speedup | Bootstrap95% speedup interval | Mean reference RSS KiB |','|---|---:|---:|---:|---:|']
for name in ['generic-tuned4','generic-tuned8','generic-tuned16','generic-tuned32','generic-tuned64','generic-tuned128','native-tuned4','native-tuned8','native-tuned16','native-tuned32','native-tuned64','native-tuned128']:
 v=frontier['comparisons']['hot']['640'][name]
 speed=f"{v['speedup_ratio_of_means']:.4f}" if v['both_complete'] else 'censored'
 interval=f"{v['bootstrap_speedup_q025']:.4f}–{v['bootstrap_speedup_q975']:.4f}" if v['both_complete'] else 'not a solve-time estimate'
 md.append(f"| {name} | {v['reference_mean_complete_wall_s']*1000:.4f} | {speed} | {interval} | {v['reference_mean_process_rss_kib']:.0f} |")
md += ['', 'Speedup is reference/primary ratio of means, calculated natively; greater than1 favors the primary. Same-code repeat and exact pair-coordinate counts remain in the final native summary. [pcc:unverified]','', '### Finite observed frontier','']
for point in frontier['observed_mean_frontiers']['hot']['640']:
 md.append(f"- {point['arm']}: mean complete {point['mean_complete_wall_s']*1000:.4f}ms, mean process-peak RSS {point['mean_process_peak_rss_kib']:.0f}KiB. [pcc:unverified]")
md += ['', 'Nondominance is computed natively over completing measured configurations, with A/A aliases excluded from frontier selection but retained in raw reports. This is a finite observed mean wall/RSS frontier, without uncertainty-adjusted dominance or proof that no further optimization exists.','']
md += ['', 'Ratios above1 indicate greater candidate cost. Intervals condition on the fixed curves and observed target population. With any censor, the finite-budget ratio is not an unconditional expected solve-time estimate. Median ratios, ratio of means and means of ratios answer different questions and are kept separate.','',
'## Complexity and prior art','',
'Sources checked (KB unavailable): repository wrapper exits127 and no KB MCP is exposed. The bounded fallback is the pinned [crypto/cryptanalysis assessment](../historical/repo_controls/repository_assessment.json), supplied Spaenlehauer paper arXiv:2610.32508v1 section8.1, retained quotient/GCD and pair controls, FLINT modular arithmetic, and current `src/cryptanalysis/prime_fast.rs`. Existing stride-start construction appears at lines1370 and3993. Progressions, Barrett reduction, batch inversion and flat hashing are established techniques; no new complexity bound or priority claim is made.','',
'[Complexity derivation](COMPLEXITY_NOTES.md): standard unbalanced meet-in-the-middle uses m≈r^(1/4), C≈sqrt(r), with nominal density m²C/r≈1 and square-root main work/storage under the stated sampling/hash assumptions. A second-moment argument treats correlated progressions and an explicit union bound treats rejected/exceptional points. This matches known generic time–memory balancing; it is not a new sub-square-root ECDLP result or a signed formal proof. [pcc:unverified]','',
'The current rho module and all configuration fields are retained with source/compatibility hashes. It is not a whole-current-root-crate build or proof that every external implementation was beaten. The final root Cargo attempt reaches the compiler and fails on pre-existing mismatched/unclosed delimiters in `src/cryptanalysis/binary_hyperelliptic_index_calculus.rs:640,1233`; its blob matches origin/main. No root unit tests execute. See `results/root-cargo-gate-final.json`. The unresolved broad-suite root-oracle mismatch is recorded in `results/current-rho-suite.json` and raw logs. No failing test is discarded or counted as passing.','',
'## Reproduce and custody','',
'Build `build.sh`, `build-target.sh`, progression/kernel siblings and `build-flat.sh` using the sealed FFI/static dependency cache specified in source. `build-current-rho.sh` uses the exact current module plus sealed older compatibility rlibs. Retained executables permit native replay; a source rebuild needs those external caches, whose hashes and limitations accompany the raw archive. New-output guards prevent overwriting runs.','',
'Run `qualify-flat.sh BUILD PUBLIC NEW_QUAL_DIR`, then `run-flat.sh BUILD PUBLIC CURRENT_RHO NEW_RUN_DIR`. Compile `analyze-flat.cpp` with the native affine translation object and FLINT; invoke `analyze-flat ROWS PUBLIC PRIVATE_VALIDATION SUMMARY`. Solvers never receive the private file. `render.py` performs layout only. Immutable per-stage source manifests and pre/post executable/dependency hashes accompany each run.','',
'All rows, warmups, misses, build errors, broad-test failures, input fixtures, qualifier binaries, source checkpoints and native analysis are retained under the authorized temporary local raw-custody exception. No S3 custody or signed result promotion is claimed. Reports and graphs are copied byte-for-byte into `benchmarks/20261123-trace-complete-cost`; canonical benchmark index and leaderboard rows remain unchanged pending admission.','',
'## Requirements remaining','',
'| Requested item | Status and evidence |','|---|---|',
'| Run and optimize complete labelled-pool recovery | Eight native matrices; complete cold adapters and retained controls; producer checks unsigned |',
'| Independently check fast trace, collisions and exceptional points | Prior retained [recurrence study](../recurrence_20261117/README.md); no change to its proof subjects |',
'| Improve implicit incidence GCD over existing controls | No improved asymptotic bound established; pair control remains separate from root speedup |',
'| Auxiliary curve/isogeny/twist end-to-end advantage | Prior toy paper routes retained; no new charged mixed-label transfer construction |',
'| Establish rho parity and global Pareto frontier | Recorded scoped criterion above; strongest whole-system reference/counting admission, cryptographic-size result and global proof remain open |',
'| Runpod execution | This continuation uses the already authorized local CPU/raw-custody exception; no new paid pod |',
'| Prime-field breakthrough / binary-field speedup | Not established by this bounded prime-field panel through31 bits; no new binary-field run |','']
fig,ax=plt.subplots(figsize=(8,4.5))
for key,g in final['groups'].items():
 arm=key.rsplit('/',1)[0]
 if arm.endswith('-aa'):continue
 rho=arm.startswith(('generic-','native-'))
 ax.scatter(g['complete_wall_s']['mean']*1000,g['peak_rss_kib']['mean']/1024,color='#677f9c' if rho else '#258679',s=30)
 for label in ['baseline-balanced','combined-native-balanced','native-tuned32']:
  if arm==label:ax.annotate(label,(g['complete_wall_s']['mean']*1000,g['peak_rss_kib']['mean']/1024),xytext=(4,6),textcoords='offset points',fontsize=8)
fpoints=sorted(frontier['observed_mean_frontiers']['hot']['640'],key=lambda x:x['mean_complete_wall_s'])
ax.plot([v['mean_complete_wall_s']*1000 for v in fpoints],[v['mean_process_peak_rss_kib']/1024 for v in fpoints],color='#677f9c',linestyle='--',label='Finite observed nondominated means')
ax.set_xlabel('Mean complete adapter wall time (ms)');ax.set_ylabel('Mean process-peak RSS (MiB)');ax.set_title('Final p31 panel: completing configurations; A/A alias excluded');ax.set_ylim(bottom=0);ax.legend(fontsize=8);fig.tight_layout();fig.savefig(O/'observed-frontier.svg');fig.savefig(O/'observed-frontier.png',dpi=150);plt.close(fig)
md += ['## Observed time and memory','', '![Finite observed time/RSS frontier](reports/observed-frontier.svg)','', 'The recorded nondominated set contains rho configurations. None of the tested IC configurations enters that finite observed mean frontier. This is a descriptive result for these inputs/builds/host, with no global optimality proof. [pcc:unverified]','']
scaling=all_data[6];construction=json.loads((D/'results'/'scaling-construction.json').read_text());primes={row['B']:row['p'] for row in construction['curves']}
fig,ax=plt.subplots(figsize=(8,4.8))
for arm in ['baseline-balanced','field-balanced','field-quarter','field-half','field-one','field-two','current-tuned8','current-tuned32','current-tuned64']:
 sizes=[128,256,512,640];ax.plot([primes[n] for n in sizes],[scaling['groups'][arm+'/'+str(n)]['complete_wall_s']['mean']*1000 for n in sizes],marker='o',label=arm)
ax.set_xscale('log');ax.set_yscale('log');ax.set_xlabel('Given field prime p (log scale)');ax.set_ylabel('Mean complete adapter wall time (ms, log scale)');ax.set_title('Run7 growing fields: selected fixed arms; full matrix in RUN7');ax.legend(fontsize=7,ncol=2);fig.tight_layout();fig.savefig(O/'growing-fields.svg');fig.savefig(O/'growing-fields.png',dpi=150);plt.close(fig)
md += ['## Growing-field measurements','', '![Growing fields](reports/growing-fields.svg)','', 'Fixed selected arms are plotted without a fitted exponent. Each point uses48 target clusters over three curves at that field and five timing rounds. All other rho policies and unbalanced shapes remain in RUN7 and its native summary; no per-target best arm or cross-stage pooling. [pcc:unverified]','']
(D/'README.md').write_text('\n'.join(md))
story=[Paragraph('Trace-incidence complete-cost results',styles['Title']),para('Eight successive native experiments · 23 November 2026 · unsigned producer data'),Spacer(1,10),para(headline),Spacer(1,10),para(method),Spacer(1,10),Image(str(O/'run8-b640.png'),width=470,height=470*max(3.3,21*.28+1.2)/8),Spacer(1,8),table(cirows,[150,65,105,70,65]),Spacer(1,10),para('Native target-cluster intervals resample within the three fixed curves. Criterion: no censors and one-sided95% upper mean-cost ratio at most1. Raw bounded costs include misses/censors. Other scales and all rho policies remain in the native summary. [pcc:unverified]'),PageBreak(),Paragraph('Construction, checks and limits',styles['Title']),para(notes[2]),Spacer(1,10),para(notes[3]),Spacer(1,10),para(notes[5]),Spacer(1,10),para(notes[6]),Spacer(1,10),para(notes[7]),Spacer(1,10),para(standing),Spacer(1,10),para('B8–640; largest field31 bits. APs are correlated and their actual labels and relation coverage are charged. Known structured PKM/isogeny factor bases do not supply these labels for free. Root-kernel improvements do not enter this pair-engine runtime.'),Spacer(1,10),para(root_gate),Spacer(1,10),para('Frozen protocols, exact sources, all eight native summaries and RUN1–RUN8 PDFs are linked from README. Failed builds, broad test mismatch, all warmups and censors remain in the raw archive. Current-module source rebuild needs the sealed older compatible crate cache. No S3 custody, canonical benchmark admission, global Pareto optimality or general ECDLP speedup is claimed.'),Spacer(1,10),para('Sources checked: supplied paper section8.1; pinned crypto/cryptanalysis quotient/GCD and pair controls; current prime_fast.rs stride starts/tuned rho; FLINT arithmetic. KB unavailable; standard engineering methods, no novelty claim.')]
story += [PageBreak(),Paragraph('Finite observed time/memory frontier',styles['Title']),Image(str(O/'observed-frontier.png'),width=470,height=264),para('Only rho configurations enter the finite observed mean-time/process-RSS frontier. The IC primary improves its matched baseline but remains slower and uses more process memory than the fastest tested rho configuration. This is descriptive nondominance, without uncertainty-adjusted dominance or proof that further optimization is impossible. [pcc:unverified]'),Spacer(1,10),Image(str(O/'growing-fields.png'),width=470,height=282),para('Run7 selected fixed arms across growing fields; no fitted exponent or per-target winning arm. Full controls, actual density, misses and memory are retained in native JSON and RUN7. [pcc:unverified]')]
SimpleDocTemplate(str(O/'RESULTS.pdf'),title='Trace-incidence complete-cost results',author='Provisional native benchmark study',topMargin=28,bottomMargin=28).build(story)
for p in O.iterdir():
 if p.suffix in ('.svg','.png','.pdf'):shutil.copyfile(p,B/p.name)
(B/'README.md').write_text('# Local complete-cost trace-incidence measurements\n\n[Full study](../../research/trace_incidence_20261116/setup_20261123/README.md) · [Combined PDF](RESULTS.pdf).\n\nEight separately frozen native panels test checked-context reuse, asymmetric pools, charged labelled progressions, exact arithmetic and flat-container kernels against retained rho controls. Fresh 480-target, growing-field192-target and fixedp31/native192-target confirmations include same-code repeats and native target-cluster intervals. These are unsigned local observations; no canonical timing admission or global Pareto claim. [pcc:unverified]\n\nByte-identical report and graph copies; raw custody and source-module compatibility limitations are explicit in the study. Canonical index remains unchanged pending normal admission.\n')
print(O)
